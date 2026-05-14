use dioxus::prelude::*;

use crate::content;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn News() -> Element {
    let articles = content::all();

    rsx! {
        Seo {
            title: "Sustainability News & Articles",
            description: "Heartland's library of articles on industrial hemp, sustainable plastics, regenerative agriculture, supply-chain decarbonization, and the future of material innovation.",
            path: "/sustainability-news",
        }

        section { class: "bg-mesh-hero section-soft-bottom",
            div { class: "container-content py-20 md:py-24 text-center",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "Library"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-3xl mx-auto",
                    "Sustainability "
                    span { class: "text-gradient-red", "News & Articles" }
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)]",
                    "Long-form coverage of how Imperium reshapes plastic compounding, automotive supply chains, regenerative agronomy, and industrial decarbonization."
                }
            }
        }

        section { class: "container-content py-16",
            if articles.is_empty() {
                div { class: "text-center text-[color:var(--color-fg-muted)] py-12",
                    "Articles are being migrated from heartland.io. Check back soon."
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
                            published: article.front.published_at.clone(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ArticleCard(
    index: usize,
    slug: String,
    title: String,
    excerpt: String,
    hero: String,
    tags: Vec<String>,
    published: String,
) -> Element {
    rsx! {
        Link {
            to: Route::Article { slug: slug.clone() },
            class: "group block surface-glass overflow-hidden hover:translate-y-[-2px] transition-transform animate-fade-in-up",
            style: "animation-delay: {index * 40}ms",
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
                div { class: "mt-3 text-xs text-[color:var(--color-fg-muted)]", "{published}" }
            }
        }
    }
}
