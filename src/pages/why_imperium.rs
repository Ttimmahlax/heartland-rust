use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
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
            VideoBackground { slug: "why-imperium".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "Why Imperium"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Natures Strongest Natural Fiber"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "Imperium farming requires 95% less water than cotton, unlocking water replenishment for brands."
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
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Sustainable Material Innovation" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium farming requires 95% less water than cotton, unlocking water replenishment for brands." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium is the only crop that can be grown anywhere; making soil more fertile, using no chemicals, replenishing water, and providing an economic benefit to rural small farms." }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/why-imperium/heartland-masterbatch.png",
                        alt: "heartland masterbatch",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Supercharging Biology To Make Better Products" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium has made nature compatible with advanced manufacturing and everyday products." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland has spent years understanding how to get Imperium to perform in modern manufacturing processes. Imperium is engineered to create unique performance that can't be seen from other mined or synthetic materials." }
                }
                div { class: "md:order-2",
                    img {
                        src: "/assets/pages/why-imperium/cellulose-research.png",
                        alt: "Supercharging Biology To Make Better Products",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "A Practical Path Forward" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium can be used as a drop-in replacement on existing manufacturing lines, eliminating capital investments to use sustainable materials." }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/why-imperium/imperium-textiles.png",
                        alt: "Imperium textiles",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "One Natural Fiber. Infinite Applications." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium fiber has a history of being one of the strongest natural fiber. We have unlocked today's practical use cases." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Functionalizing natural fibers to perform in modern manufacturing processes is an engineering feat. Imperium unlocks thousands of applications across plastics and textiles to systematically reduce the global carbon footprint." }
                }
                div { class: "md:order-2",
                    img {
                        src: "/assets/pages/why-imperium/hemp-reinforced-polypropylene.png",
                        alt: "hemp reinforced polypropylene",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/why-imperium/heartland-plastic-picture-3.png", alt: "Applications We Amplify", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/why-imperium/heartland-packaging-plastic-pallets.png", alt: "", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Lets Change The Tune, Nature Costs Less" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "For the first time in history, **Imperium unlocks** cost savings to use sustainable materials in everyday products." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland believes sustainability is a fancy term for efficiency. It is more sustainable to reduce the cost and time to make more durable products." }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/why-imperium/paper-8-7.png",
                        alt: "hemp baling drone",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "max-w-3xl mx-auto mb-16 animate-fade-in-up",
                h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 text-center", "Your Supply Chain Partner" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland partners with corn, wheat, and soy farmers to embed industrial hemp into their crop rotation. Our farming model enables us to promote local farming supporting local manufacturing." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Your Material Partner Heartland partners with material processors from every industry to augment existing materials without any retooling costs. Our Imperium product line is a drop in material alongside their existing processes to make stronger, lighter, less expensive products." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Your Converting Partner Heartland partners with raw material converters to ensure Imperium is processed properly. Our team works alongside finished goods manufacturers to process Imperium with no retooling of existing equipment." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Your Brand Partner Heartland is the decarbonization partner for brands on their journey to reduce their carbon footprint. Our team helps create stakeholder alignment so companies can effectively communicate the value of sustainable material innovation." }
            }        }
    }
}

#[component]
fn ClosingCta() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 my-12 section-soft-edges",
            div { class: "container-content text-center",
                h2 { class: "text-3xl md:text-4xl font-bold mb-6 max-w-2xl mx-auto",
                    "Learn more about Natures"
                }
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Get in touch" }
            }
        }
    }
}
