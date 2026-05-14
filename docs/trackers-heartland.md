# Trackers — Heartland Migration (2026-05-14)

Phase 1c output: what's running on heartland.io WordPress, what we keep, what we drop.

## Detected on heartland.io (live scan, 2026-05-14)

| Tracker | Detection signal | Action |
| --- | --- | --- |
| Google Analytics 4 | `gtag/js?id=G-CFVBK0N6L6` in `<head>` | **KEEP** — same measurement ID reused in [src/tracking.rs](../src/tracking.rs). Continuity for the existing GA4 property. |
| HubSpot (`_hsq` queue, scriptloader path) | Inline `_hsq` calls + `hubfs/<id>/` asset paths | **DROP** — runs the HubSpot CMS, irrelevant once off WordPress. Removes a major LCP penalty. |
| Hotjar (`hjid:2032509`) | Inline `hotjar.com/c/hotjar-` loader | **DROP** — session replay is out of scope by default. Add back via [src/tracking.rs](../src/tracking.rs) `TrackingHead` + CSP entry if needed. |
| jQuery + Slick Carousel + Owl Carousel | WordPress plugin runtime | **DROP** — replaced by native Dioxus components ([src/components/news_carousel.rs](../src/components/news_carousel.rs)). |
| Elementor + Elementor Pro + Essential Addons | `wp-content/plugins/elementor*` script src | **DROP** — page builder leaves no runtime trace once HTML is static. |
| Revslider | `wp-content/plugins/revslider/public/assets/js/rs6.min.js` | **DROP** — banner slider, replaced by Dioxus hero sections. |
| Perfmatters lazyload | `wp-content/plugins/perfmatters/js/lazyload.min.js` | **DROP** — native `loading="lazy"` on `<img>` handles this now. |
| Google Analytics dashboard for WP | `wp-content/plugins/google-analytics-dashboard-for-wp/assets/js/frontend-gtag.min.js` | **DROP** — duplicates the gtag we ship via [src/tracking.rs](../src/tracking.rs). |
| DigEco theme JS bundle | `wp-content/themes/digeco/assets/js/*.js` | **DROP** — theme runtime gone with WordPress. |
| WP Letsencrypt SSL | `wp-content/plugins/wp-letsencrypt-ssl/admin/js/jsredirect.js` | **DROP** — SSL handled by Amplify ACM. |

## Source-of-truth IDs (preserved)

| Variable | Value | Where it lives |
| --- | --- | --- |
| `GA4_MEASUREMENT_ID` | `G-CFVBK0N6L6` | [src/tracking.rs](../src/tracking.rs) |

## CSP — what's whitelisted in `customHttp.yml`

- `script-src`: `'self' 'unsafe-inline' 'wasm-unsafe-eval' https://www.googletagmanager.com`
- `connect-src`: `'self' https://*.google-analytics.com https://*.analytics.google.com https://*.googletagmanager.com https://stats.g.doubleclick.net`
- `img-src`: `'self' data: blob: https://heartland.io https://www.googletagmanager.com https://*.google-analytics.com https://*.googletagmanager.com`
- `form-action`: `'self'` (no third-party form provider on launch)
- `frame-ancestors: 'none'`
- `'wasm-unsafe-eval'` is required for Dioxus WASM hydration — without it the mobile hamburger silently fails.

## Not present (explicit — to flag accidental re-adds)

- Klaviyo / Mailchimp / ConvertKit / Brevo (no email capture provider on launch)
- Meta Pixel / LinkedIn Insight Tag (no paid social on launch)
- GTM (we use direct gtag rather than the Tag Manager wrapper)
- Calendly / Intercom / Drift / Crisp (no chat widget on launch)
- OneTrust / Cookiebot (no consent banner on launch — minimal third-party means none currently required for GDPR/CCPA except GA4, which is configured for IP anonymization by default)

If any of the above gets added later, update both:
1. [src/tracking.rs](../src/tracking.rs) (the loader)
2. [customHttp.yml](../customHttp.yml) (the CSP allowlist — `script-src`, `connect-src`, `frame-src` as applicable)
