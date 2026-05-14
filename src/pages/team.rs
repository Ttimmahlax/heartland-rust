use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn Team() -> Element {
    rsx! {
        Seo {
            title: "Our Team",
            description: "The people behind Heartland Industries — engineers, agronomists, materials scientists, and supply-chain operators reshaping the way America makes things.",
            path: "/heartland-team",
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
                    "Leadership"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    span { class: "text-gradient-red", "The Heartland Team" }
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Heartland's leadership combines deep manufacturing experience with material-science research and on-the-ground farm operations."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Leadership" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "John Ely — Chief Executive Officer." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Tim Almond — Chairman & Chief Operating Officer." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Eric Austermann — VP of Engineering." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Markus von Graf — Strategy & Capital Markets." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Roger Blackwell — Senior Advisor (former CEO, Roger Blackwell Associates)." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Robby Dameron — Materials Science." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Deborah LaBelle — General Counsel." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "How we work" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Headquartered in Detroit, with farm operations across the Midwest and US South." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Engineering staff embedded with Tier-1 automotive, packaging, and building-products partners." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Get in touch" }
            }
        }
    }
}
