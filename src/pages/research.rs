use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn Research() -> Element {
    rsx! {
        Seo {
            title: "Natural Fiber Research",
            description: "Peer-reviewed and industry research on natural fiber reinforcement, hemp materials, regenerative agronomy, and the carbon profile of bio-based polymers.",
            path: "/natural-fiber-research",
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
                    "Research library"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    span { class: "text-gradient-red", "Natural Fiber Research" }
                    " — primary sources."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Heartland's curated research library brings together academic, government, and industry literature on natural-fiber composites, hemp agronomy, and the LCA methodologies underpinning bio-based materials."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Focus areas" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Mechanical property modeling of hemp-PP and hemp-PA6 composites." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Life-cycle analysis methodology for bio-based fillers." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Hemp agronomy + soil-health outcomes in rotation studies." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Microplastic shed comparisons across hemp, polyester, and cotton textiles." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Talk to research" }
            }
        }
    }
}
