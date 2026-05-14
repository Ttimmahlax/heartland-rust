use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{product_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn WhyImperium() -> Element {
    rsx! {
        Seo {
            title: "Why Imperium",
            description: "Imperium is Heartland's flagship hemp-based additive — lower cost than talc and calcium carbonate, lower carbon than glass fiber. Drop-in for existing compounding lines.",
            path: "/why-imperium",
        }

        Hero {}
        StatCounters { stats: product_stats() }
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
                    "The flagship additive"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    "Why "
                    span { class: "text-gradient-red", "Imperium" }
                    " out-performs talc, CaCO₃ and glass fiber"
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Imperium is engineered industrial hemp — purpose-built for compounders who need cost reduction AND emissions reduction in the same drop-in step."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Drop-in. Same equipment. Same throughput." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Imperium runs on the compounding lines you already own. No mill retrofits, no twin-screw rebuilds, no PHA-style cure-time penalties." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Pellets, masterbatch, and bulk supersack — pick the format that suits your downstream." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Cost lower than talc. Carbon lower than glass." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Imperium delivers a verified cost reduction vs. talc and calcium carbonate at typical 20–40% loading levels." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "On a per-pound basis, Imperium is also up to 90% lower carbon than glass fiber — and the LCA boundary includes farm to filler." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "US-farmed. No supply-chain surprises." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Every pound of Imperium starts on an American farm in Heartland's grower network across 11 states. No port risk, no tariff exposure, no quality drift from offshore consolidators." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Contract structure protects pricing from China-sourced filler volatility — critical in the post-2025 tariff environment." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "See pricing options" }
            }
        }
    }
}
