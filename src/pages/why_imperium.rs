use dioxus::prelude::*;

use crate::components::carbon_calculator::CarbonCalculator;
use crate::components::decarb_solutions::DecarbSolutions;
use crate::components::logo_carousel::LogoCarousel;
use crate::components::markets_amplify::MarketsAmplify;
use crate::components::news_carousel::NewsCarousel;
use crate::components::supply_chain::{SupplyChainStep, SupplyIcon};
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn WhyImperium() -> Element {
    rsx! {
        Seo {
            title: "Why Imperium",
            description: "Imperium is the carbon negative additive of choice for manufacturers to improve strength, reduce cost and make products most sustainable.",
            path: "/why-imperium",
        }

        Hero {}
        ItsInOurRoots {}
        LogoCarousel { heading: "" }
        SuperchargingBiology {}
        DecarbSolutions {}
        OneNaturalFiber {}
        MarketsAmplify {
            eyebrow: "Sustainable Material Innovation".to_string(),
            heading: "Applications We Amplify".to_string(),
        }
        LetsChangeTheTune {}
        YourSupplyChainPartner {}
        LocalFarmingLocalManufacturing {}
        CarbonCalculator {}
        NewsCarousel { heading: "Related Articles" }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section {
            class: "video-hero-section section-soft-bottom min-h-[110vh] flex items-center pb-[20vh]",
            VideoBackground { slug: "why-imperium".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "Why Imperium"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Natures Strongest"
                    br {}
                    "Natural Fiber"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "95% less water than cotton"
                }
            }
        }
    }
}

#[component]
fn ItsInOurRoots() -> Element {
    rsx! {
        section { class: "container-content pt-16 md:pt-24 pb-4 md:pb-8",
            // Centered Imperium logo above the headline — 2× bigger now, more
            // bottom space so the headline feels distinct from the logo.
            div { class: "flex justify-center mb-20 md:mb-24",
                img {
                    src: "/assets/brand/imperium-logo.png",
                    alt: "Imperium",
                    loading: "lazy",
                    class: "h-20 md:h-24 w-auto",
                }
            }

            // 2-col: image LEFT, text RIGHT — matches the PracticalPathForward
            // template so all 2-col sections on this page align consistently.
            div { class: "grid md:grid-cols-2 gap-10 md:gap-14 items-center",
                // Left: image
                div { class: "animate-fade-in-up md:order-1 order-2",
                    img {
                        src: "/assets/pages/why-imperium/heartland-masterbatch.png",
                        alt: "Heartland masterbatch — Imperium-filled pellets ready for compounding",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
                // Right: text
                div { class: "animate-fade-in-up md:order-2 order-1",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "It's In Our Roots"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Sustainable "
                        span { class: "text-gradient-red", "Material Innovation" }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "95% less water than cotton."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)]",
                        "Imperium is the only crop that can be grown anywhere; making soil more fertile, using no chemicals, replenishing water, and providing an economic benefit to rural small farms."
                    }
                }
            }
        }
    }
}

#[component]
fn SuperchargingBiology() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            // Text LEFT, image RIGHT (flipped per request).
            div { class: "grid md:grid-cols-2 gap-10 md:gap-14 items-center",
                // Left: text
                div { class: "animate-fade-in-up md:order-1 order-2",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "With Imperium Inside"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Supercharging "
                        span { class: "text-gradient-red", "Biology" }
                        br { class: "hidden md:inline" }
                        " To Make Better Products"
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "Imperium has made nature compatible with advanced manufacturing and everyday products."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)]",
                        "Heartland has spent years understanding how to get Imperium to perform in modern manufacturing processes. Imperium is engineered to create unique performance that can't be seen from other mined or synthetic materials."
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/why-imperium/cellulose-research.png",
                        alt: "Cellulose research — Imperium-grade fiber engineering",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
        }
    }
}

#[component]
fn OneNaturalFiber() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            // 2-col side-by-side matching heartland.io: text LEFT, image RIGHT.
            div { class: "grid md:grid-cols-2 gap-10 md:gap-14 items-center",
                // Left: text
                div { class: "animate-fade-in-up md:order-1 order-2",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "All Everyday Products"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "One Natural Fiber."
                        br {}
                        span { class: "text-gradient-red", "Infinite Applications." }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "Imperium fiber has a history of being one of the strongest natural fiber. We have unlocked today's practical use cases."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)]",
                        "Functionalizing natural fibers to perform in modern manufacturing processes is an engineering feat. Imperium unlocks thousands of applications across plastics and textiles to systematically reduce the global carbon footprint."
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/why-imperium/hemp-reinforced-polypropylene.png",
                        alt: "Hemp-reinforced polypropylene — automotive interior part",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
        }
    }
}

#[component]
fn LetsChangeTheTune() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-10 md:gap-14 items-center",
                // Left: image
                div { class: "animate-fade-in-up md:order-1 order-2",
                    img {
                        src: "/assets/pages/why-imperium/paper-8-7.png",
                        alt: "Hemp baling drone — Heartland's fiber supply chain",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
                // Right: text
                div { class: "animate-fade-in-up md:order-2 order-1",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Unlocking Natural Fibers"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Lets Change The Tune, "
                        span { class: "text-gradient-red", "Nature Costs Less" }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "For the first time in history, Imperium unlocks cost savings to use sustainable materials in everyday products."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)]",
                        "Heartland believes sustainability is a fancy term for efficiency. It is more sustainable to reduce the cost and time to make more durable products."
                    }
                }
            }
        }
    }
}

#[component]
fn YourSupplyChainPartner() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 md:py-28 my-12 section-soft-edges",
            div { class: "container-content",
                // Section header
                div { class: "text-center mb-16 max-w-3xl mx-auto",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Why Heartland"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight",
                        "Your "
                        span { class: "text-gradient-red", "Supply Chain Partner" }
                    }
                }

                // 4-step zigzag timeline — reuses SupplyChainStep from
                // components/supply_chain.rs so the visual style matches the
                // "Throughout The Supply Chain" timeline on the landing page.
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
                        heading: "Your Material Partner",
                        body: "Heartland partners with material processors from every industry to augment existing materials without any retooling costs. Our Imperium product line is a drop-in material alongside their existing processes to make stronger, lighter, less expensive products.",
                        align_right: true,
                    }
                    SupplyChainStep {
                        number: 3,
                        icon: SupplyIcon::Gears,
                        heading: "Your Converting Partner",
                        body: "Heartland partners with raw material converters to ensure Imperium is processed properly. Our team works alongside finished goods manufacturers to process Imperium with no retooling of existing equipment.",
                        align_right: false,
                    }
                    SupplyChainStep {
                        number: 4,
                        icon: SupplyIcon::Store,
                        heading: "Your Brand Partner",
                        body: "Heartland is the decarbonization partner for brands on their journey to reduce their carbon footprint. Our team helps create stakeholder alignment so companies can effectively communicate the value of sustainable material innovation.",
                        align_right: true,
                    }
                }
            }
        }
    }
}

#[component]
fn LocalFarmingLocalManufacturing() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            // 2-col: text LEFT, image RIGHT — matches the rest of this page.
            div { class: "grid md:grid-cols-2 gap-10 md:gap-14 items-center",
                // Left: text
                div { class: "animate-fade-in-up md:order-1 order-2",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Regenerative Agriculture"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Local Farming Empowering "
                        span { class: "text-gradient-red", "Local Manufacturing" }
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)]",
                        "Heartland is the market leader in industrial hemp production in the United States. Our goal is to localize farming of Imperium to our customers, reducing costs and carbon footprint at the same time."
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/why-imperium/regenerative-agriculture-hemp-vertical-tilling-4.png",
                        alt: "Regenerative agriculture — hemp vertical tilling on the Heartland farm network",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
        }
    }
}
