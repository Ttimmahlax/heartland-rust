//! Compile-time-embedded markdown article registry.
//!
//! `build.rs` enumerates `content/articles/*.md` and emits a manifest of
//! `(slug, &str)` entries that we parse on first read.

use serde::Deserialize;
use std::sync::OnceLock;

include!(concat!(env!("OUT_DIR"), "/articles_manifest.rs"));

#[derive(Debug, Clone, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub excerpt: String,
    pub hero_image: String,
    pub hero_alt: String,
    pub published_at: String,
    #[serde(default = "default_author")]
    pub author: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub seo_title: Option<String>,
    #[serde(default)]
    pub seo_description: Option<String>,
}

fn default_author() -> String {
    "Heartland Industries".to_string()
}

#[derive(Debug, Clone)]
pub struct Article {
    pub slug: String,
    /// BCP-47 language code matching `content/articles/<lang>/*.md`. Top-level
    /// English articles have `lang = "en"`.
    pub lang: String,
    pub front: FrontMatter,
    pub body: String,
}

impl Article {
    pub fn url_path(&self) -> String {
        format!("/sustainability-news/{}", self.slug)
    }

    pub fn hero_path(&self) -> String {
        format!("/assets/articles/{}/{}", self.slug, self.front.hero_image)
    }

    pub fn seo_title(&self) -> String {
        self.front.seo_title.clone().unwrap_or_else(|| self.front.title.clone())
    }

    pub fn seo_description(&self) -> String {
        self.front
            .seo_description
            .clone()
            .unwrap_or_else(|| self.front.excerpt.clone())
    }
}

fn parse_one(slug: &str, lang: &str, source: &str) -> Option<Article> {
    let trimmed = source.trim_start_matches('\u{feff}');
    let trimmed = trimmed.strip_prefix("+++\n").or_else(|| trimmed.strip_prefix("+++\r\n"))?;
    let end_idx = trimmed.find("\n+++")?;
    let (front_str, rest) = trimmed.split_at(end_idx);
    let body = rest.trim_start_matches("\n+++").trim_start_matches('\n').trim_start_matches('\r').to_string();

    let front: FrontMatter = match toml::from_str(front_str) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("article {slug} ({lang}) front matter error: {e}");
            return None;
        }
    };

    Some(Article {
        slug: slug.to_string(),
        lang: lang.to_string(),
        front,
        body,
    })
}

fn parsed() -> &'static Vec<Article> {
    static CELL: OnceLock<Vec<Article>> = OnceLock::new();
    CELL.get_or_init(|| {
        let mut v: Vec<Article> = ARTICLES
            .iter()
            .filter_map(|(slug, lang, src)| parse_one(slug, lang, src))
            .collect();
        // Sort by date desc within each language. Language ordering doesn't
        // matter for `find()` lookups; only the date order matters for `recent()`.
        v.sort_by(|a, b| b.front.published_at.cmp(&a.front.published_at));
        v
    })
}

pub fn all() -> &'static [Article] {
    parsed().as_slice()
}

#[allow(dead_code)]
pub fn all_slugs() -> Vec<String> {
    // Unique slugs across all languages (English alone gives the same set).
    let mut seen = std::collections::BTreeSet::new();
    for (slug, _, _) in ARTICLES.iter() {
        seen.insert(slug.to_string());
    }
    seen.into_iter().collect()
}

/// English article by slug. Kept for back-compat with English-only callers.
pub fn find(slug: &str) -> Option<&'static Article> {
    find_lang("en", slug)
}

/// Languages that have a real translation for this slug (always includes
/// "en"). Useful for emitting hreflang tags — listing langs without a
/// dedicated translation creates duplicate-content signals at Google.
pub fn translations_for(slug: &str) -> Vec<&'static str> {
    let mut out: Vec<&'static str> = parsed()
        .iter()
        .filter(|a| a.slug == slug)
        .map(|a| a.lang.as_str())
        .collect();
    out.sort();
    out.dedup();
    out
}

/// Article in the requested language, with fallback to English when no
/// translation exists for that slug.
pub fn find_lang(lang: &str, slug: &str) -> Option<&'static Article> {
    parsed()
        .iter()
        .find(|a| a.slug == slug && a.lang == lang)
        .or_else(|| parsed().iter().find(|a| a.slug == slug && a.lang == "en"))
}

pub fn recent(n: usize) -> Vec<&'static Article> {
    recent_lang("en", n)
}

/// Top-N most recent articles filtered to the requested language.
/// Falls back to English entries if there aren't enough translations
/// for that language (so a carousel on a Spanish page can still fill 4
/// slots even before every article has a Spanish translation).
pub fn recent_lang(lang: &str, n: usize) -> Vec<&'static Article> {
    let same_lang: Vec<&'static Article> =
        parsed().iter().filter(|a| a.lang == lang).take(n).collect();
    if same_lang.len() >= n || lang == "en" {
        return same_lang;
    }
    // Backfill with English entries for slugs we haven't translated yet.
    let seen: std::collections::HashSet<&str> =
        same_lang.iter().map(|a| a.slug.as_str()).collect();
    let mut out = same_lang;
    for a in parsed().iter().filter(|a| a.lang == "en") {
        if out.len() >= n {
            break;
        }
        if !seen.contains(a.slug.as_str()) {
            out.push(a);
        }
    }
    out
}

/// All English articles assigned to a given category slug. Used by the
/// category archive routes at `/sustainability-news/category/:slug`.
/// Translations share the same category slugs as their English source, so we
/// filter to one language to avoid duplicate listings.
pub fn by_category(slug: &str) -> Vec<&'static Article> {
    parsed()
        .iter()
        .filter(|a| a.lang == "en" && a.front.categories.iter().any(|c| c == slug))
        .collect()
}

/// All English articles assigned to a given tag slug. Used by the tag
/// archive routes at `/sustainability-news/tag/:slug`.
pub fn by_tag(slug: &str) -> Vec<&'static Article> {
    parsed()
        .iter()
        .filter(|a| a.lang == "en" && a.front.tags.iter().any(|t| t == slug))
        .collect()
}

/// Unique sorted list of every category slug that has at least one article.
/// Used by the prerender + sitemap generators.
pub fn all_categories() -> Vec<String> {
    let mut s: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for a in parsed().iter().filter(|a| a.lang == "en") {
        for c in &a.front.categories {
            s.insert(c.clone());
        }
    }
    s.into_iter().collect()
}

/// Unique sorted list of every tag slug that has at least one article.
pub fn all_tags() -> Vec<String> {
    let mut s: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for a in parsed().iter().filter(|a| a.lang == "en") {
        for t in &a.front.tags {
            s.insert(t.clone());
        }
    }
    s.into_iter().collect()
}
