use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{default_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn Lca() -> Element {
    rsx! {
        Seo {
            title: "Imperium Farming LCA",
            description: "The first published life-cycle analysis for industrial hemp fiber used as a carbon-negative additive — farm-to-filler boundary, third-party verified.",
            path: "/lca",
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
                    "Life-cycle analysis"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    span { class: "text-gradient-red", "Imperium Farming LCA" }
                    " — the carbon math, disclosed."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Heartland published the first life-cycle analysis covering industrial hemp fiber used as a carbon-negative additive. Farm-to-filler boundary. Third-party verified. Auditable."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Boundary" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Cradle to gate: seed, planting, growing, harvest, decortication, processing, packaging, and US-domestic logistics to the manufacturer's loading dock." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Excludes downstream manufacturer-specific compounding emissions (those land in the customer's Scope 1)." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Headline numbers" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Net upstream sequestration outweighs downstream processing emissions per pound." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Per-batch verifications shipped via the Carbon Report platform." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "How to consume the data" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "The LCA pack is structured for Scope 3 disclosure under CDP, SBTi, SEC, and CSRD methodologies." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Engage /contact for the SKU-specific disclosure pack tied to your purchase order." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Request the disclosure pack" }
            }
        }
    }
}
