use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn GreenPackaging() -> Element {
    rsx! {
        Seo {
            title: "GPI",
            description: "The Green Packaging Initiative was developed by Heartland to catalyze the adoption of sustainable products throughout the supply chain.",
            path: "/green-packaging-initiative",
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
            VideoBackground { slug: "green-packaging-initiative".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "GPI"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Decarbonized Packaging"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "Natural fiber additives & recycled plastic are the easiest opportunities to decarbonize your supply chain."
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
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "A Simple First Step Toward Sustainability" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Natural fiber additives & recycled plastic are the easiest opportunities to decarbonize your supply chain." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Every manufacturer on earth uses industrial packaging to ship and protect products in transit. This opens the door for sustainable materials to be easily adopted while providing a clear carbon footprint reduction in the supply chain." }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/green-packaging-initiative/heartland-lca-1.png",
                        alt: "green packaging initiative",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/sustainable-plastic-compounding/3color.svg", alt: "greentown labs logo", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Same Packaging. Lower Carbon Footprint." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland Imperium enables brands to use the same suppliers for packaging and resin." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Additives are simple changes for plastic compounders. Heartland's materials help brands reduce the cost, weight, and carbon footprint of their packaging." }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/green-packaging-initiative/heartland-lca.png",
                        alt: "green packaging initiative pallets",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/green-packaging-initiative/carbon-neutral-plastic-pallet-47.2-x-47.2-imperium-inside-nest-a5-od-9f.png", alt: "Carbon Neutral Plastic Pallet 47.2 x 47.2 Imperium Inside Nest A5 OD-9F", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Meaningful Action Toward a Brighter Future" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "All change begins with one step forward." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Product development initiatives can take years to qualify. Brand owners and investors are looking for near-term opportunities to prove their commitment to a sustainable future." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Reusable packaging is the key." }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/green-packaging-initiative/heartland-lca-2.png",
                        alt: "green packaging initiative pallets",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Making The Impractical, Optimal" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "For the first time in history, it costs less to use sustainable materials in industrial packaging." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "The ability to work alongside mined and synthetic materials is mission-critical to the adoption of natural fibers. Making our carbon-negative additives compatible with today's materials is part of our secret sauce." }
                }
                div { class: "md:order-2",
                    img {
                        src: "/assets/pages/why-imperium/paper-8-7.png",
                        alt: "hemp baling drone",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 mt-12 text-center", "Learn More About Green Packaging" }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "The Sustainable Path" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Imperium can be used as a drop-in replacement for mined and synthetic additives. This allows manufacturers to instantly decarbonize their supply chain." }
                }
                div { class: "md:order-2",
                    img {
                        src: "/assets/pages/green-packaging-initiative/natural-fiber-filled-polyethylene-1.png",
                        alt: "hemp filled polypropylene pallets",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }        }
    }
}

#[component]
fn ClosingCta() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 my-12 section-soft-edges",
            div { class: "container-content text-center",
                h2 { class: "text-3xl md:text-4xl font-bold mb-6 max-w-2xl mx-auto",
                    "Learn more about Decarbonized"
                }
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Get in touch" }
            }
        }
    }
}
