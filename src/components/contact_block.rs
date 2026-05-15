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
                div { class: "surface-glass rounded-xl p-6 md:p-8 h-full",
                    div {
                        id: "{target_id}",
                        class: "hbspt-form-container min-h-[300px]",
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
    // Map and pin overlay share the same `viewBox`, so pin positions are
    // in the source SVG's equirectangular pixel space — not CSS percentages
    // of the rendered box. Office coordinates were derived from a 10-country
    // linear fit (R² ≈ 0.998 for longitude, 0.994 for latitude); see
    // /tmp/probe_projection2.py in the git history.
    //
    //   x_pixel ≈ 2.708 · lon + 453.8
    //   y_pixel ≈ -3.377 · lat + 342.3
    rsx! {
        // Bubble fills the form column's height (`h-full` + parent grid's
        // `items-stretch`). The map and the pin overlay both fill the inner
        // box and both crop with the SAME anchor (xMidYMid = center) — the
        // `<img>` via `object-cover`, the overlay `<svg>` via
        // `preserveAspectRatio="xMidYMid slice"`. Same crop, same viewBox,
        // so the pins stay glued to their countries regardless of column
        // aspect ratio.
        div { class: "relative rounded-xl overflow-hidden bg-[color:var(--color-surface)] border border-[color:var(--color-border)] p-4 md:p-6 h-full",
            div { class: "relative w-full h-full min-h-[300px]",
                img {
                    src: "/assets/brand/world-map.svg",
                    alt: "Heartland office locations: Detroit, New York, and Paris.",
                    class: "block w-full h-full object-cover opacity-90 dark:opacity-80 dark:invert select-none",
                    loading: "lazy",
                    draggable: "false",
                }
                svg {
                    class: "absolute inset-0 w-full h-full pointer-events-none",
                    view_box: "0 0 950 620",
                    preserve_aspect_ratio: "xMidYMid slice",

                    // Detroit (label to the left so it doesn't collide with NYC's)
                    OfficeMarker {
                        cx: 229, cy: 199,
                        label: "Detroit",
                        label_x: 210, label_y: 207, label_anchor: "end",
                    }
                    // New York (label to the right)
                    OfficeMarker {
                        cx: 253, cy: 205,
                        label: "New York",
                        label_x: 272, label_y: 213, label_anchor: "start",
                    }
                    // Paris (label to the right)
                    OfficeMarker {
                        cx: 460, cy: 177,
                        label: "Paris",
                        label_x: 479, label_y: 185, label_anchor: "start",
                    }
                }
            }
        }
    }
}

#[component]
fn OfficeMarker(
    cx: i32, cy: i32,
    label: &'static str,
    label_x: i32, label_y: i32,
    label_anchor: &'static str,
) -> Element {
    rsx! {
        // Radii and font are sized in viewBox units (0..950 × 0..620). At a
        // typical 520px-wide column the SVG renders at ~0.55× scale, so the
        // numbers below produce ~7–8px dots and ~15px label text on desktop,
        // ~5–6px dots and ~10–11px labels on mobile.
        //
        // Pulse ring — SMIL animation (universally supported in evergreen
        // browsers; Chromium rolled back its 2016 deprecation).
        circle {
            cx: "{cx}", cy: "{cy}", r: "12",
            fill: "var(--color-accent)",
            opacity: "0.5",
            animate {
                attribute_name: "r",
                values: "12; 26; 26",
                dur: "2.1s",
                repeat_count: "indefinite",
            }
            animate {
                attribute_name: "opacity",
                values: "0.5; 0; 0",
                dur: "2.1s",
                repeat_count: "indefinite",
            }
        }
        // Solid dot
        circle {
            cx: "{cx}", cy: "{cy}", r: "14",
            fill: "var(--color-accent)",
            stroke: "white", stroke_width: "2.5",
        }
        // Label — paint-order puts the stroke behind the fill as a halo so
        // the label stays readable on both the light and the inverted-dark map.
        text {
            x: "{label_x}", y: "{label_y}",
            font_size: "28", font_weight: "700",
            text_anchor: "{label_anchor}",
            fill: "var(--color-fg)",
            stroke: "var(--color-bg)", stroke_width: "6",
            paint_order: "stroke",
            "{label}"
        }
    }
}

