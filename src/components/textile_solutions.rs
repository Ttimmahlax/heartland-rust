//! Textile-specific 3-card row for the Imperium Fibers page: spin-ready fiber,
//! yarn, and fabric — each linking to its dedicated product page.

use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn TextileSolutions() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "text-center mb-12 max-w-3xl mx-auto",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "From Field To Fabric"
                }
                h2 { class: "text-3xl md:text-4xl font-bold mb-6",
                    "Imperium Textile Solutions"
                }
                p { class: "text-lg text-[color:var(--color-fg-muted)]",
                    "Heartland delivers Imperium hemp at every stage of the textile supply chain — from spin-ready fiber to finished fabric."
                }
            }

            div { class: "grid grid-cols-1 md:grid-cols-3 gap-6 md:gap-8",
                TextileCard {
                    image:  "/assets/pages/imperium-fibers/imperium-spin-ready-fiber.webp",
                    alt:    "Imperium Spin-Ready Fiber",
                    label:  "Imperium Spin-Ready Fiber",
                    route:  Route::ImperiumSpinReadyWhiteFiber {},
                }
                TextileCard {
                    image:  "/assets/pages/imperium-fibers/imperium-yarn.webp",
                    alt:    "Imperium Yarn",
                    label:  "Imperium Yarn",
                    route:  Route::ImperiumYarn {},
                }
                TextileCard {
                    image:  "/assets/pages/imperium-fibers/imperium-fabric.webp",
                    alt:    "Imperium Fabric",
                    label:  "Imperium Fabric",
                    route:  Route::ImperiumFabric {},
                }
            }
        }
    }
}

#[component]
fn TextileCard(image: &'static str, alt: &'static str, label: &'static str, route: Route) -> Element {
    rsx! {
        Link {
            to: route,
            aria_label: label,
            class: "group block overflow-hidden rounded-xl shadow-lg hover:translate-y-[-3px] transition-transform animate-fade-in-up",
            div { class: "aspect-[16/10] overflow-hidden bg-[color:var(--color-surface)]",
                img {
                    src: image,
                    alt: alt,
                    loading: "lazy",
                    class: "w-full h-full object-cover transition-transform duration-300 group-hover:scale-105",
                }
            }
        }
    }
}
