use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;

#[component]
pub fn Faq() -> Element {
    rsx! {
        Seo {
            title: "FAQ Page",
            description: "These are the frequently asked questions (FAQs) that Heartland's team receives from the agriculture, chemical, and manufacturing industries.",
            path: "/frequently-asked-questions",
        }

        Hero {}
        LogoCarousel { heading: "" }
        TitleBlock1 {}
        TitleBlock4 {}
        NewsCarousel { heading: "Related Articles" }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section {
            class: "video-hero-section section-soft-bottom min-h-[110vh] flex items-center pb-[20vh]",
            VideoBackground { slug: "frequently-asked-questions".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "FAQ Page"
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
                    "EVERYTHING YOU NEED"
                }
                h2 { class: "text-3xl md:text-5xl font-bold leading-tight",
                    "Frequently "
                    span { class: "text-gradient-red", "Asked Questions" }
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
                    "Meet Your Sustainability Partner"
                }
                h2 { class: "text-3xl md:text-5xl font-bold leading-tight",
                    "Connect With "
                    span { class: "text-gradient-red", "Our Team" }
                }
            }
        }
    }
}
