//! Press-mention logo carousel — pure CSS infinite scroll, no JS.
//!
//! Replicates the `owl-stage-outer` "as seen in" strip from heartland.io. The
//! 14 outbound links + logos are baked into a const so they ship at compile
//! time. The track is duplicated end-to-end so the keyframe loop is seamless;
//! see `.logo-carousel-track` in [tailwind.css](../../tailwind.css).

use dioxus::prelude::*;

/// (slug, asset filename in assets/press/, alt text, outbound article URL)
const LOGOS: &[(&str, &str, &str, &str)] = &[
    (
        "dbusiness",
        "dbusiness.webp",
        "Heartland in DBusiness",
        "https://www.dbusiness.com/tech-mobility-news/heartland-in-detroit-and-ravago-develop-hemp-based-plastic-additives/",
    ),
    (
        "composites-world",
        "composites-world.webp",
        "Heartland in Composites World",
        "https://www.compositesworld.com/articles/natural-fiber-composites-growing-to-fit-sustainability-needs",
    ),
    (
        "forbes",
        "forbes.webp",
        "Heartland on Forbes 30 Under 30",
        "https://www.forbes.com/profile/tim-almond/?list=30under30-manufacturing-industry",
    ),
    (
        "plastics-technology",
        "plastics-technology.webp",
        "Heartland in Plastics Technology",
        "https://www.ptonline.com/news/heartland-industries-to-open-industrial-hemp-processing-facility-",
    ),
    (
        "agweb",
        "agweb.webp",
        "Heartland in AgWeb / Farm Journal",
        "https://www.agweb.com/news/business/technology/million-dollar-math-problem-help-farmers-optimize-travel-and-fieldwork",
    ),
    (
        "cleantechnica",
        "cleantechnica.webp",
        "Heartland in CleanTechnica",
        "https://cleantechnica.com/2024/03/13/precision-agriculture-the-future-of-farming/",
    ),
    (
        "sustainable-plastics",
        "sustainable-plastics.webp",
        "Heartland in Sustainable Plastics",
        "https://www.sustainableplastics.com/news/heartland-ravago-collaborating-new-hemp-additives-plastics",
    ),
    (
        "hemp-industry-daily",
        "hemp-industry-daily.webp",
        "Heartland in Hemp Industry Daily",
        "https://hempindustrydaily.com/michigan-hemp-tech-firm-heartland-partners-with-international-recycling-company/",
    ),
    (
        "basf",
        "basf.webp",
        "Heartland × BASF investment",
        "https://www.basf.com/global/en/media/news-releases/2024/05/p-24-191.html",
    ),
    (
        "gardner-web",
        "gardner-web.webp",
        "Heartland in Gardner Web",
        "https://www.gardnerweb.com/news/building-a-case-for-hemp-based-car-parts",
    ),
    (
        "plastics-news",
        "plastics-news.webp",
        "Heartland in Plastics News",
        "https://www.plasticsnews.com/news/heartland-ravago-collaborating-new-hemp-additives-plastics",
    ),
    (
        "future-net-zero",
        "future-net-zero.webp",
        "Heartland in Future Net Zero",
        "https://www.futurenetzero.com/2022/01/25/new-partnership-to-reduce-the-carbon-footprint-of-plastics/",
    ),
    (
        "market-watch",
        "market-watch.webp",
        "Heartland in MarketWatch",
        "https://www.marketwatch.com/press-release/heartland-and-ravago-develop-products-to-reduce-the-carbon-footprint-of-plastic-2022-01-26?tesla=y",
    ),
    (
        "amazon-climate",
        "amazon-climate.webp",
        "Heartland in Amazon Climate Tech Accelerator",
        "https://press.aboutamazon.com/devices/2026/1/amazon-devices-climate-tech-accelerator-opens-applications-for-2026-program",
    ),
];

#[derive(Props, Clone, PartialEq)]
pub struct LogoCarouselProps {
    #[props(default = String::from("As Seen In"))]
    pub heading: String,
}

#[component]
pub fn LogoCarousel(props: LogoCarouselProps) -> Element {
    let heading = props.heading.clone();
    let show_heading = !heading.is_empty();
    rsx! {
        section { class: "logo-carousel-section py-20 md:py-28 overflow-hidden",
            if show_heading {
                div { class: "container-content text-center mb-8",
                    h2 { class: "text-xs md:text-sm uppercase tracking-[0.25em] text-[color:var(--color-fg-muted)] font-bold",
                        "{heading}"
                    }
                }
            }
            div { class: "logo-carousel-viewport",
                div { class: "logo-carousel-track",
                    // First copy
                    for (slug, file, alt, href) in LOGOS.iter() {
                        a {
                            key: "a-{slug}",
                            href: "{href}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "logo-carousel-item",
                            aria_label: "{alt}",
                            img {
                                src: "/assets/press/{file}",
                                alt: "{alt}",
                                loading: "lazy",
                                class: "logo-carousel-img",
                            }
                        }
                    }
                    // Second copy — required for the seamless loop. The
                    // keyframe shifts the track by -50%, so two copies create
                    // an infinite belt where the second copy starts exactly
                    // where the first one ended.
                    for (slug, file, _alt, href) in LOGOS.iter() {
                        a {
                            key: "b-{slug}",
                            href: "{href}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "logo-carousel-item",
                            aria_hidden: "true",
                            tabindex: "-1",
                            img {
                                src: "/assets/press/{file}",
                                alt: "",
                                loading: "lazy",
                                class: "logo-carousel-img",
                            }
                        }
                    }
                }
            }
        }
    }
}
