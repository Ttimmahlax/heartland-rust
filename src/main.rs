#![allow(non_snake_case)]

use dioxus::prelude::*;

mod components;
mod content;
mod pages;
mod seo;
mod tracking;

use components::layout::LayoutShell;
use pages::{
    about::About,
    article::Article,
    automotive::Automotive,
    contact::Contact,
    ebooks::Ebooks,
    engineering_earth::EngineeringEarth,
    faq::Faq,
    farmers::Farmers,
    government::Government,
    green_packaging::GreenPackaging,
    imperium_animal_feed::ImperiumAnimalFeed,
    imperium_fibers::ImperiumFibers,
    imperium_filled_resin::ImperiumFilledResin,
    imperium_filler::ImperiumFiller,
    imperium_masterbatch::ImperiumMasterbatch,
    landing::Landing,
    lca::Lca,
    marine::Marine,
    news::News,
    not_found::NotFound,
    research::Research,
    sustainable_asphalt::SustainableAsphalt,
    sustainable_building::SustainableBuilding,
    sustainable_concrete::SustainableConcrete,
    sustainable_packaging::SustainablePackaging,
    sustainable_paper::SustainablePaper,
    sustainable_plastic::SustainablePlastic,
    sustainable_rubber::SustainableRubber,
    team::Team,
    whitepapers::Whitepapers,
    why_imperium::WhyImperium,
};

pub const SITE_BASE: &str = "https://heartland.io";
pub const SITE_NAME: &str = "Heartland Industries";

#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[layout(LayoutShell)]
        #[route("/")]
        Landing {},

        // Products
        #[route("/why-imperium")]
        WhyImperium {},
        #[route("/imperium-masterbatch")]
        ImperiumMasterbatch {},
        #[route("/imperium-filled-resin")]
        ImperiumFilledResin {},
        #[route("/imperium-filler")]
        ImperiumFiller {},
        #[route("/imperium-fibers")]
        ImperiumFibers {},
        #[route("/imperium-animal-feed")]
        ImperiumAnimalFeed {},

        // Industries / Materials
        #[route("/sustainable-plastic-compounding")]
        SustainablePlastic {},
        #[route("/automotive")]
        Automotive {},
        #[route("/sustainable-packaging")]
        SustainablePackaging {},
        #[route("/sustainable-building-materials")]
        SustainableBuilding {},
        #[route("/sustainable-rubber-additives")]
        SustainableRubber {},
        #[route("/sustainable-concrete-additives")]
        SustainableConcrete {},
        #[route("/sustainable-asphalt-additives")]
        SustainableAsphalt {},
        #[route("/sustainable-paper-additives")]
        SustainablePaper {},
        #[route("/marine")]
        Marine {},
        #[route("/government")]
        Government {},

        // Resources
        #[route("/engineering-earth")]
        EngineeringEarth {},
        #[route("/e-books")]
        Ebooks {},
        #[route("/whitepapers")]
        Whitepapers {},
        #[route("/natural-fiber-research")]
        Research {},
        #[route("/frequently-asked-questions")]
        Faq {},

        // About + LCA + initiatives
        #[route("/heartland-team")]
        Team {},
        #[route("/heartland-farmers")]
        Farmers {},
        #[route("/green-packaging-initiative")]
        GreenPackaging {},
        #[route("/lca")]
        Lca {},
        #[route("/about")]
        About {},
        #[route("/contact")]
        Contact {},

        // News
        #[route("/sustainability-news")]
        News {},
        #[route("/sustainability-news/:slug")]
        Article { slug: String },

        // Branded 404
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/tailwind.css") }
        document::Link { rel: "icon", r#type: "image/svg+xml", href: "/assets/brand/favicon.svg" }
        document::Link { rel: "icon", sizes: "32x32", href: "/assets/brand/favicon-32.png" }
        document::Link { rel: "icon", sizes: "192x192", href: "/assets/brand/favicon-192.png" }
        document::Link { rel: "apple-touch-icon", sizes: "180x180", href: "/assets/brand/apple-touch-icon-180.png" }
        document::Meta { name: "theme-color", content: "#ad2929" }
        document::Meta { charset: "utf-8" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }

        Router::<Route> {}
    }
}

/// Static route enumerator. Mirrors STATIC_ROUTES in scripts/generate-sitemap.sh
/// and ROUTES in scripts/prerender.sh — keep all three in sync (see
/// docs/replicate.md §"The trio of route lists must stay synced").
#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    let mut routes: Vec<String> = vec![
        "/".into(),
        "/why-imperium".into(),
        "/imperium-masterbatch".into(),
        "/imperium-filled-resin".into(),
        "/imperium-filler".into(),
        "/imperium-fibers".into(),
        "/imperium-animal-feed".into(),
        "/sustainable-plastic-compounding".into(),
        "/automotive".into(),
        "/sustainable-packaging".into(),
        "/sustainable-building-materials".into(),
        "/sustainable-rubber-additives".into(),
        "/sustainable-concrete-additives".into(),
        "/sustainable-asphalt-additives".into(),
        "/sustainable-paper-additives".into(),
        "/marine".into(),
        "/government".into(),
        "/engineering-earth".into(),
        "/e-books".into(),
        "/whitepapers".into(),
        "/natural-fiber-research".into(),
        "/frequently-asked-questions".into(),
        "/heartland-team".into(),
        "/heartland-farmers".into(),
        "/green-packaging-initiative".into(),
        "/lca".into(),
        "/about".into(),
        "/contact".into(),
        "/sustainability-news".into(),
    ];
    for slug in content::all_slugs() {
        routes.push(format!("/sustainability-news/{slug}"));
    }
    Ok(routes)
}
