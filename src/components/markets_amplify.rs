//! "Markets We Amplify" — 6 image cards mapped to the markets Heartland
//! supplies (plastic compounders, automotive, textiles, packaging, building
//! materials, marine). Sibling of `MatsAmplify` for the **material →
//! market** narrative.

use dioxus::prelude::*;

use crate::Route;

#[derive(Props, Clone, PartialEq)]
pub struct MarketsAmplifyProps {
    #[props(default = String::from("Sustainable Material Innovation"))]
    pub eyebrow: String,
    #[props(default = String::from("Markets We Amplify"))]
    pub heading: String,
    #[props(default = String::new())]
    pub blurb: String,
}

#[component]
pub fn MarketsAmplify(props: MarketsAmplifyProps) -> Element {
    let has_blurb = !props.blurb.is_empty();
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "text-center mb-12 max-w-3xl mx-auto",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "{props.eyebrow}"
                }
                h2 { class: "text-3xl md:text-4xl font-bold mb-4",
                    "{props.heading}"
                }
                if has_blurb {
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mt-4",
                        "{props.blurb}"
                    }
                }
            }

            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-5 md:gap-6",
                MarketCard { image: "/assets/pages/landing/market-plastic.png",
                             alt:   "Plastic compounders — sustainable plastic compounding",
                             label: "Plastic Compounders",
                             route: Route::SustainablePlastic {} }
                MarketCard { image: "/assets/pages/landing/market-automotive.png",
                             alt:   "Automotive — lightweight, low-carbon parts",
                             label: "Automotive",
                             route: Route::Automotive {} }
                MarketCard { image: "/assets/pages/landing/market-textiles.png",
                             alt:   "Imperium hemp textile market",
                             label: "Textiles",
                             route: Route::ImperiumFibers {} }
                MarketCard { image: "/assets/pages/landing/market-packaging.png",
                             alt:   "Packaging — carbon-neutral pallets and bins",
                             label: "Packaging",
                             route: Route::SustainablePackaging {} }
                MarketCard { image: "/assets/pages/landing/market-building-materials.png",
                             alt:   "Heartland building materials",
                             label: "Building Materials",
                             route: Route::SustainableBuilding {} }
                MarketCard { image: "/assets/pages/landing/market-marine.png",
                             alt:   "Heartland marine — yacht and dock applications",
                             label: "Marine",
                             route: Route::Marine {} }
            }
        }
    }
}

#[component]
fn MarketCard(image: &'static str, alt: &'static str, label: &'static str, route: Route) -> Element {
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
