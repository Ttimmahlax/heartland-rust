use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{default_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn Government() -> Element {
    rsx! {
        Seo {
            title: "Government Programs",
            description: "Heartland is an active USDA grant recipient (Hemp4Soil) and engages with state-level industrial-hemp programs, MBDA, and federal procurement.",
            path: "/government",
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
                    "Public sector"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    span { class: "text-gradient-red", "Government Programs" }
                    " — USDA, MBDA, state DOTs."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Heartland engages with USDA, MBDA, state Departments of Agriculture and DOT procurement to scale Imperium into federally-aligned material specifications."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "USDA — Hemp4Soil" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Heartland was awarded a $360,000 USDA grant to study industrial hemp's soil-health impact in rotation with corn and soybean." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Results presented at the Soil & Water Conservation Society." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "MBDA + KDM partnership" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Active program with the Minority Business Development Agency and KDM Bio Materials to expand minority-owned hemp farming in the US." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "State DOT pilots" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Hemp-asphalt and hemp-concrete pilots active with multiple state Departments of Transportation." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Engage with public programs" }
            }
        }
    }
}
