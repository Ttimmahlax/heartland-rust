use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
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
        IndustriesGrid {}
        WhyImperiumStrip {}
        TestimonialBlock {}
        NewsCarousel { heading: "From The Heartland" }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section {
            class: "video-hero-section section-soft-bottom min-h-[88vh] flex items-center",
            VideoBackground { slug: "landing".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-sm uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
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
        section { class: "container-content py-16 text-center",
            h2 { class: "text-3xl md:text-4xl font-bold mb-6",
                "High Performance "
                span { class: "text-gradient-red", "Cost Reducing Materials" }
            }
            p { class: "max-w-3xl mx-auto text-lg text-[color:var(--color-fg-muted)]",
                "Heartland engineers industrial hemp into drop-in additives that lower cost AND emissions in plastics, rubber, concrete, asphalt, paper, and textiles. Same equipment. Same performance. Better economics. Lower carbon footprint."
            }
        }
    }
}

#[component]
fn IndustriesGrid() -> Element {
    let industries: Vec<(&'static str, &'static str, Route)> = vec![
        (
            "Decarbonized Solutions",
            "Imperium Filler and Filled Resin let plastic compounders cut costs and emissions in one drop-in step. Same lines, same throughput.",
            Route::SustainablePlastic {},
        ),
        (
            "Unlocking Sustainable Materials",
            "Imperium hemp augments or replaces glass fiber, talc, and calcium carbonate. Lower weight, lower carbon, lower cost.",
            Route::WhyImperium {},
        ),
        (
            "A Bright Future For Manufacturing",
            "From automotive to building materials, Heartland's drop-in additives slot into existing supply chains. No retooling required.",
            Route::Automotive {},
        ),
    ];

    rsx! {
        section { class: "container-content pb-12",
            div { class: "grid gap-6 md:grid-cols-3",
                for (i, (title, desc, route)) in industries.into_iter().enumerate() {
                    Link {
                        to: route,
                        key: "{i}",
                        class: "block surface-glass p-7 animate-fade-in-up hover:translate-y-[-2px] transition-transform",
                        style: "animation-delay: {i * 80}ms",
                        h3 { class: "font-display font-semibold text-xl mb-3 leading-snug", "{title}" }
                        p { class: "text-[color:var(--color-fg-muted)]", "{desc}" }
                        span { class: "mt-4 inline-block text-sm font-medium text-[color:var(--color-accent)]", "Learn more →" }
                    }
                }
            }
        }
    }
}

#[component]
fn WhyImperiumStrip() -> Element {
    rsx! {
        section {
            class: "bg-mesh-dramatic py-24 my-12 section-soft-edges",
            div { class: "container-content text-center",
                h2 { class: "text-3xl md:text-5xl font-bold mb-6 max-w-3xl mx-auto",
                    "Throughout The "
                    span { class: "text-gradient-red", "Supply Chain" }
                }
                p { class: "max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] mb-2",
                    "Heartland farms industrial hemp through our farmer network across 11 US states, processes it into Imperium-grade material, and ships it directly to manufacturers as filler, masterbatch, filled resin, or textile fiber."
                }
                p { class: "max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] mb-8",
                    "American farms. American mills. American manufacturers. Verified carbon-negative end-to-end."
                }
                div { class: "flex items-center justify-center gap-3 flex-wrap",
                    Link {
                        to: Route::Lca {},
                        class: "btn-accent-gradient",
                        "See The LCA"
                    }
                    Link {
                        to: Route::EngineeringEarth {},
                        class: "px-5 py-3 rounded-md border border-[color:var(--color-border)] hover:border-[color:var(--color-accent)] text-[color:var(--color-fg)] hover:text-[color:var(--color-accent)]",
                        "Engineering Earth →"
                    }
                }
            }
        }
    }
}

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
