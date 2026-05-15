use dioxus::prelude::*;

use crate::components::decarb_solutions::DecarbSolutions;
use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::supply_chain::{SupplyChainStep, SupplyIcon};
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;

#[component]
pub fn ImperiumAnimalFeed() -> Element {
    rsx! {
        Seo {
            title: "Imperium Animal Feed",
            description: "Imperium Animal Feed is a highly nutritional hemp animal feed for chickens, and in the future cattle and pork.",
            path: "/imperium-animal-feed",
        }

        Hero {}
        LogoCarousel { heading: "" }
        Section2 {}
        Section4 {}
        DecarbSolutions {}
        SupplyChainSection6 {}
        NewsCarousel { heading: "Related Articles" }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section {
            class: "video-hero-section section-soft-bottom min-h-[110vh] flex items-center pb-[20vh]",
            VideoBackground { slug: "imperium-animal-feed".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "Imperium Animal Feed"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "natural nutriton"
                    br {}
                    "For livestock"
                }
            }
        }
    }
}

#[component]
fn Section2() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-10 md:gap-14 items-center",
                // Left: text
                div { class: "animate-fade-in-up md:order-1 order-2",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Naturally Enrich Your Animal Feed"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Unlocking Commodity "
                        span { class: "text-gradient-red", "Hemp Animal Feed" }
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "Imperium Animal Feed is a highly nutritious animal feed with proven immunotherapy benefits that is available for brands and ranchers looking for livestock resiliency. A low cost, natural alternative to enrich your current corn and soybean meal to reduce costs and raise healthier animals."
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/imperium-animal-feed/imperium-animal-feed-hemp-grain.png",
                        alt: "imperium animal feed hemp grain",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
        }
    }
}

#[component]
fn Section4() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-10 md:gap-14 items-center",
                // Left: text
                div { class: "animate-fade-in-up md:order-1 order-2",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Cost Savings"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "One Grain, To "
                        span { class: "text-gradient-red", "Rule Them All." }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "Imperium Animal Feed is Hemp Meal"
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "Hemp animal feed is the most versatile and nutritious grain on Earth. The production of hemp grain requires less water and no pesticides, alongside its immunotherapy benefits it allows ranchers to raise a healthier and less expensive livestock."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "Imperium Animal Feed can be fed to chickens and is pending approval for cattle and pork markets."
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/imperium-animal-feed/imperium-animal-feed-hemp-grain-silo.png",
                        alt: "imperium animal feed hemp grain silo",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
        }
    }
}

#[component]
fn SupplyChainSection6() -> Element {
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
                        heading: "Farmer Raising Hemp",
                        body: "Heartland partners with corn, wheat, and soy farmers to embed hemp grain into their crop rotation. Our USDA Grant has given us unique insights into industrial hemp farming, regenerative agriculture, and emissions reduction.",
                        align_right: false,
                    }
                    SupplyChainStep {
                        number: 2,
                        icon: SupplyIcon::Blender,
                        heading: "Ranchers Raising Livestock",
                        body: "Heartland partners with ranchers to produce resilient and high quality livestock for global customers raised on Imperium Animal Feed.",
                        align_right: true,
                    }
                    SupplyChainStep {
                        number: 3,
                        icon: SupplyIcon::Gears,
                        heading: "Livestock Processing",
                        body: "Heartland partners with livestock processors ensure they're able to successfully articulate the value add of healthier animals to their customers.",
                        align_right: false,
                    }
                    SupplyChainStep {
                        number: 4,
                        icon: SupplyIcon::Store,
                        heading: "Retail & Brands",
                        body: "Heartland is a decarbonization partner for brands on their journey to raise healthier livestock and reduce their carbon footprint. Our team helps create stakeholder alignment so companies can effectively communicate the value of Imperium Animal Feed.",
                        align_right: true,
                    }
                }
            }
        }
    }
}
