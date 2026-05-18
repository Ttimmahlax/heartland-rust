use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::textile_solutions::TextileSolutions;
use crate::components::video_hero::VideoBackground;
use crate::popups::TextileFiberIndexPopup;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn ImperiumYarn() -> Element {
    rsx! {
        Seo {
            title: "Imperium Yarn",
            description: "Hemp yarn shipping globally for knit and woven fabric manufacturers — apparel, furniture, and home textiles with a sustainable origin story.",
            path: "/imperium-yarn",
        }

        Hero {}
        LogoCarousel { heading: "" }
        SectionShipping {}
        SectionOneFiber {}
        TextileSolutions {}
        ClosingCta {}
        NewsCarousel { heading: "Related Articles" }
        TextileFiberIndexPopup {}
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section {
            class: "video-hero-section section-soft-bottom min-h-[110vh] flex items-center pb-[20vh]",
            VideoBackground { slug: "imperium-fibers".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "Imperium Yarn"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Hemp Yarn For"
                    br {}
                    "Knit & Woven Fabric"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "Low-cost, high-performance hemp yarn shipping globally to fabric mills — with a story your brand will be proud to tell."
                }
            }
        }
    }
}

#[component]
fn SectionShipping() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-10 md:gap-14 items-center",
                div { class: "animate-fade-in-up md:order-2 order-2",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "For Knitting & Weaving"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Imperium Yarn "
                        span { class: "text-gradient-red", "Shipping Globally" }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "Imperium Yarn is engineered for knit and woven fabric manufacturers."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "Spun from Imperium hemp fiber, our yarn delivers the softness, strength, and breathability brands need — at a price point that competes with conventional fibers."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "We ship in full containers to the world's top textile manufacturing regions."
                    }
                }
                div { class: "animate-fade-in-up md:order-1 order-1",
                    img {
                        src: "/assets/pages/imperium-yarn/imperium-yarn-1.webp",
                        alt: "Imperium Yarn",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
        }
    }
}

#[component]
fn SectionOneFiber() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-10 md:gap-14 items-center",
                div { class: "animate-fade-in-up md:order-1 order-2",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "95% Less Water Than Cotton"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "One Natural Fiber "
                        span { class: "text-gradient-red", "To Rule Them All" }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "Hemp is the most versatile natural fiber on Earth."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "Imperium hemp fiber is grown without pesticides, uses 95% less water than cotton, and regenerates the soil it grows in — all while delivering performance that rivals synthetics."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4 last:mb-0",
                        "Use it in textiles, furniture, and composites."
                    }
                }
                div { class: "animate-fade-in-up md:order-2 order-1",
                    img {
                        src: "/assets/pages/imperium-yarn/1210x786-px-4.webp",
                        alt: "Heartland hemp bales",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
        }
    }
}

#[component]
fn ClosingCta() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 md:py-28 my-12 section-soft-edges",
            div { class: "container-content text-center max-w-3xl mx-auto",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "Meet Your Sustainability Partner"
                }
                h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                    "Get Imperium Yarn "
                    span { class: "text-gradient-red", "On Your Floor" }
                }
                p { class: "text-lg text-[color:var(--color-fg-muted)] mb-8",
                    "Talk to Heartland's textile team about volumes, color and blend options, and how Imperium drops into your existing knit and weave lines."
                }
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Get in touch" }
            }
        }
    }
}
