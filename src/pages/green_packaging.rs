use dioxus::prelude::*;

use crate::components::carbon_calculator::CarbonCalculator;
use crate::components::logo_carousel::LogoCarousel;
use crate::components::mats_amplify::MatsAmplify;
use crate::components::news_carousel::NewsCarousel;
use crate::components::supply_chain::{SupplyChainStep, SupplyIcon};
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;

#[component]
pub fn GreenPackaging() -> Element {
    rsx! {
        Seo {
            title: "GPI",
            description: "The Green Packaging Initiative was developed by Heartland to catalyze the adoption of sustainable products throughout the supply chain.",
            path: "/green-packaging-initiative",
        }

        Hero {}
        LogoCarousel { heading: "" }
        Section1 {}
        Section3 {}
        TitleBlock4 {}
        Section5 {}
        Section6 {}
        TitleBlock7 {}
        SupplyChainSection8 {}
        Section9 {}
        MatsAmplify {}
        CarbonCalculator {}
        NewsCarousel { heading: "Related Articles" }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section {
            class: "video-hero-section section-soft-bottom min-h-[110vh] flex items-center pb-[20vh]",
            VideoBackground { slug: "green-packaging-initiative".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "GPI"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Decarbonized Packaging"
                }
            }
        }
    }
}

#[component]
fn Section1() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-10 md:gap-14 items-center",
                // Left: text
                div { class: "animate-fade-in-up md:order-1 order-2",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Imperium Inside"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "A Simple First "
                        span { class: "text-gradient-red", "Step Toward Sustainability" }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "Natural fiber additives & recycled plastic are the easiest opportunities to decarbonize your supply chain."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "Every manufacturer on earth uses industrial packaging to ship and protect products in transit. This opens the door for sustainable materials to be easily adopted while providing a clear carbon footprint reduction in the supply chain."
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/green-packaging-initiative/heartland-lca-1.png",
                        alt: "green packaging initiative",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
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
                // Left: text
                div { class: "animate-fade-in-up md:order-1 order-2",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "No Retooling"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Same Packaging. "
                        span { class: "text-gradient-red", "Lower Carbon Footprint." }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "Heartland Imperium enables brands to use the same suppliers for packaging and resin."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "Additives are simple changes for plastic compounders. Heartland's materials help brands reduce the cost, weight, and carbon footprint of their packaging."
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/green-packaging-initiative/heartland-lca.png",
                        alt: "green packaging initiative pallets",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
        }
    }
}

#[component]
fn TitleBlock4() -> Element {
    rsx! {
        section { class: "container-content py-12 md:py-16",
            div { class: "max-w-3xl mx-auto text-center",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "Durable and Lightweight"
                }
                h2 { class: "text-3xl md:text-5xl font-bold leading-tight",
                    "Carbon Neutral "
                    span { class: "text-gradient-red", "Hemp Reinforced Pallets" }
                }
            }
        }
    }
}

#[component]
fn Section5() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-10 md:gap-14 items-center",
                // Left: text
                div { class: "animate-fade-in-up md:order-1 order-2",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Material Agnostic"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Meaningful Action Toward "
                        span { class: "text-gradient-red", "a Brighter Future" }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "All change begins with one step forward."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "Product development initiatives can take years to qualify. Brand owners and investors are looking for near-term opportunities to prove their commitment to a sustainable future."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "Reusable packaging is the key."
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/green-packaging-initiative/heartland-lca-2.png",
                        alt: "green packaging initiative pallets",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
        }
    }
}

#[component]
fn Section6() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-10 md:gap-14 items-center",
                // Left: text
                div { class: "animate-fade-in-up md:order-1 order-2",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Unlocking Natural Fibers"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Making The "
                        span { class: "text-gradient-red", "Impractical, Optimal" }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "For the first time in history, it costs less to use sustainable materials in industrial packaging."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "The ability to work alongside mined and synthetic materials is mission-critical to the adoption of natural fibers. Making our carbon-negative additives compatible with today's materials is part of our secret sauce."
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/why-imperium/paper-8-7.png",
                        alt: "hemp baling drone",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
        }
    }
}

#[component]
fn TitleBlock7() -> Element {
    rsx! {
        section { class: "container-content py-12 md:py-16",
            div { class: "max-w-3xl mx-auto text-center",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "No Retooling Costs"
                }
                h2 { class: "text-3xl md:text-5xl font-bold leading-tight",
                    "The "
                    span { class: "text-gradient-red", "Sustainable Path" }
                }
            }
        }
    }
}

#[component]
fn SupplyChainSection8() -> Element {
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
                        heading: "Your Farming Partner",
                        body: "Heartland partners with corn, wheat, and soy farmers to embed industrial hemp into their crop rotation. Our farming model enables us to promote local farming supporting local manufacturing.",
                        align_right: false,
                    }
                    SupplyChainStep {
                        number: 2,
                        icon: SupplyIcon::Blender,
                        heading: "Your Additive Partner",
                        body: "Heartland partners with plastic compounders to augment talc, calcium, and glass without any retooling costs. Our Imperium masterbatch solves dust, flammability, bonding, and bulk density problems typically associated with bio-based additives.",
                        align_right: true,
                    }
                    SupplyChainStep {
                        number: 3,
                        icon: SupplyIcon::Gears,
                        heading: "Your Converting Partner",
                        body: "Heartland partners with plastic converters to ensure the hemp-filled resin is processed properly. Our team works alongside component part manufacturers to process natural fiber-filled plastic with the same molds used today.",
                        align_right: false,
                    }
                    SupplyChainStep {
                        number: 4,
                        icon: SupplyIcon::Store,
                        heading: "Your Brand Partner",
                        body: "Heartland is the sustainability partner for brands on their journey to reduce their carbon footprint. Our team helps create stakeholder alignment so companies can effectively communicate the value of sustainable material innovation.",
                        align_right: true,
                    }
                }
            }
        }
    }
}

#[component]
fn Section9() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-10 md:gap-14 items-center",
                // Left: text
                div { class: "animate-fade-in-up md:order-1 order-2",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Regenerative Agriculture"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Local Farming "
                        span { class: "text-gradient-red", "Empowering Local Manufacturing" }
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "Heartland is the market leader in industrial hemp production in the United States. Our team has designed a farming and processing architecture that reduces production costs by more than 50%. This is our key to maintaining price points while reducing weight and carbon footprint."
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/why-imperium/regenerative-agriculture-hemp-vertical-tilling-4.png",
                        alt: "",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
        }
    }
}
