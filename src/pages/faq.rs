use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;

#[component]
pub fn Faq() -> Element {
    rsx! {
        Seo {
            title: "Frequently Asked Questions",
            description: "Common questions about Imperium, industrial hemp materials, drop-in compounding, USDA programs, carbon credits, and how to start a project with Heartland.",
            path: "/frequently-asked-questions",
        }

        Hero {}
        LogoCarousel { heading: "" }
        FaqHeader {}
        FaqAccordion {}
        NewsCarousel { heading: "Related Articles" }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section {
            class: "video-hero-section section-soft-bottom min-h-[110vh] flex items-center pb-[20vh]",
            VideoBackground { slug: "frequently-asked-questions".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "FAQ Page"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Our sustainable"
                    br {}
                    "future together"
                }
            }
        }
    }
}

#[component]
fn FaqHeader() -> Element {
    rsx! {
        section { class: "container-content pt-16 md:pt-20 pb-4",
            div { class: "max-w-3xl mx-auto text-center",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "Everything You Need"
                }
                h2 { class: "text-3xl md:text-5xl font-bold leading-tight",
                    "Frequently "
                    span { class: "text-gradient-red", "Asked Questions" }
                }
            }
        }
    }
}

/// Compile-time table of FAQ Q&A pairs, sourced verbatim from
/// heartland.io's `/frequently-asked-questions/` page via SSH.
struct FaqItem {
    q: &'static str,
    a: &'static str,
}

const FAQS: &[FaqItem] = &[
    FaqItem { q: "Where is your hemp grown?", a: "Today, our hemp fiber is grown in Michigan and 6 other states. We also purchase bales of hemp off the spot market through a reliable QA / QC process." },
    FaqItem { q: "How can I get sample materials?", a: "You can fill out the contact form below to connect with one of our team members. From there, we can determine if sample materials are appropriate." },
    FaqItem { q: "What raw materials can Heartland's additives be used in?", a: "So far, companies have used Heartland's materials as additives in plastic, rubber, foam, asphalt, concrete, paper, and ceramic. Every day we get new ideas and projects in our inbox. Feel free to reach out to see if your application is a fit." },
    FaqItem { q: "How does Heartland work with brands?", a: "Heartland works with brands and their suppliers to embed natural fibers into their supply chain to reduce cost, weight, and carbon footprint without any retooling costs." },
    FaqItem { q: "What is Heartland's USDA Grant for?", a: "Heartland's first USDA grant is a 3-year program called 'Hemp4Soil' that aims to study industrial hemp, regenerative agriculture, and carbon sequestration." },
    FaqItem { q: "Does Heartland have a lifecycle assessment?", a: "Heartland is completing an LCA with Presidio, one of the top sustainability graduate programs in the world. This LCA, alongside soil carbon data, will quantify how carbon negative Heartland's materials are." },
    FaqItem { q: "Is Heartland building an industrial hemp supply chain near me?", a: "Heartland is partnering with global brands and suppliers to support local farming and local manufacturing. We are open to strategic partnerships when and where it makes sense." },
    FaqItem { q: "Is Heartland currently working with any other natural fibers?", a: "Today, Heartland's go-to high-performance natural fiber is industrial hemp. As time (and locations) expand, we will integrate other materials into our products." },
    FaqItem { q: "Can I farm for Heartland?", a: "Potentially. Please reach out on our farming page so that someone from the Heartland team can connect with you." },
    FaqItem { q: "How does Heartland's USDA Biopreferred certification impact partners?", a: "The USDA biopreferred program is for government-mandated purchasing. Since Heartland's materials are certified, Heartland's customers can apply for biopreferred contracts (as long as Heartland's materials make up more than 22.5% of the compound)." },
    FaqItem { q: "What is industry 5.0?", a: "The 5th industrial revolution is about embedding sustainability throughout the value chain." },
    FaqItem { q: "What does 'sustainability without compromise' mean?", a: "Typically, with bio-based materials, there is a sacrifice that brands and suppliers make. Sustainability without compromise is about maintaining performance properties without compromising strength, weight, or price." },
    FaqItem { q: "What is Sustainable Material Innovation?", a: "Sustainable Material Innovation (SMI) focuses on the development of lower-carbon-footprint materials, component parts, and end products." },
    FaqItem { q: "Are Heartland's hemp additives compostable?", a: "By themselves, hemp fibers are compostable and biodegradable. But, the compostability of plastic will always depend on the resin, not on the additive." },
    FaqItem { q: "Are Heartland's hemp additives recyclable?", a: "Heartland's additives can be picked up by infrared sensors currently being used in Material Recovery Facilities (MRF's) and other recycling facilities." },
    FaqItem { q: "Does Heartland sell carbon credits?", a: "Not yet. Heartland's USDA program is creating unprecedented insights specifically for agriculture-focused carbon data. These data points will be used to create America's leading agriculture carbon credit market. Please reach out to our team if you are interested in carbon credits." },
];

#[component]
fn FaqAccordion() -> Element {
    rsx! {
        section { class: "container-content pt-8 md:pt-12 pb-16 md:pb-24",
            div { class: "max-w-3xl mx-auto space-y-3",
                for (i, item) in FAQS.iter().enumerate() {
                    details {
                        key: "{i}",
                        class: "group surface-glass rounded-xl border border-[color:var(--color-border)] overflow-hidden transition hover:border-[color:var(--color-accent)]",
                        summary { class: "flex items-center justify-between gap-4 cursor-pointer px-5 md:px-6 py-4 md:py-5 list-none font-display font-semibold text-base md:text-lg",
                            span { "{item.q}" }
                            // Chevron — rotates when <details open>
                            span { class: "shrink-0 w-7 h-7 rounded-full bg-[color:var(--color-accent-quiet)] text-[color:var(--color-accent)] flex items-center justify-center transition-transform group-open:rotate-180",
                                svg { width: "14", height: "14", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5", stroke_linecap: "round", stroke_linejoin: "round",
                                    polyline { points: "6 9 12 15 18 9" }
                                }
                            }
                        }
                        div { class: "px-5 md:px-6 pb-5 md:pb-6 text-[color:var(--color-fg-muted)] leading-relaxed",
                            "{item.a}"
                        }
                    }
                }
            }
        }
    }
}
