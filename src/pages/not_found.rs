use dioxus::prelude::*;

use crate::seo::Seo;
use crate::Route;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let path = format!("/{}", segments.join("/"));

    rsx! {
        Seo {
            title: "Page not found",
            description: "The page you were looking for has moved or never existed. Try the homepage, the articles index, or get in touch — we'll point you the right way.",
            path: "/404",
        }

        section { class: "relative overflow-hidden bg-mesh-dramatic section-soft-bottom",
            div { class: "absolute inset-0 pointer-events-none opacity-[0.18] mix-blend-screen",
                svg {
                    width: "100%", height: "100%",
                    xmlns: "http://www.w3.org/2000/svg",
                    "aria-hidden": "true",
                    defs {
                        pattern {
                            id: "nf-hex",
                            width: "56", height: "48",
                            "patternUnits": "userSpaceOnUse",
                            "patternTransform": "translate(0 0)",
                            path {
                                d: "M28 0 L56 16 L56 32 L28 48 L0 32 L0 16 Z",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1",
                            }
                        }
                    }
                    rect {
                        width: "100%", height: "100%",
                        fill: "url(#nf-hex)",
                        class: "text-[color:var(--color-accent)]",
                    }
                }
            }

            div { class: "container-content py-28 md:py-36 text-center relative",
                div { class: "relative inline-block animate-scale-in",
                    span {
                        class: "block text-[10rem] md:text-[16rem] leading-none font-black tracking-tight text-gradient-red select-none",
                        aria_hidden: "true",
                        "404"
                    }
                    span {
                        class: "absolute inset-0 -z-10 blur-3xl opacity-40 text-[10rem] md:text-[16rem] leading-none font-black tracking-tight text-[color:var(--color-accent)] select-none",
                        aria_hidden: "true",
                        "404"
                    }
                }

                h1 { class: "mt-6 text-3xl md:text-5xl font-extrabold leading-tight max-w-2xl mx-auto animate-fade-in-up",
                    "Off the beaten "
                    span { class: "text-gradient-red", "supply chain." }
                }

                p { class: "mt-5 max-w-xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "We couldn't find "
                    code { class: "px-2 py-0.5 rounded-md bg-[color:var(--color-accent-quiet)] text-[color:var(--color-fg)]",
                        "{path}"
                    }
                    " — the page may have moved when we migrated heartland.io to this stack."
                }

                div { class: "mt-10 flex items-center justify-center gap-3 flex-wrap animate-fade-in-up delay-2",
                    Link { to: Route::Landing {}, class: "btn-accent-gradient", "Back to home" }
                    Link { to: Route::Contact {},
                        class: "px-5 py-3 rounded-md border border-[color:var(--color-border)] hover:border-[color:var(--color-accent)] transition-colors",
                        "Talk to us →"
                    }
                }

                div { class: "mt-16 max-w-4xl mx-auto grid sm:grid-cols-2 lg:grid-cols-4 gap-4 text-left animate-fade-in-up delay-3",
                    DestinationCard {
                        to: Route::WhyImperium {},
                        eyebrow: "Start here",
                        title: "Why Imperium",
                        body: "The science behind the carbon-negative platform.",
                    }
                    DestinationCard {
                        to: Route::ImperiumMasterbatch {},
                        eyebrow: "Products",
                        title: "Imperium Masterbatch",
                        body: "Drop-in compounds that cut cost and carbon.",
                    }
                    DestinationCard {
                        to: Route::News {},
                        eyebrow: "News room",
                        title: "Latest articles",
                        body: "Field updates, customer wins, and announcements.",
                    }
                    DestinationCard {
                        to: Route::Lca {},
                        eyebrow: "Proof",
                        title: "Imperium Farming LCA",
                        body: "Third-party life cycle assessment of the platform.",
                    }
                }
            }
        }
    }
}

#[component]
fn DestinationCard(
    to: Route,
    eyebrow: &'static str,
    title: &'static str,
    body: &'static str,
) -> Element {
    rsx! {
        Link {
            to: to,
            class: "group relative block p-5 rounded-xl border border-[color:var(--color-border)] bg-[color:var(--color-surface)] hover:border-[color:var(--color-accent)] transition-colors",
            p { class: "text-xs uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-2",
                "{eyebrow}"
            }
            p { class: "font-bold text-base mb-1 group-hover:text-[color:var(--color-accent)] transition-colors",
                "{title}"
            }
            p { class: "text-sm text-[color:var(--color-fg-muted)] leading-snug",
                "{body}"
            }
            span { class: "absolute top-4 right-4 text-[color:var(--color-fg-muted)] group-hover:text-[color:var(--color-accent)] transition-colors",
                "→"
            }
        }
    }
}
