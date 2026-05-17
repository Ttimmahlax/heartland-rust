use dioxus::prelude::*;

use crate::components::carbon_calculator::CarbonCalculator;
use crate::components::decarb_solutions::DecarbSolutions;
use crate::components::logo_carousel::LogoCarousel;
use crate::components::markets_amplify::MarketsAmplify;
use crate::components::mats_amplify::MatsAmplify;
use crate::components::news_carousel::NewsCarousel;
use crate::components::supply_chain::{SupplyChainStep, SupplyIcon};
use crate::components::video_hero::VideoBackground;
use crate::seo::{organization_jsonld, Seo};
use crate::Route;

#[component]
pub fn Landing() -> Element {
    rsx! {
        Seo {
            title: "Heartland Industries",
            description: "Heartland is a material science company helping manufacturers exceed their cost-reduction goals while reducing their emissions with carbon-negative hemp materials.",
            path: "/",
        }
        document::Script { r#type: "application/ld+json", "{organization_jsonld()}" }

        Hero {}
        LogoCarousel { heading: "" }
        HighPerformanceSection {}
        MeetTheTeamStrip {}
        EngineeringEarthSection {}
        DecarbSolutions {}
        UnlockingSustainableFuture {}
        BrightFutureManufacturing {}
        MatsAmplify {}
        WhyImperiumStrip {}
        MarketsAmplify {}
        CarbonCalculator {}
        TestimonialBlock {}
        NewsCarousel { heading: "From The Heartland" }
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
                    "Sustainability Without Compromise"
                }
                h1 {
                    class: "text-[1.575rem] md:text-[2.625rem] lg:text-[3.15rem] font-extrabold leading-tight uppercase tracking-tight text-white max-w-5xl mx-auto animate-fade-in-up",
                    "The future of"
                    br { class: "hidden md:inline" }
                    " material innovation"
                }
                div {
                    class: "mt-10 flex items-center justify-center gap-3 flex-wrap animate-fade-in-up delay-2",
                    Link {
                        to: Route::WhyImperium {},
                        class: "btn-accent-gradient",
                        "Why Imperium"
                    }
                }
            }
        }
    }
}

#[component]
fn HighPerformanceSection() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-12 items-center",
                // Left: text
                div { class: "animate-fade-in-up",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Low Carbon Natural Fibers"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "High Performance"
                        br { class: "hidden md:inline" }
                        " "
                        span { class: "text-gradient-red", "Cost Reducing Materials" }
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4",
                        "Plastics, textiles, paper, and construction. Global demand is accelerating the need for more sustainable materials that perform better and cost less."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)]",
                        "Heartland's Imperium improves the strength of everyday products while reducing the cost, weight and carbon footprint."
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up",
                    img {
                        src: "/assets/pages/landing/farming-hemp-michigan.webp",
                        alt: "Hemp farming in Michigan — Heartland Industries' supply chain",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
        }
    }
}

#[component]
fn MeetTheTeamStrip() -> Element {
    rsx! {
        section { class: "container-content py-10 md:py-12 border-t border-b border-[color:var(--color-border)]",
            div { class: "flex flex-col items-center justify-center gap-5 md:flex-row md:gap-8",
                h3 { class: "text-xl md:text-2xl font-display font-semibold",
                    Link { to: Route::Team {},
                        class: "hover:text-[color:var(--color-accent)] transition-colors",
                        "Meet The Team →"
                    }
                }
                a {
                    href: "https://www.linkedin.com/company/therealheartland/",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    aria_label: "Follow Heartland Industries on LinkedIn",
                    class: "inline-flex items-center justify-center text-[color:var(--color-fg-muted)] hover:text-[color:var(--color-accent)] hover:scale-110 transition",
                    svg { width: "36", height: "36", view_box: "0 0 24 24", fill: "currentColor",
                        path { d: "M19 0H5a5 5 0 0 0-5 5v14a5 5 0 0 0 5 5h14a5 5 0 0 0 5-5V5a5 5 0 0 0-5-5zM8 19H5V8h3v11zM6.5 6.7A1.7 1.7 0 1 1 8.2 5a1.7 1.7 0 0 1-1.7 1.7zM20 19h-3v-5.6c0-1.4-.5-2.3-1.7-2.3-.9 0-1.5.6-1.7 1.2-.1.2-.1.5-.1.8V19h-3V8h3v1.3a3 3 0 0 1 2.7-1.5c2 0 3.5 1.3 3.5 4V19z" }
                    }
                }
            }
        }
    }
}

#[component]
fn EngineeringEarthSection() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-12 items-center",
                // Left: image
                div { class: "animate-fade-in-up md:order-1 order-2",
                    img {
                        src: "/assets/pages/landing/industrial-hemp-farm-michigan.webp",
                        alt: "Industrial hemp farm — Michigan",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
                // Right: text
                div { class: "animate-fade-in-up md:order-2 order-1",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Engineering Earth"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Changing The "
                        span { class: "text-gradient-red", "World" }
                        br {}
                        "One Farm At A Time"
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "Heartland's Imperium empowers local farming for local manufacturing to boost supply chain resiliency."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-6",
                        "Imperium can be grown anywhere in the world, using 95% less water than cotton and regenerating soil, leaving it healthier."
                    }
                    Link {
                        to: Route::EngineeringEarth {},
                        class: "inline-block btn-accent-gradient",
                        "Engineering Earth →"
                    }
                }
            }
        }
    }
}

#[component]
fn UnlockingSustainableFuture() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-12 items-center",
                // Left: text + stat
                div { class: "animate-fade-in-up",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Lower Cost Materials"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "Unlocking Our"
                        br { class: "hidden md:inline" }
                        " "
                        span { class: "text-gradient-red", "Sustainable Future" }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "Imperium can replace more than 30% of the material in existing products; plastics, textiles, concrete, & asphalt."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-4",
                        "We work with global brands and their suppliers to predictably reduce the cost and carbon footprint of everyday products."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)] mb-8",
                        "Imperium will be in everything in the next 20 years because it costs less."
                    }
                    // Carbon Footprint Reduction stat bar
                    div { class: "mt-2",
                        div { class: "flex items-center justify-between mb-2",
                            h3 { class: "text-sm font-semibold uppercase tracking-wider text-[color:var(--color-fg)]",
                                "Carbon Footprint Reduction"
                            }
                            span { class: "text-2xl font-bold text-[color:var(--color-accent)]",
                                "70%"
                            }
                        }
                        div { class: "w-full h-2 rounded-full bg-[color:var(--color-accent-quiet)] overflow-hidden",
                            div {
                                class: "h-full rounded-full bg-[color:var(--color-accent)] animate-fade-in",
                                style: "width: 70%",
                            }
                        }
                    }
                }
                // Right: image
                div { class: "animate-fade-in-up",
                    img {
                        src: "/assets/pages/landing/sustainable-future-heartland.webp",
                        alt: "Sustainable future — Heartland Industries",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
        }
    }
}

#[component]
fn BrightFutureManufacturing() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-12 items-center",
                // Left: image
                div { class: "animate-fade-in-up md:order-1 order-2",
                    img {
                        src: "/assets/pages/landing/sustainable-manufacturing.webp",
                        alt: "Sustainable manufacturing materials — Heartland Industries",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
                // Right: text
                div { class: "animate-fade-in-up md:order-2 order-1",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Simple & Scalable"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight mb-6",
                        "A Bright Future For"
                        br { class: "hidden md:inline" }
                        " "
                        span { class: "text-gradient-red", "Manufacturing" }
                    }
                    p { class: "text-lg font-medium text-[color:var(--color-accent)] mb-4",
                        "Lower Cost. Better Strength. Available to Everyone Globally."
                    }
                    p { class: "text-lg text-[color:var(--color-fg-muted)]",
                        "Everyday products made with petroleum and mined rock can be made better with Imperium. Heartland enables local farmers to empower their local industries with stronger and lower cost raw materials."
                    }
                }
            }
        }
    }
}

#[component]
fn WhyImperiumStrip() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 md:py-28 my-12 section-soft-edges",
            div { class: "container-content",
                // Section header
                div { class: "text-center mb-16 max-w-3xl mx-auto",
                    p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                        "Your Sustainability Partner"
                    }
                    h2 { class: "text-3xl md:text-5xl font-bold leading-tight",
                        "Throughout The "
                        span { class: "text-gradient-red", "Supply Chain" }
                    }
                }

                // The 4-step zigzag timeline
                div { class: "relative",
                    // Vertical center line (desktop only) — the rail that the steps anchor to
                    div { class: "hidden md:block absolute left-1/2 top-0 bottom-0 -translate-x-1/2 w-px bg-gradient-to-b from-transparent via-[color:var(--color-accent)] to-transparent opacity-40" }

                    SupplyChainStep {
                        number: 1,
                        icon: SupplyIcon::Tractor,
                        heading: "Regenerative Farmer",
                        body: "Heartland partners with corn, soy, alfalfa, and cotton farmers to successfully add industrial hemp into their crop rotation. Our USDA Grant has given us unique insights on water conservation and carbon sequestration from our farming practices.",
                        align_right: false,
                    }
                    SupplyChainStep {
                        number: 2,
                        icon: SupplyIcon::Blender,
                        heading: "Raw Material Processor",
                        body: "Heartland partners with material processors from every industry to replace existing materials without any new equipment costs. Our Imperium product line is a drop-in material alongside their existing processes to reduce costs and emissions.",
                        align_right: true,
                    }
                    SupplyChainStep {
                        number: 3,
                        icon: SupplyIcon::Gears,
                        heading: "Finished Part Converter",
                        body: "Heartland partners with finished goods manufacturers to ensure our Imperium products are handled properly. Our team works alongside component part manufacturers to process everyday products with the same technology used today.",
                        align_right: false,
                    }
                    SupplyChainStep {
                        number: 4,
                        icon: SupplyIcon::Store,
                        heading: "Brand",
                        body: "Heartland is the decarbonization partner empowering brands on their journey to reduce their costs and carbon footprint. Our team helps create stakeholder alignment to effectively communicate the consumer value of material innovation.",
                        align_right: true,
                    }
                }
            }
        }
    }
}

// Step components live in components/supply_chain.rs (reused by /why-imperium).

// CarbonCalculator moved to components/carbon_calculator.rs — reused on /why-imperium.

#[component]
fn TestimonialBlock() -> Element {
    rsx! {
        section { class: "container-content py-16 text-center",
            blockquote { class: "max-w-3xl mx-auto",
                p { class: "text-2xl md:text-3xl font-display italic leading-snug",
                    "\"Customers don't pick between cost reduction or sustainability anymore — they get both. That's what Imperium delivers.\""
                }
                footer { class: "mt-6 text-[color:var(--color-fg-muted)]",
                    "— John Ely, CEO of Heartland Industries"
                }
            }
        }
    }
}
