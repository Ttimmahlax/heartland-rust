use dioxus::prelude::*;

use crate::components::carbon_calculator::CarbonCalculator;
use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;

#[component]
pub fn Whitepapers() -> Element {
    rsx! {
        Seo {
            title: "White Papers",
            description: "Heartland is a material innovation company that engineers high performance carbon negative additives for plastic and other raw materials.",
            path: "/whitepapers",
        }

        Hero {}
        LogoCarousel { heading: "" }
        TitleBlock1 {}
        Section2 {}
        Section3 {}
        Section4 {}
        CarbonCalculator {}
        NewsCarousel { heading: "Related Articles" }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section {
            class: "video-hero-section section-soft-bottom min-h-[110vh] flex items-center pb-[20vh]",
            VideoBackground { slug: "whitepapers".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "White Papers"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Our sustainable"
                    br {}
                    "future together"
                }
            }
        }
    }
}

#[component]
fn TitleBlock1() -> Element {
    rsx! {
        section { class: "container-content py-12 md:py-16",
            div { class: "max-w-3xl mx-auto text-center",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "Thought Leadership"
                }
                h2 { class: "text-3xl md:text-5xl font-bold leading-tight",
                    "From "
                    span { class: "text-gradient-red", "The Heartland" }
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
                        "White Paper"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Decarbonized Polypropylene (PP) "
                        span { class: "text-gradient-red", "For Global Manufacturers" }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "80% of a plastic parts carbon footprint is locked in the ingredients used to make the plastic. The decarbonized polypropylene white paper outlines a systematic path to decarbonize plastic used throughout manufacturing."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "In this white paper, you will learn:"
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/whitepapers/white-paper-cover-rev1.jpg",
                        alt: "White Paper decarbonized polypropylene",
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
                        "Engineering Earth"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "One Farm "
                        span { class: "text-gradient-red", "At A Time" }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "Heartland has unlocked the secrets to practical, proven, and profitable regenerative farming practices."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "Regenerative agriculture practices have been around for 10,000+ years. But, over time, we have increased our consumption and reliance on chemicals. This has removed our need for the traditional practices our ancestors relied on for generations."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "In this e-book, you will learn:"
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/e-books/heartland-ebook-cover-regen-ag.jpg",
                        alt: "Heartland eBook Cover Regen Ag",
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
                        "Sustainable Plastic"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Lower Carbon Footprint "
                        span { class: "text-gradient-red", "& Lightweight Plastics" }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "Our society moved from metal to plastic to create stronger, lighter, and cheaper products. Now that we have commoditized plastics, what's next? Through data, it has become clear which resins will win long term."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "In this e-book, you will learn:"
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/e-books/heartland-ebook-cover-pof.jpg",
                        alt: "Heartland eBook Cover POF",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
        }
    }
}
