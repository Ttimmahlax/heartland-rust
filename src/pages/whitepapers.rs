use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

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
            VideoBackground { slug: "whitepapers".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "White Papers"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Our sustainable future together"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "Heartland's team has developed white papers on decarbonized materials for manufacturers to begin their journey to reduce their carbon footprint."
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
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland's team has developed white papers on decarbonized materials for manufacturers to begin their journey to reduce their carbon footprint." }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Decarbonized Polypropylene (PP) For Global Manufacturers" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "80% of a plastic parts carbon footprint is locked in the ingredients used to make the plastic. The decarbonized polypropylene white paper outlines a systematic path to decarbonize plastic used throughout manufacturing." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "In this white paper, you will learn:" }
                }
                div { class: "md:order-2",
                    img {
                        src: "/assets/pages/whitepapers/white-paper-cover-rev1.jpg",
                        alt: "White Paper decarbonized polypropylene",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "One Farm At A Time" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland has unlocked the secrets to practical, proven, and profitable regenerative farming practices." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Regenerative agriculture practices have been around for 10,000+ years. But, over time, we have increased our consumption and reliance on chemicals. This has removed our need for the traditional practices our ancestors relied on for generations." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "In this e-book, you will learn:" }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/e-books/heartland-ebook-cover-regen-ag.jpg",
                        alt: "Heartland eBook Cover Regen Ag",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Lower Carbon Footprint & Lightweight Plastics" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Our society moved from metal to plastic to create stronger, lighter, and cheaper products. Now that we have commoditized plastics, what's next? Through data, it has become clear which resins will win long term." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "In this e-book, you will learn:" }
                }
                div { class: "md:order-2",
                    img {
                        src: "/assets/pages/e-books/heartland-ebook-cover-pof.jpg",
                        alt: "Heartland eBook Cover POF",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 mt-12 text-center", "Try Our Carbon Footprint Calculator" }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/whitepapers/plastic-pallets-5.png", alt: "hemp farming", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
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
