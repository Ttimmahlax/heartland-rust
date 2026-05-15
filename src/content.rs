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

fn parse_one(slug: &str, source: &str) -> Option<Article> {
    let trimmed = source.trim_start_matches('\u{feff}');
    let trimmed = trimmed.strip_prefix("+++\n").or_else(|| trimmed.strip_prefix("+++\r\n"))?;
    let end_idx = trimmed.find("\n+++")?;
    let (front_str, rest) = trimmed.split_at(end_idx);
    let body = rest.trim_start_matches("\n+++").trim_start_matches('\n').trim_start_matches('\r').to_string();

    let front: FrontMatter = match toml::from_str(front_str) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("article {slug} front matter error: {e}");
            return None;
        }
    };

    Some(Article {
        slug: slug.to_string(),
        front,
        body,
    })
}

fn parsed() -> &'static Vec<Article> {
    static CELL: OnceLock<Vec<Article>> = OnceLock::new();
    CELL.get_or_init(|| {
        let mut v: Vec<Article> = ARTICLES
            .iter()
            .filter_map(|(slug, src)| parse_one(slug, src))
            .collect();
        v.sort_by(|a, b| b.front.published_at.cmp(&a.front.published_at));
        v
    })
}

pub fn all() -> &'static [Article] {
    parsed().as_slice()
}

#[allow(dead_code)]
pub fn all_slugs() -> Vec<String> {
    ARTICLES.iter().map(|(slug, _)| slug.to_string()).collect()
}

pub fn find(slug: &str) -> Option<&'static Article> {
    parsed().iter().find(|a| a.slug == slug)
}

pub fn recent(n: usize) -> Vec<&'static Article> {
    parsed().iter().take(n).collect()
}

/// All articles assigned to a given category slug. Used by the category
/// archive routes at `/sustainability-news/category/:slug`.
pub fn by_category(slug: &str) -> Vec<&'static Article> {
    parsed()
        .iter()
        .filter(|a| a.front.categories.iter().any(|c| c == slug))
        .collect()
}

/// All articles assigned to a given tag slug. Used by the tag archive
/// routes at `/sustainability-news/tag/:slug`.
pub fn by_tag(slug: &str) -> Vec<&'static Article> {
    parsed()
        .iter()
        .filter(|a| a.front.tags.iter().any(|t| t == slug))
        .collect()
}

/// Unique sorted list of every category slug that has at least one article.
/// Used by the prerender + sitemap generators.
pub fn all_categories() -> Vec<String> {
    let mut s: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for a in parsed().iter() {
        for c in &a.front.categories {
            s.insert(c.clone());
        }
    }
    s.into_iter().collect()
}

/// Unique sorted list of every tag slug that has at least one article.
pub fn all_tags() -> Vec<String> {
    let mut s: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for a in parsed().iter() {
        for t in &a.front.tags {
            s.insert(t.clone());
        }
    }
    s.into_iter().collect()
}
