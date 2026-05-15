use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

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
            VideoBackground { slug: "imperium-animal-feed".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "Imperium Animal Feed"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "natural nutriton For livestock"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "Imperium Animal Feed is a highly nutritious animal feed with proven immunotherapy benefits that is available for brands and ranchers looking for livestock resiliency. A low cost, natural alternative…"
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
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Unlocking Commodity Hemp Animal Feed" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium Animal Feed is a highly nutritious animal feed with proven immunotherapy benefits that is available for brands and ranchers looking for livestock resiliency. A low cost, natural alternative to enrich your current corn and soybean meal to reduce costs and raise healthier animals." }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/imperium-animal-feed/imperium-animal-feed-hemp-grain.png",
                        alt: "imperium animal feed hemp grain",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "One Grain, To Rule Them All." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium Animal Feed is Hemp Meal" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Hemp animal feed is the most versatile and nutritious grain on Earth. The production of hemp grain requires less water and no pesticides, alongside its immunotherapy benefits it allows ranchers to raise a healthier and less expensive livestock." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium Animal Feed can be fed to chickens and is pending approval for cattle and pork markets." }
                }
                div { class: "md:order-2",
                    img {
                        src: "/assets/pages/imperium-animal-feed/imperium-animal-feed-hemp-grain-silo.png",
                        alt: "imperium animal feed hemp grain silo",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/imperium-animal-feed/imperium-cattle-feed.png", alt: "Imperium Cattle Feed", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            div { class: "max-w-3xl mx-auto mb-16 animate-fade-in-up",
                h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 text-center", "Throughout The Supply Chain" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland partners with corn, wheat, and soy farmers to embed hemp grain into their crop rotation. Our USDA Grant has given us unique insights into industrial hemp farming, regenerative agriculture, and emissions reduction." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Ranchers Raising Livestock Heartland partners with ranchers to produce resilient and high quality livestock for global customers raised on Imperium Animal Feed." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Livestock Processing Heartland partners with livestock processors ensure they're able to successfully articulate the value add of healthier animals to their customers." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Retail & Brands Heartland is a decarbonization partner for brands on their journey to raise healthier livestock and reduce their carbon footprint. Our team helps create stakeholder alignment so companies can effectively communicate the value of Imperium Animal Feed." }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/imperium-animal-feed/decarbonize-plastic-automotive.png", alt: "decarbonize plastic automotive", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
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
