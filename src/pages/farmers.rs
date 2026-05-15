use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn Farmers() -> Element {
    rsx! {
        Seo {
            title: "Heartland Farmers",
            description: "Heartland farmers embed industrial hemp and regenerative agriculture into crop rotations to increase organic matter content and profits.",
            path: "/heartland-farmers",
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
            VideoBackground { slug: "heartland-farmers".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "Heartland Farmers"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Farm with heartland"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "We partner with traditional row crop farmers to provide the necessary resources to successfully integrate hemp fiber into their corn, wheat, and soy rotations."
                }
            }
        }
    }
}

#[component]
fn Body() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Same Equipment. New Crop. More Profit." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "We partner with traditional row crop farmers to provide the necessary resources to successfully integrate hemp fiber into their corn, wheat, and soy rotations." }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/heartland-farmers/farming-hemp-michigan.webp",
                        alt: "farming hemp michigan",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Spraying Chemicals Is Expensive Business" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "The Industrial hemp plant requires no pest management procedures and less fertilizer. This reduces costs per acre." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "The rapid growth and canopy created by hemp fiber take nutrients from competitive weeds in the ground. This is an effective benefit to further reduce input costs on a per acre basis." }
                }
                div { class: "md:order-2",
                    img {
                        src: "/assets/pages/heartland-farmers/hemp-fiber-bale.png",
                        alt: "Spraying Chemicals Is Expensive Business",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/why-imperium/heartland-plastic-picture-3.png", alt: "Applications Your Crop Amplifies", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/why-imperium/heartland-packaging-plastic-pallets.png", alt: "", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            div { class: "max-w-3xl mx-auto mb-16 animate-fade-in-up",
                h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 text-center", "Throughout The Supply Chain" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland partners with corn, wheat, and soy farmers to embed industrial hemp into their crop rotation. Our USDA Grant has given us unique insights into industrial hemp farming, regenerative agriculture, and carbon sequestration." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Material Mixer Heartland partners with companies in different industries that blend our materials into their existing products. Our Imperium masterbatch solves dust, flammability, bonding, and bulk density problems typically associated with bio-based additives." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Converter Heartland partners with industry converters to ensure the hemp-filled material is processed properly. Our team works alongside component part manufacturers to process natural fiber-filled products with the same molds used today." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Brand Heartland is the sustainability partner for brands on their journey to reduce their carbon footprint. Our team helps create stakeholder alignment so companies can effectively communicate the value of sustainable material innovation." }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/imperium-masterbatch/plastic.png", alt: "plastic", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/imperium-masterbatch/asphalt.png", alt: "asphalt", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
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
                    "Learn more about Farm"
                }
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Get in touch" }
            }
        }
    }
}
