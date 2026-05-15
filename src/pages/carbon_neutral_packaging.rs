use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn CarbonNeutralPackaging() -> Element {
    rsx! {
        Seo {
            title: "Carbon Neutral Packaging With Imperium Inside",
            description: "Sustainability means it costs less and improves your products, period. We believe new materials should exceed all criteria.",
            path: "/carbon-neutral-packaging-with-imperium-inside",
        }

        Hero {}
        LogoCarousel { heading: "" }
        Body {}
        ClosingCta {}
        NewsCarousel { heading: "Related Articles" }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section {
            class: "video-hero-section section-soft-bottom min-h-[110vh] flex items-center pb-[20vh]",
            VideoBackground { slug: "landing".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "Carbon Neutral Packaging With Imperium Inside"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Decarbonized Packaging"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "Brands and suppliers can now replace international wood pallets with carbon-neutral pallets while reducing costs. Imperium-reinforced pallets allow brands and suppliers to quickly exceed sustainability mandates."
                }
            }
        }
    }
}

#[component]
fn Body() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/carbon-neutral-packaging-with-imperium-inside/carbon-neutral-plastic-pallet-48-x-40-imperium-inside-nest-us1.1-1.png", alt: "Carbon-Neutral-Plastic-Pallet-48-x-40-Imperium-Inside-Nest-US1.1", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Same Packaging. Lower Cost. Neutral Carbon Footprint." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Brands and suppliers can now replace international wood pallets with carbon-neutral pallets while reducing costs. Imperium-reinforced pallets allow brands and suppliers to quickly exceed sustainability mandates." }
                }
                div { class: "md:order-2",
                    img { src: "/assets/pages/carbon-neutral-packaging-with-imperium-inside/imperium-filled-resin-2-1.png", alt: "Same Packaging. Lower Cost. Neutral Carbon Footprint.", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/hemp-fiber-and-hurd/heartland-plastic-picture-3.png", alt: "Markets We Amplify", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/hemp-fiber-and-hurd/heartland-packaging-plastic-pallets.png", alt: "", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/carbon-neutral-packaging-with-imperium-inside/ultimate-guide-to-hemp-wool-blends-composition-uses-and-benefits.png", alt: "From The Heartland", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            div { class: "max-w-3xl mx-auto mb-16 animate-fade-in-up",
                h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 text-center", "Throughout The Supply Chain" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland partners with corn, wheat, and soy farmers to embed industrial hemp into their crop rotation. Our USDA Grant has given us unique insights into industrial hemp farming, regenerative agriculture, and carbon sequestration." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Compounder Heartland partners with plastic compounders to augment talc, calcium, and glass without any retooling costs. Our Imperium masterbatch solves dust, flammability, bonding, and bulk density problems typically associated with bio-based additives." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Converter Heartland partners with plastic converters to ensure the hemp-filled resin is processed properly. Our team works alongside component part manufacturers to process natural fiber-filled plastic with the same molds used today." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Brand Heartland is the sustainability partner for brands on their journey to reduce their carbon footprint. Our team helps create stakeholder alignment so companies can effectively communicate the value of sustainable material innovation." }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Sustainable Solutions" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland supplies lower-carbon-footprint materials to help manufacturers create stronger, lighter, cheaper, and more sustainable products." }
                }
                div { class: "md:order-1",
                    img { src: "/assets/pages/hemp-fiber-and-hurd/imperium-filler.png", alt: "Imperium Filler", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 mt-12 text-center", "Try Our Carbon Footprint Calculator" }        }
    }
}

#[component]
fn ClosingCta() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 my-12 section-soft-edges",
            div { class: "container-content text-center",
                h2 { class: "text-3xl md:text-4xl font-bold mb-6 max-w-2xl mx-auto",
                    "Learn more about Decarbonized"
                }
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Get in touch" }
            }
        }
    }
}
