use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn ImperiumFibers() -> Element {
    rsx! {
        Seo {
            title: "Imperium Fibers",
            description: "Imperium Fiber is engineered hemp fiber designed to reduce the cost, weight, and carbon footprint of existing raw materials.",
            path: "/imperium-fibers",
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
            VideoBackground { slug: "imperium-fibers".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "Imperium Fibers"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "The Softest Hemp fiber on earth"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "Imperium Fiber is available for brands, converters, and suppliers that are need a high volume, consistent supply of hemp fiber for yarn and fabric manufacturing."
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
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Butter Soft Hemp Fiber With Incredible Strength" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium Fiber is available for brands, converters, and suppliers that are need a high volume, consistent supply of hemp fiber for yarn and fabric manufacturing." }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/imperium-fibers/heartland-hemp-fiber-textile-fabric.png",
                        alt: "heartland hemp fiber textile fabric",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Textile Fiber With An Origin Story To Be Proud Of" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium Fiber is the highest quality hemp fiber on Earth" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium fiber is grown by small farmers with care, consuming 95% less water than Cotton and no pesticides. Imperium is one of the softest, strongest natural fibers on earth that regenerates our farmland." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium fiber can be used in garments, furniture, and composites." }
                }
                div { class: "md:order-2",
                    img {
                        src: "/assets/pages/imperium-filler/1210x786-px-4.png",
                        alt: "heartland hemp bales",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/imperium-fibers/imperium-bulk-spin-ready-fiber.png", alt: "Imperium Bulk Spin Ready Fiber", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            div { class: "max-w-3xl mx-auto mb-16 animate-fade-in-up",
                h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 text-center", "Throughout The Supply Chain" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland partners with corn, wheat, and soy farmers to embed industrial hemp into their crop rotation. Our USDA Grant has given us unique insights into industrial hemp farming, regenerative agriculture, and carbon sequestration." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Yarn Manufacturer Heartland partners with yarn manufacturers to produce high quality yarns for fabric made from Imperium Hemp Fiber. We support customers in the top 10 textile manufacturing countries." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Textile Fabric Mill Heartland partners with textile mills and cut/sows to ensure they're able to successfully integrate hemp fiber alongside other natural and synthetic fibers." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Brand Heartland is the sustainability partner for brands on their journey to reduce their carbon footprint. Our team helps create stakeholder alignment so companies can effectively communicate the value of imperium hemp fiber." }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/imperium-fibers/plastic-pallets-2-1.png", alt: "cellulose fiber", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
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
