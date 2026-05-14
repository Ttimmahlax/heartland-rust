use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{default_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn Automotive() -> Element {
    rsx! {
        Seo {
            title: "Automotive Applications",
            description: "Imperium-reinforced plastics for automotive interior, under-hood, and exterior trim — drop-in lightweighting with verified LCA carbon credit.",
            path: "/automotive",
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
                    "Mobility"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    span { class: "text-gradient-red", "Heartland × Automotive" }
                    " — lighter, lower-carbon parts."
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)] animate-fade-in-up delay-1",
                    "Imperium-reinforced PP, PA6 and PC/ABS slot into automotive supplier programs to lightweight parts while cutting Scope 3 embedded carbon."
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
                        h2 { class: "text-2xl font-display font-bold mb-4", "Where Imperium shows up in a vehicle" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Interior trim panels, door cards, headliner substrates." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Under-hood air-handling ducting, fluid reservoirs, fan shrouds (Imperium-PA6)." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Cargo deck and load-floor structures." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Battery enclosure and EV under-body components in development." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "OEM + Tier 1 programs" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Continental sustainability material innovation challenge — winner." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Magna × BASF × Heartland — Altair Enlighten Award finalist for low-carbon polyamide 6." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "GM, Stellantis, Ford supplier engagements active." }
                    }
                    div { class: "surface-glass p-7 animate-fade-in-up",
                        h2 { class: "text-2xl font-display font-bold mb-4", "Scope 3 disclosure" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Imperium ships with a per-part LCA disclosure pack via Carbon Report." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0", "Aligned with SEC, CSRD, and CDP supply-chain disclosure needs." }
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
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Engage with the OEM team" }
            }
        }
    }
}
