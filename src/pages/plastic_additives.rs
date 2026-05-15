use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn PlasticAdditives() -> Element {
    rsx! {
        Seo {
            title: "Sustainable Plastic Additives",
            description: "Imperium Filler is engineered hemp fiber designed as a carbon negative additive to reduce the cost, weight, and carbon footprint of plastic",
            path: "/plastic-additives",
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
                    "Sustainable Plastic Additives"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Carbon Negative additives"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "Instead of focusing on removing petroleum based plastics, we are focused on utilizing hemp fibers as additives to strengthen the plastics that companies are already using."
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
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Unlocking Commodity Natural Fiber Additives" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Instead of focusing on removing petroleum based plastics, we are focused on utilizing hemp fibers as additives to strengthen the plastics that companies are already using." }
                }
                div { class: "md:order-1",
                    img { src: "/assets/pages/hemp-fiber-and-hurd/heartland-powder.png", alt: "Unlocking Commodity Natural Fiber Additives", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "One Additive. Unlimited Applications." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium Powder is a carbon-negative additive for materials used for mass manufacturing." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Most manufacturers use mined and synthetic additives in their raw materials. Heartland's Imperium Powder is engineered to replace and augment additives like talc, calcium, and fiberglass." }
                }
                div { class: "md:order-2",
                    img { src: "/assets/pages/hemp-fiber-and-hurd/1210x786-px-4.png", alt: "heartland hemp bales", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/hemp-fiber-and-hurd/plastic.png", alt: "plastic", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/hemp-fiber-and-hurd/asphalt.png", alt: "asphalt", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Unlocking The Sustainable Future We Need And Deserve" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland's materials replace and augment additives like talc, calcium carbonate, fiberglass, and carbon black." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "We work with global brands and their suppliers to predictably reduce the carbon footprint of everyday products without any retooling costs." }
                }
                div { class: "md:order-1",
                    img { src: "/assets/pages/hemp-fiber-and-hurd/talc-mining.png", alt: "talc mining", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/hemp-fiber-and-hurd/heartland-plastic-picture-3.png", alt: "Markets We Amplify", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/hemp-fiber-and-hurd/heartland-packaging-plastic-pallets.png", alt: "", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            div { class: "max-w-3xl mx-auto mb-16 animate-fade-in-up",
                h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 text-center", "Throughout The Supply Chain" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland partners with corn, wheat, and soy farmers to embed industrial hemp into their crop rotation. Our USDA Grant has given us unique insights into industrial hemp farming, regenerative agriculture, and carbon sequestration." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Compounder Heartland partners with plastic compounders to augment talc, calcium, and glass without any retooling costs. Our Imperium masterbatch solves dust, flammability, bonding, and bulk density problems typically associated with bio-based additives." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Converter Heartland partners with plastic converters to ensure the hemp-filled resin is processed properly. Our team works alongside component part manufacturers to process natural fiber-filled plastic with the same molds used today." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Brand Heartland is the sustainability partner for brands on their journey to reduce their carbon footprint. Our team helps create stakeholder alignment so companies can effectively communicate the value of sustainable material innovation." }
            }        }
    }
}

#[component]
fn ClosingCta() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 my-12 section-soft-edges",
            div { class: "container-content text-center",
                h2 { class: "text-3xl md:text-4xl font-bold mb-6 max-w-2xl mx-auto",
                    "Learn more about Carbon"
                }
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Get in touch" }
            }
        }
    }
}
