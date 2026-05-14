# Template Spec

The complete reference for what this Dioxus + Rust template ships, how it's wired, and how to extend it. Pair with [replicate.md](replicate.md) (the migration cookbook) to build a site from zero — either by migrating an existing one or starting greenfield.

> **Reading order for a fresh agent:** this file → [replicate.md](replicate.md) → [AMPLIFY.md](AMPLIFY.md). Together they're enough to scaffold, customize, build, deploy, and verify any new site on this stack with no prior context.

## Table of contents

1. [What ships out of the box](#what-ships-out-of-the-box)
2. [Stack & toolchain](#stack--toolchain)
3. [Project layout](#project-layout)
4. [Routing pattern](#routing-pattern)
5. [Page component skeleton](#page-component-skeleton)
6. [Article schema (TOML front matter)](#article-schema)
7. [Design system — Tailwind v4 tokens + utilities](#design-system)
8. [Dev-loop commands](#dev-loop-commands)
9. [Build pipeline (every step, in order)](#build-pipeline)
10. [Design catalog (multiple themes, bring-your-own)](#design-catalog)
11. [Where to add a feature](#where-to-add-a-feature)

---

## What ships out of the box

Every site built from this template gets these without further work:

| Capability | Implementation file(s) |
| --- | --- |
| Static-site generation (every route prerendered to HTML) | [scripts/build-ssg.sh](../scripts/build-ssg.sh), [scripts/prerender.sh](../scripts/prerender.sh) |
| Dioxus 0.7 + WASM client + hydration | [src/main.rs](../src/main.rs), `Cargo.toml` `dioxus = { version = "0.7", features = ["router", "fullstack"] }` |
| Tailwind v4 design system (Radix-style 12-step semantic tokens, auto light/dark) | [tailwind.css](../tailwind.css), [scripts/build-tailwind.sh](../scripts/build-tailwind.sh) |
| Markdown article CMS, compile-time-embedded | [build.rs](../build.rs), [src/content.rs](../src/content.rs), [src/components/markdown.rs](../src/components/markdown.rs) |
| Per-page SEO (`<title>`, `<meta>`, OG, Twitter, canonical, JSON-LD Organization + Article) | [src/seo.rs](../src/seo.rs), called from every page |
| Sitemap.xml + robots.txt | [scripts/generate-sitemap.sh](../scripts/generate-sitemap.sh) |
| AEO surfaces (`/llms.txt`, `/llms-full.txt`, `/<route>.md` siblings, `<link rel="alternate">`) | [scripts/generate-aeo.py](../scripts/generate-aeo.py), [src/pages/article.rs](../src/pages/article.rs) |
| Hardened security headers (HSTS preload, CSP with `wasm-unsafe-eval`, X-Frame-Options DENY, COOP/CORP, Permissions-Policy) | [customHttp.yml](../customHttp.yml), [infra/security-headers.md](../infra/security-headers.md) |
| Sticky/fixed glass header that's transparent at top, solid on scroll | [src/components/layout.rs](../src/components/layout.rs), [tailwind.css](../tailwind.css) `.site-header` |
| Mobile hamburger nav with toggle | [src/components/layout.rs](../src/components/layout.rs) `Header` + `MobileMenu` |
| Hero mesh background that extends behind the transparent header | [tailwind.css](../tailwind.css) `bg-mesh-hero` (negative margin-top trick) |
| Analytics framework (GA4 wired; slot for additional providers) | [src/tracking.rs](../src/tracking.rs) `TrackingHead`, `TrackingFooter` |
| Email capture form (Klaviyo embed) — **optional, delete if client doesn't use Klaviyo** | [src/components/popup.rs](../src/components/popup.rs) |
| Dark-mode logo swap | [src/components/layout.rs](../src/components/layout.rs) `dark:hidden` / `dark:block` on `<img>` |
| 4-stat counter row (configurable per page) | [src/components/stat_counters.rs](../src/components/stat_counters.rs) |
| 3-card recent-news carousel | [src/components/news_carousel.rs](../src/components/news_carousel.rs) |
| Pricing board reusable component — **optional, delete if client has no commodity-pricing table** | [src/components/pricing_board.rs](../src/components/pricing_board.rs) |
| Self-hosted Inter font (5 weights, woff2) | [scripts/download-fonts.sh](../scripts/download-fonts.sh), pulls from fontsource CDN |
| UTM/ref/hsLang capture to localStorage on first paint | [src/tracking.rs](../src/tracking.rs) `REF_CAPTURE_JS` |
| Auto-generated favicon + PWA icons + two-line wordmark | [generate_icons.py](../generate_icons.py) |
| AWS Amplify CI/CD config | [amplify.yml](../amplify.yml), [docs/AMPLIFY.md](AMPLIFY.md) |

Out of scope (build as plugins, see EXISTING_REPLICATE.md):
- E-commerce / cart / checkout
- User auth / member-gated content
- Multi-language i18n
- Real-time / websockets
- Server-rendered dynamic content (the site is fully static)

### Optional components — delete during reset (Phase 3) if the new client doesn't need them

These ship in the template because the seed client (HFGA) used them, but most clients don't. Each row below is safe to delete cleanly — pull the `.rs` file, the `pub mod` line in `src/components/mod.rs`, the import in any page that referenced it, and the corresponding CSP allowlist entries.

| Component | When to keep | When to delete |
| --- | --- | --- |
| [src/components/popup.rs](../src/components/popup.rs) (Klaviyo embed) | Phase 1c scan turns up Klaviyo on the source site | Otherwise — also strip `klaviyo.com`, `kmail-lists.com` from `customHttp.yml` |
| [src/components/pricing_board.rs](../src/components/pricing_board.rs) (commodity-pricing table) | Client has a multi-row commodity-board pricing page | Otherwise — most B2B SaaS pricing pages don't use it; a 3-tier card grid in [pages/pricing.rs](../src/pages/pricing.rs) is simpler |

Don't ship dead code: every unused component still compiles, increases WASM bundle size, expands CSP attack surface, and bait-and-switches the next agent into thinking the client uses it. Delete decisively in Phase 3.

---

## Stack & toolchain

### `Cargo.toml` minimum (matches [Cargo.toml](../Cargo.toml))

```toml
[package]
name = "<client>-website"
version = "0.1.0"
edition = "2021"

[dependencies]
dioxus = { version = "0.7", features = ["router", "fullstack"] }
serde = { version = "1", features = ["derive"] }
toml = "0.8"
pulldown-cmark = { version = "0.12", default-features = false, features = ["html"] }
chrono = { version = "0.4", default-features = false, features = ["clock", "serde"] }

[features]
default = []
web = ["dioxus/web"]
server = ["dioxus/server"]

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### Toolchain

| Tool | Version | Why |
| --- | --- | --- |
| `rustc` | stable (1.95+ tested) | compile target |
| `wasm32-unknown-unknown` | rustup target | WASM compile output |
| `dx` (dioxus-cli) | **0.7.7 exact** | bundler — `cargo install dioxus-cli --version 0.7.7 --locked` |
| `tailwindcss` | v4.x standalone binary | CSS compile (auto-downloaded by [scripts/build-tailwind.sh](../scripts/build-tailwind.sh) if absent) |
| `python3` | 3.11+ (for `tomllib`) | AEO generator + favicon generator |
| `Pillow` | 9+ | favicon + wordmark image generation |
| `curl`, `bash` | universal | scripts |

---

## Project layout

```
src/
  main.rs                              router + LaunchBuilder + static_routes server fn
  components/
    layout.rs                          Header (fixed, glass-on-scroll) + Footer (3-col + social)
    popup.rs                           Email capture (Klaviyo embed)
    stat_counters.rs                   Hero stat row (configurable per page)
    news_carousel.rs                   Top-N recent articles slider
    pricing_board.rs                   Reusable commodity-pricing board
    markdown.rs                        pulldown-cmark walker → Dioxus elements (no dangerous_inner_html)
  pages/
    landing.rs                         /
    <inner-page>.rs                    one per static route
    article.rs                         /:slug — dynamic article route
  content.rs                           Reads compile-time embedded ARTICLES, parses front matter + body
  seo.rs                               canonical URLs + Article/Organization JSON-LD + meta + OG + Twitter
  tracking.rs                          Head + Footer analytics slots + GA4 + Klaviyo + utm/ref capture
content/
  articles/<slug>.md                   Markdown source for each article (TOML front matter + CommonMark body)
  README.md                            Article author SOP
assets/
  brand/                               Logos (light + dark variants), favicons, PWA icons (generated)
  fonts/                               Self-hosted woff2 (downloaded by scripts/download-fonts.sh)
  articles/<slug>/                     Per-article hero + inline images
tailwind.css                           Design tokens (@theme) + custom utilities
customHttp.yml                         Amplify response headers (CSP, HSTS, cache control)
amplify.yml                            Amplify build spec
build.rs                               Compile-time enumeration of content/articles/*.md
generate_icons.py                      Favicon + wordmark generator (Python — Pillow)
scripts/
  build-ssg.sh                         End-to-end production build pipeline (calls everything below)
  build-tailwind.sh                    Tailwind v4 standalone-binary compile (dx 0.7.7 doesn't auto-compile)
  prerender.sh                         Manual SSG prerender (dx --ssg silently fails for our layout)
  generate-sitemap.sh                  sitemap.xml + robots.txt
  generate-aeo.py                      llms.txt + llms-full.txt + per-article .md siblings
  download-fonts.sh                    Pulls Inter woff2 from fontsource CDN
.github/
  workflows/audit.yml                  cargo audit on push/PR/daily
  dependabot.yml                       Weekly cargo dep PRs
docs/
  template-spec.md                     this file
  replicate.md                         migration cookbook
  AMPLIFY.md                           Amplify deploy guide
infra/
  security-headers.md                  Security header rationale
```

---

## Routing pattern

Every static route is a variant on the `Route` enum. Every variant maps to one component in `src/pages/`.

```rust
// src/main.rs
use dioxus::prelude::*;

mod components;
mod content;
mod pages;
mod seo;
mod tracking;

use components::layout::LayoutShell;
use pages::{
    article::Article, faq::Faq, landing::Landing, news::News,
    /* … all your page components */
};

pub const SITE_BASE: &str = "https://<client-domain>";
pub const SITE_NAME: &str = "<Client Brand Name>";

#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[layout(LayoutShell)]
        #[route("/")]
        Landing {},
        #[route("/some-page")]
        SomePage {},
        #[route("/blog")]
        News {},
        #[route("/blog/:slug")]
        Article { slug: String },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/tailwind.css") }
        document::Link { rel: "icon", r#type: "image/svg+xml", href: "/assets/brand/favicon.svg" }
        document::Meta { name: "theme-color", content: "<brand-hex>" }
        document::Meta { charset: "utf-8" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        Router::<Route> {}
    }
}

/// dx --ssg uses this to enumerate routes (not currently functional in 0.7.7;
/// scripts/prerender.sh does the work — but keep this in sync anyway in case
/// dx fixes the bug and so consumers can auto-discover routes)
#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    let mut routes: Vec<String> = vec![
        "/".into(),
        "/some-page".into(),
        "/blog".into(),
    ];
    for slug in content::all_slugs() {
        routes.push(format!("/blog/{slug}"));
    }
    Ok(routes)
}
```

### Adding a new static route — 4 places to update

1. `Route` enum in [main.rs](../src/main.rs)
2. `use pages::<name>::<Component>` import in [main.rs](../src/main.rs)
3. New file `src/pages/<name>.rs` (use the [page skeleton](#page-component-skeleton))
4. Mirror in:
   - `static_routes` server fn in [main.rs](../src/main.rs)
   - `STATIC_ROUTES` array in [scripts/generate-sitemap.sh](../scripts/generate-sitemap.sh) (with priority + changefreq)
   - `ROUTES` array in [scripts/prerender.sh](../scripts/prerender.sh)
   - `CORE_PAGES` list in [scripts/generate-aeo.py](../scripts/generate-aeo.py) (with description)
   - Header nav in [src/components/layout.rs](../src/components/layout.rs) (Header + MobileMenu)
   - Footer column in [src/components/layout.rs](../src/components/layout.rs) Footer (if relevant)

The migration checklist in [replicate.md Phase 4](replicate.md#phase-4--per-client-customization-findreplace-map) is the same list.

---

## Page component skeleton

Every static page in `src/pages/` follows this shape. Drop this template in, swap content:

```rust
// src/pages/<name>.rs
use dioxus::prelude::*;

use crate::components::news_carousel::NewsCarousel;
use crate::components::stat_counters::{default_stats, StatCounters};
use crate::seo::Seo;
use crate::Route;

#[component]
pub fn <Name>() -> Element {
    rsx! {
        Seo {
            title: "<Page-specific title — appears in <title> with site-name suffix>",
            description: "<140–180 char meta description, keyword-rich>",
            path: "/<route>",
        }

        Hero {}
        StatCounters { stats: default_stats() }
        Sections {}
        NewsCarousel {}     // optional — drop on pages where recent posts add value
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section { class: "bg-mesh-hero section-soft-bottom",
            div { class: "container-content py-20 md:py-28 text-center",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "<eyebrow>"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-4xl mx-auto animate-fade-in-up",
                    "<headline first half>"
                    br { class: "hidden md:inline" }       // optional desktop-only break
                    " "
                    span { class: "text-gradient-red", "<keyword to highlight>" }
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)]",
                    "<sub-headline keyword-rich one or two sentences>"
                }
                div { class: "mt-8",
                    Link { to: Route::SomeOtherPage {}, class: "btn-accent-gradient", "<CTA Label>" }
                }
            }
        }
    }
}

#[component]
fn Sections() -> Element {
    let blocks: Vec<(&'static str, Vec<&'static str>)> = vec![
        ("<H2 heading>", vec!["paragraph 1", "paragraph 2"]),
        // …
    ];

    rsx! {
        section { class: "container-content py-16",
            div { class: "grid gap-10 md:grid-cols-2",
                for (i, (heading, paras)) in blocks.into_iter().enumerate() {
                    div {
                        key: "{heading}",
                        class: "surface-glass p-7 animate-fade-in-up",
                        style: "animation-delay: {i * 60}ms",
                        h2 { class: "text-2xl font-display font-bold mb-4", "{heading}" }
                        for (pi, p) in paras.into_iter().enumerate() {
                            p {
                                key: "{pi}",
                                class: "text-[color:var(--color-fg-muted)] mb-3 last:mb-0",
                                "{p}"
                            }
                        }
                    }
                }
            }
        }
    }
}
```

Real examples to copy from:
- [src/pages/landing.rs](../src/pages/landing.rs) — full hero + stat row + feature grid + CTA + testimonial + news
- [src/pages/buy_seed.rs](../src/pages/buy_seed.rs) — minimal: hero + stats + content card + news
- [src/pages/faq.rs](../src/pages/faq.rs) — accordion `<details>` for Q&A
- [src/pages/news.rs](../src/pages/news.rs) — article index with cards

---

## Article schema

Every file in `content/articles/` follows this shape. `build.rs` enumerates them at compile time and embeds via `include_str!`. `src/content.rs` parses TOML front matter on first read.

```toml
+++
title = "Headline that contains the primary keyword"
excerpt = "140–180 chars. Used as <meta description>, OG description, and the card preview."
hero_image = "your-image-filename.jpg"
hero_alt = "Description of what the user sees, not what the article is about."
published_at = "2025-03-31"           # ISO 8601 — yyyy-mm-dd or with time

# OPTIONAL
author = "Henry the Hemp Fiber Expert" # default per-project in content.rs
tags = ["hemp-fiber", "economics"]     # 1–3 tags
seo_title = "..."                      # override <title> if needed
seo_description = "..."                # override meta description if needed
+++

## H2 — first section

CommonMark body. No raw HTML (the markdown walker drops it for security).
Internal links use absolute paths: [text](/some-page).
External links auto-get `target="_blank" rel="noopener noreferrer"`.

## H2 — second section

…
```

**Required fields** (build fails without):

| Field | Type | Notes |
| --- | --- | --- |
| `title` | string | Renders as both `<title>` and `<h1>` on the article page |
| `excerpt` | string (140–180 chars) | `<meta description>`, OG description, card preview |
| `hero_image` | string (filename) | Lives in `assets/articles/<slug>/<filename>` |
| `hero_alt` | string | Accessibility + image SEO |
| `published_at` | string (ISO 8601) | Used for sort, sitemap `<lastmod>`, JSON-LD `datePublished` |

**Optional fields:**

| Field | Type | Default |
| --- | --- | --- |
| `author` | string | `default_author()` in [src/content.rs](../src/content.rs) |
| `tags` | array of strings | `[]` |
| `seo_title` | string | falls back to `title` |
| `seo_description` | string | falls back to `excerpt` |

The author SOP (structure rules, SEO checklist, brand voice) lives in [README.md](../README.md) under "SOP TO WRITE ARTICLES" — keep it per-client.

---

## Design system

### Semantic tokens (auto light/dark via `prefers-color-scheme`)

Every component uses these tokens, NOT raw hex values. Defined in [tailwind.css](../tailwind.css) `@theme`.

| Token | Light | Dark | Usage |
| --- | --- | --- | --- |
| `bg` | `gray-1` | `gray-dark-1` | Page background |
| `surface` | `gray-2` | `gray-dark-2` | Cards, panels, dropdowns |
| `border` | `gray-6` | `gray-dark-7` | Default border |
| `border-strong` | `gray-8` | `gray-dark-8` | Emphasized border |
| `fg` | `gray-12` | `gray-dark-12` | Primary text |
| `fg-muted` | `gray-10` | `gray-dark-9` | Secondary text |
| `fg-inverse` | `white` | `black` | Text on accent fills |
| `accent` | `red-10` | `red-dark-10` | Primary CTA, links |
| `accent-hover` | `red-11` | `red-dark-11` | Hover state |
| `accent-quiet` | `red-3` | `red-dark-3` | Subtle accent fill (tag pills, hover bg) |
| `accent-contrast` | `white` | `white` | Text on accent |
| `success` / `warning` / `danger` | `green-9` / `amber-9` / `red-12` | dark variants | Status colors |

Use them in components like:
```rust
class: "text-[color:var(--color-fg-muted)] bg-[color:var(--color-surface)]"
class: "border border-[color:var(--color-border)]"
```

The `red-{6..12}` brand scale is **flat** — every rung is the brand hex. Rungs 1–5 are soft tints. This pattern lets the gradient + glass utilities reach into the brand color naturally without invoking specific hex values.

### Custom utilities

| Class | Effect |
| --- | --- |
| `bg-mesh` | Subtle radial-gradient mesh + brand-color glow — for content sections |
| `bg-mesh-dramatic` | Stronger mesh + center bloom — for landing/marketing sections |
| `bg-mesh-hero` | Mode-aware (light bg in light mode, dark bg in dark mode) with brand-red bloom + negative-margin trick to extend behind the fixed transparent header |
| `section-soft-edges` | Top + bottom 160px gradient strips fading to `--soft-fade-color` |
| `section-soft-bottom` | Bottom-only fade variant — pair with `bg-mesh-hero` on first hero |
| `surface-glass` | Frosted-glass card (semi-transparent surface, blurred bg, soft border) |
| `glass` | Frosted treatment for arbitrary containers (sticky header, dropdowns) |
| `header-glow` | Subtle accent-tinted bottom border + glow — for the top nav |
| `btn-accent-gradient` | 135° gradient button with hover scale + glow + active press. White text via `a.btn-accent-gradient { color: #fff }` selector to beat global anchor color |
| `text-gradient-red` | Gradient-clipped text — for emphatic words inside headlines |
| `glow-border` | Animated 3s accent-glow pulse — for high-emphasis cards |
| `animate-fade-in-up`, `animate-scale-in`, `animate-fade-in` | Entrance animations |
| `delay-1` … `delay-5` | Stagger animation delays (50ms → 250ms) for sequential reveals |
| `prose-article` | Long-form article body styling (h2/h3/p/ul/ol/blockquote/img sizing + spacing) |
| `container-content` | Centered max-width 1200px with responsive padding |

**Site header CSS** (not a utility — applies to `.site-header` class):

```css
.site-header {
    background-color: transparent;
    /* transitions to solid on scroll via [data-scrolled] attr toggled by JS */
}
.site-header[data-scrolled] {
    background-color: color-mix(in oklab, var(--color-bg) 80%, transparent);
    backdrop-filter: blur(16px) saturate(130%);
    /* … */
}
```

The JS that toggles `data-scrolled` lives in [src/components/layout.rs](../src/components/layout.rs) `HEADER_SCROLL_JS` const, injected via `document::Script` inside `LayoutShell`.

### Typography

```css
--font-sans: "Inter", ui-sans-serif, system-ui, -apple-system, ...
--font-display: "Inter", ui-sans-serif, system-ui, -apple-system, ...
```

Inter handles both body and display. 5 weights ship: 400, 500, 600, 700, 800. Self-hosted from fontsource CDN by [scripts/download-fonts.sh](../scripts/download-fonts.sh).

To swap to a different font family: edit the `@font-face` declarations + `--font-sans` / `--font-display` vars in [tailwind.css](../tailwind.css), and adjust [scripts/download-fonts.sh](../scripts/download-fonts.sh) URLs.

---

## Dev-loop commands

```bash
# ── First time only ───────────────────────────────────────────────
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --version 0.7.7 --locked
./scripts/download-fonts.sh                  # populate assets/fonts/
python3 generate_icons.py                    # populate assets/brand/

# ── Hot-reload dev (every code change auto-reloads in browser) ───
dx serve --platform web                      # → http://localhost:8080

# ── Type/check pass (zero warnings target) ───────────────────────
cargo check --features web --target wasm32-unknown-unknown
cargo check --features server                # SSG + server fn types

# ── Production build (full pipeline: bundle + prerender + tw + sitemap + aeo) ─
./scripts/build-ssg.sh
                                             # writes target/dx/<name>-website/release/web/public/

# ── Local preview of the production bundle ───────────────────────
cd target/dx/<name>-website/release/web/public && python3 -m http.server 3000
                                             # → http://localhost:3000

# ── Asset regeneration (after editing brand vars or articles) ────
python3 generate_icons.py                    # favicons + wordmark
./scripts/build-tailwind.sh <out>            # Tailwind only (build-ssg.sh calls this internally)
./scripts/generate-sitemap.sh <out>          # sitemap + robots
python3 scripts/generate-aeo.py <out>        # AEO surfaces

# ── Add a new article ────────────────────────────────────────────
# 1. Drop hero image at assets/articles/<slug>/<filename>
# 2. Write content/articles/<slug>.md per the article schema above
# 3. ./scripts/build-ssg.sh — picks up automatically (build.rs scans content/articles)
# 4. Verify: open target/.../public/<route>/<slug>/index.html and the .md sibling
```

---

## Build pipeline

[scripts/build-ssg.sh](../scripts/build-ssg.sh) orchestrates these in order:

1. **`dx bundle --platform web --release`** — compiles the WASM client + the server binary used for prerendering. Writes to `target/dx/<name>-website/release/web/{public,server}`. **No `--ssg` flag** — it silently skips for our layout (see Phase 5b in replicate.md).
2. **[scripts/prerender.sh](../scripts/prerender.sh)** — starts the server binary on port 8089, walks every route from a hardcoded list (mirroring the `Route` enum) plus every article slug from `content/articles/`, saves the HTTP 200 response body as `<route>/index.html`. Deterministic, fully observable.
3. **[scripts/build-tailwind.sh](../scripts/build-tailwind.sh)** — compiles `tailwind.css` (which uses `@import "tailwindcss"` + `@utility` directives that browsers can't parse) using the Tailwind v4 standalone binary. Auto-downloads platform-specific binary if missing. Replaces every `tailwind*.css` file dx wrote into `public/assets/`.
4. **`cp -R assets/. <OUT>/assets/`** — copies brand, fonts, articles into the public dir.
5. **[scripts/generate-sitemap.sh](../scripts/generate-sitemap.sh)** — emits `sitemap.xml` (every static route + every article slug, with `<lastmod>` from `published_at`) and `robots.txt`.
6. **[scripts/generate-aeo.py](../scripts/generate-aeo.py)** — emits `/llms.txt`, `/llms-full.txt`, and `<route>/<slug>.md` siblings for every article.

After the pipeline completes, the contents of `target/dx/<name>-website/release/web/public/` is the deployable artifact. Drop into any static host.

---

## Design catalog

> **Status:** concept — not yet implemented in code. The variables to swap and the mechanism are documented; the catalog directory is reserved for future presets.

A site's "design" in this template is the union of:

| Aspect | File(s) | Variable |
| --- | --- | --- |
| Brand primary color | [tailwind.css](../tailwind.css), [generate_icons.py](../generate_icons.py) | `--color-red-{6..12}`, `--color-red-12` (deepest), `BRAND_RED` const |
| Body + display font family | [tailwind.css](../tailwind.css), [scripts/download-fonts.sh](../scripts/download-fonts.sh) | `--font-sans`, `--font-display`, `@font-face` srcs, fontsource URLs |
| Logo wordmark text | [generate_icons.py](../generate_icons.py) `render_wordmark()` | line 1 + line 2 strings |
| Favicon mark | [generate_icons.py](../generate_icons.py) `render_icon()` | `rows = ["HF", "GA"]` (two-line monogram pattern) |
| Hero shape (mesh background, padding scale, headline weight) | `bg-mesh-hero` utility in [tailwind.css](../tailwind.css) | gradient stops, color-mix percentages |
| Header behavior | `.site-header` in [tailwind.css](../tailwind.css) + scroll JS in [layout.rs](../src/components/layout.rs) | transition durations, glass intensity |

### Proposed preset format

```toml
# designs/<name>/preset.toml
[brand]
primary_hex = "#ad2929"
primary_dark_hex = "#6e1414"            # rung 12 — for gradients + headline accents
deepest_dark_hex = "#7a2929"

[typography]
body_family = "Inter"
display_family = "Inter"
weights = [400, 500, 600, 700, 800]
font_source = "fontsource"               # or "google" or "self-hosted"
font_source_package = "@fontsource/inter@5.0.16"

[wordmark]
line1 = "Hemp Fiber and Grain"
line2 = "Association"

[favicon]
monogram_lines = ["HF", "GA"]

[hero]
mesh_intensity = 0.28                    # primary radial-gradient opacity 0.0–1.0
glow_color = "primary"                   # or any hex
```

### Proposed catalog directory

```
designs/
  default/                              # the current HFGA-style design
    preset.toml
    overview.md                          # description, screenshot path, license
    preview.png
  minimalist/                            # future
    preset.toml
    overview.md
    preview.png
  bold-editorial/                        # future
    preset.toml
    overview.md
    preview.png
```

### Proposed `apply-design.sh` (future)

```bash
./scripts/apply-design.sh <preset_path>
# Reads preset.toml and:
#   1. Updates --color-red-* tokens in tailwind.css
#   2. Updates --font-sans / --font-display + @font-face srcs in tailwind.css
#   3. Updates BRAND_RED + render_wordmark text + render_icon rows in generate_icons.py
#   4. Updates fontsource URLs in scripts/download-fonts.sh
#   5. Runs python3 generate_icons.py
#   6. Runs ./scripts/download-fonts.sh
```

Until `apply-design.sh` is built, swapping designs is a manual edit pass (see [replicate.md Phase 4](replicate.md#phase-4--per-client-customization-findreplace-map)).

### Bring-your-own-design via URL extraction (future)

A user drops a URL of any existing site they like the look of. The system extracts brand vars and produces a preset.toml.

```bash
./scripts/extract-design-from-url.sh https://example.com > designs/extracted/preset.toml
```

The extractor uses the same techniques from [replicate.md Phase 1a](replicate.md#1a-brand-extraction-from-the-live-html):

| Variable | Extraction technique |
| --- | --- |
| `primary_hex` | Frequency-rank every `#rrggbb` and `rgb(...)` in HTML + linked CSS; the most-cited non-gray hex is the brand color |
| `body_family`, `display_family` | Parse `font-family:` declarations; identify the most-used non-fallback family |
| Logo wordmark text | OCR the logo image, OR fall back to the company name from `<meta property="og:site_name">` / `<title>` |
| `weights` | Parse `@font-face { font-weight: ... }` from linked CSS |
| Hero mesh intensity | (Hard to extract — default to template value) |

Then `./scripts/apply-design.sh designs/extracted/preset.toml` applies it.

This is a 1–2 day project to build, but the foundation (the variable list, the scrape commands) is already in [replicate.md Phase 1a](replicate.md#1a-brand-extraction-from-the-live-html). The URL extraction tool is essentially that bash one-liner, structured into a TOML emitter.

### Per-design custom components (advanced — future)

If a preset wants different page layouts (e.g. a magazine-style article template vs the current centered-prose template), introduce a `components/` override directory in the preset:

```
designs/
  bold-editorial/
    preset.toml
    components/
      article.rs                         # alternative article layout
      hero.rs                            # alternative hero shape
```

`apply-design.sh` then symlinks (or copies) the override files over the defaults in `src/components/`. This keeps the build system unchanged while enabling structural design swaps.

---

## Where to add a feature

A quick map for common extensions:

| I want to add… | Touch these files |
| --- | --- |
| A new static page | `src/main.rs` (Route enum + import), `src/pages/<name>.rs` (component), header nav in `src/components/layout.rs`, sitemap + prerender + AEO scripts |
| A new analytics provider | `src/tracking.rs` `TrackingHead` (load script) + `customHttp.yml` CSP allowlist |
| A new tracker pixel (e.g. Meta) | same as analytics |
| Google Search Console verification | meta tag in `App` component in `src/main.rs` (or in `TrackingHead`) |
| A cookie consent banner | new component in `src/components/consent.rs`; render from `LayoutShell`; CSP allowlist for the consent provider's host |
| A contact form | new component in `src/components/contact.rs`; HTML form posting to a third-party endpoint (Formspree, Web3Forms, AWS API Gateway) — add the endpoint to `form-action` directive in `customHttp.yml` |
| A new article | `content/articles/<slug>.md` + `assets/articles/<slug>/<hero>.png` — auto-discovered |
| Per-page custom JSON-LD | `src/seo.rs` add a new builder fn (mirroring `article_jsonld`); call from the page component |
| A new tracker event (custom GA4 event) | inline JS in the page component via `document::Script`, or a Dioxus `onclick` that calls `gtag('event', ...)` (requires the user to interact post-hydration) |
| A new design preset | `designs/<name>/preset.toml` (concept above) — manual `apply-design.sh` until that script ships |
| A plugin (e-commerce, booking, etc.) | new top-level module under `src/` + new component(s); see EXISTING_REPLICATE.md "Plugin repository" for the architecture |

---

*Last updated from learnings on the HFGA migration (2026-05). When this template's structure changes, update this file FIRST so future agents work from accurate spec.*
