//! Contact section — world map (Detroit + NY + Paris office pins) on the
//! left, a HubSpot form embed on the right. Each page passes its own
//! `form_id` so HubSpot can route the lead to the correct workflow.
//!
//! HubSpot embed pattern: insert a target `<div>` with a unique id, then a
//! `<script>` that calls `hbspt.forms.create({ portalId, formId, target })`.
//! The form loader script (`//js.hsforms.net/forms/v2.js`) is allow-listed in
//! `customHttp.yml` under `script-src`. The injected iframe is permitted via
//! `frame-src https://*.hsforms.com`.

use dioxus::prelude::*;

/// Public HubSpot portal ID — same across every page on heartland.io.
pub const HUBSPOT_PORTAL_ID: &str = "8084764";

/// Fallback / default form when a page-specific form ID isn't known.
/// This is the form that the homepage uses today.
pub const DEFAULT_FORM_ID: &str = "e226d00b-79ca-4ade-9b0e-79d5b597e053";

/// Per-page form-ID lookup, mined from each page's WP `_elementor_data` via
/// SSH. Add new entries here if HubSpot routing for a page changes. The
/// landing route ("/") uses [`DEFAULT_FORM_ID`].
pub const PAGE_FORM_IDS: &[(&str, &str)] = &[
    ("automotive",                                    "b181238e-f26b-4a9e-8ea7-9c4dfc8e4c1d"),
    ("carbon-neutral-packaging-with-imperium-inside", "335b5581-c37a-456b-9cdd-16d00785f1b4"),
    ("case-studies",                                  "5f0f2851-cc01-4731-8d7e-c6b2fc896d80"),
    ("e-books",                                       "0a2df500-6ba0-4d31-82ab-a1447d3d584d"),
    ("engineering-earth",                             "6dc80989-a8dc-441e-abba-282ba802d091"),
    ("frequently-asked-questions",                    "92d6d747-b45d-4996-9fc2-9017397b40ff"),
    ("government",                                    "3106649c-1f94-4bf9-84b1-60ebbc59cfc0"),
    ("green-packaging-initiative",                    "a6a0c655-d529-412f-9099-1bfb6703092c"),
    ("heartland-e-books",                             "0a2df500-6ba0-4d31-82ab-a1447d3d584d"),
    ("heartland-farmers",                             "a6ed5577-2510-4a5b-82a8-ae03ac4ade07"),
    ("heartland-team",                                "5b258087-81eb-43d6-b54a-4f3cf219a954"),
    ("hemp-fiber-and-hurd",                           "e510d5b3-a84a-4568-9e28-daa57163999e"),
    ("imperium-animal-feed",                          "e510d5b3-a84a-4568-9e28-daa57163999e"),
    ("imperium-cattle-feed",                          "e510d5b3-a84a-4568-9e28-daa57163999e"),
    ("imperium-chicken-feed",                         "e510d5b3-a84a-4568-9e28-daa57163999e"),
    ("imperium-fabric",                               "e510d5b3-a84a-4568-9e28-daa57163999e"),
    ("imperium-fibers",                               "e510d5b3-a84a-4568-9e28-daa57163999e"),
    ("imperium-filled-resin",                         "9d7b8aae-e479-4699-9f61-a3e31e88c1cf"),
    ("imperium-filler",                               "e510d5b3-a84a-4568-9e28-daa57163999e"),
    ("imperium-graphene",                             "82d717a4-89e3-406f-9a49-6a477ef0183c"),
    ("imperium-masterbatch",                          "e0a59bfe-4bab-4fa2-a345-777a3fe0b2f7"),
    ("imperium-pork-feed",                            "e510d5b3-a84a-4568-9e28-daa57163999e"),
    ("imperium-spin-ready-white-fiber",               "e510d5b3-a84a-4568-9e28-daa57163999e"),
    ("imperium-yarn",                                 "e510d5b3-a84a-4568-9e28-daa57163999e"),
    ("lca",                                           "3d8fa253-c10e-4aa1-9674-dd70a698ed76"),
    ("marine",                                        "7ab909fd-d889-4ac2-a029-502e2b8fe7f2"),
    ("natural-fiber-research",                        "dc9185c3-c77d-4cf2-a104-c7e2e4bf97a2"),
    ("plastic-additives",                             "e510d5b3-a84a-4568-9e28-daa57163999e"),
    ("portfolios",                                    "e226d00b-79ca-4ade-9b0e-79d5b597e053"),
    ("sustainability-news",                           "aa7a259c-a0a5-4579-aefc-baf3ddf185f5"),
    ("sustainable-asphalt-additives",                 "c6f1ec0d-0d86-477a-ba2f-244e33619a93"),
    ("sustainable-building-materials",                "615fa37a-c3b7-4df3-816c-b8450c7e2893"),
    ("sustainable-concrete-additives",                "77158bd9-9f24-43fe-a70e-e7a795408bc4"),
    ("sustainable-foam",                              "fedea4ba-d319-4048-8bde-d493eea3e121"),
    ("sustainable-packaging",                         "48ccc4dd-b96c-446b-95d3-6344d1aec779"),
    ("sustainable-paper-additives",                   "8795deb6-c204-41ea-a095-008e42dd1fdf"),
    ("sustainable-plastic-compounding",               "3d8fa253-c10e-4aa1-9674-dd70a698ed76"),
    ("sustainable-rubber-additives",                  "be73dd1a-8535-4304-82fd-f73ef12b0c14"),
    ("usda",                                          "84c77057-59e7-48f4-8282-870458bc2dbe"),
    ("whitepapers",                                   "0a2df500-6ba0-4d31-82ab-a1447d3d584d"),
    ("why-imperium",                                  "1c76c10f-2686-437d-9e82-e89102db72bb"),
    ("wood-products",                                 "aa409daf-c5f3-4ba8-8922-9ff892a9ec6a"),
];

/// Look up a page-specific form ID, falling back to [`DEFAULT_FORM_ID`].
pub fn form_id_for(page_slug: &str) -> &'static str {
    PAGE_FORM_IDS
        .iter()
        .find(|(slug, _)| *slug == page_slug)
        .map(|(_, id)| *id)
        .unwrap_or(DEFAULT_FORM_ID)
}

#[derive(Props, Clone, PartialEq)]
pub struct ContactBlockProps {
    /// Page slug used to look up the HubSpot form ID. Pass an empty string
    /// (or omit) to use [`DEFAULT_FORM_ID`].
    #[props(default = String::new())]
    pub page_slug: String,
    #[props(default = String::from("Get In Touch"))]
    pub heading: String,
    #[props(default = String::from("Talk to our materials science team — replies within one business day."))]
    pub blurb: String,
}

#[component]
pub fn ContactBlock(props: ContactBlockProps) -> Element {
    let form_id = form_id_for(&props.page_slug);
    let target_id = format!("hbspt-form-{}", form_id);
    // Inline JS that injects the form once the loader script is ready.
    // Polls hbspt for up to 10s in case the script loads after this snippet.
    let init_js = format!(
        r##"
(function(){{
  var tries = 0, max = 50;
  function tryLoad() {{
    if (window.hbspt && window.hbspt.forms) {{
      try {{
        window.hbspt.forms.create({{
          region: "na1",
          portalId: "{portal}",
          formId: "{form_id}",
          target: "#{target_id}"
        }});
      }} catch(e) {{}}
      return;
    }}
    if (tries++ < max) setTimeout(tryLoad, 200);
  }}
  tryLoad();
}})();
"##,
        portal = HUBSPOT_PORTAL_ID,
        form_id = form_id,
        target_id = target_id,
    );

    rsx! {
        section {
            id: "contact",
            class: "container-content py-16 md:py-24",

            div { class: "text-center mb-12 max-w-3xl mx-auto",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "Contact"
                }
                h2 { class: "text-3xl md:text-4xl font-bold mb-4",
                    "{props.heading}"
                }
                p { class: "text-lg text-[color:var(--color-fg-muted)]",
                    "{props.blurb}"
                }
            }

            div { class: "grid lg:grid-cols-2 gap-10 lg:gap-16 items-stretch",
                // LEFT: world map with pins
                OfficesMap {}

                // RIGHT: HubSpot form
                div { class: "surface-glass rounded-xl p-6 md:p-8 h-full flex flex-col",
                    div {
                        id: "{target_id}",
                        class: "hbspt-form-container min-h-[300px] flex-1",
                        // Fallback while JS hasn't loaded yet
                        p { class: "text-sm text-[color:var(--color-fg-muted)] italic",
                            "Loading contact form…"
                        }
                    }
                    // Hard fallback: a direct mailto link if scripts are blocked
                    noscript {
                        p { class: "text-sm text-[color:var(--color-fg-muted)] mt-4",
                            "Form requires JavaScript. "
                            a { href: "mailto:Hello@heartland.io",
                                class: "text-[color:var(--color-accent)] underline",
                                "Email us directly →"
                            }
                        }
                    }
                }
            }

            // Load the HubSpot forms v2 loader once per page render.
            // Render order: the script is appended at the end of the section
            // so the target divs exist before the loader fires.
            document::Script {
                src: "https://js.hsforms.net/forms/v2.js",
                r#async: true,
                defer: true,
            }
            document::Script { "{init_js}" }
        }
    }
}

#[component]
fn OfficesMap() -> Element {
    // City positions are percentages of the world-map.svg viewBox (the
    // Wikimedia "World_map_-_low_resolution.svg" uses Equirectangular-ish
    // projection over a 950×620 viewBox). Adjust if the map source changes.
    rsx! {
        // Container stretches to match the form column height (parent uses
        // `items-stretch`). The map image fills the entire box with
        // `object-cover` so it visually balances the form on the right.
        div { class: "relative rounded-xl overflow-hidden bg-[color:var(--color-surface)] border border-[color:var(--color-border)] h-full min-h-[400px] lg:min-h-[500px]",
            img {
                src: "/assets/brand/world-map.svg",
                alt: "Heartland office locations: Detroit, New York, and Paris.",
                class: "absolute inset-0 w-full h-full object-cover opacity-90 dark:opacity-80 dark:invert",
                loading: "lazy",
            }
            // Pin overlay — positioned in percent space relative to the
            // container (which matches the image's coordinate system since
            // both use object-cover with the same aspect).
            MapPin { top_pct: "37%", left_pct: "23%", city: "Detroit" }
            MapPin { top_pct: "39%", left_pct: "27%", city: "New York" }
            MapPin { top_pct: "30%", left_pct: "49%", city: "Paris" }
        }
    }
}

#[component]
fn MapPin(top_pct: &'static str, left_pct: &'static str, city: &'static str) -> Element {
    let style = format!("top: {top_pct}; left: {left_pct}; transform: translate(-50%, -50%);");
    rsx! {
        div {
            class: "absolute z-10 pointer-events-none",
            style: "{style}",
            // Outer pulse ring (animated via Tailwind's animate-ping)
            span { class: "absolute inset-0 rounded-full bg-[color:var(--color-accent)] opacity-40 animate-ping w-4 h-4" }
            // Solid dot
            span { class: "relative block w-3 h-3 rounded-full bg-[color:var(--color-accent)] ring-2 ring-white shadow-md" }
            // Inline city label
            span { class: "absolute left-4 top-1/2 -translate-y-1/2 text-xs md:text-sm font-bold text-[color:var(--color-fg)] whitespace-nowrap drop-shadow",
                "{city}"
            }
        }
    }
}

