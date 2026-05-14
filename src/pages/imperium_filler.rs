use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{product_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn ImperiumFiller() -> Element {
    rsx! {
        Seo {
            title: "Imperium Filler",
            description: "Imperium Filler — dry milled hemp filler shipped in supersack or bulk, ready for direct addition on your existing compounding lines.",
            path: "/imperium-filler",
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
                    "Dry filler"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    span { class: "text-gradient-red", "Imperium Filler" }
                    " — drop-in for talc and calcium carbonate."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "The most direct way to lower cost AND emissions in your existing plastic compound: swap your current mineral filler for Imperium."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Cost reduction vs. talc" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "At equivalent loading, Imperium typically lands at or below talc on a per-pound basis. Your sourcing team will see savings on the invoice before they see them on the dock." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Hemp is also de-coupled from talc and CaCO₃ supply-chain shocks (mining bans, tariff exposure, etc.)." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Carbon reduction vs. glass fiber" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Where glass fiber typically contributes 2–3 kg CO₂e/kg, Imperium contributes net-negative carbon thanks to the upstream photosynthesis credit on the LCA." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Verified through the Imperium Farming LCA — see /lca for the full disclosure pack." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Specification" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Available particle sizes from coarse mill to micronized." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Bulk density and moisture engineered for stable feeder behavior at typical extruder feed throats." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Bag, supersack, or pneumatic bulk delivery." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Request a TDS" }
            }
        }
    }
}
