use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{default_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn ImperiumAnimalFeed() -> Element {
    rsx! {
        Seo {
            title: "Imperium Animal Feed",
            description: "Hemp-derived feed ingredients for pork, cattle, chicken, and other livestock — high-protein, high-fatty-acid, USDA-compatible.",
            path: "/imperium-animal-feed",
        }

        Hero {}
        StatCounters { stats: default_stats() }
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
                    "Livestock nutrition"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    "Imperium "
                    span { class: "text-gradient-red", "Animal Feed" }
                    " — pork, cattle, chicken."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Heartland processes hemp seed and seed meal into protein-dense, fatty-acid-rich feed ingredients for pork, cattle, poultry, and aquaculture operations."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "High-protein, high-omega" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Hemp seed meal lands around 30–35% crude protein and a favorable omega-3:omega-6 ratio." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Use as a soybean-meal supplement or replacement, depending on the species and the formulation goal." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Compatible with existing rations" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Drop-in compatible with TMR mixing, pelleting, and extrusion-pellet feed processes." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Storage and handling similar to soybean meal." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Sustainable upstream" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Hemp grows with less water and less pesticide than soybean." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Hemp also fits into rotation slots that pull moisture and nutrients from the soil differently — supporting better long-term agronomy." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Talk to a feed specialist" }
            }
        }
    }
}
