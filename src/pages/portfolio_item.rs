use dioxus::prelude::*;

use crate::components::news_carousel::NewsCarousel;
use crate::seo::Seo;
use crate::Route;

/// Portfolio entry — DigEco custom-post-type data embedded at compile time.
#[derive(Clone, Copy)]
pub struct PortfolioEntry {
    pub slug: &'static str,
    pub title: &'static str,
    pub h1: &'static str,
    pub description: &'static str,
    pub paragraphs: &'static [&'static str],
    pub categories: &'static [&'static str],
}

pub const PORTFOLIO: &[PortfolioEntry] = &[
    PortfolioEntry { slug: "agriculture", title: "Agriculture", h1: "Agriculture", description: "Heartland is a material innovation company that works alongside farmers to engineer hemp fibers as high performance carbon negative additives", paragraphs: &[], categories: &["industry"] },
    PortfolioEntry { slug: "building-materials-library", title: "Building Materials", h1: "Building Materials", description: "Building Materials dictionary and glossary of terms related to many different variables throughout the plastic industry.", paragraphs: &[], categories: &["library"] },
    PortfolioEntry { slug: "carbon-neutral-plastic-pallet-43-3-x-43-3-c5-od-9f", title: "Carbon Neutral Plastic Pallet 43.3 x 43.3 C5 (OD-9F)", h1: "Carbon Neutral Plastic Pallet 43.3 x 43.3 With Imperium Inside | C5 OD-9F", description: "The Carbon Neutral Plastic Pallet 43.3 x 43.3 With Imperium Inside is nestable and holds 8800 lb static and 3300 lb dynamic weight.", paragraphs: &[], categories: &["imperium-masterbatch", "pallet", "reusable-packaging", "rhdpe"] },
    PortfolioEntry { slug: "carbon-neutral-plastic-pallet-45-x-45-c5-2-od-9f", title: "Carbon Neutral Plastic Pallet 45 x 45 C5.2 OD-9F", h1: "Carbon Neutral Plastic Pallet 45 x 45 With Imperium Inside | C5.2 OD-9F", description: "The Carbon Neutral Plastic Pallet 45 x 45 With Imperium Inside is nestable and holds 8800 lb static and 3300 lb dynamic weight.", paragraphs: &[], categories: &["imperium-masterbatch", "pallet", "reusable-packaging", "rhdpe"] },
    PortfolioEntry { slug: "carbon-neutral-plastic-pallet-47-2-x-31-5-e5-od9f", title: "Carbon Neutral Plastic Pallet 47.2 x 31.5 E5 - OD9F", h1: "Carbon Neutral Plastic Pallet 47.2 x 31.5 With Imperium Inside | E5 OD9F", description: "The Carbon Neutral Plastic Pallet 47.2 x 31.5 With Imperium Inside is nestable and holds 7200 lb static and 2600 lb dynamic weight.", paragraphs: &[], categories: &["imperium-masterbatch", "pallet", "reusable-packaging", "rhdpe"] },
    PortfolioEntry { slug: "carbon-neutral-plastic-pallet-47-2-x-47-2-s5-od-9f", title: "Carbon Neutral Plastic Pallet 47.2 x 47.2 A5 S5 OD-9F", h1: "Carbon Neutral Plastic Pallet 47.2 x 47.2 With Imperium Inside | S5 OD-9F", description: "The Carbon Neutral Plastic Pallet 47.2 x 47.2 With Imperium Inside is nestable and holds 8800 lb static and 3300 lb dynamic weight.", paragraphs: &[], categories: &["imperium-masterbatch", "pallet", "reusable-packaging", "rhdpe"] },
    PortfolioEntry { slug: "carbon-neutral-plastic-pallet-48-x-40-imperium-inside-nest-us1-1", title: "Carbon Neutral Plastic Pallet 48 x 40 Nest US1.1", h1: "Carbon Neutral Plastic Pallet 48 x 40 With Imperium Inside", description: "The Carbon Neutral Plastic Pallet 48 x 40 With Imperium Inside is nestable and holds 3500 lb static and 2200 lb dynamic weight.", paragraphs: &[], categories: &["imperium-masterbatch", "pallet", "reusable-packaging", "rhdpe"] },
    PortfolioEntry { slug: "carbon-neutral-plastic-pallet-48-x-40-nest-us5", title: "Carbon Neutral Plastic Pallet 48 x 40 Nest US5", h1: "Carbon Neutral Plastic Pallet 48 x 40 With Imperium Inside | US5", description: "The Carbon Neutral Plastic Pallet 48 x 40 With Imperium Inside is nestable and holds 6500 lb static and 3100 lb dynamic weight.", paragraphs: &[], categories: &["imperium-masterbatch", "pallet", "reusable-packaging", "rhdpe"] },
    PortfolioEntry { slug: "carbon-neutral-plastic-pallet-48-x-40-nest-us5-1", title: "Carbon Neutral Plastic Pallet 48 x 40 Nest US5.1", h1: "Carbon Neutral Plastic Pallet 48 x 40 With Imperium Inside | US5.1", description: "The Carbon Neutral Plastic Pallet 48 x 40 With Imperium Inside is nestable and holds 8800 lb static and 2800 lb dynamic weight.", paragraphs: &[], categories: &["imperium-masterbatch", "pallet", "reusable-packaging", "rhdpe"] },
    PortfolioEntry { slug: "carbon-neutral-plastic-pallet-48-x-40-nest-us5-cd9f", title: "Carbon Neutral Plastic Pallet 48 x 40 Nest US5 - CD9F", h1: "Carbon Neutral Plastic Pallet 48 x 40 With Imperium Inside | US5 – CD9F", description: "The Carbon Neutral Plastic Pallet 48 x 40 With Imperium Inside is nestable and holds 11000 lb static and 3000 lb dynamic weight.", paragraphs: &[], categories: &["imperium-masterbatch", "pallet", "reusable-packaging", "rhdpe"] },
    PortfolioEntry { slug: "carbon-neutral-plastic-pallet-48-x-40-us8-od6r", title: "Carbon Neutral Plastic Pallet 48 x 40 US8 - OD6R", h1: "Carbon Neutral Plastic Pallet 48 x 40 With Imperium Inside | US8 – OD6R", description: "The Carbon Neutral Plastic Pallet 48 x 40 With Imperium Inside is nestable and holds 13200 lb static and 4400 lb dynamic weight.", paragraphs: &[], categories: &["imperium-masterbatch", "pallet", "reusable-packaging", "rhdpe"] },
    PortfolioEntry { slug: "carbon-neutral-plastic-pallet-48-x-45-a5-od-9f", title: "Carbon Neutral Plastic Pallet 48 x 45 A5 OD-9F", h1: "Carbon Neutral Plastic Pallet 48 x 45 With Imperium Inside | A5 OD-9F", description: "The Carbon Neutral Plastic Pallet 48 x 45 With Imperium Inside is nestable and holds 8800 lb static and 3300 lb dynamic weight.", paragraphs: &[], categories: &["imperium-masterbatch", "pallet", "reusable-packaging", "rhdpe"] },
    PortfolioEntry { slug: "chemical", title: "Chemical", h1: "Chemical", description: "Heartland is a material innovation company that engineers hemp fibers as additives for different chemicals. Learn more here.", paragraphs: &[], categories: &["industry"] },
    PortfolioEntry { slug: "extruding-materials", title: "Extruding", h1: "Extruding", description: "plastic dictionary and glossary of terms related to many different variables throughout the plastic industry.", paragraphs: &[], categories: &["materials"] },
    PortfolioEntry { slug: "foam-library", title: "Foam", h1: "Foam", description: "Foam dictionary and glossary of terms related to many different variables throughout the plastic industry.", paragraphs: &[], categories: &["library"] },
    PortfolioEntry { slug: "forming-materials", title: "Forming", h1: "Forming", description: "plastic dictionary and glossary of terms related to many different variables throughout the plastic industry.", paragraphs: &[], categories: &["materials"] },
    PortfolioEntry { slug: "manufacturing", title: "Manufacturing", h1: "Manufacturing", description: "Heartland is a material innovation company that engineers hemp fibers as high performance carbon negative additives for manufacturing.", paragraphs: &[], categories: &["industry"] },
    PortfolioEntry { slug: "molding-materials", title: "Molding", h1: "Molding", description: "plastic dictionary and glossary of terms related to many different variables throughout the plastic industry.", paragraphs: &[], categories: &["materials"] },
    PortfolioEntry { slug: "paper-library", title: "Paper", h1: "Paper", description: "Paper dictionary and glossary of terms related to many different variables throughout the plastic industry.", paragraphs: &[], categories: &["library"] },
    PortfolioEntry { slug: "plastic-library", title: "Plastic", h1: "Plastic", description: "plastic dictionary and glossary of terms related to many different variables throughout the plastic industry.", paragraphs: &[], categories: &["library"] },
    PortfolioEntry { slug: "polypropylene-with-20-imperium-inside", title: "Polypropylene with 20% Imperium Inside", h1: "Polypropylene with 20% Imperium Inside", description: "Polypropylene with imperium inside is designed for strong, durable, high impact applications that require light weight low carbon footprint.", paragraphs: &[], categories: &["polypropylene"] },
    PortfolioEntry { slug: "rubber-library", title: "Rubber", h1: "Rubber", description: "rubber dictionary and glossary of terms related to many different variables throughout the plastic industry.", paragraphs: &[], categories: &["library"] },
];

pub fn find_portfolio(slug: &str) -> Option<&'static PortfolioEntry> {
    PORTFOLIO.iter().find(|e| e.slug == slug)
}

pub fn all_portfolio_slugs() -> Vec<String> {
    PORTFOLIO.iter().map(|e| e.slug.to_string()).collect()
}

#[component]
pub fn PortfolioItem(slug: String) -> Element {
    let Some(entry) = find_portfolio(&slug) else {
        return rsx! { NotFound {} };
    };
    let path = format!("/sustainability-news/portfolio/{slug}");
    rsx! {
        Seo {
            title: "{entry.title}",
            description: "{entry.description}",
            path: "{path}",
        }
        section { class: "bg-mesh-hero section-soft-bottom",
            div { class: "container-content py-20 md:py-24 text-center",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "Portfolio"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-3xl mx-auto",
                    "{entry.h1}"
                }
            }
        }
        section { class: "container-content py-16 max-w-3xl",
            for (i, p) in entry.paragraphs.iter().enumerate() {
                p {
                    key: "{i}",
                    class: "text-lg text-[color:var(--color-fg-muted)] mb-5 last:mb-0",
                    "{p}"
                }
            }
            if !entry.categories.is_empty() {
                div { class: "mt-10 flex flex-wrap gap-2",
                    for c in entry.categories.iter() {
                        span {
                            key: "{c}",
                            class: "text-xs px-3 py-1 rounded-full bg-[color:var(--color-accent-quiet)] text-[color:var(--color-accent)]",
                            "{c}"
                        }
                    }
                }
            }
            div { class: "mt-12 pt-8 border-t border-[color:var(--color-border)]",
                Link { to: Route::Portfolios {}, class: "text-sm hover:text-[color:var(--color-accent)]", "← All portfolio items" }
            }
        }
        NewsCarousel { heading: "Related Articles" }
    }
}

#[component]
fn NotFound() -> Element {
    rsx! {
        Seo {
            title: "Portfolio item not found",
            description: "The requested portfolio item could not be found.",
            path: "/portfolios",
        }
        section { class: "container-content py-24 text-center",
            h1 { class: "text-4xl font-bold mb-4", "Portfolio item not found" }
            Link { to: Route::Portfolios {}, class: "btn-accent-gradient", "Back to portfolios" }
        }
    }
}
