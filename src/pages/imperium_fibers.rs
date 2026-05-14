use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{product_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn ImperiumFibers() -> Element {
    rsx! {
        Seo {
            title: "Imperium Textile Fiber",
            description: "Imperium textile fiber — soft American hemp fiber for yarn, polyhemp, hemp-lyocell and hemp-cotton blends. No microplastics, no offshore supply risk.",
            path: "/imperium-fibers",
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
                    "Hemp textile"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    span { class: "text-gradient-red", "Imperium Fiber" }
                    " — the new super-fiber for textiles."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Soft American hemp fiber engineered for spin-ready white yarn, polyhemp, hemp-lyocell, hemp-linen, hemp-cotton, hemp-silk and hemp-wool blends."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "The natural alternative to nylon and polycotton" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Polyhemp delivers the same hand and drape as polycotton at lower cost — and without the microplastic shed." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Hemp-lyocell preserves the strength of nylon without the oxidative-stress static-electricity penalties." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "US fiber, no port risk" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Most premium hemp fiber today moves through European or Indian processors. Heartland grows, processes, and bales in the US." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "For brands sourcing under the Uyghur Forced Labor Prevention Act or similar, US-origin fiber removes a layer of audit complexity." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "What ships" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Spin-ready white fiber (combed, ready for the spinning frame)." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Yarn (custom counts, blend ratios on request)." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Bales of conditioned hemp fiber for downstream conversion." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Sample-quantity fabric for protyping." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Sample request" }
            }
        }
    }
}
