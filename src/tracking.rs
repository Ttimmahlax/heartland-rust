//! Site-wide analytics + email loader injection points.
//!
//! Heartland ships GA4 only. Klaviyo is dropped (the live site uses HubSpot
//! forms which are not migrated). REF_CAPTURE_JS persists inbound UTM / ref
//! attribution to localStorage for any future form provider to pick up.

use dioxus::prelude::*;

/// Google Analytics 4 measurement ID. Reused from the existing heartland.io
/// property — same ID is wired up on the WordPress source so analytics
/// continuity is preserved through the migration.
pub const GA4_MEASUREMENT_ID: &str = "G-CFVBK0N6L6";

/// Emit into `<head>` — GA4 (gtag.js) loader + init.
#[component]
pub fn TrackingHead() -> Element {
    let gtag_src =
        format!("https://www.googletagmanager.com/gtag/js?id={GA4_MEASUREMENT_ID}");
    let init = format!(
        "window.dataLayer=window.dataLayer||[];function gtag(){{dataLayer.push(arguments);}}gtag('js',new Date());gtag('config','{GA4_MEASUREMENT_ID}');"
    );
    rsx! {
        document::Script {
            src: "{gtag_src}",
            r#async: true,
        }
        document::Script { "{init}" }
    }
}

/// Emit at the end of `<body>`. Currently empty — placeholder for any future
/// non-blocking third-party loader (chat, consent, etc).
#[component]
pub fn TrackingFooter() -> Element {
    rsx! {}
}

/// Inline ref/utm capture — runs once on first interactive load and persists
/// the inbound attribution to localStorage so any later form submission can
/// pick it up. Non-blocking; safe to inject in body-end.
pub const REF_CAPTURE_JS: &str = r#"
(function(){
  try {
    var url = new URL(window.location.href);
    var keys = ["utm_source","utm_medium","utm_campaign","utm_term","utm_content","ref","hsLang"];
    var capture = {};
    var changed = false;
    keys.forEach(function(k){
      var v = url.searchParams.get(k);
      if (v) { capture[k] = v; changed = true; }
    });
    if (changed) {
      var prior = JSON.parse(localStorage.getItem("heartland.attribution") || "{}");
      Object.assign(prior, capture, { capturedAt: new Date().toISOString() });
      localStorage.setItem("heartland.attribution", JSON.stringify(prior));
    }
    if (url.searchParams.has("hsLang")) {
      url.searchParams.delete("hsLang");
      window.history.replaceState({}, "", url.toString());
    }
  } catch(e) {}
})();
"#;
