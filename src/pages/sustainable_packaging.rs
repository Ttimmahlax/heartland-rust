use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{default_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn SustainablePackaging() -> Element {
    rsx! {
        Seo {
            title: "Sustainable Packaging",
            description: "Carbon-neutral pallets, returnable bins, industrial crates, and consumer-facing packaging — Imperium-inside formulations available at every standard pallet footprint.",
            path: "/sustainable-packaging",
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
                    "Pallets & bins"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    "Sustainable "
                    span { class: "text-gradient-red", "Packaging" }
                    " — pallets, bins, crates."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Heartland partners with packaging molders to deliver Imperium-inside HDPE pallets, bins, and returnable crates in every common footprint."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Carbon-neutral pallet sizes available" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "48×40 nest US5 / US8 — the GMA standard footprint." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "48×45 A5 OD9F, 47.2×47.2 S5 OD9F, 45×45 C5.2 OD9F, 43.3×43.3 C5 OD9F." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "47.2×31.5 E5 OD9F — European half-footprint." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "The Green Packaging Initiative" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Multi-brand commitment to swap fossil-derived packaging for Imperium-inside HDPE in industrial returnable applications." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "See /green-packaging-initiative for the partner list and the supply-chain map." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Consumer-facing packaging" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "For consumer-facing applications, Imperium can be paired with mass-balance certified PCR HDPE for a 'Imperium + PCR' co-marketing story." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Spec a pallet" }
            }
        }
    }
}
