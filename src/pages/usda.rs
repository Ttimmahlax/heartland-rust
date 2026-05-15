use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn Usda() -> Element {
    rsx! {
        Seo {
            title: "USDA",
            description: "Heartland is a material innovation company that engineers USDA biopreferred certified materials for government mandated purchasing.",
            path: "/usda",
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
                    "USDA"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Government mandated purchasing"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "The BioPreferred Program was created by the 2002 Farm Bill and reauthorized and expanded in the 2018 Farm Bill. The Program's purpose is to spur economic development, create new jobs, and create new…"
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
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "USDA's Biopreferred Program" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "The BioPreferred Program was created by the 2002 Farm Bill and reauthorized and expanded in the 2018 Farm Bill. The Program's purpose is to spur economic development, create new jobs, and create new markets for farm commodities." }
                }
                div { class: "md:order-1",
                    img { src: "/assets/pages/usda/biopreferred-logo-final.webp", alt: "USDA's Biopreferred Program", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Certified Materials For Lower Carbon Footprint Products" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland's materials fulfill mandatory purchasing requirements for federal agencies and their contractors." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "The increased development, purchase, and use of bio-based products reduce our nation's reliance on petroleum, increase the use of renewable agricultural resources, and contribute to reducing adverse environmental and health impacts." }
                }
                div { class: "md:order-2",
                    img { src: "/assets/pages/usda/heartland-government-affairs.webp", alt: "Certified Materials For Lower Carbon Footprint Products", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "For Our Biopreferred Materials" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland supplies lower-carbon-footprint materials to help manufacturers create stronger, lighter, cheaper, and more sustainable products." }
                }
                div { class: "md:order-1",
                    img { src: "/assets/pages/hemp-fiber-and-hurd/imperium-filler.webp", alt: "Imperium Filler", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Unlocking The Sustainable Future We Need And Deserve" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland's materials replace and augment additives like talc, calcium carbonate, fiberglass, and carbon black." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "We work with global brands and their suppliers to predictably reduce the carbon footprint of everyday products without any retooling costs." }
                }
                div { class: "md:order-2",
                    img { src: "/assets/pages/hemp-fiber-and-hurd/talc-mining.webp", alt: "talc mining", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/hemp-fiber-and-hurd/plastic.webp", alt: "plastic", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/hemp-fiber-and-hurd/asphalt.webp", alt: "asphalt", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            div { class: "max-w-3xl mx-auto mb-16 animate-fade-in-up",
                h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 text-center", "Throughout The Supply Chain" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland partners with corn, wheat, and soy farmers to embed industrial hemp into their crop rotation. Our USDA Grant has given us unique insights into industrial hemp farming, regenerative agriculture, and carbon sequestration." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Compounder Heartland partners with plastic compounders to augment talc, calcium, and glass without any retooling costs. Our Imperium masterbatch solves dust, flammability, bonding, and bulk density problems typically associated with bio-based additives." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Converter Heartland partners with plastic converters to ensure the hemp-filled resin is processed properly. Our team works alongside component part manufacturers to process natural fiber-filled plastic with the same molds used today." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Brand Heartland is the sustainability partner for brands on their journey to reduce their carbon footprint. Our team helps create stakeholder alignment so companies can effectively communicate the value of sustainable material innovation." }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/hemp-fiber-and-hurd/heartland-plastic-picture-3.webp", alt: "Markets We Amplify", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }        }
    }
}

#[component]
fn ClosingCta() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 my-12 section-soft-edges",
            div { class: "container-content text-center",
                h2 { class: "text-3xl md:text-4xl font-bold mb-6 max-w-2xl mx-auto",
                    "Learn more about Government"
                }
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Get in touch" }
            }
        }
    }
}
