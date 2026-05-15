use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::seo::Seo;

#[component]
pub fn Team() -> Element {
    rsx! {
        Seo {
            title: "Heartland Team",
            description: "Meet the Heartland Industries team — executives and advisors building the future of carbon-negative materials and sustainable supply chains.",
            path: "/heartland-team",
        }

        Hero {}
        LogoCarousel { heading: "" }
        Executives {}
        Advisors {}
        NewsCarousel { heading: "Related Articles" }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        // Reuses .video-hero-section / .video-hero-scrim / .video-hero-content
        // so the header / scrim / animations behave identically to other pages.
        // The video element is swapped for a static <img> filling the section.
        section {
            class: "video-hero-section section-soft-bottom min-h-[110vh] flex items-center pb-[20vh]",
            img {
                class: "video-hero-bg",
                src: "/assets/pages/heartland-team/team-hero.png",
                alt: "Heartland's Detroit, Michigan headquarters",
                loading: "eager",
            }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "Our Heartland To Yours"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "The Heartland"
                    br {}
                    "Team"
                }
            }
        }
    }
}

/// Compile-time data table of teammates. Order = display order.
#[derive(Clone, Copy, PartialEq)]
struct Person {
    slug: &'static str,
    name: &'static str,
    role: &'static str,
}

const EXECUTIVES: &[Person] = &[
    Person { slug: "john-ely",        name: "John Ely",        role: "Chief Executive Officer" },
    Person { slug: "tim-almond",      name: "Tim Almond",      role: "Chairman & Chief Operating Officer" },
    Person { slug: "robby-dameron",   name: "Robby Dameron",   role: "Materials Science" },
    Person { slug: "markus-von-graf", name: "Markus Von Graf", role: "Strategy & Capital Markets" },
];

const ADVISORS: &[Person] = &[
    Person { slug: "eric-austermann", name: "Eric Austermann", role: "Engineering Advisor" },
    Person { slug: "roger-blackwell", name: "Roger Blackwell", role: "Senior Advisor" },
    Person { slug: "deborah-labelle", name: "Deborah Labelle", role: "General Counsel" },
];

#[component]
fn Executives() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "text-center mb-12 max-w-3xl mx-auto",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "Leadership"
                }
                h2 { class: "text-3xl md:text-5xl font-bold leading-tight",
                    "Heartland "
                    span { class: "text-gradient-red", "Executives" }
                }
            }
            div { class: "grid grid-cols-2 md:grid-cols-4 gap-6 md:gap-8",
                for p in EXECUTIVES.iter() {
                    TeamCard { key: "{p.slug}", person: *p }
                }
            }
        }
    }
}

#[component]
fn Advisors() -> Element {
    rsx! {
        section { class: "container-content pb-16 md:pb-24",
            div { class: "text-center mb-12 max-w-3xl mx-auto",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "Supply Chain & Strategy"
                }
                h2 { class: "text-3xl md:text-5xl font-bold leading-tight",
                    "Heartland "
                    span { class: "text-gradient-red", "Advisors" }
                }
            }
            div { class: "grid grid-cols-2 md:grid-cols-3 gap-6 md:gap-8 max-w-4xl mx-auto",
                for p in ADVISORS.iter() {
                    TeamCard { key: "{p.slug}", person: *p }
                }
            }
        }
    }
}

#[component]
fn TeamCard(person: Person) -> Element {
    let img_src = format!("/assets/pages/heartland-team/{}.png", person.slug);
    let alt = format!("{} — {}", person.name, person.role);
    rsx! {
        div { class: "group surface-glass rounded-xl overflow-hidden text-center animate-fade-in-up",
            div { class: "aspect-[3/4] overflow-hidden bg-[color:var(--color-surface)]",
                img {
                    src: "{img_src}",
                    alt: "{alt}",
                    loading: "lazy",
                    class: "w-full h-full object-cover transition-transform duration-300 group-hover:scale-105",
                }
            }
            div { class: "p-4 md:p-5",
                h3 { class: "text-base md:text-lg font-display font-bold mb-1 group-hover:text-[color:var(--color-accent)]",
                    "{person.name}"
                }
                p { class: "text-xs md:text-sm text-[color:var(--color-fg-muted)] leading-snug",
                    "{person.role}"
                }
            }
        }
    }
}
