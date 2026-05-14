use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{default_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn SustainableBuilding() -> Element {
    rsx! {
        Seo {
            title: "Sustainable Building Materials",
            description: "Imperium-filled construction materials: hemp-PP composite decking, hempcrete blocks, hemp-reinforced WPC, and hemp-additive concrete.",
            path: "/sustainable-building-materials",
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
                    "Construction"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    span { class: "text-gradient-red", "Sustainable Building" }
                    " materials, made domestically."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Imperium ports into building products as filler, fiber, or structural reinforcement — across decking, panels, blocks, concrete, asphalt, and acoustic boards."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Hemp-PP composite decking" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Imperium-filled wood-plastic composite (WPC) decking with a verified-lower carbon footprint vs. talc-filled or virgin WPC." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Lower water absorption and dimensional stability competitive with hardwood alternatives." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Hemp-additive concrete" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "See /sustainable-concrete-additives — hemp hurd and engineered hemp fiber reduce crack propagation and carbon intensity in mid-strength concretes." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Hemp-additive asphalt" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "See /sustainable-asphalt-additives — hemp pulp lowers binder content and improves cold-temperature crack behavior." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Engage the construction team" }
            }
        }
    }
}
