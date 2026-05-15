# heartland-rust

Dioxus + Rust + WASM rebuild of the **Heartland Industries** website ([heartland.io](https://heartland.io)) — migrated off WordPress + Elementor + DigEco to a static, statically-rendered, security-headered Rust frontend.

## Company Tenets

1 — **Cost reduction AND emissions reduction.** Manufacturers don't have to choose. Imperium-grade hemp materials drop straight into existing compounding lines as a substitute for talc, calcium carbonate, and (at higher loadings) glass fiber — and they're cheaper.

2 — **American supply chain, end to end.** Heartland farms industrial hemp through a contract grower network across 11 US states, processes it domestically, and ships it directly to American manufacturers. No port risk, no offshore filler exposure, no tariff surprises.

3 — **Verified carbon, not claimed carbon.** Every pound of Imperium ships with a published life-cycle analysis (the [Imperium Farming LCA](/lca/)). Per-batch disclosure packs are issued via Carbon Report. The carbon story is auditable, not marketing.

## Branding Primitives

Performance, Cost, and Carbon.

## Technical Requirements

1 — must use Dioxus most recent version, Rust, and WASM in all code language scenarios. If none of these are possible then request permission to use a different language before using it.
2 — Website is optimized for SEO, speed, and security with Dioxus and rust
3 — generate_icons.py is an exception for python code

## About Heartland

**Heartland Industries** is a material science company helping manufacturers exceed their cost-reduction goals while reducing their emissions. Headquartered in Detroit, we farm industrial hemp through a 12,000+-acre grower network across 11 US states, process it into our flagship Imperium-grade material, and ship it directly to manufacturers as filler, masterbatch, filled resin, or textile fiber.

We sell across six material categories — plastic, rubber, concrete, asphalt, paper, and textiles — and across six industry verticals — plastic compounding, automotive, packaging, construction, marine, and government. Recent program highlights: winning Continental's sustainable-material innovation challenge, joining the Amazon Devices Climate Tech Accelerator, and an Altair Enlighten Award finalist nod for the Magna × BASF × Heartland low-carbon polyamide-6 program.

> *"Customers don't pick between cost reduction or sustainability anymore — they get both. That's what Imperium delivers."*
> — John Ely, CEO of Heartland Industries

This site is the informational + lead-gen frontend — it does not handle e-commerce, customer auth, or member-gated content. The accompanying Carbon Report platform ([carbon-report.com](https://carbon-report.com)) is operated separately.

## Pages

The Dioxus router mirrors the existing heartland.io URLs **exactly** so all inbound SEO equity is preserved on migration:

| Route | Source page (heartland.io) | Purpose |
| --- | --- | --- |
| `/` | Landing | Brand-name hero + industry pillars |
| `/why-imperium` | Why Imperium | The flagship pitch — cost vs talc, carbon vs glass |
| `/imperium-masterbatch` | Imperium Masterbatch | Pre-dispersed hemp filler concentrate |
| `/imperium-filled-resin` | Performance Plastics | Pre-compounded Imperium-filled resin |
| `/imperium-filler` | Imperium Filler | Dry filler — bulk or supersack |
| `/imperium-fibers` | Imperium Textile Fiber | Hemp textile fiber + yarn + polyhemp |
| `/imperium-animal-feed` | Imperium Animal Feed | Pork, cattle, chicken feed ingredients |
| `/sustainable-plastic-compounding` | Plastic Compounding | For plastic compounders |
| `/automotive` | Automotive | OEM + Tier 1 programs |
| `/sustainable-packaging` | Packaging | Carbon-neutral pallets + bins |
| `/sustainable-building-materials` | Construction | Decking, hempcrete, hemp-WPC |
| `/sustainable-rubber-additives` | Rubber | Carbon-black partial replacement |
| `/sustainable-concrete-additives` | Concrete | Hemp-fiber + hurd reinforcement |
| `/sustainable-asphalt-additives` | Asphalt | Hot-mix asphalt modifier |
| `/sustainable-paper-additives` | Paper | Hemp pulp partial substitute |
| `/marine` | Marine | Saltwater-stable composites |
| `/government` | Government | USDA + MBDA + state DOT pilots |
| `/engineering-earth` | Engineering Earth | Regenerative agronomy program |
| `/e-books` | E-Books | Long-form downloadable guides |
| `/whitepapers` | White Papers | Technical pack |
| `/natural-fiber-research` | Research | Primary-source research library |
| `/frequently-asked-questions` | FAQ | Q&A — pricing, drop-in, supply |
| `/heartland-team` | Team | Leadership |
| `/heartland-farmers` | Our Farmers | Grower network across 11 states |
| `/green-packaging-initiative` | Green Packaging Initiative | Multi-brand commitment |
| `/lca` | Imperium Farming LCA | Life-cycle analysis disclosure |
| `/about` | (new) | Org overview |
| `/contact` | Contact | Hello@heartland.io |
| `/sustainability-news` | News index | Article library (178 posts) |
| `/sustainability-news/:slug` | Individual article | Per-article pages |

URL stability is non-negotiable on this migration — slugs match the live WordPress site one-for-one, including the `/sustainability-news/` prefix (vs. the more common `/blog/`). Deliberately skipped from migration: `portfolios/`, `page-piling/`, `heartland-blog/` (dupe of `sustainability-news`), `heartland-e-books/` (dupe of `e-books`), `case-studies/`, `usda/`, `imperium-{pork,cattle,chicken}-feed/` (covered by `imperium-animal-feed`), `imperium-{yarn,fabric,spin-ready-white-fiber}/` (covered by `imperium-fibers`), `imperium-graphene/` (planned product), `engage/`, `shop/`, `z-landing-page-*/` and the WordPress digeco_team/digeco_portfolio custom post types. The full list is in [docs/discovery-heartland.yml](docs/discovery-heartland.yml).

## Site Chrome & Tracking

Inventory of every shared UI surface and third-party integration on the live heartland.io site, captured here as the spec for [src/components/layout.rs](src/components/layout.rs), and [src/tracking.rs](src/tracking.rs).

### Header

- **Logo:** top-left, links to `/`. Source asset: `heartland-logo-light.png` for light mode (2080×202 dark wordmark) and `heartland-logo-dark.png` for dark mode (2080×202 white wordmark). Both extracted from `https://heartland.io/wp-content/uploads/2022/08/Heartland-{MAIN-LOGO,White-Logo}.png` per replicate.md Phase 1d.
- **Nav items (left → right):**
  - **Why Imperium** → `/why-imperium`
  - **Products** (dropdown) — Imperium Masterbatch / Performance Plastics / Imperium Filler / Imperium Textile Fiber / Imperium Animal Feed
  - **Industries** (dropdown) — Plastic Compounding / Automotive / Packaging / Construction / Marine / Government
  - **Materials** (dropdown) — Plastic / Rubber / Textiles / Concrete / Asphalt / Paper
  - **Resources** (dropdown) — Engineering Earth / Carbon Report (external) / E-Books / HFGA (external) / FAQ / Research / White Papers
  - **About Us** (dropdown) — Team / Our Farmers / Green Packaging Initiative / Imperium Farming LCA / Contact
  - **Articles** → `/sustainability-news`
- **Header CTAs (right side):** none — mobile reveals the same dropdowns inside a hamburger drawer.
- **Locale:** English-only. The source site has 60+ hreflang stubs (WP plugin auto-generated), all of which redirect to `/` — they are intentionally not migrated.
- **Behavior:** sticky/fixed + glass treatment (per `glass` / `header-glow` / `.site-header` utilities in [tailwind.css](tailwind.css)). Mobile: collapse nav into a hamburger with stacked, section-labelled drawer.

### Footer

4-column grid + social row. `© {year} Heartland Industries`.

| Column | Contents |
| --- | --- |
| **Brand** | Logo + one-paragraph pitch + `mailto:Hello@heartland.io` |
| **Products** | Imperium Filler / Imperium Textile Fiber / Imperium Filled Resin / Why Imperium |
| **Industries** | Plastic Compounders / Automotive / Marine / Packaging / Building Materials / Government |
| **Library + Follow** | Articles / E-Books / White Papers / Engineering Earth / Green Packaging Initiative — then social icons |

- **Social icons:** Facebook ([therealheartland](https://www.facebook.com/therealheartland/)), LinkedIn ([therealheartland](https://linkedin.com/company/therealheartland)), X ([@HeartlandXL](https://twitter.com/HeartlandXL)), Instagram ([heartlandmaterials](https://www.instagram.com/heartlandmaterials)), YouTube ([Heartland channel](https://www.youtube.com/channel/UCw3n3hnQX8PqgG-QIDb4BjA))
- **Newsletter form:** none. The source site used HubSpot forms which are not migrated; future email capture should be wired through Klaviyo or a JAMstack-friendly provider.

### Popups & Forms

**HubSpot forms v2** (portal `8084764`) are embedded on the contact block, which appears on every page anchored at `#contact`. Per-page form IDs live in [src/components/contact_block.rs](src/components/contact_block.rs) `PAGE_FORM_IDS`. The HubSpot CMS, `_hsq` analytics runtime, and HubSpot popups are dropped — forms-only is intentional. No standalone popups (Klaviyo popup.rs was deleted during migration).

### Analytics & Tracking Integrations

| Service | ID | Source URL | Purpose |
| --- | --- | --- | --- |
| Google Analytics 4 | `G-CFVBK0N6L6` | `https://www.googletagmanager.com/gtag/js?id=G-CFVBK0N6L6` | Pageviews, sessions, attribution |
| Google Floodlight | (GA4 collection shard) | `https://stats.g.doubleclick.net` | GA4 conversion-collection endpoint |
| HubSpot Forms v2 | Portal `8084764` | `https://js.hsforms.net/forms/v2.js` | Contact + demo-request lead capture (`contact_block.rs`) |
| Retool Calculator | (public embed) | `https://heartland.retool.com/embedded/public/...` | Carbon calculator iframe (`carbon_calculator.rs`) |

- **Not present** (explicitly removed during migration so future agents don't re-add by accident):
  - HubSpot Analytics + `_hsq` runtime — CMS/analytics dropped (forms-only is intentional)
  - Hotjar (`hjid:2032509`) — session replay, out of scope by default
  - Klaviyo — not on source site, popup.rs component deleted
- **Inbound query params captured + persisted:** `utm_source`, `utm_medium`, `utm_campaign`, `utm_term`, `utm_content`, `ref`, `hsLang` — stored in `localStorage["heartland.attribution"]` so any later form submission carries attribution.

### Stat Counters

The landing-page hero exposes 4 animated counters. Values live in [src/components/stat_counters.rs](src/components/stat_counters.rs):

| Label | Value |
| --- | --- |
| Acres of Imperium Farmed | 12,000+ |
| Cost Reduction vs. Talc | 30% |
| Less CO₂ vs. Glass Fiber | 90% |
| States Growing Imperium | 11 |

Per-page overrides (`product_stats()`, `farm_stats()`) ship in the same file.

### Recent News Carousel

Landing + most static pages include a 3-card recent-news slider. Built off `content/articles/*.md` enumeration — picks top-3 by `published_at` desc.

### Fonts

The site uses **Inter** (weights 400, 500, 600, 700, 800) — self-hosted from [scripts/download-fonts.sh](scripts/download-fonts.sh). The live heartland.io site uses Roboto + Poppins; we swapped to Inter per [docs/replicate.md §5i](docs/replicate.md) for CI font portability (Google's Roboto endpoint returned `.woff` not `.woff2` for fontsource user-agents during template-spec testing on a sibling client).

## AEO (Answer Engine Optimization)

Every site shipped from this playbook produces AEO surfaces alongside the HTML:

- **`/llms.txt`** — index per [llmstxt.org](https://llmstxt.org) spec. One H1, brand pitch as the blockquote, a `## Core` section listing every static page with description, and `## Articles` listing every article with excerpt.
- **`/llms-full.txt`** — the entire article corpus concatenated in one fetch. ~280 KB for the current 178 articles. Designed for one-shot LLM ingestion.
- **`/sustainability-news/<slug>.md`** — clean Markdown sibling of every article HTML page. Linked via `<link rel="alternate" type="text/markdown">` in the HTML head.

**Topical clusters** anchored by the article library: industrial hemp materials science, plastics decarbonization (the 12-part "Ultimate Guide to Plastics of the Future"), regenerative agriculture, supply-chain economics, hemp textiles, and automotive lightweighting.

Pitch and curated page list live in [scripts/generate-aeo.py](scripts/generate-aeo.py) `PITCH` and `CORE_PAGES`.

## Links

| Link | URL |
| --- | --- |
| Live site | [heartland.io](https://heartland.io) |
| Facebook | [facebook.com/therealheartland](https://www.facebook.com/therealheartland/) |
| LinkedIn | [linkedin.com/company/therealheartland](https://linkedin.com/company/therealheartland) |
| X (Twitter) | [@HeartlandXL](https://twitter.com/HeartlandXL) |
| Instagram | [@heartlandmaterials](https://www.instagram.com/heartlandmaterials) |
| YouTube | [Heartland channel](https://www.youtube.com/channel/UCw3n3hnQX8PqgG-QIDb4BjA) |
| Sister org | [Hemp Fiber and Grain Association (hfga.io)](https://hfga.io) |
| Partner platform | [Carbon Report (carbon-report.com)](https://carbon-report.com) |

## Brand Design

### Logo

| Asset | File | Source | Usage |
| --- | --- | --- | --- |
| Logo, light-mode | [assets/brand/heartland-logo-light.png](assets/brand/heartland-logo-light.png) | `heartland.io/wp-content/uploads/2022/08/Heartland-MAIN-LOGO.png` | Header + footer in light mode (dark wordmark on light surface) |
| Logo, dark-mode | [assets/brand/heartland-logo-dark.png](assets/brand/heartland-logo-dark.png) | `heartland.io/wp-content/uploads/2022/08/Heartland-White-Logo.png` | Header + footer in dark mode (white wordmark on dark surface) |
| Favicon (SVG) | [assets/brand/favicon.svg](assets/brand/favicon.svg) | Generated | Modern browsers — preferred |
| Favicon (ICO) | [assets/brand/favicon.ico](assets/brand/favicon.ico) | Generated (16/32/48 bundle) | Legacy browser tab |
| Favicon | [assets/brand/favicon-32.png](assets/brand/favicon-32.png) | Generated | Browser tab |
| Favicon (large) | [assets/brand/favicon-192.png](assets/brand/favicon-192.png) | Generated | PWA / hi-DPI |
| Icon (PWA) | [assets/brand/icon-512.png](assets/brand/icon-512.png) | Generated | PWA maskable |
| Apple touch icon | [assets/brand/apple-touch-icon-180.png](assets/brand/apple-touch-icon-180.png) | Generated | iOS home screen |

The wordmark logos are **real brand assets** sourced from the live site (Phase 1d). They are checked into the repo verbatim. The favicons (single "H" letter centered on brand red `#ad2929` with 22% rounded corners) are generated by [generate_icons.py](generate_icons.py); re-running that script does NOT touch the wordmark PNGs (the wordmark-generation block is intentionally disabled — see the inline comment in the script for the curl commands to refresh from the live site if the brand updates).

### Color System

**Radix-style 12-step scales** (gray, red, amber, green), each with a parallel dark variant, layered with semantic tokens that auto-flip on `prefers-color-scheme: dark`. Source of truth: [tailwind.css](tailwind.css).

**Brand red — flat across rungs 6→12** (used for accent, gradients, focus rings, and the icon background):

| Hex | Usage |
| --- | --- |
| `#ad2929` | Brand primary — `red-{6..12}` and `red-dark-{8..12}` |

**Semantic tokens** auto-flip with `prefers-color-scheme: dark`:

| Token | Light value | Dark value | Where used |
| --- | --- | --- | --- |
| `bg`           | `gray-1`  | `gray-dark-1`  | Page background |
| `surface`      | `gray-2`  | `gray-dark-2`  | Cards, glass panels |
| `border`       | `gray-6`  | `gray-dark-7`  | Default border |
| `fg`           | `gray-12` | `gray-dark-12` | Primary text |
| `fg-muted`     | `gray-10` | `gray-dark-9`  | Secondary text |
| `accent`       | `red-10`  | `red-dark-10`  | CTA, link |
| `accent-hover` | `red-11`  | `red-dark-11`  | Hover state |
| `accent-quiet` | `red-3`   | `red-dark-3`   | Subtle accent fill |

### Typography

- **Body family:** `Inter, ui-sans-serif, system-ui, …` — defined as `--font-sans` and applied to `<body>` globally.
- **Display / headline family:** `Inter` — defined as `--font-display`.
- **Weights shipped:** 400, 500, 600, 700, 800 (woff2 each).
- **Scale:** Tailwind's default type scale (`text-sm`, `text-base`, … `text-7xl`).
- **Self-hosted:** [scripts/download-fonts.sh](scripts/download-fonts.sh) pulls woff2 files from the fontsource CDN into [assets/fonts/](assets/fonts/).

### Custom Utilities

| Class | Effect |
| --- | --- |
| `bg-mesh`, `bg-mesh-dramatic`, `bg-mesh-hero` | Radial-gradient mesh backgrounds (subtle → bold → header-aware) |
| `surface-glass` | Frosted-glass card |
| `glass`, `header-glow` | Sticky-header treatments |
| `btn-accent-gradient` | Brand-red gradient CTA button with hover scale + glow |
| `text-gradient-red` | Brand-red gradient text for emphatic words in headlines |
| `animate-fade-in-up`, `animate-scale-in`, `animate-fade-in` | Entrance animations |
| `delay-1` … `delay-5` | Stagger animation delays (50ms → 250ms) |
| `prose-article` | Long-form article body styling |
| `container-content` | Centered max-width 1200px with responsive padding |

### Layout

- **Content max-width:** `1200px` (defined as `--container-content`)
- **Header height:** `5rem` (defined as `--header-height`)
- **Mode handling:** Defaults to user's OS `prefers-color-scheme`; force with `html.dark` or `html.light`.

## Tech Stack

- **Frontend framework:** Dioxus 0.7 (Rust → WebAssembly)
- **Routing:** Dioxus Router with `Routable` enum
- **Styling:** Tailwind v4 with Radix-style semantic tokens
- **Static site generation:** [scripts/prerender.sh](scripts/prerender.sh) walks the dx-built server binary
- **Markdown:** `pulldown-cmark` walker → Dioxus elements (no `dangerous_inner_html`)
- **Analytics:** Google Analytics 4 (GA4) only
- **Hosting:** AWS Amplify ([amplify.yml](amplify.yml), [customHttp.yml](customHttp.yml))
- **DNS:** TBD at launch — see [docs/replicate.md §6](docs/replicate.md)

## Project Layout

```
src/
  main.rs                      router + LaunchBuilder + static_routes server fn
  components/
    layout.rs                  Header (fixed, glass-on-scroll) + Footer + 6 dropdown menus
    stat_counters.rs           Hero stat row (default/product/farm variants)
    news_carousel.rs           Top-N recent articles slider
    markdown.rs                pulldown-cmark walker → Dioxus elements
  pages/
    landing.rs                 /
    why_imperium.rs            /why-imperium
    imperium_*.rs              the 5 product pages
    sustainable_*.rs           the 6 material/industry pages
    automotive.rs, marine.rs, government.rs
    engineering_earth.rs, lca.rs, green_packaging.rs
    ebooks.rs, whitepapers.rs, research.rs, faq.rs
    heartland_team.rs (team.rs), heartland_farmers.rs (farmers.rs)
    about.rs, contact.rs
    news.rs                    /sustainability-news index
    article.rs                 /sustainability-news/:slug dynamic article route
    not_found.rs               branded 404
  content.rs                   Compile-time embedded ARTICLES manifest + parser
  seo.rs                       canonical URLs + Article/Organization JSON-LD + OG/Twitter
  tracking.rs                  GA4 head + body-end slots + UTM/ref capture
content/articles/*.md          178 article files (TOML front matter + Markdown body)
assets/
  brand/                       Logos (real, extracted) + favicons (generated)
  fonts/                       Self-hosted Inter woff2 (5 weights)
  articles/<slug>/             Per-article hero (SVG placeholder for current migration)
tailwind.css                   Design tokens (@theme) + custom utilities
customHttp.yml                 Amplify response headers (CSP, HSTS, cache control)
amplify.yml                    Amplify build spec
build.rs                       Compile-time enumeration of content/articles/*.md
generate_icons.py              Favicon generator (Python — Pillow)
scripts/
  build-ssg.sh                 End-to-end production build pipeline
  build-tailwind.sh            Tailwind v4 standalone-binary compile
  prerender.sh                 Manual SSG prerender via the dx-built server
  generate-sitemap.sh          sitemap.xml + robots.txt + ads.txt
  generate-aeo.py              llms.txt + llms-full.txt + per-article .md siblings
  download-fonts.sh            Pulls Inter woff2 from fontsource CDN
docs/
  discovery-heartland.yml      Phase 1 discovery output — brand vars, routes, trackers
  trackers-heartland.md        Phase 1c — what we kept vs. dropped from the source
  slug-map-heartland.csv       Phase 2c — old URL → new slug map (178 articles)
  AMPLIFY.md, replicate.md, template-spec.md, build_readme.md, console.md
```

The build artifact lives at `target/dx/heartland-website/release/web/public/` after `./scripts/build-ssg.sh` completes.

## Local Development

```bash
# ── First time only ──────────────────────────────────────────────
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --version 0.7.7 --locked
./scripts/download-fonts.sh        # populate assets/fonts/
python3 generate_icons.py          # populate assets/brand/

# ── Hot-reload dev (every code change auto-reloads in browser) ──
dx serve --platform web            # → http://localhost:8080

# ── Type/check pass (zero warnings target) ──────────────────────
cargo check --features web --target wasm32-unknown-unknown
cargo check --features server      # SSG + server fn types

# ── Production build (full pipeline) ────────────────────────────
./scripts/build-ssg.sh             # writes target/dx/heartland-website/release/web/public/

# ── Local preview of the production bundle ──────────────────────
cd target/dx/heartland-website/release/web/public && python3 -m http.server 3000
```

## Deployment

Hosted on AWS Amplify. The build runs entirely from [amplify.yml](amplify.yml). Headers are configured via [customHttp.yml](customHttp.yml). The SPA rewrite rule must include `.md` in the static-extension allowlist so the AEO `/sustainability-news/<slug>.md` siblings serve as Markdown instead of being rewritten to the SPA shell — full details in [docs/AMPLIFY.md](docs/AMPLIFY.md).

DNS configuration (apex + www) is set at launch time — see [docs/replicate.md §6](docs/replicate.md).

## Website Hardening

Static, no-auth, no-PII frontend — the attack surface is small, but the hardening below is non-negotiable. All response headers are enforced by [customHttp.yml](customHttp.yml) and must be re-verified on every deploy.

### Response headers (enforced on every path)

| Header | Value | Rationale |
| --- | --- | --- |
| `Strict-Transport-Security` | `max-age=63072000; includeSubDomains; preload` | 2-year HSTS, preload-eligible. Submit apex to [hstspreload.org](https://hstspreload.org) after DNS cutover. |
| `Content-Security-Policy` | scoped allowlist — see below | Locks scripts/connects/frames/forms to known tracker hosts only. |
| `X-Frame-Options` | `DENY` | Belt-and-suspenders alongside `frame-ancestors 'none'` for legacy clients. |
| `X-Content-Type-Options` | `nosniff` | Blocks MIME-sniffing of `.md` / `.txt` AEO surfaces into executable contexts. |
| `Referrer-Policy` | `strict-origin-when-cross-origin` | Strips path + query from outbound referrers. |
| `Permissions-Policy` | `camera=(), microphone=(), geolocation=(), payment=(), usb=()` | We never use these — explicitly denied so injected iframes can't either. |
| `Cross-Origin-Opener-Policy` | `same-origin` | Isolates browsing context — neutralizes `window.opener` attacks. |
| `Cross-Origin-Resource-Policy` | `same-origin` | Prevents off-site embedding of our static assets. |

### CSP — what's whitelisted and why

The CSP is intentionally narrow. Every host in the allowlist must correspond to a tracker or form provider actually shipped in [src/tracking.rs](src/tracking.rs) or [src/components/contact_block.rs](src/components/contact_block.rs):

- `script-src` includes `'wasm-unsafe-eval'` — **required** for Dioxus WASM hydration. Without it every `onclick` is dead (mobile hamburger, all interactive components). See [docs/replicate.md §5c](docs/replicate.md).
- `'unsafe-inline'` survives only because HubSpot Forms v2 injects inline `<style>` and inline event attributes. We do **not** ship `'unsafe-eval'`.
- Tracker hosts: `googletagmanager.com`, `*.google-analytics.com`, `stats.g.doubleclick.net` (GA4); `*.hsforms.com` / `*.hsforms.net` / `*.hubspot.com` / `*.hs-scripts.com` / `*.hs-analytics.net` (HubSpot Forms v2); `*.retool.com` (Carbon Calculator iframe).
- Always-on lockdowns: `frame-ancestors 'none'`, `object-src 'none'`, `base-uri 'self'`, `form-action 'self' https://forms.hsforms.com https://*.hsforms.com`, `upgrade-insecure-requests`, `default-src 'self'`.

**CSP hygiene rules:**

1. **Adding a tracker = updating CSP in three places**: `script-src`, `connect-src`, and (if it iframes) `frame-src`. A new form provider also needs `form-action`.
2. **Dropping a tracker = removing every CSP entry that referenced it.** Orphan hosts are pure attack surface.
3. **Never add `'unsafe-eval'`.** If a library demands it, find another library.
4. **Never widen `default-src` past `'self'`.** Always extend the specific directive.

### TLS & transport

- HTTPS-only. `upgrade-insecure-requests` auto-rewrites any stray `http://` reference at the browser.
- HSTS is 2 years + `preload`. Submit `heartland.io` to [hstspreload.org](https://hstspreload.org) after DNS cutover so browsers enforce HTTPS before the first request.
- Amplify terminates TLS. No service worker, by design — keeps the cache simple and removes a known persistence/poisoning vector.

### Static-content & cache hygiene

- `/assets/**` and `/wasm/**` ship `Cache-Control: public, max-age=31536000, immutable` — safe because filenames are content-hashed by `dx build`.
- HTML is `must-revalidate, max-age=0` so a revert deploys instantly with no stale-window.
- AEO surfaces (`/llms.txt`, `/llms-full.txt`, `/sustainability-news/*.md`) ship a 5-minute `must-revalidate` cache with explicit `Content-Type` so they can never be coerced into being interpreted as scripts (combined with the global `nosniff`).

### Forms & user input

- No auth, no user accounts, no first-party cookies, no server-side input handling. All form POSTs go out-of-band to HubSpot.
- `form-action` in CSP restricts where forms can POST, so an injected `<form action="evil.com">` cannot ever leave the browser.
- **No `dangerous_inner_html` anywhere.** Markdown is parsed with `pulldown-cmark` and walked into Dioxus elements ([src/components/markdown.rs](src/components/markdown.rs)). Reject any PR that introduces `dangerous_inner_html`.

### Dependency hygiene

- **`cargo audit` + `cargo deny` run on every PR** via [.github/workflows/audit.yml](.github/workflows/audit.yml) (`cargo audit --deny warnings`). Failures block merge.
- **Dependabot** ([.github/dependabot.yml](.github/dependabot.yml)) opens weekly PRs for Rust crates and monthly PRs for GitHub Actions, with a supply-chain **cooldown** (patch: 3 days, minor: 7 days, major: 14 days) so freshly-published versions sit before we adopt them — mitigates typosquatting and hijacked-maintainer publishes. Security advisories (CVE-tagged) bypass cooldown.
- `dioxus-cli` is **version-pinned** (`0.7.7 --locked`) in [scripts/build-ssg.sh](scripts/build-ssg.sh), [amplify.yml](amplify.yml), and the README quickstart — floating versions risk supply-chain drift.
- Python helpers (`generate_icons.py`, `scripts/generate-aeo.py`) are stdlib + Pillow only. Pin Pillow if/when CI adopts it.

### Deploy & infrastructure

- **DNS hardening at launch** (⏳ pending cutover — site still serves WordPress today): DNSSEC at the registrar, CAA records limiting cert issuance to Amazon (`0 issue "amazon.com"`), Amplify-assigned ACM cert covering apex + www. Details in [docs/replicate.md §6](docs/replicate.md).
- Amplify build is wired from [amplify.yml](amplify.yml) on `main`. **PR previews should be disabled in the Amplify Console** so arbitrary branch code never executes in our build environment — confirm in App settings → Previews before launch.
- **Headers do not apply to Amplify's default error page.** The 404 rewrite (manual Amplify Console step, see [docs/AMPLIFY.md](docs/AMPLIFY.md)) must be configured so unknown paths serve our branded [src/pages/not_found.rs](src/pages/not_found.rs) (which inherits all headers), not Amplify's default 404. Re-verify on every deploy.

### Verification (run after each deploy)

⏳ **The Rust site is not yet at `heartland.io` — that DNS still points at the WordPress origin.** Until cutover, run these against the Amplify-assigned preview URL (e.g. `https://main.<app-id>.amplifyapp.com`).

```bash
SITE=https://heartland.io   # or the Amplify preview URL pre-cutover

# 1. All hardening headers present
curl -sI "$SITE/" | grep -iE "strict-transport-security|x-frame-options|x-content-type-options|content-security-policy|referrer-policy|permissions-policy|cross-origin"

# 2. CSP allows wasm-unsafe-eval (otherwise every onclick is dead)
curl -sI "$SITE/" | grep -i content-security-policy | grep -o "wasm-unsafe-eval" || echo "MISSING — site will not hydrate"

# 3. 404 rewrite is wired (must return branded 404 WITH headers)
curl -sI "$SITE/__definitely_not_a_page__" | grep -iE "content-security-policy|strict-transport-security"
```

External grade-card check: [securityheaders.com](https://securityheaders.com/?q=heartland.io) should return **A or A+**. Anything below is a regression.

## SOP for new articles

Articles live in `content/articles/<slug>.md`. The slug is the URL path under `/sustainability-news/`, kebab-case, never changes after publish. Each article:

```toml
+++
title = "Headline that contains the primary keyword"
excerpt = "140–180 chars. <meta description>, OG description, card preview."
hero_image = "your-keyword-bearing-filename.jpg"
hero_alt = "Description of what's in the image — not what the article is about."
published_at = "2026-05-14"
author = "Heartland Industries"
tags = ["imperium", "automotive", "carbon"]   # 1–3 tags
+++

## H2 — opening section

CommonMark body. Use absolute paths for internal links: [Imperium Filler](/imperium-filler).
External links auto-get `target="_blank" rel="noopener noreferrer"`.

## H2 — body section

…
```

**Voice rules (the three primitives — Performance, Cost, Carbon):**

- **Performance.** Drop-in compatibility with existing equipment is non-negotiable. Imperium runs on lines you already own. Lead with that.
- **Cost.** Imperium beats talc and CaCO₃ on a per-pound basis at typical loadings. Quantify wherever you can.
- **Carbon.** Use the LCA. Don't claim what isn't in the boundary. Reference [/lca](/lca) for verifiable numbers.

**Trusted outbound sources** (cite freely):

- US government — `usda.gov`, `epa.gov`, `nrcs.usda.gov`, `bls.gov`
- Industry / standards — `astm.org`, `sae.org`, `iso.org`, `cdp.net`
- Academic / industry journals — `*.edu`, `nature.com`, `pubs.acs.org`, `cefic.org`

**Default internal links to seed in body:**

- `/why-imperium` (when claiming cost or carbon advantage)
- `/imperium-filler`, `/imperium-fibers`, `/imperium-masterbatch` (when naming products)
- `/lca` (any time carbon numbers appear)
- `/engineering-earth` (when discussing agronomy)
- `/sustainability-news` (back to the index)

After adding an article: `./scripts/build-ssg.sh` will pick it up automatically via `build.rs`.

---

*Site rebuilt from heartland.io WordPress export, 2026-05. Source-of-truth playbook: [docs/replicate.md](docs/replicate.md).*
