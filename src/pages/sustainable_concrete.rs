use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn SustainableConcrete() -> Element {
    rsx! {
        Seo {
            title: "Sustainable Concrete",
            description: "Heartland works directly with leaders in construction to engineer lower carbon footprint with sustainable concrete additives.",
            path: "/sustainable-concrete-additives",
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
            VideoBackground { slug: "sustainable-concrete-additives".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "Sustainable Concrete"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Sustainable Concrete additives"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "One of the most consumed materials on the planet is concrete. Today, traditional concrete includes aggregate, cement, and other materials. Heartland's partners are formulating lower-carbon-footprint concrete."
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
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Natural Fiber Reinforced Concrete" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "One of the most consumed materials on the planet is concrete. Today, traditional concrete includes aggregate, cement, and other materials. Heartland's partners are formulating lower-carbon-footprint concrete." }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/sustainable-concrete-additives/hemp-filled-concrete-curved-wall.png",
                        alt: "Natural Fiber Reinforced Concrete",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Building A Sustainable Future From The Ground Up" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland supports local farming and construction by working with local leaders in building material supply chains." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Our team works alongside engineers, architects, building material suppliers, general contractors, and software suppliers to embed natural fiber additives into concrete with no retooling costs." }
                }
                div { class: "md:order-2",
                    img {
                        src: "/assets/pages/sustainable-concrete-additives/hemp-filled-concrete-tunnel.png",
                        alt: "Building A Sustainable Future From The Ground Up",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "For The Construction Industry" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland supplies lower-carbon-footprint materials to help builders and suppliers create stronger, lighter, cheaper, and more sustainable products." }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/imperium-masterbatch/imperium-filler.png",
                        alt: "Imperium Filler",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Unlocking The Sustainable Future We Need And Deserve" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland's additives replace and augment mined and synthetic materials like cement and aggregate." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "We work with global brands and their suppliers to predictably reduce the carbon footprint of everyday products without any retooling costs." }
                }
                div { class: "md:order-2",
                    img {
                        src: "/assets/pages/imperium-filler/talc-mining.png",
                        alt: "talc mining",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "max-w-3xl mx-auto mb-16 animate-fade-in-up",
                h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 text-center", "Throughout The Supply Chain" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland partners with corn, wheat, and soy farmers to embed industrial hemp into their crop rotation. Our USDA Grant has given us unique insights into industrial hemp farming, regenerative agriculture, and carbon sequestration." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Architect & Engineer Heartland partners with architect's and engineer's to specify hemp filled concrete in future projects." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Building Material Supplier Heartland partners with building material suppliers to ensure the hemp-filled concrete is distributed properly. Our team works alongside the suppliers to ensure natural fiber-filled concrete is used in the same way it is used today." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ General Contractor Heartland is the sustainability partner for leaders in construction focused on reducing their carbon footprint. Our team helps create stakeholder alignment so companies can effectively communicate the value of sustainable innovation." }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/sustainable-concrete-additives/turtle-stuck-on-plastic.png", alt: "turtle stuck on plastic", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
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
