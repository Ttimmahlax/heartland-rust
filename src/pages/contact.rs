use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn Contact() -> Element {
    rsx! {
        Seo {
            title: "Contact",
            description: "Get in touch with Heartland Industries — sales, engineering, farmer programs, press, and partnership inquiries. Hello@heartland.io.",
            path: "/contact",
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
                    "Get in touch"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    "Talk to "
                    span { class: "text-gradient-red", "Heartland" }
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "We route inbound requests to the right team within one business day. Engineering questions, sample requests, farmer applications, and partnership opportunities — every one ends up in the right inbox."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Email" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Hello@heartland.io — the catch-all. We triage daily." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "For press: include 'press' in the subject line." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "What helps us respond fast" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "For engineering inquiries: include the resin family, the current filler, and the loading percentage." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "For farmer inquiries: include your state, current crops, and acreage." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "For partnerships: include the structure you're thinking about + the volume range." }
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
                Link { to: Route::Faq {}, class: "btn-accent-gradient", "Read the FAQ" }
            }
        }
    }
}
