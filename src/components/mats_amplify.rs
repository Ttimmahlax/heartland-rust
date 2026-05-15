//! "Materials We Amplify" — 6 image cards (3×2 grid on desktop, 2×3 on
//! tablet, 1-col on mobile). Eyebrow + heading + image grid, image-only
//! cards linking to the matching material/industry route. Designed to be
//! reusable across landing + product overview pages.

use dioxus::prelude::*;

use crate::Route;

#[derive(Props, Clone, PartialEq)]
pub struct MatsAmplifyProps {
    #[props(default = String::from("Carbon Negative Materials"))]
    pub eyebrow: String,
    #[props(default = String::from("Materials We Amplify"))]
    pub heading: String,
    #[props(default = String::new())]
    pub blurb: String,
}

#[component]
pub fn MatsAmplify(props: MatsAmplifyProps) -> Element {
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
                MatCard { image: "/assets/pages/landing/plastic.png",
                          alt:   "Plastic — Imperium-filled plastic compounding",
                          label: "Plastic",
                          route: Route::SustainablePlastic {} }
                MatCard { image: "/assets/pages/landing/imperium-fabric-hemp-fiber.png",
                          alt:   "Imperium fabric hemp fiber — textiles",
                          label: "Textiles",
                          route: Route::ImperiumFibers {} }
                MatCard { image: "/assets/pages/landing/rubber.png",
                          alt:   "Rubber — bio-based rubber additives",
                          label: "Rubber",
                          route: Route::SustainableRubber {} }
                MatCard { image: "/assets/pages/landing/asphalt.png",
                          alt:   "Asphalt — hemp-pulp asphalt additive",
                          label: "Asphalt",
                          route: Route::SustainableAsphalt {} }
                MatCard { image: "/assets/pages/landing/paper.png",
                          alt:   "Paper — hemp pulp additive",
                          label: "Paper",
                          route: Route::SustainablePaper {} }
                MatCard { image: "/assets/pages/landing/concrete.png",
                          alt:   "Concrete — hemp fiber concrete additive",
                          label: "Concrete",
                          route: Route::SustainableConcrete {} }
            }
        }
    }
}

#[component]
fn MatCard(image: &'static str, alt: &'static str, label: &'static str, route: Route) -> Element {
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
