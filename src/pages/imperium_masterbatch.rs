use dioxus::prelude::*;

use crate::components::carbon_calculator::CarbonCalculator;
use crate::components::logo_carousel::LogoCarousel;
use crate::components::mats_amplify::MatsAmplify;
use crate::components::news_carousel::NewsCarousel;
use crate::components::supply_chain::{SupplyChainStep, SupplyIcon};
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;

#[component]
pub fn ImperiumMasterbatch() -> Element {
    rsx! {
        Seo {
            title: "Imperium Masterbatch",
            description: "Imperium Masterbatch is hemp fiber engineered to reinforce plastic while reducing the cost, weight, and carbon footprint of plastics.",
            path: "/imperium-masterbatch",
        }

        Hero {}
        ImperiumLogoStrip {}
        UnlockingSection {}
        LogoCarousel { heading: "" }
        SolvingProblems {}
        MatsAmplify {}
        UnlockingFutureSection {}
        SupplyChainSection {}
        CarbonCalculator {}
        NewsCarousel { heading: "Related Articles" }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section {
            class: "video-hero-section section-soft-bottom min-h-[110vh] flex items-center pb-[20vh]",
            VideoBackground { slug: "imperium-masterbatch".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "Imperium Masterbatch"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "carbon negative"
                    br {}
                    "material innovation"
                }
            }
        }
    }
}

#[component]
fn ImperiumLogoStrip() -> Element {
    rsx! {
        section { class: "container-content pt-16 md:pt-24 pb-4 md:pb-8",
            // Same logo + spacing pattern as the why-imperium ItsInOurRoots header.
            div { class: "flex justify-center mb-20 md:mb-24",
                img {
                    src: "/assets/brand/imperium-logo.png",
                    alt: "Imperium",
                    loading: "lazy",
                    class: "h-20 md:h-24 w-auto",
                }
            }
        }
    }
}

#[component]
fn UnlockingSection() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            // Text LEFT, image RIGHT — matches the other product pages.
            div { class: "grid md:grid-cols-2 gap-10 md:gap-14 items-center",
                // Left: text
                div { class: "animate-fade-in-up md:order-1 order-2",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "With Imperium Inside"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Unlocking Sustainable "
                        br { class: "hidden md:inline" }
                        span { class: "text-gradient-red", "Material Innovation" }
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)]",
                        "Imperium Masterbatch enables compounders to develop decarbonized plastic for brands and molders that are looking for high-performance carbon-negative products. Brands now have a path to predictably reduce the cost, weight, and carbon footprint of their products."
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/why-imperium/heartland-masterbatch.webp",
                        alt: "Heartland Imperium masterbatch — pellets ready for compounding",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
        }
    }
}

#[component]
fn SolvingProblems() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-10 md:gap-14 items-center",
                // Left: image (farming hemp)
                div { class: "animate-fade-in-up md:order-1 order-2",
                    img {
                        src: "/assets/pages/heartland-farmers/farming-hemp-michigan.webp",
                        alt: "Farming hemp — Heartland's industrial hemp supply chain",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
                // Right: text
                div { class: "animate-fade-in-up md:order-2 order-1",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Sustainability Engineered For Mass Manufacturing"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Solving Problems For "
                        span { class: "text-gradient-red", "Brands & Plastic Compounders" }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "Imperium Masterbatch is designed to bond and perform inside thermoplastics without any retooling costs."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)]",
                        "Most manufacturers use mined and synthetic materials every day. Heartland's natural fiber masterbatch is engineered to solve concerns around dust, flammability, moisture uptake, bulk density, and product consistency."
                    }
                }
            }
        }
    }
}

#[component]
fn UnlockingFutureSection() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-10 md:gap-14 items-center",
                div { class: "animate-fade-in-up md:order-1 order-2",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Lower Cost Materials"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Unlocking Our "
                        span { class: "text-gradient-red", "Sustainable Future" }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "Heartland's materials replace and augment additives like talc, calcium carbonate, fiberglass, and carbon black."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)]",
                        "We work with global brands and their suppliers to predictably reduce the carbon footprint of everyday products without any retooling costs."
                    }
                }
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/imperium-masterbatch/sustainable-future-heartland.webp",
                        alt: "Sustainable future — Heartland Imperium masterbatch",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
        }
    }
}

#[component]
fn SupplyChainSection() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 md:py-28 my-12 section-soft-edges",
            div { class: "container-content",
                // Section header
                div { class: "text-center mb-16 max-w-3xl mx-auto",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Your Sustainability Partner"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight",
                        "Throughout The "
                        span { class: "text-gradient-red", "Supply Chain" }
                    }
                }

                // 4-step zigzag timeline — same shared component as landing
                // and /why-imperium. Copy is Imperium-Masterbatch specific.
                div { class: "relative",
                    div { class: "hidden md:block absolute left-1/2 top-0 bottom-0 -translate-x-1/2 w-px bg-gradient-to-b from-transparent via-[color:var(--color-accent)] to-transparent opacity-40" }

                    SupplyChainStep {
                        number: 1,
                        icon: SupplyIcon::Tractor,
                        heading: "Farmer",
                        body: "Heartland partners with corn, wheat, and soy farmers to embed industrial hemp into their crop rotation. Our USDA Grant has given us unique insights into industrial hemp farming, regenerative agriculture, and carbon sequestration.",
                        align_right: false,
                    }
                    SupplyChainStep {
                        number: 2,
                        icon: SupplyIcon::Blender,
                        heading: "Compounder",
                        body: "Heartland partners with plastic compounders to augment talc, calcium, and glass without any retooling costs. Our Imperium masterbatch solves dust, flammability, bonding, and bulk density problems typically associated with bio-based additives.",
                        align_right: true,
                    }
                    SupplyChainStep {
                        number: 3,
                        icon: SupplyIcon::Gears,
                        heading: "Converter",
                        body: "Heartland partners with plastic converters to ensure the hemp-filled resin is processed properly. Our team works alongside component part manufacturers to process natural fiber-filled plastic with the same molds used today.",
                        align_right: false,
                    }
                    SupplyChainStep {
                        number: 4,
                        icon: SupplyIcon::Store,
                        heading: "Brand",
                        body: "Heartland is the sustainability partner for brands on their journey to reduce their carbon footprint. Our team helps create stakeholder alignment so companies can effectively communicate the value of sustainable material innovation.",
                        align_right: true,
                    }
                }
            }
        }
    }
}
