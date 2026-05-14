use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{farm_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn EngineeringEarth() -> Element {
    rsx! {
        Seo {
            title: "Engineering Earth",
            description: "Heartland's regenerative agronomy program — rethinking 100 years of agriculture innovation around carbon sequestration, soil health, and farmer profitability.",
            path: "/engineering-earth",
        }

        Hero {}
        StatCounters { stats: farm_stats() }
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
                    "Agronomy program"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    span { class: "text-gradient-red", "Engineering Earth" }
                    " — regenerative ag, by the numbers."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Engineering Earth is Heartland's regenerative agronomy program — a framework for re-thinking 100 years of conventional agriculture around soil health, carbon sequestration, and farmer profitability."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "What it means in practice" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Industrial hemp in rotation with corn and soybean." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Cover-crop integration on Imperium-contracted acres." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Soil health measurement on a per-acre, per-season basis." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Carbon sequestration verified by third-party (see Carbon Report)." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Why it matters for materials" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Every pound of Imperium ships with a verified upstream carbon credit." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Manufacturers using Imperium can claim Scope 3 reductions backed by a defensible chain of custody." }
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
                Link { to: Route::News {}, class: "btn-accent-gradient", "Read the methodology" }
            }
        }
    }
}
