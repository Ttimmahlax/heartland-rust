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

        section { class: "bg-mesh-hero section-soft-bottom",
            div { class: "container-content py-24 text-center",
                p { class: "text-sm uppercase tracking-[0.25em] text-[color:var(--color-accent)] mb-4",
                    "404"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-2xl mx-auto",
                    "This page took a "
                    span { class: "text-gradient-red", "different rotation." }
                }
                p { class: "mt-6 max-w-xl mx-auto text-lg text-[color:var(--color-fg-muted)]",
                    "We couldn't find "
                    code { class: "text-[color:var(--color-fg)]", "{path}" }
                    " — it may have moved when we migrated heartland.io to this stack."
                }
                div { class: "mt-8 flex items-center justify-center gap-3 flex-wrap",
                    Link { to: Route::Landing {}, class: "btn-accent-gradient", "Back to home" }
                    Link { to: Route::News {},
                        class: "px-5 py-3 rounded-md border border-[color:var(--color-border)] hover:border-[color:var(--color-accent)]",
                        "Browse articles →"
                    }
                }
            }
        }
    }
}
