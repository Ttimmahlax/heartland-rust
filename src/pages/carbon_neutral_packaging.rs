use dioxus::prelude::*;

use crate::components::carbon_calculator::CarbonCalculator;
use crate::components::decarb_solutions::DecarbSolutions;
use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::supply_chain::{SupplyChainStep, SupplyIcon};
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;

#[component]
pub fn CarbonNeutralPackaging() -> Element {
    rsx! {
        Seo {
            title: "Carbon Neutral Packaging With Imperium Inside",
            description: "Sustainability means it costs less and improves your products, period. We believe new materials should exceed all criteria.",
            path: "/carbon-neutral-packaging-with-imperium-inside",
        }

        Hero {}
        Section2 {}
        LogoCarousel { heading: "" }
        TitleBlock {}
        Section3 {}
        SupplyChainSection {}
        DecarbSolutions {}
        CarbonCalculator {}
        NewsCarousel { heading: "Related Articles" }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section {
            class: "video-hero-section section-soft-bottom min-h-[110vh] flex items-center pb-[20vh]",
            VideoBackground { slug: "sustainable-packaging".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "Carbon Neutral Packaging With Imperium Inside"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Decarbonized"
                    br {}
                    "packaging"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "Carbon-neutral pallets and bins that reduce cost"
                }
            }
        }
    }
}

#[component]
fn Section2() -> Element {
    rsx! {
        section { class: "container-content pt-16 md:pt-24 pb-16 md:pb-20",
            div { class: "flex justify-center mb-20 md:mb-24",
                img {
                    src: "/assets/brand/imperium-logo.png",
                    alt: "Imperium",
                    loading: "lazy",
                    class: "h-20 md:h-24 w-auto",
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 md:gap-14 items-center",
                // Left: text
                div { class: "animate-fade-in-up md:order-1 order-2",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Imperium-Reinforced Pallets"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Same Packaging. Lower Cost. "
                        span { class: "text-gradient-red", "Neutral Carbon Footprint." }
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "Brands and suppliers can now replace international wood pallets with carbon-neutral pallets while reducing costs. Imperium-reinforced pallets allow brands and suppliers to quickly exceed sustainability mandates."
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/carbon-neutral-packaging-with-imperium-inside/carbon-neutral-plastic-pallet-48-x-40-imperium-inside-nest-us1.1-1.webp",
                        alt: "Carbon-neutral plastic pallet with Imperium inside",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
        }
    }
}

#[component]
fn TitleBlock() -> Element {
    rsx! {
        section { class: "container-content py-12 md:py-16",
            div { class: "max-w-3xl mx-auto text-center",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "Sustainable Material Innovation"
                }
                h2 { class: "text-3xl md:text-5xl font-bold leading-tight",
                    "Built To "
                    span { class: "text-gradient-red", "Stack Up" }
                }
            }
        }
    }
}

#[component]
fn Section3() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-10 md:gap-14 items-center",
                // Left: image
                div { class: "animate-fade-in-up md:order-1 order-2",
                    img {
                        src: "/assets/pages/hemp-fiber-and-hurd/heartland-packaging-plastic-pallets.webp",
                        alt: "Heartland packaging plastic pallets",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
                // Right: text
                div { class: "animate-fade-in-up md:order-2 order-1",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Sustainable Solutions"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Stronger, Lighter, "
                        span { class: "text-gradient-red", "And Lower Cost" }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "Lower-carbon-footprint materials engineered for mass manufacturing."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "Heartland supplies lower-carbon-footprint materials to help manufacturers create stronger, lighter, cheaper, and more sustainable products — no retooling required."
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
                div { class: "text-center mb-16 max-w-3xl mx-auto",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Your Sustainability Partner"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight",
                        "Throughout The "
                        span { class: "text-gradient-red", "Supply Chain" }
                    }
                }
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
