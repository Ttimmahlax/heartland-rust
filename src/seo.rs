//! Per-page SEO injection — title, meta, OG, canonical, JSON-LD.

use dioxus::prelude::*;

use crate::{SITE_BASE, SITE_NAME};

pub const DEFAULT_OG_IMAGE_PATH: &str = "/assets/brand/icon-512.png";
pub const DEFAULT_OG_IMAGE_WIDTH: &str = "512";
pub const DEFAULT_OG_IMAGE_HEIGHT: &str = "512";
pub const DEFAULT_OG_IMAGE_ALT: &str =
    "Heartland Industries — high performance, cost-reducing, carbon-negative materials.";
pub const OG_LOCALE: &str = "en_US";

#[derive(Props, Clone, PartialEq)]
pub struct SeoProps {
    pub title: String,
    pub description: String,
    pub path: String,
    #[props(default = String::new())]
    pub image: String,
    /// Image pixel width as a string. Defaults to the brand-icon size (512);
    /// article pages override to their hero dimensions (1210×786 standard).
    #[props(default = String::from(DEFAULT_OG_IMAGE_WIDTH))]
    pub image_width: String,
    #[props(default = String::from(DEFAULT_OG_IMAGE_HEIGHT))]
    pub image_height: String,
    /// Image alt text. Defaults to the brand alt; article pages should pass
    /// the per-image `hero_alt` so screen readers + social previews are accurate.
    #[props(default = String::from(DEFAULT_OG_IMAGE_ALT))]
    pub image_alt: String,
    #[props(default = String::from("website"))]
    pub og_type: String,
}

pub fn canonical(path: &str) -> String {
    if path.starts_with("http") {
        path.to_string()
    } else {
        format!("{SITE_BASE}{}", path.trim_end_matches('/'))
    }
}

/// Emit `<link rel="alternate" hreflang="..." href="...">` tags for every
/// language that has a real translation of `english_path`. Always includes
/// an `x-default` pointing at the English version.
///
/// `english_path` is the bare English route (no language prefix), e.g.
/// `/sustainability-news/heartland-raises-seed-capital`.
#[component]
pub fn HreflangAlternates(english_path: String, available_langs: Vec<String>) -> Element {
    let english_url = canonical(&english_path);
    let trimmed = english_path.trim_end_matches('/').to_string();
    rsx! {
        // x-default → English (Google's recommended convention).
        document::Link {
            rel: "alternate",
            hreflang: "x-default",
            href: "{english_url}",
        }
        for lang in available_langs.iter() {
            {
                let href = if lang == "en" {
                    english_url.clone()
                } else {
                    format!("{SITE_BASE}/{}{}", lang, trimmed)
                };
                let lang_attr = lang.clone();
                rsx! {
                    document::Link {
                        key: "{lang_attr}",
                        rel: "alternate",
                        hreflang: "{lang_attr}",
                        href: "{href}",
                    }
                }
            }
        }
    }
}

#[component]
pub fn Seo(props: SeoProps) -> Element {
    let canonical_url = canonical(&props.path);
    let image = if props.image.is_empty() {
        format!("{SITE_BASE}{DEFAULT_OG_IMAGE_PATH}")
    } else if props.image.starts_with("http") {
        props.image.clone()
    } else {
        format!("{SITE_BASE}{}", props.image)
    };
    let title = if props.title == SITE_NAME || props.title.is_empty() {
        SITE_NAME.to_string()
    } else {
        format!("{} | {SITE_NAME}", props.title)
    };

    rsx! {
        document::Title { "{title}" }
        document::Meta { name: "description", content: "{props.description}" }
        document::Link { rel: "canonical", href: "{canonical_url}" }
        document::Meta { property: "og:type", content: "{props.og_type}" }
        document::Meta { property: "og:site_name", content: "{SITE_NAME}" }
        document::Meta { property: "og:locale", content: OG_LOCALE }
        document::Meta { property: "og:url", content: "{canonical_url}" }
        document::Meta { property: "og:title", content: "{props.title}" }
        document::Meta { property: "og:description", content: "{props.description}" }
        document::Meta { property: "og:image", content: "{image}" }
        document::Meta { property: "og:image:width", content: "{props.image_width}" }
        document::Meta { property: "og:image:height", content: "{props.image_height}" }
        document::Meta { property: "og:image:alt", content: "{props.image_alt}" }
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: "{props.title}" }
        document::Meta { name: "twitter:description", content: "{props.description}" }
        document::Meta { name: "twitter:image", content: "{image}" }
        document::Meta { name: "twitter:image:alt", content: "{props.image_alt}" }
    }
}

pub fn organization_jsonld() -> String {
    format!(
        r#"{{"@context":"https://schema.org","@type":"Organization","name":"{SITE_NAME}","url":"{SITE_BASE}","logo":"{SITE_BASE}/assets/brand/heartland-logo-light.png","sameAs":["https://www.facebook.com/therealheartland/","https://www.linkedin.com/company/therealheartland","https://twitter.com/HeartlandXL","https://www.instagram.com/heartlandmaterials","https://www.youtube.com/channel/UCw3n3hnQX8PqgG-QIDb4BjA"],"description":"Heartland is a material science company helping manufacturers exceed their cost-reduction goals while reducing their emissions."}}"#
    )
}

pub fn article_jsonld(
    title: &str,
    description: &str,
    path: &str,
    image: &str,
    published_at: &str,
    author: &str,
) -> String {
    let url = canonical(path);
    let img = if image.starts_with("http") {
        image.to_string()
    } else {
        format!("{SITE_BASE}{}", image)
    };
    format!(
        r#"{{"@context":"https://schema.org","@type":"Article","headline":{title},"description":{desc},"image":[{image:?}],"datePublished":{date},"author":{{"@type":"Person","name":{author}}},"publisher":{{"@type":"Organization","name":"{site}","logo":{{"@type":"ImageObject","url":"{base}/assets/brand/heartland-logo-light.png"}}}},"mainEntityOfPage":{{"@type":"WebPage","@id":"{url}"}}}}"#,
        title = json_escape(title),
        desc = json_escape(description),
        image = img,
        date = json_escape(published_at),
        author = json_escape(author),
        site = SITE_NAME,
        base = SITE_BASE,
        url = url,
    )
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
