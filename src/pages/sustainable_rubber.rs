use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{default_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn SustainableRubber() -> Element {
    rsx! {
        Seo {
            title: "Sustainable Rubber Additives",
            description: "Hemp-fiber and hemp-meal additives for rubber compounds — partial replacement of carbon black with verified emissions reduction.",
            path: "/sustainable-rubber-additives",
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
                    "Rubber"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    "Sustainable "
                    span { class: "text-gradient-red", "Rubber" }
                    " additives — partial carbon-black replacement."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Carbon-black is the single largest emissions hotspot in tire and conveyor-belt rubber. Heartland's hemp-meal additives partially displace it while preserving mechanical performance."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Where it lands" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Industrial conveyor belt rubber." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Lower-spec consumer tire compounds." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Floor tile / matting rubber." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Vibration-damping mounts." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Partial replacement, not full" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "At 5–15% replacement of carbon black, the mechanical property envelope is preserved." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Higher replacement levels require recipe re-engineering — engage Heartland's compounding team." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Request a rubber sample" }
            }
        }
    }
}
