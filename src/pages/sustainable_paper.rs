use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{default_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn SustainablePaper() -> Element {
    rsx! {
        Seo {
            title: "Sustainable Paper Additives",
            description: "Hemp pulp additives for paper, paperboard, and tissue — lower-impact, lower-carbon, partial replacement for kraft hardwood fiber.",
            path: "/sustainable-paper-additives",
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
                    "Paper"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    "Sustainable "
                    span { class: "text-gradient-red", "Paper" }
                    " — hemp pulp additive."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Industrial hemp fiber, partially substituted for hardwood kraft pulp, lowers the carbon and water footprint of paper and paperboard without sacrificing strength."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Why hemp pulp" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Hemp fiber has length and tensile properties more comparable to softwood kraft than to hardwood." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Lower water demand than virgin kraft on a per-ton basis." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Where it slots in" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Mid-grade paperboard for packaging." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Tissue products where wet-strength matters." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Specialty papers for filtration and currency-like substrates." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Engage the pulp team" }
            }
        }
    }
}
