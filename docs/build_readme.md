# Build README — From Template to Client-Specific

A structured prompt + fill-in-the-blanks template for generating a fresh `README.md` for a new client site, using inputs from [replicate.md](replicate.md) Phase 1 discovery + [template-spec.md](template-spec.md) reference.

> **Audience:** an agent (or operator) handed a client engagement after Phases 1–3 of [replicate.md](replicate.md) are done, asked to produce the project's `README.md`. Read the discovery doc, fill in each section below, paste into `README.md` at the project root.

## Required inputs (collect first)

The README is populated from these sources. Confirm each is in hand BEFORE drafting:

| Input | Source | Used in section |
| --- | --- | --- |
| `docs/discovery-<client>.yml` | Phase 1a/1b/1c/1d output | About, Pages, Site Chrome & Tracking, Links |
| `SITE_NAME`, `SITE_BASE` | [src/main.rs](../src/main.rs) | every section that references the brand |
| Brand primary hex | [tailwind.css](../tailwind.css) `--color-red-{6..12}`, [generate_icons.py](../generate_icons.py) `BRAND_RED` | Brand Design |
| Brand fonts (display + body) | [tailwind.css](../tailwind.css) `--font-sans`, `--font-display` | Brand Design > Typography |
| Klaviyo IDs (or other email provider) | [src/tracking.rs](../src/tracking.rs) `KLAVIYO_*` | Site Chrome & Tracking |
| GA4 measurement ID | [src/tracking.rs](../src/tracking.rs) `GA4_MEASUREMENT_ID` | Site Chrome & Tracking |
| List of static routes | [src/main.rs](../src/main.rs) `Route` enum | Pages |
| List of migrated/created articles | `content/articles/*.md` | (referenced indirectly via SOP) |
| Logo files (real brand assets — NOT generated) | `assets/brand/<client>-logo-{light,dark}.png` — extracted from the live site per replicate.md Phase 1d | Brand Design > Logo |
| Stat counter values | [src/components/stat_counters.rs](../src/components/stat_counters.rs) | Site Chrome & Tracking > Stat Counters |
| 3-bullet company tenets | client interview / brand kit | Company Tenets |
| 3-word branding primitives | client interview | Branding Primitives |

If a row is blank, **don't draft yet** — bounce back to Phase 1 in [replicate.md](replicate.md) and finish discovery first. Drafting from missing data produces lies that take 5x longer to unwind than they did to write.

## README section map

The README this template produces has these sections in this order. Each section below shows: purpose, source of content, and a fill-in template.

### 1. H1 — project repo name

```markdown
# <client-slug>-rust
```

Source: client short name + `-rust` convention (e.g. `hfga-rust`, `acme-rust`). Matches `Cargo.toml` `package.name` minus the `-website` suffix.

A one-line description follows the H1:

```markdown
Dioxus + Rust + WASM rebuild of the <Client Brand Name> website ([<client-domain>](https://<client-domain>)) — migrated off <previous CMS> to a static, statically-rendered, security-headered Rust frontend.
```

For greenfield, drop "rebuild of … migrated off …" and write: "Dioxus + Rust + WASM site for <Client Brand Name> ([<domain>](https://<domain>))."

### 2. Company Tenets

```markdown
## Company Tenets

1 - **<Tenet 1>** — <one-sentence elaboration explaining the principle in the client's own voice>
2 - **<Tenet 2>** — <ditto>
3 - **<Tenet 3>** — <ditto>
```

Source: client interview. These are the **three things the brand stands for** — used throughout content, SOP brand voice rules, and CTA copy. Hold the line on three (more dilutes; fewer feels incomplete).

### 3. Branding Primitives

```markdown
## Branding Primitives

<Primitive 1>, <Primitive 2>, and <Primitive 3>
```

Source: client interview. Three single-word concepts the brand wants to evoke (e.g. *Trust, Education, Profit* / *Security, Privacy, Simplicity*). These are NOT the same as Tenets — Tenets are operating principles; Primitives are emotional / brand-feel words.

### 4. Technical Requirements

Standard block — same across every client engagement, do not modify per client:

```markdown
## Technical Requirements

1 - must use Dioxus most recent version, Rust, and WASM in all code language scenarios. If none of these are possible then request permission to use a different language before using it.
2 - Website is optimized for SEO, speed, and security with Dioxus and rust
3 - generate_icons.py is an exception for python code
```

### 5. About <Brand>

```markdown
## About <Brand>

The **<Client Long Name> (<Short Acronym>)** is <one-sentence pitch from discovery 1a>.
We <one-paragraph description of what they do, ~3 sentences>.

> *"<Founder/leader quote that captures the brand voice, from interview>"*
> — <Person name, title, organization>

<Optional second paragraph: scale numbers, partner orgs, geographic reach.>

<One sentence on the site's role: "informational" / "lead-gen" / "e-commerce" — and what it does NOT do.>
```

Source: discovery doc 1a (pitch), client interview (founder quote), Phase 1 statistics if any.

### 6. Pages

```markdown
## Pages

The Dioxus router mirrors the existing <client-domain> URLs **exactly** so all inbound SEO equity is preserved on migration:

| Route | Source page | Purpose |
| --- | --- | --- |
| `/` | Landing | <one-line purpose> |
| `/<route-2>` | <Source page name> | <one-line purpose> |
| ... | ... | ... |
| `/<news>` | News index | News index |
| `/<news>/<slug>` | Individual article | Individual article |

URL stability is non-negotiable on this migration — slugs match the live <previous CMS> site one-for-one.
```

For greenfield: replace "rebuild" / "previous CMS" framing with "the site exposes these routes" — same table, no migration framing.

Source: [src/main.rs](../src/main.rs) `Route` enum. The "Source page" column for migrations is from discovery 1b; for greenfield, it's the same as the route purpose.

### 7. Site Chrome & Tracking

The big section — inventory of every shared UI surface and third-party integration. This is the **operational handoff** for whoever builds out `components/layout.rs`, `components/popup.rs`, and `tracking.rs`.

```markdown
## Site Chrome & Tracking

Inventory of every shared UI surface and third-party integration on the live <client-domain> site, captured here as the spec for `components/layout.rs` (header + footer), `components/popup.rs` (forms + modals), and `tracking.rs` (analytics + page metadata).

### Header

- **Logo:** top-left, links to `/`. Source asset: `<filename>` (<dimensions>, native).
- **Nav items (left → right):**
  - **<Dropdown Item 1 Trigger>** (dropdown)
    - <Sub-item> → `/<route>`
    - <Sub-item> → `/<route>`
  - **<Nav item 2>** → `/<route>`
  - …
- **Header CTAs (right side):** <or "none" if removed during build>
- **Locale:** <if multi-locale, describe; else "English-only">
- **Behavior:** sticky/fixed + glass treatment (per `glass` / `header-glow` / `.site-header` utilities in [tailwind.css](tailwind.css)). Mobile: collapse nav into a hamburger.

### Footer

<Number of columns>-column grid + social row. <Copyright text>.

| Column | Contents |
| --- | --- |
| **<Column 1>** | … |
| **<Column 2>** | … |
| **<Column 3>** | … |

- **Social icons:** <list each with full URL>
- **Newsletter form:** <yes/no, location>

### Popups & Forms (`popup.rs`)

| Element | Source | Selector / ID | Where it appears | Purpose |
| --- | --- | --- | --- | --- |
| <e.g. Klaviyo embedded form> | Klaviyo onsite JS | `.klaviyo-form-<form_id>` | Mid-page on `/<route1>`, `/<route2>` | Email capture |

- **Provider company ID:** `<id>`
- **Form ID:** `<id>`
- **Implementation note:** <e.g. "Keep the embed; JS-only render. Native Rust replacement requires the provider's subscribe API + own UI.">

### Analytics & Tracking Integrations (`tracking.rs`)

| Service | ID | Source URL | Purpose |
| --- | --- | --- | --- |
| Google Analytics 4 | `<G-XXXXXXXXXX>` | `https://www.googletagmanager.com/gtag/js?id=<id>` | Pageviews, sessions |
| <other tracker if any> | | | |

- **Not present:** <list trackers explicitly NOT on the site so future agents don't add them by accident>
- **Inbound query params to capture + persist:** `utm_*`, `?ref=` — stored in `localStorage["<client-slug>.attribution"]` so any later form submission carries attribution.

### Stat Counters

The landing-page hero exposes 4 animated counters. Values live in [src/components/stat_counters.rs](src/components/stat_counters.rs) so they're trivially editable:

| Label | Value |
| --- | --- |
| <Label 1> | <Value 1> |
| <Label 2> | <Value 2> |
| <Label 3> | <Value 3> |
| <Label 4> | <Value 4> |

### Recent News Carousel

Landing + most static pages include a 3-card recent-news slider. Built off `content/articles/*.md` enumeration — picks top-3 by `published_at` desc.

### Fonts

The site uses **<font family>** (<weights>) — self-hosted from [scripts/download-fonts.sh](scripts/download-fonts.sh).
```

Source: a mix of discovery docs (1a fonts, 1c trackers, 1d assets) + the actual code (`tracking.rs`, `stat_counters.rs`, `layout.rs`, `tailwind.css`).

### 8. AEO (Answer Engine Optimization)

Standard block — copy from this template's [README.md](../README.md) `## AEO` section. Per-client edits:

- The "**Topical clusters**" sentence — adjust to mention the client's actual cluster topics
- Article counts in the size estimate (e.g. "~6KB" for 10 articles; bigger if more)

Source: [scripts/generate-aeo.py](../scripts/generate-aeo.py) `PITCH` + `CORE_PAGES` (already client-specific).

### 9. Links

```markdown
## Links

| Link | URL |
| --- | --- |
| Live site | [<domain>](https://<domain>) |
| Facebook | [<URL>](<URL>) |
| LinkedIn | [<URL>](<URL>) |
| <Other socials as applicable> | |
| Partner | <partner orgs from interview> |
```

Source: discovery 1a (social handles) + 1c (any partner orgs).

### 10. Brand Design

Standard structure; per-client values:

```markdown
## Brand Design

### Logo

<p>
  <img src="assets/brand/<client-slug>-logo-light.png" alt="<Brand> logo (for light surfaces)" width="320" />
</p>

| Asset | File | Source | Usage |
| --- | --- | --- | --- |
| Logo, light-mode | [assets/brand/<client-slug>-logo-light.png](assets/brand/<client-slug>-logo-light.png) | <`live-site URL` if extracted, else "generated by render_wordmark()"> | For light surfaces — header + footer in default/light mode |
| Logo, dark-mode  | [assets/brand/<client-slug>-logo-dark.png](assets/brand/<client-slug>-logo-dark.png) | <`live-site URL` if extracted, else "generated"> | For dark surfaces — header + footer in dark mode |
| Favicon (SVG) | [assets/brand/favicon.svg](assets/brand/favicon.svg) | Generated | Modern browsers — preferred |
| Favicon (ICO) | [assets/brand/favicon.ico](assets/brand/favicon.ico) | Generated | Legacy browser tab |
| Favicon | [assets/brand/favicon-32.png](assets/brand/favicon-32.png) | Generated | Browser tab |
| Favicon (large) | [assets/brand/favicon-192.png](assets/brand/favicon-192.png) | Generated | PWA / hi-DPI |
| Icon (PWA) | [assets/brand/icon-512.png](assets/brand/icon-512.png) | Generated | PWA maskable |
| Apple touch icon | [assets/brand/apple-touch-icon-180.png](assets/brand/apple-touch-icon-180.png) | Generated | iOS home screen |

The wordmark logos are <**real brand assets** sourced from the live site per replicate.md Phase 1d, OR a Pillow-generated fallback if the live site has no usable logo>. Either way they are checked into the repo verbatim. The favicons (square <`<monogram>`> on brand color `<hex>` with 22% rounded corners) are generated by [generate_icons.py](generate_icons.py); rerunning that script does NOT touch the wordmark PNGs (the wordmark-generation block is intentionally disabled — see the inline comment in the script for the curl commands to refresh from the live site if the brand updates).

### Color System

**Radix-style 12-step scales** (gray, <brand-color-name>, amber, green), each with a parallel dark variant, layered with semantic tokens that auto-flip on `prefers-color-scheme: dark`. Source of truth: [tailwind.css](tailwind.css).

**Brand <color-name> — flat across rungs 6→12** (used for accent, gradients, focus rings, and the icon background):

| Hex | Usage |
| --- | --- |
| `<brand-hex>` | Brand primary — `<scale>-{6..12}` and `<scale>-dark-{8..12}` |

[Standard gray scale + dark variants table — copy verbatim from this template]

**<Brand-color> tints (light mode only — rungs 1–5 form a soft <hue>, then snap to flat brand):**

[Generated tints table — see template README for hue progression]

**Status scales** — `amber-{1..12}` (warning) and `green-{1..12}` (success), plus `-dark` variants. <Note on `danger`: e.g. "brand red occupies the accent slot, so error states use `red-12` (deepest rung)" — only relevant if the brand color is in the red family>

**Semantic tokens** — copy verbatim from this template's README.

### Typography

- **Body family:** `<Font>, ui-sans-serif, system-ui, …` — defined as `--font-sans` and applied to `<body>` globally.
- **Display / headline family:** `<Font>` — defined as `--font-display`.
- **Weights shipped:** <list>
- **Scale:** Tailwind's default type scale (`text-sm`, `text-base`, … `text-7xl`).
- **Self-hosted:** `scripts/download-fonts.sh` pulls woff2 files into `assets/fonts/`.

### Custom Utilities

[Copy verbatim from this template's README — Tailwind utilities are template-wide, not client-specific]

### Layout

- **Content max-width:** `1200px` (defined as `--container-content`)
- **Header height:** `5rem` (defined as `--header-height`)
- **Mode handling:** Defaults to user's OS `prefers-color-scheme`; force with `html.dark` or `html.light`.
```

### 11. Tech Stack

Standard block — copy verbatim from template, edit:
- "Hosting" line (Amplify is default; change if client uses Cloudflare/Netlify/etc.)
- Note any per-client tracker provider (analytics, email, etc.)

### 12. Project Layout

Copy verbatim from this template's README, with these per-client adjustments:
- Update component names if the client added/removed any
- Update page filenames in `src/pages/` to match the route slugs
- Update `target/dx/<client-slug>-website/release/web/public/` paths

### 13. Local Development

Copy verbatim from this template — same dev loop for every client.

### 14. Deployment

Copy verbatim from this template; edit:
- Hosting provider name if not Amplify
- Custom domain notes if relevant

### 15. SOP for new articles

Standard block from this template's README. Per-client edits:
- Replace `<client-slug>` in paths
- Article URL pattern (e.g. `/en/news/<slug>` vs `/blog/<slug>`)
- "Three primitives" line — substitute the client's actual three branding primitives from section 3
- Author default — match `default_author()` in `src/content.rs`
- Outbound source allowlist — substitute the client's three trusted-source domains from discovery 1a
- Internal link list — substitute the client's actual route paths

## Final validation checklist

Before declaring the README done, run through:

```
[ ] Every <placeholder> in angle brackets is filled in (grep for `<` to find leftovers)
[ ] Every URL resolves (curl -sI on each link in the Links table)
[ ] H1 matches the repo / Cargo.toml package name
[ ] About <Brand> section has at most 1 founder/leader quote (more is too much)
[ ] Pages table matches Route enum exactly (count + slugs)
[ ] Tracking table lists every script that ships in TrackingHead/TrackingFooter
[ ] Stat counter values match stat_counters.rs constants
[ ] Color system table reflects tailwind.css token values
[ ] Article SOP "three primitives" matches Branding Primitives section
[ ] Article SOP author default matches default_author() in content.rs
[ ] Article SOP internal link list uses the client's actual routes
[ ] No references to "Hemp Fiber" / "HFGA" / "ad2929" / "industrial hemp" left over from this template (grep -i)
[ ] No references to "MATA" / older template seeds left over (grep -i)
[ ] Logo asset file paths exist (ls assets/brand/)
[ ] README renders correctly in GitHub's preview (drag-and-drop, no broken images, no broken table syntax)
```

## Why this doc exists

The README is the project's front door. A junior engineer landing on the repo, a future agent picking up the engagement, a client doing a code review — all of them read the README first. If it's accurate and complete, every subsequent question they have answers itself. If it's stale or generic, every subsequent question becomes a back-and-forth.

This template was hand-edited section-by-section during the HFGA build, which is fine the first time but slow + error-prone to repeat. `build_readme.md` formalizes that work into a single fillable form so the next client engagement gets a perfect README in under an hour.

## Companion docs

- [template-spec.md](template-spec.md) — what the template ships (project layout, schemas, design system). The truth that the README *describes*.
- [replicate.md](replicate.md) — how to migrate or scaffold (Phases 1–7). The process that *produces* the README inputs.
- This file — the form that *fills in* the README from those inputs.

The trio together is enough for a fresh agent or operator to deliver a complete site with no prior context.
