use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn Ebooks() -> Element {
    rsx! {
        Seo {
            title: "E-Books",
            description: "Downloadable e-books on industrial hemp, sustainable materials, regenerative agriculture, and the future of manufacturing.",
            path: "/e-books",
        }

        Hero {}
        Sections {}
        LogoCarousel { heading: "As Seen In" }
        ClosingCta {}
        NewsCarousel { heading: "Related Articles" }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section { class: "bg-mesh-hero section-soft-bottom",
            div { class: "container-content py-20 md:py-28 text-center",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4 animate-fade-in",
                    "Library"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    span { class: "text-gradient-red", "E-Books" }
                    " from Heartland."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Long-form guides covering hemp's role in manufacturing, regenerative agriculture, plastics of the future, carbon-negative materials, and the supply-chain decisions ahead."
                }
            }
        }
    }
}

#[component]
fn Sections() -> Element {
    rsx! {
        section { class: "container-content py-16",
            div { class: "grid gap-8 md:grid-cols-2",
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Featured e-books" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "The Ultimate Guide to Plastics of the Future — a 12-part series exploring every angle of the plastic decarbonization question." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Engineering Earth — Rethinking Agriculture After 100 Years of Innovation." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "The Manufacturers Guide to Sustainable Materials." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Hemp Fiber Production: A Roadmap for US Farmers." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Where to find each" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Each title is embedded as a long-form article under /sustainability-news. The PDF download path is being migrated." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Browse all of them in the /sustainability-news index." }
                    }
            }
        }
    }
}

#[component]
fn ClosingCta() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 my-12 section-soft-edges",
            div { class: "container-content text-center",
                h2 { class: "text-3xl md:text-4xl font-bold mb-6 max-w-2xl mx-auto",
                    "Ready to dig in?"
                }
                Link { to: Route::News {}, class: "btn-accent-gradient", "Browse all articles" }
            }
        }
    }
}
