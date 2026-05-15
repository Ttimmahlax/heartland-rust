use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn Team() -> Element {
    rsx! {
        Seo {
            title: "Heartland Team",
            description: "Heartland's team is building Earth's most sustainable company by enabling our customers to exceed their sustainability goals.",
            path: "/heartland-team",
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
                    "Heartland Team"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Heartland team"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "Heartland’s values are grounded in education, innovation, and collaboration. Every day we work alongside our partners to build a more sustainable future."
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
                h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 text-center", "Our Heartland To Yours" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland’s values are grounded in education, innovation, and collaboration. Every day we work alongside our partners to build a more sustainable future." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Become Earth's Most Sustainable Company" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Streamline Sustainable Material Innovation" }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/heartland-team/untitled-design-6.png", alt: "heartland executive tim almond", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Throughout The Supply Chain" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland has a team of scientists, engineers, and technologists working alongside our executive team and advisors to fulfill our commitment to sustainable innovation." }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/heartland-team/untitled-design-6.png",
                        alt: "Eric Austermann",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/heartland-team/foam-7.png", alt: "greentown labs heartland", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }        }
    }
}

#[component]
fn ClosingCta() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 my-12 section-soft-edges",
            div { class: "container-content text-center",
                h2 { class: "text-3xl md:text-4xl font-bold mb-6 max-w-2xl mx-auto",
                    "Learn more about Heartland"
                }
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Get in touch" }
            }
        }
    }
}
