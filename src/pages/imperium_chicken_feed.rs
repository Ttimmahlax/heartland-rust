use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn ImperiumChickenFeed() -> Element {
    rsx! {
        Seo {
            title: "Imperium Chicken Feed",
            description: "Imperium Chicken Feed is currently shipping throughout North America in full containers to ranchers looking for natural nutrition.",
            path: "/imperium-chicken-feed",
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
                    "Imperium Chicken Feed"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "natural nutriton For livestock"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "Imperium Chicken Feed is for ranchers looking for natural nutrition to raise more resilient and healthy livestock."
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
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Imperium Chicken Feed Available In North America" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium Chicken Feed is for ranchers looking for natural nutrition to raise more resilient and healthy livestock." }
                }
                div { class: "md:order-1",
                    img { src: "/assets/pages/imperium-pork-feed/imperium-animal-feed-hemp-grain.png", alt: "imperium animal feed hemp grain", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Natural Nutrition That Saves You Money" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium Chicken Feed is Hemp Grain." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Hemp grain is one of the most versatile food sources on Earth. The production of hemp grain less water and no pesticides to produce one of the most nutritious grains available." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium Chicken Feed can be used for egg laying hens." }
                }
                div { class: "md:order-2",
                    img { src: "/assets/pages/wood-products/1210x786-px-4.png", alt: "heartland hemp bales", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
                }
            }
            h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 mt-12 text-center", "Connect With Our Team" }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/imperium-pork-feed/imperium-cattle-feed.png", alt: "Imperium Cattle Feed", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/imperium-chicken-feed/industry-5.0-article-series-1210-x-786-px-8.png", alt: "Image of a computer screen with the title, Chief Sustainability Officer displayed", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }        }
    }
}

#[component]
fn ClosingCta() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 my-12 section-soft-edges",
            div { class: "container-content text-center",
                h2 { class: "text-3xl md:text-4xl font-bold mb-6 max-w-2xl mx-auto",
                    "Learn more about natural"
                }
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Get in touch" }
            }
        }
    }
}
