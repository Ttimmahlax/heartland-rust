//! "Decarbonized Solutions" — eyebrow + heading + paragraph + 3-image card row.
//!
//! Reusable across landing + product pages. Each card is a clickable image
//! linking to a Route. Side-by-side on desktop (`md:grid-cols-3`), stacked on
//! mobile (default single column).

use dioxus::prelude::*;

use crate::Route;

#[derive(Props, Clone, PartialEq)]
pub struct DecarbSolutionsProps {
    #[props(default = String::from("No Retooling Costs"))]
    pub eyebrow: String,
    #[props(default = String::from("Decarbonized Solutions"))]
    pub heading: String,
    #[props(default = String::from(
        "Heartland produces and distributes low carbon Imperium materials that are proven to reduce the cost of your finished goods."
    ))]
    pub blurb: String,
}

#[component]
pub fn DecarbSolutions(props: DecarbSolutionsProps) -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "text-center mb-12 max-w-3xl mx-auto",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "{props.eyebrow}"
                }
                h2 { class: "text-3xl md:text-4xl font-bold mb-6",
                    "{props.heading}"
                }
                p { class: "text-lg text-[color:var(--color-fg-muted)]",
                    "{props.blurb}"
                }
            }

            div { class: "grid grid-cols-1 md:grid-cols-3 gap-6 md:gap-8",
                DecarbCard {
                    image:  "/assets/pages/landing/imperium-textiles.png",
                    alt:    "Imperium textiles",
                    label:  "Imperium Textiles",
                    route:  Route::ImperiumFibers {},
                }
                DecarbCard {
                    image:  "/assets/pages/landing/imperium-masterbatch.png",
                    alt:    "Imperium Masterbatch",
                    label:  "Imperium Masterbatch",
                    route:  Route::ImperiumMasterbatch {},
                }
                DecarbCard {
                    image:  "/assets/pages/landing/imperium-reinforced-plastic.png",
                    alt:    "Imperium-reinforced plastic",
                    label:  "Imperium Reinforced Plastic",
                    route:  Route::ImperiumFilledResin {},
                }
            }
        }
    }
}

#[component]
fn DecarbCard(image: &'static str, alt: &'static str, label: &'static str, route: Route) -> Element {
    rsx! {
        Link {
            to: route,
            // `aria-label` carries the destination semantics now that we
            // dropped the visible caption — keeps the card accessible.
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
