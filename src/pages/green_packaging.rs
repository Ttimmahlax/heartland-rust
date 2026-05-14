use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{default_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn GreenPackaging() -> Element {
    rsx! {
        Seo {
            title: "Green Packaging Initiative",
            description: "A multi-brand commitment to replace fossil-derived industrial packaging — pallets, returnable bins, crates — with Imperium-inside HDPE alternatives.",
            path: "/green-packaging-initiative",
        }

        Hero {}
        StatCounters { stats: default_stats() }
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
                    "Industry commitment"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    "Green "
                    span { class: "text-gradient-red", "Packaging Initiative" }
                    " — swap fossil for Imperium."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "A coordinated commitment by manufacturers and packaging molders to swap fossil-derived industrial returnable packaging for Imperium-inside HDPE alternatives."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "What's in scope" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Pallets — 48×40 GMA + every common international footprint." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Returnable bins, totes, and crates." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Industrial-grade pallet-collars and accessory components." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Why a coordinated initiative" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Single-brand Imperium adoption stalls on tooling amortization. A coordinated initiative spreads tooling costs across multiple SKUs and gives molders predictable volumes." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Partners" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Magna, Magnolia Partners, Continental, and others — see /sustainability-news for partnership announcements." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "BASF + Magna joint development for low-carbon polyamide-6 — Altair Enlighten Award finalist." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Join the initiative" }
            }
        }
    }
}
