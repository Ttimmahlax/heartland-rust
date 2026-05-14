use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{default_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn SustainableConcrete() -> Element {
    rsx! {
        Seo {
            title: "Sustainable Concrete Additives",
            description: "Hemp-fiber and hemp-hurd additives for concrete — crack resistance, lower carbon intensity, and improved thermal performance.",
            path: "/sustainable-concrete-additives",
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
                    "Concrete"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    span { class: "text-gradient-red", "Sustainable Concrete" }
                    " — fiber-reinforced, carbon-aware."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Heartland's hemp additives improve concrete crack resistance, lower the embodied carbon, and improve thermal envelope performance in mid-strength concretes."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Hempcrete" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Hempcrete (hemp hurd + lime binder) is a well-established alternative to traditional concrete for low-rise wall systems." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Lower compressive strength than Portland-cement concrete, but dramatically lower embodied carbon and excellent insulating R-value." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Imperium fiber reinforcement" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "For non-structural toppings, sidewalks, and decorative concrete, hemp fiber improves crack arrest." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Reduces shrinkage cracking during cure." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Engineer a concrete additive" }
            }
        }
    }
}
