use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

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
            VideoBackground { slug: "frequently-asked-questions".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "FAQ Page"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Our sustainable future together"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "Below are a few frequently asked questions our team receives from the agriculture, chemical, manufacturing, and sustainability industries."
                }
            }
        }
    }
}

#[component]
fn Body() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "max-w-3xl mx-auto mb-16 animate-fade-in-up",
                h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 text-center", "Frequently Asked Questions" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Below are a few frequently asked questions our team receives from the agriculture, chemical, manufacturing, and sustainability industries." }
            }
            div { class: "max-w-3xl mx-auto mb-16 animate-fade-in-up",
                h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 text-center", "" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Today, our hemp fiber is grown in Michigan and 6 other states. We also purchase bales of hemp off the spot market through a reliable QA / QC process." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "You can fill out the contact form below to connect with one of our team members. From there, we can determine if sample materials are appropriate." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "So far, companies have used Heartland’s materials as additives in plastic, rubber, foam, asphalt, concrete, paper, and ceramic. Every day we get new ideas and projects in our inbox. Feel free to reach out to see if your application is a fit." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland works with brands and their suppliers to embed natural fibers into their supply chain to reduce cost, weight, and carbon footprint without any retooling costs." }
            }        }
    }
}

#[component]
fn ClosingCta() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 my-12 section-soft-edges",
            div { class: "container-content text-center",
                h2 { class: "text-3xl md:text-4xl font-bold mb-6 max-w-2xl mx-auto",
                    "Learn more about Our"
                }
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Get in touch" }
            }
        }
    }
}
