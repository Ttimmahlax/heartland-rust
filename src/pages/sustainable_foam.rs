use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn SustainableFoam() -> Element {
    rsx! {
        Seo {
            title: "Sustainable Foam",
            description: "Heartland is a material innovation company that engineers high performance carbon negative additives for foam manufacturers and suppliers.",
            path: "/sustainable-foam",
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
                    "Sustainable Foam"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Sustainable foam additives"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "Heartland's natural fiber additives can be used in styrenes to expand supply and reduce carbon footprint. Many of today's applications use unfilled styrene. This makes it easy to reduce the cost and…"
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
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Functional Carbon Negative Additives" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland's natural fiber additives can be used in styrenes to expand supply and reduce carbon footprint. Many of today's applications use unfilled styrene. This makes it easy to reduce the cost and carbon footprint of traditional materials with sustainable additives." }
                }
                div { class: "md:order-1",
                    img { src: "/assets/pages/sustainable-foam/hemp-filled-foam-food-trays-2.webp", alt: "Functional Carbon Negative Additives", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Product Consistency Through Farming & Processing" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland supports local farming for local manufacturing by engineering hemp fiber additives for bonding & performance." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland's additives are being used by styrene producers, compounders, and molders to extend supply, alter performance, and go green." }
                }
                div { class: "md:order-2",
                    img { src: "/assets/pages/sustainable-foam/hemp-filled-foam-sheets.webp", alt: "Product Consistency Through Farming & Processing", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "For Foam Suppliers" }
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
                img { src: "/assets/pages/hemp-fiber-and-hurd/heartland-plastic-picture-3.webp", alt: "Markets We Amplify", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/hemp-fiber-and-hurd/heartland-packaging-plastic-pallets.webp", alt: "", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            div { class: "max-w-3xl mx-auto mb-16 animate-fade-in-up",
                h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 text-center", "Throughout The Supply Chain" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland partners with corn, wheat, and soy farmers to embed industrial hemp into their crop rotation. Our USDA Grant has given us unique insights into industrial hemp farming, regenerative agriculture, and carbon sequestration." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Compounder Heartland partners with plastic compounders to augment talc, calcium, and glass without any retooling costs. Our Imperium masterbatch solves dust, flammability, bonding, and bulk density problems typically associated with bio-based additives." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Converter Heartland partners with plastic converters to ensure the hemp-filled resin is processed properly. Our team works alongside component part manufacturers to process natural fiber-filled plastic with the same molds used today." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Brand Heartland is the sustainability partner for brands on their journey to reduce their carbon footprint. Our team helps create stakeholder alignment so companies can effectively communicate the value of sustainable material innovation." }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/hemp-fiber-and-hurd/plastic.webp", alt: "plastic", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }        }
    }
}

#[component]
fn ClosingCta() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 my-12 section-soft-edges",
            div { class: "container-content text-center",
                h2 { class: "text-3xl md:text-4xl font-bold mb-6 max-w-2xl mx-auto",
                    "Learn more about Sustainable"
                }
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Get in touch" }
            }
        }
    }
}
