use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn Faq() -> Element {
    rsx! {
        Seo {
            title: "Frequently Asked Questions",
            description: "Common questions on Imperium, industrial hemp materials, drop-in compounding, carbon-negative claims, supply security, and how to start a program with Heartland.",
            path: "/frequently-asked-questions",
        }

        Hero {}
        Sections {}
        LogoCarousel { heading: "As Seen In" }
        ClosingCta {}
        NewsCarousel { heading: "Related Articles" }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section { class: "bg-mesh-hero section-soft-bottom",
            div { class: "container-content py-20 md:py-28 text-center",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4 animate-fade-in",
                    "Common questions"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    span { class: "text-gradient-red", "Frequently Asked Questions" }
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "If you have a question that isn't covered below, email Hello@heartland.io and our team will route you to the right specialist."
                }
            }
        }
    }
}

#[component]
fn Sections() -> Element {
    rsx! {
        section { class: "container-content py-16",
            div { class: "grid gap-8 md:grid-cols-2",
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Is Imperium really cost-competitive with talc and calcium carbonate?" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Yes. At typical loading levels (20–40% in PP and PE), Imperium delivers a net cost reduction vs. talc and equivalent or lower cost vs. CaCO₃ — depending on regional logistics." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "We'll quote against your current spec — get in touch via /contact to start." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Does Imperium require new compounding equipment?" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "No. Imperium is engineered as a direct drop-in for talc / CaCO₃ on standard twin-screw and single-screw compounding lines." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "For high-loading formulations (>40%), some feeder configuration tuning may be needed." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "How is Imperium 'carbon-negative'?" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "The upstream photosynthesis of the hemp crop sequesters more CO₂ than the downstream processing emits. The LCA boundary is published at /lca." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Each ton of Imperium ships with a per-batch LCA disclosure pack via Carbon Report." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Is the supply chain secure?" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Imperium is farmed across 11 US states under Heartland's grower contracts. The supply chain is decoupled from offshore filler markets and not exposed to current China tariff dynamics." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "What's the minimum order quantity?" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Sample quantities (1–5 lb) ship for free for qualified engineering teams. Production quantities scale to truckload + supersack." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Long-term off-take agreements with locked pricing are available." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Can I see independent test data?" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Yes — request the TDS + SDS + LCA pack via /contact. The pack includes mechanical, flammability, thermal, and LCA results." }
                    }
            }
        }
    }
}

#[component]
fn ClosingCta() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 my-12 section-soft-edges",
            div { class: "container-content text-center",
                h2 { class: "text-3xl md:text-4xl font-bold mb-6 max-w-2xl mx-auto",
                    "Ready to dig in?"
                }
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Email us your question" }
            }
        }
    }
}
