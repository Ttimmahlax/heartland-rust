use dioxus::prelude::*;

use crate::components::logo_carousel::LogoCarousel;
use crate::components::news_carousel::NewsCarousel;
use crate::components::video_hero::VideoBackground;
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn EngineeringEarth() -> Element {
    rsx! {
        Seo {
            title: "Engineering Earth",
            description: "Engineering Earth is focused on supercharging the greatest carbon sequestration vehicle we have: our planet.",
            path: "/engineering-earth",
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
            VideoBackground { slug: "engineering-earth".to_string() }
            div { class: "video-hero-scrim" }
            div { class: "video-hero-content container-content w-full py-24 md:py-32 text-center",
                p { class: "text-[0.7438rem] uppercase tracking-[0.25em] text-white/90 mb-4 animate-fade-in",
                    "Engineering Earth"
                }
                h1 {
                    class: "text-3xl md:text-5xl font-extrabold leading-tight uppercase tracking-tight text-white max-w-4xl mx-auto animate-fade-in-up",
                    "Engineering Earth One Farm at a time"
                }
                p {
                    class: "mt-5 max-w-2xl mx-auto text-base md:text-lg text-white/85 animate-fade-in-up delay-1",
                    "Earth is a carbon sequestration powerhouse. Our planet’s rainforests act as its lungs, and its soil acts as its skin."
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
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Empowering Earth, Our Most Effective Tool." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Earth is a carbon sequestration powerhouse. Our planet’s rainforests act as its lungs, and its soil acts as its skin." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Engineering Earth to optimize the carbon cycle is the most effective solution we have to create a sustainable future. Farmers are the key." }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/engineering-earth/mining.png",
                        alt: "rainforest",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Farmland is Four Times Bigger Than The Amazon" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Agricultural practices have the potential to help or harm our planet more than the Amazon." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "4.25 billion acres on Earth are farmland. Implementing regenerative farming practices can empower Earth to store more carbon in less time than any manmade technology ever could." }
                }
                div { class: "md:order-2",
                    img {
                        src: "/assets/pages/engineering-earth/mining-1.png",
                        alt: "farmland 1",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "max-w-3xl mx-auto mb-16 animate-fade-in-up",
                h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 text-center", "For Regenerative Farming" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "The Heartland team has identified 3 practical regenerative agriculture practices that will unlock carbon credit markets and revenue opportunities for farmers." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Soil compaction is a persistent problem with heavy mechanized equipment. Plants do not grow well in compressed soil. Farmers who adopt tracks instead of tires reduce the amount of compression on the ground." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Ditch Your Plow One of the most common ways to eliminate soil compression is with a deep plow, which can dig 36\" into the ground. By switching to a vertical tiller, you only disrupt 3\" of soil and preserve more than 80% of the organic matter removed by the plow." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "_ Cover Your Soil Cover crops are more important than simply preventing soil erosion. During cold winters, they act as a canopy for organic life. This ensures healthy soils while reducing nitrate runoff into local water supplies." }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-1",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "Unlocking The Sustainable Future We Have Earned" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "Regenerative agriculture can empower Earth to store more carbon in the soil than any technology." }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "We work with global brands and their suppliers to understand the opportunity and impact of carbon farming." }
                }
                div { class: "md:order-2",
                    img {
                        src: "/assets/pages/engineering-earth/mining-2.png",
                        alt: "farmhouse",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            div { class: "grid md:grid-cols-2 gap-10 items-center mb-16 md:mb-24 animate-fade-in-up",
                div { class: "md:order-2",
                    h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5", "The Electric Vehicle Transition Brings Uncertainty To Ethanol" }
                            p { class: "text-[color:var(--color-fg-muted)] mb-4 last:mb-0", "There are 40 million acres of corn farmed for ethanol here in America. By 2030, every major car manufacturer has committed to only making electric vehicles. This will create a significant reduction in the demand for ethanol." }
                }
                div { class: "md:order-1",
                    img {
                        src: "/assets/pages/engineering-earth/mining-3.png",
                        alt: "tesla",
                        loading: "lazy",
                        class: "w-full rounded-xl shadow-lg",
                    }
                }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/why-imperium/heartland-plastic-picture-3.png", alt: "Markets We Amplify", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            figure { class: "mb-16 animate-fade-in-up",
                img { src: "/assets/pages/why-imperium/heartland-packaging-plastic-pallets.png", alt: "", loading: "lazy", class: "w-full rounded-xl shadow-lg" }
            }
            h2 { class: "text-2xl md:text-3xl font-display font-bold mb-5 mt-12 text-center", "Try Our Carbon Footprint Calculator" }        }
    }
}

#[component]
fn ClosingCta() -> Element {
    rsx! {
        section { class: "bg-mesh-dramatic py-20 my-12 section-soft-edges",
            div { class: "container-content text-center",
                h2 { class: "text-3xl md:text-4xl font-bold mb-6 max-w-2xl mx-auto",
                    "Learn more about Engineering"
                }
                Link { to: Route::Contact {}, class: "btn-accent-gradient", "Get in touch" }
            }
        }
    }
}
