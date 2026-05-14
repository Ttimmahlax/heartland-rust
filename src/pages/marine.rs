use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{default_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn Marine() -> Element {
    rsx! {
        Seo {
            title: "Marine Applications",
            description: "Imperium-reinforced plastics for boat hulls, decking, dock structures, and aquaculture gear — corrosion-immune, lower carbon.",
            path: "/marine",
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
                    "Marine"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    span { class: "text-gradient-red", "Marine" }
                    " — Imperium where the saltwater stops glass fiber."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "For boat builders, dock fabricators, and aquaculture gear makers, Imperium delivers glass-fiber-class stiffness without the corrosion and lifecycle-impact penalties."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Boat decking and trim" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Imperium-PP and Imperium-HDPE deliver weather-stable surfaces for non-skid decking, swim platforms, and gunwales." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Lower carbon footprint than imported FRP options." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Dock and aquaculture" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Recyclable HDPE+Imperium pier deck boards replace pressure-treated lumber." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Aquaculture cages and pen substrates — corrosion-immune." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Marine engineering" }
            }
        }
    }
}
