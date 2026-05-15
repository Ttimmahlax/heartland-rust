use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn ImperiumYarn() -> Element {
    rsx! {
        Seo {
            title: "Imperium Yarn",
            description: "Imperium Yarn is currently shipping globally in full containers to fabric manufacturers for clothing, furniture, and more.",
            path: "/imperium-yarn",
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
                    "Imperium Yarn"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "The Softest Hemp fiber on earth"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "Imperium Yarn is for knit and woven fabric manufacturers looking for low-cost, high performance hemp fiber with a great origin story."
                }
            }
        }
    }
}

#[component]
fn Body() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Imperium Yarn Shipping Globally" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium Yarn is for knit and woven fabric manufacturers looking for low-cost, high performance hemp fiber with a great origin story." }
                }
                div { class: "md:order-1",
                    img { src: "/assets/pages/imperium-yarn/imperium-yarn-1.webp", alt: "Imperium Yarn", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "One Natural Fiber To Rule Them All" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium Fiber is hemp fiber." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Hemp fiber is one of the most versatile fibers on Earth. The production of hemp fiber requires 95% less water than cotton while reducing chemical runoff into our local water supplies." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium hemp fiber can be used in textiles or composites." }
                }
                div { class: "md:order-2",
                    img { src: "/assets/pages/hemp-fiber-and-hurd/1210x786-px-4.webp", alt: "heartland hemp bales", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 mt-12 text-center", "Connect With Our Team" }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/imperium-yarn/imperium-bulk-spin-ready-fiber.webp", alt: "Imperium Bulk Spin Ready Fiber", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/imperium-pork-feed/industry-5.0-article-series-1210-x-786-px-6.webp", alt: "Industry 5.0 Article Series", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }        }
    }
}

#[component]
fn ClosingCta() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 my-12 section-soft-edges",
            div { class: "container-content text-center",
                h2 { class: "text-3xl md:text-4xl font-bold mb-6 max-w-2xl mx-auto",
                    "Learn more about The"
                }
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Get in touch" }
            }
        }
    }
}
