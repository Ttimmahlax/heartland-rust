use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn CaseStudies() -> Element {
    rsx! {
        Seo {
            title: "Case Studies",
            description: "Heartland is a material innovation company that engineers high performance carbon negative additives for plastic and other raw materials.",
            path: "/case-studies",
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
            VideoBackground { slug: "landing".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "Case Studies"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Our sustainable future together"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "Heartland's team has worked alongside earth's largest brands and suppliers in product development. Below is some of the research our clients have shared with us."
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
                h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 text-center", "From The Heartland" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland's team has worked alongside earth's largest brands and suppliers in product development. Below is some of the research our clients have shared with us." }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "A Leap Forward In Sustainable Material Innovation" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "This case study is based on research and development from a multi-billion dollar European building material supplier. The PVC industry has been looking for sustainable alternatives for decades. Their findings show that Heartland's materials have the ability to maintain the performance and cost of PVC while reducing weight and carbon footprint." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "In this case study, you will find:" }
                }
                div { class: "md:order-2",
                    img { src: "/assets/pages/case-studies/pvc-case-studies.webp", alt: "A Leap Forward In Sustainable Material Innovation", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Upgrading The World's Commodity Thermoplastic" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland has successfully engineered hemp fibers as a drop-in replacement for talc." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "This case study is based on research and development from a multi-billion dollar Asian chemical company. Their findings show that Heartland's materials have the ability to maintain the performance and cost of polypropylene while reducing weight and carbon footprint. Our team has Imperium-filled resin formulations that outperform 20% talc-filled resins." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "In this case study, you will find:" }
                }
                div { class: "md:order-1",
                    img { src: "/assets/pages/case-studies/polypropylene-case-study-1.webp", alt: "Upgrading The World's Commodity Thermoplastic", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 mt-12 text-center", "Try Our Hemp Filled Plastic Calculator" }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/case-studies/rubber-11.webp", alt: "plastic of the future", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 mt-12 text-center", "PROJECTS COMPLATED" }        }
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
