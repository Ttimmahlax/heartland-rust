use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{default_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn About() -> Element {
    rsx! {
        Seo {
            title: "About Heartland Industries",
            description: "Heartland Industries — a Detroit-headquartered material science company. We help manufacturers exceed cost-reduction goals while reducing emissions.",
            path: "/about",
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
                    "About"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    span { class: "text-gradient-red", "Heartland Industries" }
                    " — material science from America's heartland."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Headquartered in Detroit, Heartland Industries is a material-science company connecting American farms to American manufacturers through industrial hemp."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "What we make" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Imperium — engineered industrial hemp fiber for plastics, rubber, concrete, asphalt, paper, and textiles." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Imperium Animal Feed — protein- and omega-rich livestock nutrition." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Carbon Report — emissions tracking and disclosure software for material manufacturers." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Our network" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "12,000+ acres of contracted industrial hemp across 11 US states." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Engineering teams embedded with Tier-1 automotive, packaging, and building-products partners." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Funded backers including Chemovator (BASF), Amazon Climate Tech Accelerator, and leading Michigan banks." }
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
                Link { to: Route::Team {}, class: "btn-accent-gradient", "Meet the team" }
            }
        }
    }
}
