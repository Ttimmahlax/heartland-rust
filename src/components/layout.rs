//! Header (sticky + glass) + Footer (4-col + social), wired around `<Outlet>`.

use dioxus::prelude::*;

use crate::components::contact_block::ContactBlock;
use crate::tracking::{TrackingFooter, TrackingHead, REF_CAPTURE_JS};
use crate::Route;

const HEADER_SCROLL_JS: &str = r#"
(function(){
  function bind() {
    var hdr = document.getElementById('site-header');
    if (!hdr) { setTimeout(bind, 50); return; }
    var update = function() {
      if (window.scrollY > 4) hdr.dataset.scrolled = '1';
      else delete hdr.dataset.scrolled;
    };
    window.addEventListener('scroll', update, { passive: true });
    update();
  }
  bind();
})();
"#;

#[component]
pub fn LayoutShell() -> Element {
    // Derive the top-level slug from the current route so the contact block
    // can pick the right HubSpot form ID. e.g. "/why-imperium" → "why-imperium".
    // For nested routes like "/sustainability-news/<x>" we use the first
    // segment ("sustainability-news"), which maps to the news-index form.
    let page_slug = use_route::<Route>()
        .to_string()
        .trim_start_matches('/')
        .trim_end_matches('/')
        .split('/')
        .next()
        .unwrap_or("")
        .to_string();

    rsx! {
        TrackingHead {}
        Header {}
        main { class: "flex-1",
            Outlet::<Route> {}
            ContactBlock { page_slug }
        }
        Footer {}
        TrackingFooter {}
        document::Script { "{REF_CAPTURE_JS}" }
        document::Script { "{HEADER_SCROLL_JS}" }
    }
}

#[component]
pub fn Header() -> Element {
    let mut menu_open = use_signal(|| false);

    rsx! {
        header {
            id: "site-header",
            class: "site-header fixed top-0 left-0 right-0 z-50",
            div {
                class: "container-content grid grid-cols-[1fr_auto_1fr] items-center gap-6 py-[1.2rem]",

                Link {
                    to: Route::Landing {},
                    class: "flex items-center gap-2 justify-self-start",
                    aria_label: "Heartland Industries — home",
                    img {
                        src: "/assets/brand/heartland-logo-light.png",
                        alt: "Heartland Industries",
                        class: "site-header-logo-light h-[1.1rem] md:h-[1.375rem] w-auto block dark:hidden",
                    }
                    img {
                        src: "/assets/brand/heartland-logo-dark.png",
                        alt: "",
                        aria_hidden: "true",
                        class: "site-header-logo-dark h-[1.1rem] md:h-[1.375rem] w-auto hidden dark:block",
                    }
                }

                nav {
                    class: "hidden lg:flex items-center gap-5 text-sm font-bold justify-self-center",
                    aria_label: "Primary navigation",
                    Link { to: Route::WhyImperium {}, class: "hover:text-[color:var(--color-accent)]", "Why Imperium" }
                    ProductsDropdown {}
                    IndustriesDropdown {}
                    ResourcesDropdown {}
                    AboutDropdown {}
                }

                // Right-side cluster: solid Contact CTA + mobile hamburger.
                // The Contact link scrolls to the ContactBlock anchor on the
                // current page (every page renders <section id="contact">
                // via LayoutShell).
                div { class: "flex items-center gap-2 md:gap-3 justify-self-end col-start-3",
                    a {
                        href: "#contact",
                        class: "site-header-cta inline-flex items-center justify-center px-3 py-2 md:px-4 md:py-2 rounded-md bg-[#ad2929] text-white text-xs md:text-sm font-bold tracking-wide hover:bg-[#931f1f] hover:text-white transition-colors shadow-md",
                        "Contact"
                    }
                    button {
                        class: "lg:hidden inline-flex items-center justify-center w-10 h-10 rounded-md border border-[color:var(--color-border)]",
                        aria_label: "Toggle navigation menu",
                        onclick: move |_| menu_open.toggle(),
                        if menu_open() {
                            svg { width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                line { x1: "18", y1: "6", x2: "6", y2: "18" }
                                line { x1: "6", y1: "6", x2: "18", y2: "18" }
                            }
                        } else {
                            svg { width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                line { x1: "3", y1: "6", x2: "21", y2: "6" }
                                line { x1: "3", y1: "12", x2: "21", y2: "12" }
                                line { x1: "3", y1: "18", x2: "21", y2: "18" }
                            }
                        }
                    }
                }
            }

            if menu_open() {
                MobileMenu { close: move || menu_open.set(false) }
            }
        }
    }
}

#[component]
fn DropdownItem(to: Route, title: &'static str) -> Element {
    rsx! {
        Link { to: to, class: "block px-3 py-2 rounded-md text-sm font-medium hover:bg-[color:var(--color-accent-quiet)]",
            "{title}"
        }
    }
}

#[component]
fn ExternalDropdownItem(href: &'static str, title: &'static str) -> Element {
    rsx! {
        a { href: href, target: "_blank", rel: "noopener noreferrer",
            class: "block px-3 py-2 rounded-md text-sm font-medium hover:bg-[color:var(--color-accent-quiet)]",
            "{title} ↗"
        }
    }
}

#[component]
fn ColumnHeading(label: &'static str) -> Element {
    rsx! {
        div { class: "px-3 pt-1 pb-2 text-xs uppercase tracking-[0.15em] text-[color:var(--color-fg-muted)]",
            "{label}"
        }
    }
}

#[component]
fn ProductsDropdown() -> Element {
    rsx! {
        div { class: "relative group",
            button { class: "inline-flex items-center gap-1 font-bold hover:text-[color:var(--color-accent)]", r#type: "button", aria_haspopup: "true",
                "Products"
                svg { width: "12", height: "12", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                    polyline { points: "6 9 12 15 18 9" }
                }
            }
            div { class: "absolute left-0 top-full pt-2 invisible group-hover:visible group-focus-within:visible",
                div { class: "nav-dropdown-panel min-w-60 rounded-lg bg-[color:var(--color-surface)] border border-[color:var(--color-border)] p-2 shadow-xl",
                    DropdownItem { to: Route::ImperiumMasterbatch {}, title: "Imperium Masterbatch" }
                    DropdownItem { to: Route::ImperiumFilledResin {}, title: "Performance Plastics" }
                    DropdownItem { to: Route::ImperiumFiller {}, title: "Imperium Filler" }
                    DropdownItem { to: Route::ImperiumFibers {}, title: "Imperium Textile Fiber" }
                    DropdownItem { to: Route::ImperiumAnimalFeed {}, title: "Imperium Animal Feed" }
                }
            }
        }
    }
}

#[component]
fn IndustriesDropdown() -> Element {
    rsx! {
        div { class: "relative group",
            button { class: "inline-flex items-center gap-1 font-bold hover:text-[color:var(--color-accent)]", r#type: "button", aria_haspopup: "true",
                "Industries"
                svg { width: "12", height: "12", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                    polyline { points: "6 9 12 15 18 9" }
                }
            }
            div { class: "absolute left-0 top-full pt-2 invisible group-hover:visible group-focus-within:visible",
                div { class: "nav-dropdown-panel rounded-lg bg-[color:var(--color-surface)] border border-[color:var(--color-border)] p-2 shadow-xl grid grid-cols-2 gap-2 min-w-[28rem]",
                    div {
                        ColumnHeading { label: "Markets" }
                        DropdownItem { to: Route::SustainablePlastic {}, title: "Plastic Compounding" }
                        DropdownItem { to: Route::Automotive {}, title: "Automotive" }
                        DropdownItem { to: Route::SustainablePackaging {}, title: "Packaging" }
                        DropdownItem { to: Route::SustainableBuilding {}, title: "Construction" }
                        DropdownItem { to: Route::Marine {}, title: "Marine" }
                        DropdownItem { to: Route::Government {}, title: "Government" }
                    }
                    div {
                        ColumnHeading { label: "Materials" }
                        DropdownItem { to: Route::SustainablePlastic {}, title: "Plastic" }
                        DropdownItem { to: Route::SustainableRubber {}, title: "Rubber" }
                        DropdownItem { to: Route::ImperiumFibers {}, title: "Textiles" }
                        DropdownItem { to: Route::SustainableConcrete {}, title: "Concrete" }
                        DropdownItem { to: Route::SustainableAsphalt {}, title: "Asphalt" }
                        DropdownItem { to: Route::SustainablePaper {}, title: "Paper" }
                    }
                }
            }
        }
    }
}

#[component]
fn ResourcesDropdown() -> Element {
    rsx! {
        div { class: "relative group",
            button { class: "inline-flex items-center gap-1 font-bold hover:text-[color:var(--color-accent)]", r#type: "button", aria_haspopup: "true",
                "Resources"
                svg { width: "12", height: "12", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                    polyline { points: "6 9 12 15 18 9" }
                }
            }
            div { class: "absolute right-0 top-full pt-2 invisible group-hover:visible group-focus-within:visible",
                div { class: "nav-dropdown-panel rounded-lg bg-[color:var(--color-surface)] border border-[color:var(--color-border)] p-2 shadow-xl grid grid-cols-3 gap-2 min-w-[42rem]",
                    div {
                        ColumnHeading { label: "Innovation" }
                        DropdownItem { to: Route::EngineeringEarth {}, title: "Engineering Earth" }
                        DropdownItem { to: Route::Lca {}, title: "Imperium Farming LCA" }
                        ExternalDropdownItem { href: "https://www.carbon-report.com", title: "Carbon Report" }
                    }
                    div {
                        ColumnHeading { label: "News Room" }
                        DropdownItem { to: Route::News {}, title: "Articles" }
                        DropdownItem { to: Route::Ebooks {}, title: "E-Books" }
                        ExternalDropdownItem { href: "https://hfga.io", title: "Hemp Fiber and Grain Association" }
                    }
                    div {
                        ColumnHeading { label: "Knowledge Base" }
                        DropdownItem { to: Route::Faq {}, title: "FAQ" }
                        DropdownItem { to: Route::Research {}, title: "Research" }
                        DropdownItem { to: Route::Whitepapers {}, title: "White Papers" }
                    }
                }
            }
        }
    }
}

#[component]
fn AboutDropdown() -> Element {
    rsx! {
        div { class: "relative group",
            button { class: "inline-flex items-center gap-1 font-bold hover:text-[color:var(--color-accent)]", r#type: "button", aria_haspopup: "true",
                "About Us"
                svg { width: "12", height: "12", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                    polyline { points: "6 9 12 15 18 9" }
                }
            }
            div { class: "absolute right-0 top-full pt-2 invisible group-hover:visible group-focus-within:visible",
                div { class: "nav-dropdown-panel min-w-60 rounded-lg bg-[color:var(--color-surface)] border border-[color:var(--color-border)] p-2 shadow-xl",
                    DropdownItem { to: Route::Team {}, title: "Team" }
                    DropdownItem { to: Route::Farmers {}, title: "Our Farmers" }
                    DropdownItem { to: Route::GreenPackaging {}, title: "Green Packaging Initiative" }
                }
            }
        }
    }
}

#[component]
fn MobileMenu(close: EventHandler<()>) -> Element {
    rsx! {
        div { class: "lg:hidden border-t border-[color:var(--color-border)] max-h-[80vh] overflow-y-auto",
            div { class: "container-content py-4 flex flex-col gap-1 text-base",
                MobileLink { to: Route::WhyImperium {}, label: "Why Imperium", close }

                MobileSectionLabel { label: "Products" }
                MobileLink { to: Route::ImperiumMasterbatch {}, label: "Imperium Masterbatch", close }
                MobileLink { to: Route::ImperiumFilledResin {}, label: "Performance Plastics", close }
                MobileLink { to: Route::ImperiumFiller {}, label: "Imperium Filler", close }
                MobileLink { to: Route::ImperiumFibers {}, label: "Imperium Textile Fiber", close }
                MobileLink { to: Route::ImperiumAnimalFeed {}, label: "Imperium Animal Feed", close }

                MobileSectionLabel { label: "Industries — Markets" }
                MobileLink { to: Route::SustainablePlastic {}, label: "Plastic Compounding", close }
                MobileLink { to: Route::Automotive {}, label: "Automotive", close }
                MobileLink { to: Route::SustainablePackaging {}, label: "Packaging", close }
                MobileLink { to: Route::SustainableBuilding {}, label: "Construction", close }
                MobileLink { to: Route::Marine {}, label: "Marine", close }
                MobileLink { to: Route::Government {}, label: "Government", close }

                MobileSectionLabel { label: "Industries — Materials" }
                MobileLink { to: Route::SustainablePlastic {}, label: "Plastic", close }
                MobileLink { to: Route::SustainableRubber {}, label: "Rubber", close }
                MobileLink { to: Route::ImperiumFibers {}, label: "Textiles", close }
                MobileLink { to: Route::SustainableConcrete {}, label: "Concrete", close }
                MobileLink { to: Route::SustainableAsphalt {}, label: "Asphalt", close }
                MobileLink { to: Route::SustainablePaper {}, label: "Paper", close }

                MobileSectionLabel { label: "Resources — Innovation" }
                MobileLink { to: Route::EngineeringEarth {}, label: "Engineering Earth", close }
                MobileLink { to: Route::Lca {}, label: "Imperium Farming LCA", close }

                MobileSectionLabel { label: "Resources — News Room" }
                MobileLink { to: Route::News {}, label: "Articles", close }
                MobileLink { to: Route::Ebooks {}, label: "E-Books", close }

                MobileSectionLabel { label: "Resources — Knowledge Base" }
                MobileLink { to: Route::Faq {}, label: "FAQ", close }
                MobileLink { to: Route::Research {}, label: "Research", close }
                MobileLink { to: Route::Whitepapers {}, label: "White Papers", close }

                MobileSectionLabel { label: "About" }
                MobileLink { to: Route::Team {}, label: "Team", close }
                MobileLink { to: Route::Farmers {}, label: "Our Farmers", close }
                MobileLink { to: Route::GreenPackaging {}, label: "Green Packaging Initiative", close }
            }
        }
    }
}

#[component]
fn MobileSectionLabel(label: &'static str) -> Element {
    rsx! {
        div { class: "text-xs uppercase tracking-[0.15em] text-[color:var(--color-fg-muted)] mt-3 px-2", "{label}" }
    }
}

#[component]
fn MobileLink(to: Route, label: String, close: EventHandler<()>) -> Element {
    rsx! {
        Link {
            to: to,
            class: "py-2 px-2 rounded-md hover:bg-[color:var(--color-accent-quiet)]",
            onclick: move |_| close.call(()),
            "{label}"
        }
    }
}

#[component]
pub fn Footer() -> Element {
    let year = chrono::Datelike::year(&chrono::Utc::now());
    rsx! {
        footer {
            class: "mt-24 border-t border-[color:var(--color-border)] bg-[color:var(--color-surface)]",
            div {
                class: "container-content py-12 grid gap-10 md:grid-cols-4",

                div {
                    Link { to: Route::Landing {},
                        class: "flex items-center mb-3",
                        img {
                            src: "/assets/brand/heartland-logo-light.png",
                            alt: "Heartland Industries",
                            class: "h-6 w-auto block dark:hidden",
                        }
                        img {
                            src: "/assets/brand/heartland-logo-dark.png",
                            alt: "",
                            aria_hidden: "true",
                            class: "h-6 w-auto hidden dark:block",
                        }
                    }
                    p { class: "text-sm text-[color:var(--color-fg-muted)] max-w-xs",
                        "Heartland Imperium materials reduce the cost and carbon footprint of everyday products."
                    }

                    // Office addresses — Detroit + Brooklyn
                    div { class: "mt-5 space-y-3 max-w-xs",
                        div {
                            div { class: "text-xs uppercase tracking-[0.15em] text-[color:var(--color-fg)] font-semibold mb-1",
                                "Detroit Headquarters"
                            }
                            address { class: "text-sm text-[color:var(--color-fg-muted)] not-italic",
                                "2050 15th St, Detroit, MI 48216"
                            }
                        }
                        div {
                            div { class: "text-xs uppercase tracking-[0.15em] text-[color:var(--color-fg)] font-semibold mb-1",
                                "New York"
                            }
                            address { class: "text-sm text-[color:var(--color-fg-muted)] not-italic",
                                "19 Morris Ave, Brooklyn, NY 11205"
                            }
                        }
                    }

                    p { class: "mt-5 text-sm",
                        a { href: "mailto:Hello@heartland.io",
                            class: "text-[color:var(--color-fg-muted)] hover:text-[color:var(--color-accent)]",
                            "Hello@heartland.io"
                        }
                    }
                }

                FooterColumn {
                    title: "Products",
                    items: vec![
                        ("Imperium Filler",        Route::ImperiumFiller {}),
                        ("Imperium Textile Fiber", Route::ImperiumFibers {}),
                        ("Imperium Filled Resin",  Route::ImperiumFilledResin {}),
                        ("Why Imperium",           Route::WhyImperium {}),
                    ],
                }

                FooterColumn {
                    title: "Industries",
                    items: vec![
                        ("Plastic Compounders",     Route::SustainablePlastic {}),
                        ("Automotive",              Route::Automotive {}),
                        ("Marine",                  Route::Marine {}),
                        ("Packaging",               Route::SustainablePackaging {}),
                        ("Building Materials",      Route::SustainableBuilding {}),
                        ("Government",              Route::Government {}),
                    ],
                }

                div {
                    h3 { class: "font-display font-semibold mb-3", "Library" }
                    ul { class: "space-y-2 text-sm",
                        li { Link { to: Route::News {}, class: "text-[color:var(--color-fg-muted)] hover:text-[color:var(--color-accent)]", "Articles" } }
                        li { Link { to: Route::Ebooks {}, class: "text-[color:var(--color-fg-muted)] hover:text-[color:var(--color-accent)]", "E-Books" } }
                        li { Link { to: Route::Whitepapers {}, class: "text-[color:var(--color-fg-muted)] hover:text-[color:var(--color-accent)]", "White Papers" } }
                        li { Link { to: Route::EngineeringEarth {}, class: "text-[color:var(--color-fg-muted)] hover:text-[color:var(--color-accent)]", "Engineering Earth" } }
                        li { Link { to: Route::GreenPackaging {}, class: "text-[color:var(--color-fg-muted)] hover:text-[color:var(--color-accent)]", "Green Packaging Initiative" } }
                    }
                    h3 { class: "font-display font-semibold mt-6 mb-3", "Follow" }
                    div { class: "flex items-center gap-2 flex-wrap",
                        SocialIcon { href: "https://www.facebook.com/therealheartland/", label: "Follow Heartland on Facebook", icon: SocialKind::Facebook }
                        SocialIcon { href: "https://www.linkedin.com/company/therealheartland", label: "Follow Heartland on LinkedIn", icon: SocialKind::Linkedin }
                        SocialIcon { href: "https://twitter.com/HeartlandXL", label: "Follow Heartland on X", icon: SocialKind::Twitter }
                        SocialIcon { href: "https://www.instagram.com/heartlandmaterials", label: "Follow Heartland on Instagram", icon: SocialKind::Instagram }
                        SocialIcon { href: "https://www.youtube.com/channel/UCw3n3hnQX8PqgG-QIDb4BjA", label: "Subscribe to Heartland on YouTube", icon: SocialKind::Youtube }
                    }
                }
            }

            div {
                class: "border-t border-[color:var(--color-border)] py-4 text-center text-xs text-[color:var(--color-fg-muted)]",
                "© {year} Heartland Industries"
            }
        }
    }
}

#[component]
fn FooterColumn(title: String, items: Vec<(&'static str, Route)>) -> Element {
    rsx! {
        div {
            h3 { class: "font-display font-semibold mb-3", "{title}" }
            ul { class: "space-y-2 text-sm",
                for (label, route) in items {
                    li {
                        Link {
                            to: route,
                            class: "text-[color:var(--color-fg-muted)] hover:text-[color:var(--color-accent)]",
                            "{label}"
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
enum SocialKind {
    Facebook,
    Linkedin,
    Twitter,
    Instagram,
    Youtube,
}

#[component]
fn SocialIcon(href: &'static str, label: &'static str, icon: SocialKind) -> Element {
    rsx! {
        a {
            href: href,
            target: "_blank",
            rel: "noopener noreferrer",
            class: "inline-flex items-center justify-center w-10 h-10 rounded-md border border-[color:var(--color-border)] hover:border-[color:var(--color-accent)]",
            aria_label: label,
            match icon {
                SocialKind::Facebook => rsx! { svg { width: "18", height: "18", view_box: "0 0 24 24", fill: "currentColor", path { d: "M22 12a10 10 0 1 0-11.6 9.9v-7H8v-2.9h2.4V9.4c0-2.4 1.4-3.7 3.6-3.7 1 0 2.1.2 2.1.2v2.3h-1.2c-1.2 0-1.6.7-1.6 1.5v1.8h2.7l-.4 2.9h-2.3V22A10 10 0 0 0 22 12z" } } },
                SocialKind::Linkedin => rsx! { svg { width: "18", height: "18", view_box: "0 0 24 24", fill: "currentColor", path { d: "M19 0H5a5 5 0 0 0-5 5v14a5 5 0 0 0 5 5h14a5 5 0 0 0 5-5V5a5 5 0 0 0-5-5zM8 19H5V8h3v11zM6.5 6.7A1.7 1.7 0 1 1 8.2 5a1.7 1.7 0 0 1-1.7 1.7zM20 19h-3v-5.6c0-1.4-.5-2.3-1.7-2.3-.9 0-1.5.6-1.7 1.2-.1.2-.1.5-.1.8V19h-3V8h3v1.3a3 3 0 0 1 2.7-1.5c2 0 3.5 1.3 3.5 4V19z" } } },
                SocialKind::Twitter => rsx! { svg { width: "18", height: "18", view_box: "0 0 24 24", fill: "currentColor", path { d: "M18.244 2H21l-6.534 7.464L22 22h-6.41l-5.018-6.564L4.8 22H2.04l6.987-7.99L2 2h6.566l4.534 5.998L18.244 2zm-1.123 18h1.522L7.029 4H5.396l11.725 16z" } } },
                SocialKind::Instagram => rsx! { svg { width: "18", height: "18", view_box: "0 0 24 24", fill: "currentColor", path { d: "M12 2.2c3.2 0 3.6 0 4.8.1 1.2.1 1.8.2 2.2.4.6.2 1 .5 1.4.9.4.4.7.8.9 1.4.2.4.3 1 .4 2.2.1 1.2.1 1.6.1 4.8s0 3.6-.1 4.8c-.1 1.2-.2 1.8-.4 2.2-.2.6-.5 1-.9 1.4-.4.4-.8.7-1.4.9-.4.2-1 .3-2.2.4-1.2.1-1.6.1-4.8.1s-3.6 0-4.8-.1c-1.2-.1-1.8-.2-2.2-.4-.6-.2-1-.5-1.4-.9-.4-.4-.7-.8-.9-1.4-.2-.4-.3-1-.4-2.2-.1-1.2-.1-1.6-.1-4.8s0-3.6.1-4.8c.1-1.2.2-1.8.4-2.2.2-.6.5-1 .9-1.4.4-.4.8-.7 1.4-.9.4-.2 1-.3 2.2-.4 1.2-.1 1.6-.1 4.8-.1zm0 2c-3.1 0-3.5 0-4.7.1-1 0-1.6.2-1.9.3-.5.2-.8.4-1.2.8-.4.4-.6.7-.8 1.2-.1.3-.3.9-.3 1.9 0 1.2-.1 1.6-.1 4.7s0 3.5.1 4.7c0 1 .2 1.6.3 1.9.2.5.4.8.8 1.2.4.4.7.6 1.2.8.3.1.9.3 1.9.3 1.2.1 1.6.1 4.7.1s3.5 0 4.7-.1c1 0 1.6-.2 1.9-.3.5-.2.8-.4 1.2-.8.4-.4.6-.7.8-1.2.1-.3.3-.9.3-1.9.1-1.2.1-1.6.1-4.7s0-3.5-.1-4.7c0-1-.2-1.6-.3-1.9-.2-.5-.4-.8-.8-1.2-.4-.4-.7-.6-1.2-.8-.3-.1-.9-.3-1.9-.3-1.2-.1-1.6-.1-4.7-.1zm0 3.5a4.3 4.3 0 1 1 0 8.6 4.3 4.3 0 0 1 0-8.6zm0 7.1a2.8 2.8 0 1 0 0-5.6 2.8 2.8 0 0 0 0 5.6zm5.5-7.3a1 1 0 1 1-2 0 1 1 0 0 1 2 0z" } } },
                SocialKind::Youtube => rsx! { svg { width: "18", height: "18", view_box: "0 0 24 24", fill: "currentColor", path { d: "M21.6 7.2s-.2-1.4-.8-2c-.8-.8-1.6-.8-2-.9C16 4 12 4 12 4s-4 0-6.8.3c-.4 0-1.3.1-2 .9-.6.6-.8 2-.8 2S2 8.9 2 10.5v1.4c0 1.6.2 3.3.2 3.3s.2 1.4.8 2c.8.8 1.8.8 2.3.9 1.7.1 6.7.2 6.7.2s4 0 6.8-.3c.4 0 1.3-.1 2-.9.6-.6.8-2 .8-2s.2-1.7.2-3.3v-1.4c0-1.6-.2-3.2-.2-3.2zM10 14V8l5.2 3-5.2 3z" } } },
            }
        }
    }
}
