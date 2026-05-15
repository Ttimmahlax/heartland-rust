use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn SustainablePaper() -> Element {
    rsx! {
        Seo {
            title: "Sustainable Paper",
            description: "Heartland supplies sustainable paper additives to reduce wood and calcium carbonate usage in the paper industry.",
            path: "/sustainable-paper-additives",
        }

        Hero {}
        LogoCarousel { heading: "" }
        Body {}
        ClosingCta {}
        NewsCarousel { heading: "Related Articles" }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section {
            class: "video-hero-section section-soft-bottom min-h-[110vh] flex items-center pb-[20vh]",
            VideoBackground { slug: "sustainable-paper-additives".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "Sustainable Paper"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Sustainable paper additives"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "The price and supply of hardwood and softwood have been under pressure for years. Heartland's Imperium Filler can be used to replace and augment wood pulp, calcium carbonate, and other paper additives."
                }
            }
        }
    }
}

#[component]
fn Body() -> Element {
    rsx! {
        section { class: "container-content py-16 md:py-20",
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Alternative Natural Fiber Additives" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "The price and supply of hardwood and softwood have been under pressure for years. Heartland's Imperium Filler can be used to replace and augment wood pulp, calcium carbonate, and other paper additives." }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/sustainable-paper-additives/hemp-reinforced-boxes-in-a-storage-facility.png",
                        alt: "Alternative Natural Fiber Additives",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Regenerative Agriculture Supercharges Biomass" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland supports local farming for local manufacturing by growing industrial hemp to enable supply chain resiliency." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Hemp fiber grows in 90-110 days, whereas trees grow in 10-20 years. Companies that rely on wood and paper can hedge their risk by creating formulations that use hemp additives." }
                }
                div { class: "md:order-2",
                    img {
                        src: "/assets/pages/sustainable-paper-additives/hemp-reinforced-paper-bowls.png",
                        alt: "Regenerative Agriculture Supercharges Biomass",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "For The Paper Industry" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland supplies lower-carbon-footprint materials to help manufacturers create stronger, lighter, cheaper, and more sustainable products." }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/imperium-masterbatch/imperium-filler.png",
                        alt: "Imperium Filler",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Unlocking The Sustainable Future We Need And Deserve" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland's materials replace and augment materials used in pulping like calcium carbonate and wood." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "We work with global brands and their suppliers to predictably reduce the carbon footprint of everyday paper products without any retooling costs." }
                }
                div { class: "md:order-2",
                    img {
                        src: "/assets/pages/sustainable-paper-additives/plastic-pallets-6-1.png",
                        alt: "hemps saving the forest",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/sustainable-paper-additives/natural-fiber-filled-polypropylene-pp-1.png", alt: "hemp single use packaging", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            div { class: "max-w-3xl mx-auto mb-16 animate-fade-in-up",
                h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 text-center", "Throughout The Supply Chain" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Heartland partners with corn, wheat, and soy farmers to embed industrial hemp into their crop rotation. Our USDA Grant has given us unique insights into industrial hemp farming, regenerative agriculture, and carbon sequestration." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Pulper Heartland partners with paper pulpers to augment wood and calcium carbonate without any retooling costs. Our Imperium fillers improve strength and reduce the cost and carbon footprint of pulped products." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Converter Heartland partners with paper converters to ensure the hemp-filled pulp is processed properly. Our team works alongside component manufacturers to process natural fiber-filled pulp with the same molds used today." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Brand Heartland is the sustainability partner for brands on their journey to reduce their carbon footprint. Our team helps create stakeholder alignment so companies can effectively communicate the value of sustainable material innovation." }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/why-imperium/heartland-plastic-picture-3.png", alt: "Markets We Amplify", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/why-imperium/heartland-packaging-plastic-pallets.png", alt: "", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }        }
    }
}

#[component]
fn ClosingCta() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 my-12 section-soft-edges",
            div { class: "container-content text-center",
                h2 { class: "text-3xl md:text-4xl font-bold mb-6 max-w-2xl mx-auto",
                    "Learn more about Sustainable"
                }
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Get in touch" }
            }
        }
    }
}
