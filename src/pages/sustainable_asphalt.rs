use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{default_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn SustainableAsphalt() -> Element {
    rsx! {
        Seo {
            title: "Sustainable Asphalt Additives",
            description: "Hemp-pulp additives for hot-mix asphalt — lower binder demand, improved cold-temperature performance, reduced thermal cracking.",
            path: "/sustainable-asphalt-additives",
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
                    "Asphalt"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    "Sustainable "
                    span { class: "text-gradient-red", "Asphalt" }
                    " — bio-based modifier for hot-mix."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Heartland's hemp-pulp asphalt modifier reduces binder demand and improves cold-temperature behavior in road and parking-lot applications."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Where it slots in" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Drop-in at the hot-mix plant prior to aggregate addition." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Compatible with standard mix designs in Superpave + Marshall families." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Performance impact" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Lower thermal cracking susceptibility — particularly relevant in northern climates." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Up to ~3% binder reduction at parity rut/fatigue performance." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Talk to a paving team" }
            }
        }
    }
}
