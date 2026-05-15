use dioxus::prelude::*;

use crate::content;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn CategoryArchive(slug: String) -> Element {
    let articles = content::by_category(&slug);
    // Pretty-print category name from slug
    let pretty = title_case(&slug);
    let count = articles.len();
    let path = format!("/sustainability-news/category/{slug}");

    rsx! {
        Seo {
            title: "{pretty} — Articles",
            description: "Articles tagged in the {pretty} category — coverage of {pretty} from Heartland's sustainability library.",
            path: "{path}",
        }

        section { class: "bg-mesh-hero section-soft-bottom",
            div { class: "container-content py-20 md:py-24 text-center",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "Category"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-3xl mx-auto",
                    "{pretty}"
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)]",
                    if count == 1 { "1 article in this category." } else { "{count} articles in this category." }
                }
            }
        }

        section { class: "container-content py-16",
            if articles.is_empty() {
                div { class: "text-center text-[color:var(--color-fg-muted)] py-12",
                    "No articles in this category yet. "
                    Link { to: Route::News {}, class: "text-[color:var(--color-accent)] underline", "Browse all articles" }
                }
            } else {
                div { class: "grid gap-6 md:grid-cols-2 lg:grid-cols-3",
                    for (i, article) in articles.iter().enumerate() {
                        ArticleCard {
                            key: "{article.slug}",
                            index: i,
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
}

#[component]
pub fn TagArchive(slug: String) -> Element {
    let articles = content::by_tag(&slug);
    let pretty = title_case(&slug);
    let count = articles.len();
    let path = format!("/sustainability-news/tag/{slug}");

    rsx! {
        Seo {
            title: "{pretty} — Tagged Articles",
            description: "Articles tagged \"{pretty}\" from Heartland's library on industrial hemp, sustainable materials, and the supply chain.",
            path: "{path}",
        }

        section { class: "bg-mesh-hero section-soft-bottom",
            div { class: "container-content py-20 md:py-24 text-center",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "Tag"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-3xl mx-auto",
                    "#{pretty}"
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)]",
                    if count == 1 { "1 article tagged with this." } else { "{count} articles tagged with this." }
                }
            }
        }

        section { class: "container-content py-16",
            if articles.is_empty() {
                div { class: "text-center text-[color:var(--color-fg-muted)] py-12",
                    "No articles tagged with that yet. "
                    Link { to: Route::News {}, class: "text-[color:var(--color-accent)] underline", "Browse all articles" }
                }
            } else {
                div { class: "grid gap-6 md:grid-cols-2 lg:grid-cols-3",
                    for (i, article) in articles.iter().enumerate() {
                        ArticleCard {
                            key: "{article.slug}",
                            index: i,
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
}

fn title_case(slug: &str) -> String {
    slug.split('-')
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[component]
fn ArticleCard(
    index: usize,
    slug: String,
    title: String,
    excerpt: String,
    hero: String,
    tags: Vec<String>,
) -> Element {
    rsx! {
        Link {
            to: Route::Article { slug: slug.clone() },
            class: "group block surface-glass overflow-hidden hover:translate-y-[-2px] transition-transform animate-fade-in-up",
            style: "animation-delay: {index * 30}ms",
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
                h2 { class: "font-display font-semibold text-lg leading-snug group-hover:text-[color:var(--color-accent)]",
                    "{title}"
                }
                if !excerpt.is_empty() {
                    p { class: "mt-2 text-sm text-[color:var(--color-fg-muted)] line-clamp-3",
                        "{excerpt}"
                    }
                }
            }
        }
    }
}
