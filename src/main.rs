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
    carbon_neutral_packaging::CarbonNeutralPackaging,
    case_studies::CaseStudies,
    category_archive::{CategoryArchive, TagArchive},
    contact::Contact,
    ebooks::Ebooks,
    engineering_earth::EngineeringEarth,
    faq::Faq,
    farmers::Farmers,
    government::Government,
    green_packaging::GreenPackaging,
    heartland_ebooks::HeartlandEbooks,
    hemp_fiber_and_hurd::HempFiberAndHurd,
    imperium_animal_feed::ImperiumAnimalFeed,
    imperium_cattle_feed::ImperiumCattleFeed,
    imperium_chicken_feed::ImperiumChickenFeed,
    imperium_fabric::ImperiumFabric,
    imperium_fibers::ImperiumFibers,
    imperium_filled_resin::ImperiumFilledResin,
    imperium_filler::ImperiumFiller,
    imperium_graphene::ImperiumGraphene,
    imperium_masterbatch::ImperiumMasterbatch,
    imperium_pork_feed::ImperiumPorkFeed,
    imperium_spin_ready_white_fiber::ImperiumSpinReadyWhiteFiber,
    imperium_yarn::ImperiumYarn,
    landing::Landing,
    lca::Lca,
    marine::Marine,
    news::News,
    not_found::NotFound,
    plastic_additives::PlasticAdditives,
    portfolio_item::{all_portfolio_slugs, PortfolioItem},
    portfolios::Portfolios,
    research::Research,
    sustainable_asphalt::SustainableAsphalt,
    sustainable_building::SustainableBuilding,
    sustainable_concrete::SustainableConcrete,
    sustainable_foam::SustainableFoam,
    sustainable_packaging::SustainablePackaging,
    sustainable_paper::SustainablePaper,
    sustainable_plastic::SustainablePlastic,
    sustainable_rubber::SustainableRubber,
    team::Team,
    usda::Usda,
    whitepapers::Whitepapers,
    why_imperium::WhyImperium,
    wood_products::WoodProducts,
};

pub const SITE_BASE: &str = "https://heartland.io";
pub const SITE_NAME: &str = "Heartland Industries";

#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[layout(LayoutShell)]
        #[route("/")]
        Landing {},

        // Why
        #[route("/why-imperium")]
        WhyImperium {},

        // Products
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
        #[route("/imperium-pork-feed")]
        ImperiumPorkFeed {},
        #[route("/imperium-cattle-feed")]
        ImperiumCattleFeed {},
        #[route("/imperium-chicken-feed")]
        ImperiumChickenFeed {},
        #[route("/imperium-spin-ready-white-fiber")]
        ImperiumSpinReadyWhiteFiber {},
        #[route("/imperium-yarn")]
        ImperiumYarn {},
        #[route("/imperium-fabric")]
        ImperiumFabric {},
        #[route("/imperium-graphene")]
        ImperiumGraphene {},

        // Industries / Materials
        #[route("/sustainable-plastic-compounding")]
        SustainablePlastic {},
        #[route("/automotive")]
        Automotive {},
        #[route("/sustainable-packaging")]
        SustainablePackaging {},
        #[route("/carbon-neutral-packaging-with-imperium-inside")]
        CarbonNeutralPackaging {},
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
        #[route("/sustainable-foam")]
        SustainableFoam {},
        #[route("/marine")]
        Marine {},
        #[route("/government")]
        Government {},

        // Legacy product / category landings (re-added for SEO continuity)
        #[route("/hemp-fiber-and-hurd")]
        HempFiberAndHurd {},
        #[route("/wood-products")]
        WoodProducts {},
        #[route("/plastic-additives")]
        PlasticAdditives {},
        #[route("/case-studies")]
        CaseStudies {},
        #[route("/usda")]
        Usda {},

        // Resources
        #[route("/engineering-earth")]
        EngineeringEarth {},
        #[route("/e-books")]
        Ebooks {},
        #[route("/heartland-e-books")]
        HeartlandEbooks {},
        #[route("/whitepapers")]
        Whitepapers {},
        #[route("/natural-fiber-research")]
        Research {},
        #[route("/frequently-asked-questions")]
        Faq {},
        #[route("/portfolios")]
        Portfolios {},

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

        // News + taxonomy archives + portfolio items.
        // SPECIFIC paths MUST come before the generic /:slug fallback,
        // or every archive URL would route to Article and miss.
        #[route("/sustainability-news")]
        News {},
        #[route("/sustainability-news/category/:slug")]
        CategoryArchive { slug: String },
        #[route("/sustainability-news/tag/:slug")]
        TagArchive { slug: String },
        #[route("/sustainability-news/portfolio/:slug")]
        PortfolioItem { slug: String },
        #[route("/sustainability-news/:slug")]
        Article { slug: String },

        // Branded 404 — catch-all, must be last
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
        "/imperium-pork-feed".into(),
        "/imperium-cattle-feed".into(),
        "/imperium-chicken-feed".into(),
        "/imperium-spin-ready-white-fiber".into(),
        "/imperium-yarn".into(),
        "/imperium-fabric".into(),
        "/imperium-graphene".into(),
        "/sustainable-plastic-compounding".into(),
        "/automotive".into(),
        "/sustainable-packaging".into(),
        "/carbon-neutral-packaging-with-imperium-inside".into(),
        "/sustainable-building-materials".into(),
        "/sustainable-rubber-additives".into(),
        "/sustainable-concrete-additives".into(),
        "/sustainable-asphalt-additives".into(),
        "/sustainable-paper-additives".into(),
        "/sustainable-foam".into(),
        "/marine".into(),
        "/government".into(),
        "/hemp-fiber-and-hurd".into(),
        "/wood-products".into(),
        "/plastic-additives".into(),
        "/case-studies".into(),
        "/usda".into(),
        "/engineering-earth".into(),
        "/e-books".into(),
        "/heartland-e-books".into(),
        "/whitepapers".into(),
        "/natural-fiber-research".into(),
        "/frequently-asked-questions".into(),
        "/portfolios".into(),
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
    for slug in content::all_categories() {
        routes.push(format!("/sustainability-news/category/{slug}"));
    }
    for slug in content::all_tags() {
        routes.push(format!("/sustainability-news/tag/{slug}"));
    }
    for slug in all_portfolio_slugs() {
        routes.push(format!("/sustainability-news/portfolio/{slug}"));
    }
    Ok(routes)
}
