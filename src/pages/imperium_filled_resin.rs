use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{product_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn ImperiumFilledResin() -> Element {
    rsx! {
        Seo {
            title: "Imperium Filled Resin — Performance Plastics",
            description: "Pre-compounded Imperium-filled performance resin — ship-ready pellets that drop straight into your injection-molding or extrusion line.",
            path: "/imperium-filled-resin",
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
                    "Compounded & shipped"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    "Performance Plastics: "
                    span { class: "text-gradient-red", "Imperium-Filled Resin" }
                    " ready to mold."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Skip the compounding step entirely. Order Imperium pre-compounded into your spec'd resin, ready for the molding press."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Why pre-compounded?" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "For molders who don't compound in-house, Imperium Filled Resin removes the entire dispersion + handling question from your process." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Spec the carrier, the loading percentage, and the additive package; we deliver bagged pellets to your dock." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Typical formulations" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "PP + 20% Imperium — automotive interior trim." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "HDPE + 30% Imperium — returnable pallets and bins." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Hemp-reinforced PC/ABS — first-in-class for consumer electronics." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Hemp-reinforced PA6 — under-hood automotive." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Performance vs. virgin resin" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Specific stiffness and impact comparable to or exceeding talc-filled benchmarks." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Density 5–8% lower than CaCO₃-filled comparables — translates to lightweighting wins downstream." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Request a sample" }
            }
        }
    }
}
