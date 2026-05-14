use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn Whitepapers() -> Element {
    rsx! {
        Seo {
            title: "White Papers",
            description: "Technical white papers on hemp-reinforced polymers, LCA methodology, carbon disclosures, and the supply-chain economics of Imperium-grade materials.",
            path: "/whitepapers",
        }

        Hero {}
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
                    "Technical library"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    span { class: "text-gradient-red", "White Papers" }
                    " — the technical pack."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Engineering teams use these papers to make the case internally for Imperium-grade material substitution."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Available white papers" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "First Industrial Hemp Fiber LCA for Carbon-Negative Additives." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Engineering New Carbon-Negative Materials — Polypropylene focus." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Hemp Reinforced Polypropylene Composites in Automotive." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "The Scope 3 Carbon Footprint of a Plastic Automotive Part." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Citation policy" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Each white paper is published with primary-source citations and is available for redistribution with attribution to Heartland Industries." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Request a paper" }
            }
        }
    }
}
