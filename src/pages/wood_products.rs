use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn WoodProducts() -> Element {
    rsx! {
        Seo {
            title: "Wood Products",
            description: "Heartland wood products are made from Imperium Fibers, designed as sustainable, low-cost alternatives to wood boards.",
            path: "/wood-products",
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
                    "Wood Products"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Not wood makes better wood"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "Heartland Wood Products are carbon-negative wood alternatives that are so much better than wood."
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
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Unearthing Natural Fibers True Potential" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium Fibers make better wood products than wood." }
                }
                div { class: "md:order-1",
                    img { src: "/assets/pages/wood-products/heartland-powder.webp", alt: "Unearthing Natural Fibers True Potential", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "One Natural Fiber. Unlimited Applications." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland Wood Products are carbon-negative wood alternatives that are so much better than wood." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland wood products look like wood, cut like wood, and quack like wood. Surprisingly, not wood." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium Fibers make wood products less expensive, a lot stronger, better for the world, and frankly, way cooler. Not wood, remember it." }
                }
                div { class: "md:order-2",
                    img { src: "/assets/pages/wood-products/1210x786-px-4.webp", alt: "heartland hemp bales", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Unlocking The Sustainable Future We Need And Deserve" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland's materials replace and augment forestry products." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "We work with global brands and their suppliers to predictably reduce the carbon footprint of everyday products without any retooling costs." }
                }
                div { class: "md:order-1",
                    img { src: "/assets/pages/wood-products/untitled-1027-768-px-1210-768-px.webp", alt: "heartland wood products replacing forestry", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/wood-products/heartland-marine-1.webp", alt: "heartland building materials 2", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 mt-12 text-center", "PROJECTS COMPLATED" }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/wood-products/heartland-lca.webp", alt: "heartland magnolia partners", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }        }
    }
}

#[component]
fn ClosingCta() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 my-12 section-soft-edges",
            div { class: "container-content text-center",
                h2 { class: "text-3xl md:text-4xl font-bold mb-6 max-w-2xl mx-auto",
                    "Learn more about Not"
                }
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Get in touch" }
            }
        }
    }
}
