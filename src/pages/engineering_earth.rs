use dioxus::prelude::*;

use crate::components::carbon_calculator::CarbonCalculator;
use crate::components::logo_carousel::LogoCarousel;
use crate::components::mats_amplify::MatsAmplify;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;

#[component]
pub fn EngineeringEarth() -> Element {
    rsx! {
        Seo {
            title: "Engineering Earth",
            description: "Engineering Earth is focused on supercharging the greatest carbon sequestration vehicle we have: our planet.",
            path: "/engineering-earth",
        }

        Hero {}
        LogoCarousel { heading: "" }
        Section1 {}
        Section2 {}
        Section4 {}
        Section5 {}
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
            VideoBackground { slug: "engineering-earth".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "Engineering Earth"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Engineering Earth One"
                    br {}
                    "Farm at a time"
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
                        "Lungs Of The Earth"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Empowering Earth, Our "
                        span { class: "text-gradient-red", "Most Effective Tool." }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "Earth is a carbon sequestration powerhouse. Our planet’s rainforests act as its lungs, and its soil acts as its skin."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "Engineering Earth to optimize the carbon cycle is the most effective solution we have to create a sustainable future. Farmers are the key."
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/engineering-earth/mining.webp",
                        alt: "rainforest",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
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
                // Left: image
                div { class: "animate-fade-in-up md:order-1 order-1",
                    img {
                        src: "/assets/pages/engineering-earth/mining-1.webp",
                        alt: "farmland 1",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
                // Right: text
                div { class: "animate-fade-in-up md:order-2 order-2",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "25% Of All Biodiversity"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Farmland is Four Times "
                        span { class: "text-gradient-red", "Bigger Than The Amazon" }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "Agricultural practices have the potential to help or harm our planet more than the Amazon."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "4.25 billion acres on Earth are farmland. Implementing regenerative farming practices can empower Earth to store more carbon in less time than any manmade technology."
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
                        "Stronger Farms For Everyone"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Unlocking The Sustainable "
                        span { class: "text-gradient-red", "Future We Have Earned" }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "Regenerative agriculture can empower Earth to store more carbon in the soil than any technology."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "We work with global brands and their suppliers to understand the opportunity and impact of carbon farming."
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/engineering-earth/mining-2.webp",
                        alt: "farmhouse",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
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
                // Left: image
                div { class: "animate-fade-in-up md:order-1 order-1",
                    img {
                        src: "/assets/pages/engineering-earth/mining-3.webp",
                        alt: "tesla",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
                // Right: text
                div { class: "animate-fade-in-up md:order-2 order-2",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Looking To The Future"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "The Electric Vehicle Transition "
                        span { class: "text-gradient-red", "Brings Uncertainty To Ethanol" }
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "There are 40 million acres of corn farmed for ethanol here in America. By 2030, every major car manufacturer has committed to only making electric vehicles. This will create a significant reduction in the demand for ethanol."
                    }
                }
            }
        }
    }
}
