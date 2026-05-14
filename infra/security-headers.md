# Security headers — canonical reference

The active spec is implemented in [customHttp.yml](../customHttp.yml). This file
documents the *why* behind each header so deviations are intentional, not accidental.

| Header | Value | Why |
| --- | --- | --- |
| `Strict-Transport-Security` | `max-age=63072000; includeSubDomains; preload` | 2-year HSTS with preload eligibility. |
| `X-Frame-Options` | `DENY` | The site is informational; no embed scenario. |
| `X-Content-Type-Options` | `nosniff` | Reject MIME-sniffing for served assets. |
| `Referrer-Policy` | `strict-origin-when-cross-origin` | Send origin only on cross-origin GETs. |
| `Permissions-Policy` | `camera=(), microphone=(), geolocation=(), payment=(), usb=()` | Disable powerful APIs we never need. |
| `Cross-Origin-Opener-Policy` | `same-origin` | Isolate browsing context. |
| `Cross-Origin-Resource-Policy` | `same-origin` | Block unrelated origins from loading our resources. |
| `Content-Security-Policy` | see below | Allowlists Klaviyo (forms + tracking), self-hosted fonts/images. |

## CSP allowances

- `script-src` — `self`, `'wasm-unsafe-eval'` (required for browsers to instantiate the Dioxus WASM bundle — without it, every reactive component on the page is dead), `static.klaviyo.com`, `static-tracking.klaviyo.com` (Klaviyo onsite + form rendering), `www.googletagmanager.com` (GA4 gtag.js loader).
- `style-src` — `self` + `unsafe-inline` (Klaviyo injects inline form styles).
- `img-src` — `self`, `data:`, `blob:`, `hfga.io` (legacy CDN images during transition), `static.klaviyo.com`, `www.googletagmanager.com`, `*.google-analytics.com`, `*.googletagmanager.com` (GA4 collect pixel beacons).
- `connect-src` — `self`, `a.klaviyo.com`, `static-tracking.klaviyo.com`, `*.google-analytics.com`, `*.analytics.google.com`, `*.googletagmanager.com`, `stats.g.doubleclick.net` (GA4 collect endpoints + region shards).
- `font-src` — `self`, `data:` (we self-host woff2; data: is for CSS-inlined fallbacks).
- `form-action` — `self`, `manage.kmail-lists.com` (Klaviyo form submission endpoint).
- `frame-ancestors` — `none` (matches X-Frame-Options DENY).

## What's *not* allowed

- No Meta Pixel, LinkedIn Insight, Hotjar, Intercom, Drift — none on the live site, and adding any would require a CSP update.
- No third-party CDNs for fonts (we self-host) — runtime privacy + CSP simplicity.
