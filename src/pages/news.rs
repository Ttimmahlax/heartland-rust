use dioxus::prelude::*;

use crate::content;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn News() -> Element {
    let articles = content::all();
    let mut query = use_signal(|| String::new());

    let q = query();
    let q_lc = q.to_lowercase();
    let filtered: Vec<&content::Article> = if q_lc.is_empty() {
        articles.iter().collect()
    } else {
        articles
            .iter()
            .filter(|a| {
                a.front.title.to_lowercase().contains(&q_lc)
                    || a.front.excerpt.to_lowercase().contains(&q_lc)
                    || a.slug.to_lowercase().contains(&q_lc)
                    || a.front.tags.iter().any(|t| t.to_lowercase().contains(&q_lc))
            })
            .collect()
    };
    let total = articles.len();
    let shown = filtered.len();

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

        section { class: "container-content pt-8 md:pt-12",
            div { class: "max-w-2xl mx-auto",
                label { class: "block",
                    span { class: "sr-only", "Search articles" }
                    div { class: "relative",
                        // Magnifier icon
                        span { class: "pointer-events-none absolute left-4 top-1/2 -translate-y-1/2 text-[color:var(--color-fg-muted)]",
                            svg { width: "18", height: "18", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                circle { cx: "11", cy: "11", r: "8" }
                                line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                            }
                        }
                        input {
                            r#type: "search",
                            placeholder: "Search articles by title, topic, or keyword…",
                            value: "{q}",
                            oninput: move |e| query.set(e.value()),
                            aria_label: "Search articles",
                            autocomplete: "off",
                            spellcheck: "false",
                            class: "w-full pl-12 pr-4 py-3 rounded-full border border-[color:var(--color-border)] bg-[color:var(--color-surface)] text-[color:var(--color-fg)] placeholder:text-[color:var(--color-fg-muted)] focus:outline-none focus:border-[color:var(--color-accent)] focus:ring-2 focus:ring-[color:var(--color-accent-quiet)] transition",
                        }
                        if !q.is_empty() {
                            button {
                                r#type: "button",
                                class: "absolute right-3 top-1/2 -translate-y-1/2 inline-flex items-center justify-center w-7 h-7 rounded-full hover:bg-[color:var(--color-accent-quiet)] text-[color:var(--color-fg-muted)] hover:text-[color:var(--color-accent)]",
                                aria_label: "Clear search",
                                onclick: move |_| query.set(String::new()),
                                svg { width: "14", height: "14", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round",
                                    line { x1: "18", y1: "6", x2: "6", y2: "18" }
                                    line { x1: "6", y1: "6", x2: "18", y2: "18" }
                                }
                            }
                        }
                    }
                }
                div { class: "mt-3 text-center text-sm text-[color:var(--color-fg-muted)]",
                    if q.is_empty() {
                        "{total} articles"
                    } else if shown == 0 {
                        "No articles match \"{q}\". Try a different keyword."
                    } else if shown == 1 {
                        "1 match for \"{q}\""
                    } else {
                        "{shown} matches for \"{q}\""
                    }
                }
            }
        }

        section { class: "container-content pb-16 pt-8",
            if filtered.is_empty() && q.is_empty() {
                div { class: "text-center text-[color:var(--color-fg-muted)] py-12",
                    "Articles are being migrated from heartland.io. Check back soon."
                }
            } else if filtered.is_empty() {
                div { class: "text-center text-[color:var(--color-fg-muted)] py-12",
                    "No articles match your search. "
                    button {
                        r#type: "button",
                        class: "text-[color:var(--color-accent)] underline",
                        onclick: move |_| query.set(String::new()),
                        "Clear filter"
                    }
                }
            } else {
                div { class: "grid gap-6 md:grid-cols-2 lg:grid-cols-3",
                    for (i, article) in filtered.iter().enumerate() {
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
