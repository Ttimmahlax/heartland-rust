use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{product_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn SustainablePlastic() -> Element {
    rsx! {
        Seo {
            title: "Sustainable Plastic Compounding",
            description: "Plastic compounders use Imperium to cut cost and embedded carbon in PP, PE, PVC, PA6 and engineered thermoplastics — no line modifications required.",
            path: "/sustainable-plastic-compounding",
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
                    "For plastic compounders"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    "Sustainable "
                    span { class: "text-gradient-red", "Plastic Compounding" }
                    " starts with the right filler."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Imperium swaps in for talc, calcium carbonate, and (at higher loadings) glass fiber — without requiring screw changes or feeder upgrades."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Polypropylene" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Imperium-PP composites match or exceed talc-PP on flex modulus at lower density." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Typical loadings of 20–40% deliver the cost AND carbon wins simultaneously." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Polyethylene" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "HDPE + Imperium for returnable pallets, bins, and industrial packaging — see /sustainable-packaging." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "LDPE + Imperium for film and bag products where opacity is a feature." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Engineered thermoplastics" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "First-in-class natural fiber reinforced PC/ABS for electronics." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Hemp-reinforced PA6 has shipped on the Magna × BASF × Heartland Altair Enlighten Award finalist program." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Talk to a compounding engineer" }
            }
        }
    }
}
