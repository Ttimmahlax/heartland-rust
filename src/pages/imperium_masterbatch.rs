use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{product_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn ImperiumMasterbatch() -> Element {
    rsx! {
        Seo {
            title: "Imperium Masterbatch",
            description: "Imperium Masterbatch — pre-dispersed hemp filler concentrate, ready for direct let-down into polypropylene, polyethylene, PVC, and engineered resins.",
            path: "/imperium-masterbatch",
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
                    "Concentrate format"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    "Imperium "
                    span { class: "text-gradient-red", "Masterbatch" }
                    " — high-loading concentrate for plastic compounders"
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "A 60–70% loaded Imperium concentrate engineered for direct let-down at 5–30% in PP, PE, PVC and PA. Eliminates the dispersion and feed-handling penalties of dry filler."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Drop-in let-down" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Add Imperium Masterbatch at your normal masterbatch ratio. No new feeders, no special drying, no twin-screw retrofit." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Compatible with single-screw extrusion, twin-screw compounding, and injection-molding direct let-down." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Engineered dispersion" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Pre-wetted, pre-coupled, and stabilized for downstream processing temperatures up to 230°C." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Particle distribution targeted for the specific carrier resin you specify." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Available carriers" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Polypropylene (PP) — most common." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Polyethylene (HDPE / LDPE) — pallet, packaging, pipe." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Polyvinyl chloride (PVC) — building products, profiles." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Polyamide 6 (PA6) — automotive engineering thermoplastics." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Custom carriers available on a per-program basis." }
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
