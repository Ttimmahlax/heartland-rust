//! Carbon Footprint Calculator — embedded Retool iframe + CTA.
//!
//! Same widget heartland.io ships at the bottom of the landing page; reused
//! across pages (currently /, /why-imperium). The iframe is allow-listed in
//! `customHttp.yml` under `frame-src https://*.retool.com`.

use dioxus::prelude::*;

#[component]
pub fn CarbonCalculator() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "text-center mb-10 max-w-3xl mx-auto",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "Sustainable Material Innovation"
                }
                h2 { class: "text-3xl md:text-4xl font-bold",
                    "Try Our "
                    span { class: "text-gradient-red", "Carbon Footprint Calculator" }
                }
            }
            div { class: "max-w-5xl mx-auto rounded-xl overflow-hidden shadow-lg border border-[color:var(--color-border)] bg-[color:var(--color-surface)]",
                iframe {
                    src: "https://heartland.retool.com/embedded/public/62ab75c7-b652-4139-90a4-393a67b6148e",
                    width: "100%",
                    height: "450",
                    style: "border: 0; display: block;",
                    title: "Carbon Footprint Calculator",
                }
            }
            div { class: "mt-8 text-center",
                a {
                    href: "https://www.carbon-report.com",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "btn-accent-gradient inline-block",
                    "Try Carbon Report Free →"
                }
            }
        }
    }
}
