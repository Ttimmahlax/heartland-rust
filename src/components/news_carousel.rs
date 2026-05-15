//! Recent News strip — replaces the HubSpot "Quest recent post" slider.
//! Pulls top-N most recent articles by `published_at` from `content::recent_lang`.

use dioxus::prelude::*;

use crate::content;
use crate::Route;

#[derive(Props, Clone, PartialEq)]
pub struct NewsCarouselProps {
    #[props(default = 3)]
    pub count: usize,
    #[props(default = String::from("Recent News"))]
    pub heading: String,
    /// Active language. Defaults to English. When set to a non-English
    /// code, the carousel pulls the matching translated articles (falling
    /// back to English entries for slugs without a translation yet) and
    /// each card links to the lang-prefixed article URL.
    #[props(default = String::from("en"))]
    pub lang: String,
}

#[component]
pub fn NewsCarousel(props: NewsCarouselProps) -> Element {
    let articles = content::recent_lang(&props.lang, props.count);
    let lang = props.lang.clone();

    if articles.is_empty() {
        return rsx! {};
    }

    rsx! {
        section { class: "container-content py-16",
            div { class: "flex items-end justify-between mb-8",
                h2 { class: "text-3xl font-bold", "{props.heading}" }
                Link {
                    to: Route::News {},
                    class: "text-sm font-medium hover:text-[color:var(--color-accent-hover)]",
                    "All news →"
                }
            }
            div { class: "grid gap-6 md:grid-cols-3",
                for (i, article) in articles.into_iter().enumerate() {
                    NewsCard {
                        key: "{article.slug}",
                        index: i,
                        lang: lang.clone(),
                        slug: article.slug.clone(),
                        title: article.front.title.clone(),
                        excerpt: article.front.excerpt.clone(),
                        hero: article.hero_path(),
                        tags: article.front.tags.clone(),
                    }
                }
            }
        }
    }
}

#[component]
fn NewsCard(
    index: usize,
    lang: String,
    slug: String,
    title: String,
    excerpt: String,
    hero: String,
    tags: Vec<String>,
) -> Element {
    let card_class = "group block surface-glass overflow-hidden hover:translate-y-[-2px] transition-transform animate-fade-in-up";
    let card_style = format!("animation-delay: {}ms", index * 80);
    let body = rsx! {
        div { class: "aspect-[16/9] overflow-hidden bg-[color:var(--color-surface)]",
            img {
                src: "{hero}",
                alt: "{title}",
                loading: "lazy",
                class: "w-full h-full object-cover transition-transform duration-300 group-hover:scale-105",
            }
        }
        div { class: "p-5",
            if !tags.is_empty() {
                div { class: "flex flex-wrap gap-2 mb-2",
                    for tag in tags.iter().take(3) {
                        span {
                            key: "{tag}",
                            class: "text-xs px-2 py-0.5 rounded-full bg-[color:var(--color-accent-quiet)] text-[color:var(--color-accent)]",
                            "{tag}"
                        }
                    }
                }
            }
            h3 { class: "font-display font-semibold text-lg leading-snug group-hover:text-[color:var(--color-accent)]",
                "{title}"
            }
            if !excerpt.is_empty() {
                p { class: "mt-2 text-sm text-[color:var(--color-fg-muted)] line-clamp-3",
                    "{excerpt}"
                }
            }
        }
    };
    if lang == "en" {
        rsx! {
            Link {
                to: Route::Article { slug: slug.clone() },
                class: "{card_class}",
                style: "{card_style}",
                {body}
            }
        }
    } else {
        rsx! {
            Link {
                to: Route::LangArticle { lang: lang.clone(), slug: slug.clone() },
                class: "{card_class}",
                style: "{card_style}",
                {body}
            }
        }
    }
}
