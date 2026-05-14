use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{farm_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn Farmers() -> Element {
    rsx! {
        Seo {
            title: "Our Farmers",
            description: "Heartland's farmer network — 11 US states, 12,000+ acres of contracted industrial hemp. Premium contracts, agronomic support, and a guaranteed buyer.",
            path: "/heartland-farmers",
        }

        Hero {}
        StatCounters { stats: farm_stats() }
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
                    "Grower network"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    span { class: "text-gradient-red", "Our Farmers" }
                    " — 11 states, one network."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Heartland farms industrial hemp through a contract grower network across 11 US states. We provide seed, agronomic support, harvest planning, and a guaranteed buyer — at a premium to corn and soybean."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Why farmers join the network" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Premium pricing relative to corn and soybean in the same rotation slot." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Agronomic support on planting density, harvest timing, and decortication logistics." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Guaranteed buyer for every ton harvested." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Carbon credit qualification — additional revenue per acre." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "How to apply" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Active grower applications open every February for the spring planting cycle, and every June for the winter-wheat-double-crop summer planting cycle." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Email Hello@heartland.io to start the conversation." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Apply to grow" }
            }
        }
    }
}
