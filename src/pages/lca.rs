use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn Lca() -> Element {
    rsx! {
        Seo {
            title: "LCA",
            description: "Heartland's LCA shows that every 1KG of Imperium Filler removes 3KG of CO2e from the atmosphere. Find out more here.",
            path: "/lca",
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
            VideoBackground { slug: "lca".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "LCA"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "1 kg of imperium filler removes 2.85 kg of co2e"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "This is the world's first industrial hemp fiber LCA that focuses on plastic additives. Heartland's LCA was completed by Presidio Graduate School and Lifecycle Associates under ISO 14040 standards."
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
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "1 KG of Imperium Filler Removes 2.85 KG of CO2e" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "This is the world's first industrial hemp fiber LCA that focuses on plastic additives. Heartland's LCA was completed by Presidio Graduate School and Lifecycle Associates under ISO 14040 standards." }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/lca/foam-7.webp",
                        alt: "problems with natural fibers",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Unlocking Sustainable Plastic, Paper, & Concrete" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Beyond plastic, Heartland's Imperium Filler can be used to make any material carbon-negative." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Our additives work as drop-in replacements for traditional additives used across raw material supply chains." }
                }
                div { class: "md:order-2",
                    img {
                        src: "/assets/pages/lca/lca-pr-pictures-1.webp",
                        alt: "heartland LCA",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "For Plastic Compounders" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland supplies carbon-negative materials to help brands and their suppliers create stronger, lighter, cheaper, and more sustainable products." }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/imperium-masterbatch/imperium-filler.webp",
                        alt: "Imperium Filler",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Developing SEC Climate Disclosure Readiness" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland's Imperium Filler creates a competitive advantage for brands and suppliers that will be disclosing scope 1, 2, and 3 emissions." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Manufacturers that use carbon-negative additives are positioned to outperform competitors who are lagging in sustainable material innovation." }
                }
                div { class: "md:order-2",
                    img {
                        src: "/assets/pages/lca/heartland-powder.webp",
                        alt: "Developing SEC Climate Disclosure Readiness",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "max-w-3xl mx-auto mb-16 animate-fade-in-up",
                h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 text-center", "Throughout The Supply Chain" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland partners with corn, wheat, and soy farmers to embed industrial hemp into their crop rotation. Our USDA Grant has given us unique insights into industrial hemp farming, regenerative agriculture, and carbon sequestration." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Compounder Heartland partners with plastic compounders to augment talc, calcium, and glass without any retooling costs. Our Imperium masterbatch solves dust, flammability, bonding, and bulk density problems typically associated with bio-based additives." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Converter Heartland partners with plastic converters to ensure the hemp-filled resin is processed properly. Our team works alongside component part manufacturers to process natural fiber-filled plastic with the same molds used today." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Brand Heartland is the sustainability partner for brands on their journey to reduce their carbon footprint. Our team helps create stakeholder alignment so companies can effectively communicate the value of sustainable material innovation." }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/why-imperium/heartland-plastic-picture-3.webp", alt: "Markets We Amplify", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/why-imperium/heartland-packaging-plastic-pallets.webp", alt: "", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
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
                    "Learn more about 1"
                }
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Get in touch" }
            }
        }
    }
}
