//! Per-page popups & modals.
//!
//! Adding a new popup:
//!   1. Define a wrapper component that calls `PopupShell { id, children }`.
//!      The `id` must be unique across the whole app — it keys both the
//!      DOM ids and the localStorage cooldown.
//!   2. Render the wrapper on whichever page(s) should show it. Multiple
//!      pages including the same wrapper share one cooldown.
//!
//! What `PopupShell` gives you:
//!   - Hidden overlay + content card with a corner `×` close button
//!   - Auto-open after `delay_seconds` (default 10s)
//!   - Suppress for `cooldown_days` after each show (default 14d)
//!   - Close on: button click, Escape key, or click outside the card
//!   - Sets `heartland_popup_<id>_last_shown` in localStorage on open
//!   - Cross-popup mutex (`__heartlandPopupShownThisLoad`) — only one popup
//!     can open per page load; first to fire wins
//!
//! All client behavior is a single inline `<script>` per popup — no global
//! popup runtime, no Dioxus signals, so SSR output stays clean.
//!
//! Mirrors the architecture of mata-website-rust's `src/popups.rs`, stripped
//! of MATA-specific gating (extension probe, Brave check) that doesn't apply
//! to heartland.io.

use dioxus::prelude::*;

// ─── Newsletter (Beehiiv — Heartland Textile Fiber Index) ─────────────────

const BEEHIIV_TEXTILE_INDEX_SRC: &str =
    "https://embeds.beehiiv.com/b5a0516a-58e8-453a-99ad-3b51922d8e70";

/// Heartland Textile Fiber Index newsletter signup. Drop into any page that
/// should prompt visitors to subscribe — currently the Imperium fibers
/// product family and textile-related sustainability articles. The 14-day
/// cooldown is shared site-wide via the shell id.
#[component]
pub fn TextileFiberIndexPopup() -> Element {
    rsx! {
        PopupShell {
            id: "textile-fiber-index",
            delay_seconds: 5,
            cooldown_days: 14,
            aria_label: "Heartland Textile Fiber Index newsletter signup",
            // Beehiiv embeds ship a single background color, so our
            // light/dark auto-flipping card would look broken in one theme.
            // Strip the chrome and let Beehiiv carry itself.
            bare: true,

            iframe {
                src: BEEHIIV_TEXTILE_INDEX_SRC,
                "data-test-id": "beehiiv-embed",
                width: "100%",
                height: "320",
                "frameborder": "0",
                "scrolling": "no",
                // Sandbox: a Beehiiv compromise can't pivot into our top
                // window or our origin's storage. Allowed: run their JS,
                // talk to their origin, submit the form, open confirmations.
                // Disallowed: top navigation, top-context modals, pointer
                // lock, storage access.
                "sandbox": "allow-scripts allow-same-origin allow-forms allow-popups",
                "referrerpolicy": "no-referrer",
                style: "border-radius: 8px; border: 0; margin: 0; background: transparent;",
            }
        }
    }
}

// ─── Textile keyword detection (used by article.rs auto-gating) ───────────

/// Keywords that flag an article as textile-relevant. Matched
/// case-insensitively against the article title + body in
/// [crate::pages::article].
pub const TEXTILE_KEYWORDS: &[&str] =
    &["linen", "cotton", "yarn", "fabric", "textile", "silk", "wool"];

/// Return true if `text` contains any [`TEXTILE_KEYWORDS`] as a whole word
/// (case-insensitive).
pub fn has_textile_keyword(text: &str) -> bool {
    let lower = text.to_lowercase();
    let bytes = lower.as_bytes();
    TEXTILE_KEYWORDS.iter().any(|kw| {
        let mut start = 0;
        while let Some(pos) = lower[start..].find(kw) {
            let abs = start + pos;
            let before_ok = abs == 0 || !is_word_byte(bytes[abs - 1]);
            let after = abs + kw.len();
            let after_ok = after >= bytes.len() || !is_word_byte(bytes[after]);
            if before_ok && after_ok {
                return true;
            }
            start = abs + kw.len();
        }
        false
    })
}

fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

// ─── Generic shell ────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
pub struct PopupShellProps {
    /// Unique slug — used as a DOM id suffix and the localStorage cooldown
    /// key. Lowercase letters and `-` only. Must be unique across all
    /// popups in the app.
    pub id: &'static str,
    /// Seconds to wait after page load before auto-opening.
    #[props(default = 10)]
    pub delay_seconds: u32,
    /// Days to suppress the popup after each show.
    #[props(default = 14)]
    pub cooldown_days: u32,
    /// Accessible label for the dialog (announced by screen readers).
    #[props(default = "Dialog")]
    pub aria_label: &'static str,
    /// Strip the card chrome (background, border, padding, shadow, radius)
    /// so the embed's own background shows through unaltered. Use for
    /// embeds that ship their own visual frame and don't tolerate our
    /// auto-flipping light/dark surface (Beehiiv is one — its embed only
    /// supports a single background color, so wrapping it in our card
    /// looks broken in whichever theme it doesn't match).
    #[props(default = false)]
    pub bare: bool,
    /// Suppress the popup on phones / tablets. Detection in the inline init
    /// combines a UA check (Android / iPhone / iPad / iPod / generic
    /// "Mobile" token) with a viewport-width threshold (< 900px), so
    /// resized desktop windows and tablets both bail out.
    #[props(default = false)]
    pub desktop_only: bool,
    /// Body of the popup card — the iframe / form / content.
    pub children: Element,
}

/// Reusable popup overlay. Renders the markup + a per-instance init script
/// that handles cooldown, delay, ESC, background-click, and close button.
#[component]
pub fn PopupShell(props: PopupShellProps) -> Element {
    let overlay_id = format!("popup-overlay-{}", props.id);
    let content_id = format!("popup-content-{}", props.id);
    let close_id = format!("popup-close-{}", props.id);
    let storage_key = format!("heartland_popup_{}_last_shown", props.id);
    let cooldown_ms: u64 = (props.cooldown_days as u64) * 24 * 60 * 60 * 1000;
    let delay_ms: u64 = (props.delay_seconds as u64) * 1000;
    let desktop_only_js: &str = if props.desktop_only { "1" } else { "0" };

    // Inline init — self-contained IIFE per popup. Idempotent: a second
    // run (e.g. dev hot-reload) no-ops because `data-popup-bound` flips
    // to "1" the first time through.
    //
    // Cross-popup mutex: `window.__heartlandPopupShownThisLoad` flips to
    // true when ANY popup opens. Subsequent popups whose timers fire after
    // that point bail out — only one popup can appear per page load. First
    // to fire wins; ordering follows `delay_seconds`. The cooldown
    // localStorage key is only written when the popup actually opens, so a
    // popup that bails on the mutex stays eligible for the next visit.
    let init_js = format!(
        r#"(function() {{
  var STORAGE_KEY = "{storage_key}";
  var COOLDOWN_MS = {cooldown_ms};
  var DELAY_MS = {delay_ms};
  var OVERLAY_ID = "{overlay_id}";
  var CLOSE_ID = "{close_id}";
  var DESKTOP_ONLY = {desktop_only_js};

  function safeGet() {{ try {{ return localStorage.getItem(STORAGE_KEY); }} catch (e) {{ return null; }} }}
  function safeSet(v) {{ try {{ localStorage.setItem(STORAGE_KEY, v); }} catch (e) {{}} }}

  // Desktop-only gate. UA covers Android / iOS (incl. iPad pre-iPadOS spoof);
  // viewport-width covers iPad-spoofed-as-desktop and resized windows.
  if (DESKTOP_ONLY) {{
    var ua = (navigator && navigator.userAgent) || "";
    var isMobileUa = /Android|iPhone|iPad|iPod|Mobile/i.test(ua);
    var isNarrow = (window.innerWidth || 0) < 900;
    if (isMobileUa || isNarrow) return;
  }}

  var last = parseInt(safeGet() || "0", 10);
  if (last && (Date.now() - last) < COOLDOWN_MS) return;

  function init() {{
    var overlay = document.getElementById(OVERLAY_ID);
    var btn = document.getElementById(CLOSE_ID);
    if (!overlay || !btn) return;
    if (overlay.dataset.popupBound === "1") return;
    overlay.dataset.popupBound = "1";

    function close() {{
      overlay.classList.remove("is-open");
      overlay.setAttribute("aria-hidden", "true");
      // Re-arm the inline fallback so a closed popup stays hidden even
      // if the stylesheet is missing the .popup-overlay rule.
      overlay.style.display = "none";
      document.removeEventListener("keydown", onKey);
    }}

    function onKey(e) {{
      if (e.key === "Escape") close();
    }}

    btn.addEventListener("click", close);
    overlay.addEventListener("click", function(e) {{
      if (e.target === overlay) close();
    }});

    setTimeout(function() {{
      // Cross-popup mutex — see comment on init_js above.
      if (window.__heartlandPopupShownThisLoad) return;
      window.__heartlandPopupShownThisLoad = true;
      overlay.classList.add("is-open");
      overlay.setAttribute("aria-hidden", "false");
      // Clear the inline `display: none` fallback so the .is-open class's
      // `display: flex` can take over.
      overlay.style.display = "";
      safeSet(String(Date.now()));
      document.addEventListener("keydown", onKey);
    }}, DELAY_MS);
  }}

  if (document.readyState === "loading") {{
    document.addEventListener("DOMContentLoaded", init);
  }} else {{
    init();
  }}
}})();"#
    );

    let content_class = if props.bare {
        "popup-content popup-content--bare"
    } else {
        "popup-content"
    };

    rsx! {
        div {
            id: "{overlay_id}",
            class: "popup-overlay",
            role: "dialog",
            aria_modal: "true",
            aria_label: "{props.aria_label}",
            aria_hidden: "true",
            // Inline structural fallback. If the stylesheet is stale or
            // failed to load, the popup must STILL stay hidden and pinned
            // to the viewport instead of rendering as a giant inline iframe
            // bar in the document flow. The init JS clears `display` on open.
            style: "display: none; position: fixed; inset: 0; z-index: 100;",
            div {
                id: "{content_id}",
                class: "{content_class}",
                button {
                    id: "{close_id}",
                    class: "popup-close",
                    r#type: "button",
                    aria_label: "Close",
                    "×"
                }
                {props.children}
            }
        }
        document::Script { "{init_js}" }
    }
}
