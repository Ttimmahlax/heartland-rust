# Replicate — Migration & New-Site Playbook

End-to-end recipe for spinning up a new Dioxus + Rust + Tailwind site, **or migrating an existing site** (HubSpot, WordPress, Wix, Squarespace) onto this stack. Captured from the live HFGA migration; every step is what actually worked or hit a wall.

> **Companion docs:** [template-spec.md](template-spec.md) is the technical reference (project layout, page skeleton, article schema, design system). [EXISTING_REPLICATE.md](EXISTING_REPLICATE.md) is the complete service playbook. This file is the migration / new-site cookbook — read all three.

> **Two paths through this doc:**
> - **Migrating an existing site** — follow Phases 1 + 2 verbatim (scrape, harvest).
> - **Greenfield (no existing site)** — replace Phases 1 + 2 with the **"Greenfield path"** subsections inside each. Phases 3–7 are identical for both.

## Table of contents

1. [Prerequisites](#prerequisites)
2. [Phase 1 — Site discovery (45 minutes)](#phase-1--site-discovery)
3. [Phase 2 — Content harvest (60–120 minutes)](#phase-2--content-harvest)
4. [Phase 3 — Project scaffold (30 minutes)](#phase-3--project-scaffold)
5. [Phase 4 — Per-client customization (find/replace map)](#phase-4--per-client-customization-findreplace-map)
6. [Phase 5 — Build pipeline gotchas (lessons)](#phase-5--build-pipeline-gotchas-lessons)
7. [Phase 6 — Deploy & DNS](#phase-6--deploy--dns)
8. [Phase 7 — Verify (curl checks)](#phase-7--verify-curl-checks)
9. [Per-client customization checklist](#per-client-customization-checklist)
10. [Migration provenance — what to capture for the audit trail](#migration-provenance)

---

## Prerequisites

| Item | Why | Verify |
| --- | --- | --- |
| Rust stable + `wasm32-unknown-unknown` | compile target | `rustup target list --installed \| grep wasm` |
| `dx` CLI **v0.7.7 exact** | Dioxus build driver | `dx --version` → `dioxus 0.7.7` |
| Python 3.11+ with `tomllib` | AEO + asset generators | `python3 -c "import tomllib"` |
| Pillow (Python) | favicon + wordmark generation | `python3 -c "import PIL; print(PIL.__version__)"` |
| `tailwindcss` standalone v4 (or skip — script downloads on demand) | CSS compile (dx 0.7.7 ships uncompiled — see Phase 5) | `tailwindcss --version` → `v4.x` |
| `curl`, `bash`, GNU coreutils on dev machine | scripts | universal |

Lock these in a fresh shell BEFORE starting; mismatches surface late and cost hours.

---

## Phase 1 — Site discovery

The goal of this phase: a **single document** with every brand variable, every URL, and every third-party integration the new site depends on. For migrations, scrape the existing site. For greenfield, interview the client. Either way, the output is the same shape — and Phases 3–7 don't care which route you took.

> **Greenfield shortcut:** if the client has *no* existing site, jump to the [Phase 1 — Greenfield path](#phase-1--greenfield-path) at the end of this section.

### 1a. Brand extraction (from the live HTML)

```bash
SITE="https://example.com"

# Tagline / hero copy / company description
curl -sSL "$SITE" | grep -oE '<meta[^>]*(name="description"|property="og:description")[^>]*content="[^"]+"' | head -3

# Page <title>
curl -sSL "$SITE" | grep -oE '<title>[^<]+</title>'

# Logo: every <img> in the header / first 4KB
curl -sSL "$SITE" | head -c 4096 | grep -oE '<img[^>]*src="[^"]*logo[^"]*"' | head -3

# Brand color — TWO sources to scan, not just one:
#
#   (a) Inline styles on the homepage HTML (often only 1–3 hits — easy to miss)
curl -sSL "$SITE" | grep -oE '(color|background)[^;:]*:[^;}"]*#[0-9a-fA-F]{6}' | head -20
#
#   (b) The theme's compiled CSS file — far more representative
CSS_URL=$(curl -sSL "$SITE" | grep -oE 'https://[^"]+\.css[^"]*' | head -1)
curl -sSL "$CSS_URL" | grep -oE '#[0-9a-fA-F]{6}' | sort | uniq -c | sort -rn | head -15
#
# Cross-reference the two: the brand color is what appears in (a) AND
# stays high in (b) AFTER you discount Bootstrap defaults (#0d6efd) /
# Tailwind defaults / pure white / pure black / common gray ramps.
#
# Carbon Report example: #741160 had 2 hits in homepage HTML but ranked #2
# in the theme CSS hex frequency table — that's the brand color.
# #0d6efd had 37 hits in the theme CSS but only because the WP theme
# inherits Bootstrap defaults that are never actually rendered.

# Font families
curl -sSL "$SITE" | grep -oE 'font-family:[^;}"]+' | sort -u | head -10
curl -sSL "$SITE" | grep -oE 'fonts\.(googleapis|gstatic)\.com[^"\)]+' | sort -u | head -10

# Social links
curl -sSL "$SITE" | grep -oE 'https://(www\.)?(facebook|twitter|x|linkedin|instagram|youtube)\.com/[^"]+' | sort -u
```

Capture into a discovery doc:

```yaml
# docs/discovery-<client>.yml — keep this for the audit trail
brand:
  name: "Hemp Fiber and Grain Association"
  short_name: "HFGA"
  tagline: "Building a roadmap for farmers to easily start raising industrial hemp fiber and grain."
  primary_color: "#ad2929"   # most-referenced hex
  body_font: "Kumbh Sans"     # but consider replacing with Inter for portability
  display_font: "Montserrat"
  logo_url: "https://example.com/.../logo.png"
  social:
    facebook: https://...
    linkedin: https://...
```

> **Font portability note:** specialty Google Fonts (Kumbh Sans, etc.) sometimes fail in CI font-fetch scripts. **Default to Inter** unless the brand insists otherwise — Inter is a single family that covers display + body + numerics, available reliably from `cdn.jsdelivr.net/npm/@fontsource/inter`. We hit this on HFGA — original Google Fonts CSS endpoint returned `.woff` not `.woff2` for our user-agent; switching to fontsource fixed it instantly.

### 1b. Sitemap enumeration (every URL the new build must cover)

Try in this order:

```bash
# 1. WordPress + Yoast SEO ships a sitemap_index that points to multiple
#    sub-sitemaps (post / page / category / post_tag). Enumerate ALL of them.
curl -sSL "$SITE/sitemap_index.xml" | grep -oE '<loc>[^<]+</loc>'
# Then walk each sub-sitemap:
for s in post page category post_tag; do
  curl -sSL "$SITE/$s-sitemap.xml" | grep -oE '<loc>[^<]+</loc>' \
    | sed 's|<loc>||; s|</loc>||'
done > /tmp/urls-from-sitemap.txt

# 2. Plain /sitemap.xml — non-Yoast WP, Wix, Squarespace, hand-rolled CMSes
curl -sSL "$SITE/sitemap.xml" | grep -oE '<loc>[^<]+</loc>' \
  | sed 's|<loc>||; s|</loc>||' >> /tmp/urls-from-sitemap.txt

# 3. The robots.txt — sometimes points to non-default sitemap paths
curl -sSL "$SITE/robots.txt"

# 4. The header navigation
curl -sSL "$SITE" | grep -oE 'href="[^"]+"' | grep -v 'http' | sort -u

# 5. The footer
curl -sSL "$SITE" | tail -c 8192 | grep -oE 'href="[^"]+"' | sort -u

# 6. Manual click-through — make sure no orphan pages

# 7. Sort + dedupe everything
sort -u /tmp/urls-from-sitemap.txt | sed -E "s|^https?://[^/]+||; s|/$||" \
  > /tmp/urls-clean.txt
```

**Pages to deliberately skip** (in sitemap but should NOT migrate to the Rust build):

| Pattern | Why skip |
| --- | --- |
| `/thank-you/`, `/thank-you-guide/` | Post-form-submission landing — only reachable after lead capture, no inbound SEO. |
| `/blog-roll-test/`, `/test/`, `*-staging/` | Internal staging/test pages left in the sitemap by accident. |
| `/wp-login.php`, `/wp-admin/*` | WordPress admin paths — never expose. |
| `/news/category/*`, `/news/tag/*` (WordPress category/tag indexes) | Auto-generated WP archive pages. The Articles index page covers the same content; migrate them as redirects later if needed, but don't replicate as separate routes. |
| `/feed/`, `/comments/feed/` | RSS feeds — out of scope for this stack. |
| `/?p=NNN` query-string permalinks | Already 301'd by WordPress to the slug-form URL. |

Document every skipped URL in `docs/discovery-<client>.yml` so it's deliberate, not an oversight.

> **URL stability is non-negotiable.** Every existing public URL must resolve to the same content on the new site. Slugs migrate **exactly**, including any typos (HFGA had `corn-vs-hemp-fiber-economics-hemp-fiber-associatio` — the missing `n` was preserved on purpose for SEO equity). Never "clean up" slugs at migration time.

### 1c. Tracking & integrations inventory

The goal of this scan is **two lists**: what's running on the source site (capture every ID), and what's NOT running (so the new site doesn't accidentally add it). Both lists go into `docs/trackers-<client>.md` for the audit trail.

```bash
# 1. Every external <script> src on the homepage
curl -sSL "$SITE" | grep -oE '<script[^>]+src="[^"]+"' | grep -oE 'https?://[^"]+' | sort -u

# 2. Every inline tracker pattern
curl -sSL "$SITE" | grep -oiE '(gtag\(|fbq\(|_hsq|_paq\.|clarity\(|hjEvent|intercomSettings|driftt|crisp|drift|heap\.|smartlook|tealium|onetrust|cookiebot|chmln|pendo|amplitude\.|klaviyo)' | sort -u

# 3. Tracker IDs visible in the HTML
curl -sSL "$SITE" | grep -oE '(G-[A-Z0-9]+|GTM-[A-Z0-9]+|UA-[0-9]+-[0-9]+)' | sort -u

# 4. iframes (chat widgets, consent banners)
curl -sSL "$SITE" | grep -oE '<iframe[^>]+src="[^"]+"' | head -10

# 5. <noscript> fallbacks (often hide FB Pixel / GTM that gtag config refers to)
curl -sSL "$SITE" | grep -oE '<noscript[^>]*>[^<]+</noscript>' | head -5

# Klaviyo specifically (very common on HubSpot sites)
curl -sSL "$SITE" | grep -oE 'klaviyo\.com/onsite[^"]+' | head -3
curl -sSL "$SITE" | grep -oE 'klaviyo-form-[a-zA-Z0-9]+' | head -3

# HubSpot script loader → reveals Hub ID + Portal ID
curl -sSL "$SITE" | grep -oE '/hs/scriptloader/[0-9]+' | head -1
curl -sSL "$SITE" | grep -oE 'hubfs/[0-9]+' | head -1
```

Document each integration with **what we keep** and **what we drop** explicitly:

| Integration | Action |
| --- | --- |
| HubSpot Analytics | **Drop** (goes away with HubSpot CMS — replace with GA4 or similar) |
| HubSpot CWV / template JS / `_hsq` queue | **Drop** (HubSpot-specific runtime) |
| jQuery + slick carousel | **Drop** (rebuild as native Dioxus components) |
| Klaviyo | **Keep** if the client uses it for email; capture company ID + form ID. **If absent, delete `src/components/popup.rs` and remove the Klaviyo CSP entries.** |
| GA4 / GTM / Meta Pixel / LinkedIn Insight | **Keep** if present (capture IDs) |
| Calendly | **Keep** if used as the demo/booking channel; allow `https://calendly.com` in `script-src`, `connect-src`, `frame-src` of CSP. |
| Cookie consent banner | Keep architecturally; replace the implementation |

**CSP allowlist hygiene.** [customHttp.yml](../customHttp.yml) ships with Klaviyo and GA4 hosts whitelisted by default. After the inventory is locked:

- **Remove** `static.klaviyo.com`, `static-tracking.klaviyo.com`, `a.klaviyo.com`, `manage.kmail-lists.com` if Klaviyo is dropped.
- **Add** any new tracker hosts (Calendly, Hotjar, Intercom, etc.) in three CSP directives: `script-src`, `connect-src`, and `frame-src` (for iframes).
- **Replace** `https://hfga.io` with the new client domain in `img-src`.

A tighter CSP is also a security hygiene improvement — every directive that allows a host you don't actually use is a wider attack surface.

### 1d. Asset extraction (logos + images)

> **Real brand assets > generated wordmark.** Always try to extract the real brand logo from the live site **first**. Pillow-generated wordmarks (`render_wordmark()` in [generate_icons.py](../generate_icons.py)) are a *fallback*, not the default — a generated text logo will never match the customer's actual brand mark. Only use generation when the live site has no logo at all (rare) or the logo is a hostile vector format we can't reasonably extract.

```bash
mkdir -p assets/brand assets/articles

# 1. Search for ALL logo candidates — not just files matching *logo*. WP, Wix,
#    Squarespace often upload brand assets with auto-generated names like
#    "Untitled-9.png", "asset_v2.svg", or "logo_main_v3-final-final.png".
#
#    Cast a wide net: every <img> on the homepage, every <link rel="icon">
#    target, every image referenced in the theme's CSS. Visually inspect the
#    candidates (Read tool can render PNGs) to find the real brand mark.

# Every <img> in the header / first 4KB of the homepage
curl -sSL "$SITE" | head -c 4096 | grep -oE '<img[^>]+(src|data-src)="[^"]+"'

# Every <link rel="icon"> target (the favicon source is often the brand mark)
curl -sSL "$SITE" | grep -oE 'rel="icon"[^>]+href="[^"]+"'

# 2. The live site usually only ships ONE color variant (e.g. white-on-
#    transparent for dark headers, or full-color for the WP customizer
#    favicon). For the new site we need BOTH:
#       carbon-report-logo-light.png  — for light backgrounds (dark or color ink)
#       carbon-report-logo-dark.png   — for dark  backgrounds (white or light ink)
#    If only one exists, programmatically derive the other:
#       white-on-transparent → recolor white pixels to brand color or near-black
#       color-on-transparent → recolor colored pixels to white
#    Pillow handles this in ~10 lines (see scripts/recolor-logo.py if shipped).

# Largest unsized version on WordPress sites:
curl -sSL -o assets/brand/source-logo.png \
  "$SITE/wp-content/uploads/YYYY/MM/<filename-without-size-suffix>.png"
file assets/brand/source-logo.png  # confirm: PNG image data, NxM, RGBA

# 3. Disable wordmark regeneration in generate_icons.py once the real PNGs
#    are committed. Otherwise a future `python3 generate_icons.py` run
#    silently overwrites the real brand mark with a generated text logo.
#    Replace the wordmark-generation block with a comment + the curl commands
#    used to refresh the source files (see Carbon Report's
#    generate_icons.py for the pattern).

# Article hero images: scrape the news page
curl -sSL "$SITE/blog" | grep -oE 'https?://[^"]+\.(png|jpg|jpeg|webp)' | sort -u > /tmp/image-urls.txt
# Manually map URL → article slug → desired keyword-bearing filename
```

**Filename pitfalls:**

- HubSpot hero images live at `${SITE}/hubfs/<filename>.png`; filenames are URL-encoded (spaces → `%20`).
- WordPress + Yoast `og:image` is sometimes stale — the article's published HTML moved on but the meta tag didn't. **Always verify with `curl -sSI` before downloading.** If the og:image returns 404 or HTML, fall back to scraping the `<img>` tags on the article page itself for the actual hero. (We hit this on Carbon Report's `creating-your-first-carbon-report` article — substituted the closely-related "adding-your-first-product" hero rather than ship a broken image.)
- Always confirm with `file` after download — silent 404s come back as `HTML document text`, not `PNG image data`.

### Phase 1 — Greenfield path

No existing site to scrape — interview the client. Use this question script; treat each answer as a row in `docs/discovery-<client>.yml` so Phase 4's find/replace map has the same shape regardless of source.

#### Brand intake (replaces 1a)

Send the client this list. Collect answers BEFORE starting Phase 3 — guesswork in this stage is the #1 cause of two-week reworks.

| Question | Why we need it | If they don't have one |
| --- | --- | --- |
| **Legal company name + short brand name** | `<title>` suffix, JSON-LD `name`, header logo alt | Use a working draft; mark as "TBD-confirm before launch" |
| **One-sentence elevator pitch (≤ 200 chars)** | Hero subheadline + `og:description` + `<meta description>` baseline + AEO `PITCH` constant | Co-write with client during kickoff — should answer "what does this org do for whom?" |
| **One-paragraph mission statement (~400 chars)** | About-us copy + AEO `llms.txt` blockquote + JSON-LD `description` | Same as above |
| **Primary brand color (hex)** | `--color-red-{6..12}`, `BRAND_RED`, theme-color meta | Recommend a hue from the brand kit; if no kit, pick from a moodboard |
| **Secondary / accent colors (optional, hex × 2)** | Reserved for status colors; usually we use the standard amber/green | Skip — defaults work |
| **Logo files** (SVG preferred; high-res PNG OK) | Header + footer logo + favicon source | Generate a wordmark via [generate_icons.py](../generate_icons.py) `render_wordmark()` |
| **Brand fonts (display + body)** | `--font-sans`, `--font-display`, font-source URLs | **Default to Inter** — single family, reliable, free, covers display + body. Only override if brand insists |
| **3–5 keywords the brand owns** | Primary keyword for SEO + AEO descriptions | Discover via competitor analysis (see Note below) |
| **Tone of voice** (3 adjectives + 1 anti-pattern) | Article SOP voice rules in [README.md](../README.md) | Default: "confident, direct, plain-English. Never corporate-speak or LLM tells." |
| **3 outbound source domains they trust** | Article SOP outbound link allowlist | Default: government / academic / industry-standard depending on vertical |
| **Social handles** (Facebook, LinkedIn, X, Instagram, YouTube) | Footer social row + JSON-LD `sameAs` | Skip the row if none |

> **Keyword note:** if the client doesn't have keywords ready, ask "what do customers Google when they're looking for what you do, and they don't yet know your brand name?" → those phrases ARE the keywords. Or run a competitor's URL through the Phase 1a scraping commands and inspect the words they emphasize.

#### Page architecture intake (replaces 1b)

Sitemap-by-conversation. Walk the client through these standard buckets and have them mark which apply:

| Standard page | Purpose | Default route | Skip if… |
| --- | --- | --- | --- |
| **Landing** (always required) | Brand-name H1 + hero pitch + CTAs + features + recent content | `/` | never skip |
| **About / Mission** | Org story, team, history | `/about` or `/mission` | client wants Landing to absorb this content |
| **Product/service detail** (one per offering) | Detail page per major offering | `/<offering-slug>` | one-page sites only |
| **Buy / Order / Sign up** | Conversion destination — usually external CTA | `/buy` or external | conversion happens elsewhere (Stripe, Calendly, etc.) |
| **Pricing** | Pricing tiers | `/pricing` | irrelevant for the business model |
| **FAQ** | Bottom-funnel objections, AEO-friendly Q&A | `/faq` | unusual for a marketing site |
| **Blog / News index** | Article CMS frontend | `/blog` or `/news` | client doesn't want to publish content |
| **Article template** (dynamic) | Per-article HTML page | `/blog/:slug` | comes free with blog index |
| **Contact** | Email / form / phone | `/contact` | the FAQ has the email |
| **Press / Media** | Logo downloads + factsheet | `/press` | not press-relevant |
| **Privacy + Terms** | Legal | `/privacy`, `/terms` | rare, but ask — if they do GDPR/CCPA it's required |

For each page they want, capture:
- **Route slug** (URL path — keyword-bearing, kebab-case, never change after publish)
- **Page title** (`<title>` content)
- **One-sentence purpose** (drives `<meta description>` baseline + AEO description)
- **Primary keyword** (SEO target; should appear in `<title>`, H1, body)
- **Source content** (do they have copy ready? Markdown? Slack thread? Brand-team interview transcript? Or write from scratch?)

#### Tracking intake (replaces 1c)

```
☐ Google Analytics 4   — measurement ID (G-XXXXXXXXXX)?            [if no, recommend they create one — free, takes 5 min]
☐ Google Tag Manager   — container ID (GTM-XXXXXXX)?               [skip unless they need multi-tag orchestration]
☐ Email capture        — provider + form ID? (Klaviyo, Mailchimp, ConvertKit, Brevo)
☐ Meta Pixel           — pixel ID? (only if running Facebook ads)
☐ LinkedIn Insight     — partner ID? (only if running LinkedIn ads)
☐ Customer chat        — Intercom / Drift / Crisp / none?           [skip unless support-heavy]
☐ Cookie consent       — required? (GDPR/CCPA region?) Provider?    [if yes: OneTrust / Cookiebot / Osano / iubenda]
☐ Hotjar / Clarity     — heatmaps + session replay?                 [optional; default off]
```

Default recommendation: **GA4 only**. Add others only when the client has a concrete reason. Every tracker you add eats CSP allowlist space and slightly increases LCP.

#### Asset intake (replaces 1d)

For greenfield, these come via brand kit handoff (Drive / Dropbox / Figma) rather than scraping:

| Asset | Format | Where it goes | If missing |
| --- | --- | --- | --- |
| Primary logo (full color, on white) | SVG (preferred) or 2000+ px PNG | `assets/brand/hfga-logo-light.png` (rename) | Generate via [generate_icons.py](../generate_icons.py) `render_wordmark()` — clean two-line wordmark |
| Inverted logo (for dark mode) | SVG or PNG with white/light fill | `assets/brand/hfga-logo-dark.png` (rename) | Generate via Pillow luminance inversion (see [generate_icons.py](../generate_icons.py)) |
| Favicon source — color symbol if they have one | SVG or 512×512 PNG | `assets/brand/icon-512.png` (rename / regenerate) | Use `render_icon()` two-line monogram on brand color |
| Hero image / section imagery | JPG/PNG ≥ 1600×900 | `assets/articles/<slug>/<keyword-bearing>.jpg` for articles; inline `assets/<purpose>.jpg` for static pages | Source from a stock library; client approves licensing |
| Brand fonts (if not from Google) | TTF / OTF / WOFF2 | `assets/fonts/` | Default Inter via [scripts/download-fonts.sh](../scripts/download-fonts.sh) |

---

## Phase 2 — Content harvest

> **Greenfield shortcut:** if there's no existing site, jump to [Phase 2 — Greenfield path](#phase-2--greenfield-path) at the end of this section.

### 2a. Per-page transcription

WebFetch (or any HTML→Markdown extractor) is the lever here. Use a **structured prompt** that asks for faithful transcription, not summary:

> "Give me a faithful, COMPLETE transcription of every visible text element on this page in document order: hero headline, subheadline, all body paragraphs, all stat counters with labels and values, all section headings, all card titles and descriptions, all CTA button labels, testimonials/quotes with attribution. **Preserve original wording. Do NOT summarize.** Output as structured markdown. Skip navigation and footer."

Run that prompt against every URL in your sitemap from Phase 1b. Save to `docs/source/<slug>.md`.

**Parallelize aggressively.** Fire WebFetch calls in batches of **6–7 in parallel** per assistant turn. For ~30 articles that's 4 turns × ~30 sec each ≈ 2 min total wall-clock. Bigger batches (10+) risk context-window pressure on the harvest output; smaller batches (1–2) waste latency.

### 2b. Per-article transcription

Same idea, scoped to article body:

> "Give me the COMPLETE article body text in document order, including all headings (preserve H2/H3 levels), every paragraph, every list, and every block quote. **Preserve original wording.** Do NOT summarize. Output as faithful markdown. Skip site nav, footer, and the recent-news sidebar."

### 2c. Slug + filename map

Build a CSV — one row per migrated article:

```
old_url,new_slug,hero_image_old_url,hero_image_new_filename,published_at,author,tags
.../how-trade-tariffs-will-impact-us-hemp-fiber-and-grain,how-trade-tariffs-will-impact-us-hemp-fiber-and-grain,.../How%20Trade%20Tariffs....png,trade-tariffs-impact-us-hemp.png,2025-02-03,Henry...,"hemp-fiber,policy"
```

The `new_slug` ALWAYS equals the existing slug verbatim. The hero image filename gets renamed to be **keyword-bearing** (article SOP requires this) but the slug stays.

### 2d. Bulk image download (bash heredoc, portable)

Use bash arrays with explicit quoting; zsh-style associative arrays choke under stripped-PATH parents:

```bash
bash <<'EOF'
declare -a ENTRIES=(
  "slug-1|new-filename-1.png|hubfs%20encoded%20path%201.png"
  "slug-2|new-filename-2.png|hubfs%20encoded%20path%202.png"
)
for entry in "${ENTRIES[@]}"; do
  IFS='|' read -r slug fn src <<< "$entry"
  out="assets/articles/$slug/$fn"
  mkdir -p "$(dirname "$out")"
  curl -sSL -o "$out" "$SITE/hubfs/$src"
  file "$out"  # always verify — silent 404s are common
done
EOF
```

If `file` reports `HTML document text` instead of `PNG image data`, the source URL is stale — re-scrape the article page for the actual hero image URL.

### Phase 2 — Greenfield path

No existing content to harvest — write it. The [page component skeleton in template-spec.md](template-spec.md#page-component-skeleton) gives you the shape; the work is filling in copy.

#### Per-page content capture (replaces 2a / 2b)

For each page identified in Phase 1's "Page architecture intake," capture content into `docs/source/<slug>.md`. Same destination as the migration path; same shape downstream:

```markdown
# <Page Name>     <!-- becomes the H1 / SEO title -->

> <one-sentence purpose>     <!-- becomes the <meta description> + AEO description -->

## Hero

- **Eyebrow** (small uppercase text above H1): "..."
- **H1** (huge): "Brand Name" or "Better X for Y" — keyword-rich
- **Subheadline** (paragraph): 1–2 sentences resolving the headline question
- **CTAs**: text + destination route per button (max 2)

## Stat row (optional)

If the page benefits from a 4-stat row, list 4 (value, label) pairs. e.g. "12,000+ acres", "21% premium". If not, skip — page renders without `StatCounters`.

## Section 1 — <H2 heading>

Body paragraphs (3–5 short paragraphs).

## Section 2 — <H2 heading>

Body paragraphs (3–5 short paragraphs).

## CTA section

Closing pitch + final CTA button.
```

Where the copy comes from:

| Source | When to use |
| --- | --- |
| **Client SME interview** (60-min call, recorded, transcribed) | When the client has expert content but no writer — extract their words verbatim, structure into the skeleton |
| **Existing client deck / pitch / RFP responses** | Often the elevator pitch + pillars + FAQs are already written for sales |
| **Competitor analysis** | Read 3 competitor sites, identify what claims/structure work, write better — never copy |
| **AI-assisted draft, client SME edit** | Generate a v0 draft from the client's notes, hand back for SME review/edits — fastest first-draft loop |

> **Rule:** content goes through a human SME review before launch. AI-drafted text reads as competent but generic; the SME pass adds the specificity that earns trust + ranks.

#### Article generation (replaces 2c)

For greenfield blogs starting from zero, write the **first 5 articles before launch** so the news index isn't empty on day one. Pick topics that:

1. Hit the brand's primary keywords from Phase 1
2. Answer questions a customer would Google before knowing the brand
3. Cite primary sources from the client's outbound allowlist

Each article follows the SOP in [README.md](../README.md): H1 (auto from title) + 3 H2s + 6 H3s, ≥800 words, ≥5 internal links, ≥3 outbound links, hero image with keyword-bearing filename and specific alt text.

The first 5 articles serve a strategic purpose: they form the **topical cluster** that anchors the site's authority. Pick adjacent topics, link them densely, cite shared sources — answer engines reward consistent topical depth.

#### Asset generation (replaces 2d)

For greenfield articles, hero images come from one of:

- **Brand-supplied stock** — preferred (matches brand visual language)
- **Stock library** (Unsplash, Pexels, brand's licensed library) — confirm licensing, name keyword-bearingly
- **AI generation** — last resort, requires brand approval (some industries / brands forbid it)

Save at `assets/articles/<slug>/<keyword-bearing-filename>.<ext>` — same path convention as migration. Build picks them up automatically.

---

## Phase 3 — Project scaffold

> **The canonical template is THIS repository.** Fork or clone the `hfga-rust` repo as the seed for any new site. If you can't access it, [template-spec.md](template-spec.md) has the Cargo.toml + project layout + page skeleton + design system reference — enough to recreate the seed by hand, but cloning is dramatically faster.

> **CRITICAL: reset client-specific files BEFORE you write a single line of new code.** The template ships with the previous client's articles, hero images, brand assets, page components, and copy in place. If you skip the reset and start editing, you'll burn 20+ minutes hand-deleting old content + risk shipping mixed-brand artifacts (e.g. a hemp-fiber article showing up on a carbon-tracking site). The reset is **non-negotiable** — even if the previous client is "kind of similar" to the new one.

### 3a. Clone + remote rewire

```bash
# Clone (or rsync from a local checkout if you have one)
git clone <hfga-rust> <client>-rust
cd <client>-rust
git remote remove origin
git remote add origin <new-client-repo>
```

### 3b. Reset client-specific files (do this FIRST, before any per-client work)

```bash
# DELETE — these contain previous-client content and MUST go before any edits
rm -rf content/articles/*.md          # markdown article bodies
rm -rf assets/articles/*              # per-article hero images
rm -rf assets/brand/*                 # logos, favicons, PWA icons
rm -f  README.md                      # we regenerate from build_readme.md
rm -f  docs/discovery-*.yml           # previous client's discovery doc
rm -f  docs/slug-map-*.csv            # previous client's slug map
rm -f  docs/trackers-*.md             # previous client's tracker inventory
rm -rf docs/source/*.md               # previous client's transcriptions

# DELETE the previous client's page components (we'll write fresh ones in Phase 4)
# Keep: src/pages/{landing,article,mod}.rs (universal scaffolds we rewrite in place)
ls src/pages/*.rs
# Inspect the list, then rm -f every page that's client-specific (typically
# everything except landing.rs / article.rs / mod.rs).
# Don't forget to also delete the corresponding `pub mod <name>;` lines in
# src/pages/mod.rs — leaving them in causes a compile error before you've
# written the replacement files.

# OPTIONALLY delete (only if the new client doesn't use these capabilities):
#   - src/components/popup.rs        — Klaviyo email-capture form
#   - src/components/pricing_board.rs — commodity-pricing board widget
#   If you delete either, also remove the corresponding `pub mod <name>;`
#   line from src/components/mod.rs.

# Verify nothing client-specific remains in the source tree:
grep -rinE "<previous-client-slug>|<previous-brand-name>|<previous-brand-hex>" \
  src/ scripts/ tailwind.css customHttp.yml amplify.yml Cargo.toml \
  generate_icons.py 2>/dev/null
# (empty output = clean reset)
```

**Why this matters:** the next steps (regenerate icons, wire up tracking, write pages) all overwrite specific files in place. If a previous client's `src/pages/buy_seed.rs` is still in the tree, it'll keep getting prerendered into your build until you notice it weeks later in production. The reset takes 30 seconds; the cleanup-after-the-fact takes hours.

### 3c. Keep (these are universal infrastructure, not per-client)

- `Cargo.toml`, `build.rs`, `tailwind.css` (edit color tokens; structure stays)
- `src/components/{layout,markdown,news_carousel,stat_counters}.rs` (universally reusable — adjust labels)
- `src/pages/{landing,article,mod}.rs` (rewrite content; structure stays)
- `src/{main,seo,content,tracking}.rs` (find/replace edits in Phase 4)
- `scripts/*.sh`, `scripts/*.py` (universal — but client-specific paths get find/replaced in Phase 4)
- `customHttp.yml`, `amplify.yml`, `infra/security-headers.md`
- `.github/`, `docs/AMPLIFY.md`, `docs/template-spec.md`, `docs/replicate.md`, `docs/build_readme.md`, `docs/initial_prompt.md`

---

## Phase 4 — Per-client customization (find/replace map)

Every per-client variable, where it lives. Keep this list exhaustive — anything missed here is a runtime bug or a brand mismatch you'll discover days later.

> **The trio of route lists must stay synced.** Three files independently enumerate the static routes — if you add a route to the `Route` enum but forget one of the three, that route either won't ship in the sitemap, won't get prerendered, or won't appear in `dx --ssg` discovery. Always check all three at once when adding/renaming/removing a route:
>
> 1. **`#[server(endpoint = "static_routes")]`** in [src/main.rs](../src/main.rs) — used by `dx --ssg` (currently no-op in 0.7.7 but kept for future).
> 2. **`STATIC_ROUTES`** + **`PRIORITIES`** + **`CHANGEFREQS`** arrays in [scripts/generate-sitemap.sh](../scripts/generate-sitemap.sh) — feeds `sitemap.xml`.
> 3. **`ROUTES`** array in [scripts/prerender.sh](../scripts/prerender.sh) — what actually gets HTTP-walked + saved as `index.html`.
>
> Article slugs are auto-discovered from `content/articles/*.md` in (2) and (3), but every static route needs to be added to all three by hand. There's no compile-time check linking them — only the [Phase 7](#phase-7--verify-curl-checks) curl loop catches a missed entry.

| Variable | File(s) | Notes |
| --- | --- | --- |
| **Site name (long)** | [src/main.rs](../src/main.rs) `SITE_NAME`, [seo.rs](../src/seo.rs) JSON-LD, [generate-aeo.py](../scripts/generate-aeo.py) | Used in `<title>` suffix and structured data |
| **Site base URL** | [src/main.rs](../src/main.rs) `SITE_BASE`, [scripts/generate-sitemap.sh](../scripts/generate-sitemap.sh) `BASE`, [scripts/generate-aeo.py](../scripts/generate-aeo.py) `BASE_URL` | `https://example.com` (no trailing slash) |
| **Brand primary color** | [tailwind.css](../tailwind.css) `--color-red-{6..12}`, `--color-red-12` (deepest), [generate_icons.py](../generate_icons.py) `BRAND_RED` | Sweep all references to the OLD hex with `grep -rn` to be sure |
| **Logo wordmark text** | [generate_icons.py](../generate_icons.py) `render_wordmark()` line 1 + line 2 strings | Run `python3 generate_icons.py` after editing |
| **Favicon mark** | [generate_icons.py](../generate_icons.py) `render_icon()` `rows = ["HF", "GA"]` | Two-line stacked monogram |
| **Body + display font** | [tailwind.css](../tailwind.css) `--font-sans`, `--font-display`, `@font-face` declarations, [scripts/download-fonts.sh](../scripts/download-fonts.sh) | Default Inter; override only if brand demands |
| **Klaviyo IDs** | [src/tracking.rs](../src/tracking.rs) `KLAVIYO_COMPANY_ID`, `KLAVIYO_FORM_ID` | Pull from old site HTML if migrating |
| **GA4 measurement ID** | [src/tracking.rs](../src/tracking.rs) `GA4_MEASUREMENT_ID` | `G-XXXXXXXXXX` from `analytics.google.com` |
| **Stat counters (hero numbers)** | [src/components/stat_counters.rs](../src/components/stat_counters.rs) `default_stats()`, `buy_seed_2023()`, `sell_stats()`, `exports_stats()` | Per-page stat overrides each have their own constructor |
| **Routable enum routes** | [src/main.rs](../src/main.rs) `Route` derive | Slugs MUST match the old site verbatim |
| **Header nav labels + dropdown items** | [src/components/layout.rs](../src/components/layout.rs) `NavDropdown`, `Header` body, `MobileMenu` | Change link text; keep route enum consistent |
| **Footer columns** | [src/components/layout.rs](../src/components/layout.rs) `Footer` `FooterColumn { title: ..., items: vec![...] }` | Three columns: About, Mid, Right |
| **Social links** | [src/components/layout.rs](../src/components/layout.rs) `Footer` `<a href="...">` for Facebook + LinkedIn | Update href; keep aria-label |
| **Per-page hero text** | [src/pages/landing.rs](../src/pages/landing.rs), [src/pages/planting_spring.rs](../src/pages/planting_spring.rs), etc. | Each page is its own component |
| **AEO pitch + curated page list** | [scripts/generate-aeo.py](../scripts/generate-aeo.py) `PITCH`, `CORE_PAGES` | Tight (<400 chars) elevator pitch |
| **Sitemap URL list** | [scripts/generate-sitemap.sh](../scripts/generate-sitemap.sh) `STATIC_ROUTES`, `PRIORITIES`, `CHANGEFREQS` | Mirror the `Route` enum |
| **Prerender URL list** | [scripts/prerender.sh](../scripts/prerender.sh) `ROUTES` array | Mirror the `Route` enum (article slugs auto-enumerated from `content/articles/`) |
| **Static_routes server fn** | [src/main.rs](../src/main.rs) `#[server(endpoint = "static_routes")]` | Mirror the `Route` enum (used by dx --ssg if it ever works) |
| **CSP allowlist** | [customHttp.yml](../customHttp.yml) `Content-Security-Policy` header value | Add hosts for any new tracker/CDN |
| **Robots / sitemap host** | [scripts/generate-sitemap.sh](../scripts/generate-sitemap.sh) `BASE` | Same `https://...` as SITE_BASE |
| **Article author default** | [src/content.rs](../src/content.rs) `default_author()` | Returns `"HFGA"` — change to client default |
| **Article SOP brand voice** | [README.md](../README.md) under "SOP TO WRITE ARTICLES" | Three primitives, voice rules, outbound source allowlist |
| **JSON-LD Organization** | [src/seo.rs](../src/seo.rs) `organization_jsonld()` | `name`, `url`, `logo`, `sameAs`, `description` |
| **Klaviyo CSP form-action** | [customHttp.yml](../customHttp.yml) `form-action` directive | Klaviyo posts to `manage.kmail-lists.com` |
| **Email contact** | [src/pages/faq.rs](../src/pages/faq.rs) `mailto:` link, FAQ answer | `hello@<client>.com` |

When the find/replace pass is done, run:

```bash
# Sanity grep for anything we missed
grep -rn "hfga" src/ scripts/ docs/ 2>/dev/null | grep -vE 'old|hfga-' | head -30
grep -rn "Hemp Fiber and Grain" src/ scripts/ docs/ 2>/dev/null | head -30
grep -rn "ad2929" src/ scripts/ tailwind.css 2>/dev/null | head -10
```

Anything that comes back is something you forgot to update.

---

## Phase 5 — Build pipeline gotchas (lessons)

These are the bugs we hit on HFGA. Every one cost real time. Pre-applying these fixes saves the next migration hours.

### 5a. dx 0.7.7 does not compile Tailwind v4

**Symptom:** Site renders with NO styles. `tailwind*.css` in `public/assets/` contains literal `@import "tailwindcss"` and `@utility` directives — browsers ignore them.

**Why:** dx 0.7.7's "auto-Tailwind" claim from the release notes is currently inert; the CSS is fingerprinted and shipped raw.

**Fix:** [scripts/build-tailwind.sh](../scripts/build-tailwind.sh) compiles separately. Prefers a local `tailwindcss` binary; falls back to downloading the platform-specific standalone from `https://github.com/tailwindlabs/tailwindcss/releases/download/v${VERSION}/tailwindcss-${os}-${arch}` and caching under `$HOME/.cache/hfga-tailwindcss/`.

**Do NOT** use `npx @tailwindcss/cli`. v4's CLI needs to resolve the `tailwindcss` package against the project root for `@import "tailwindcss"`, and npx-installed packages aren't always findable. We hit `Error: Can't resolve 'tailwindcss' in '<project root>'` on Amplify CI.

### 5b. dx 0.7.7's `--ssg` flag silently skips prerender

**Symptom:** `dx bundle --platform web --ssg` produces a single root `index.html`; sub-routes have no prerendered HTML.

**Why:** dx tries to invoke the `#[server(endpoint = "static_routes")]` function but the resolution against our route layout doesn't fire. It builds the server binary but never executes it.

**Fix:** [scripts/prerender.sh](../scripts/prerender.sh). Starts the dx-built server binary in the background, walks every route via `curl`, saves the response HTML at `<route>/index.html`. Deterministic, fully observable.

```bash
# In build-ssg.sh:
dx bundle --platform web --release       # NOTE: no --ssg flag
./scripts/prerender.sh "$SERVER_BIN" "$OUT"
```

### 5c. WASM hydration requires `'wasm-unsafe-eval'` in CSP

**Symptom:** Mobile hamburger doesn't open. Any `onclick` handler in Dioxus is dead. SSG content renders fine, all interactivity broken. Console shows: `Refused to compile or instantiate WebAssembly module because 'wasm-unsafe-eval' is not an allowed source of script in the following Content Security Policy directive`.

**Fix:** [customHttp.yml](../customHttp.yml) `script-src` must include `'wasm-unsafe-eval'`. Desktop nav uses plain `<a>` so the bug hides on desktop and only manifests on mobile (the only place that uses `onclick`).

### 5d. Amplify rewrite rule needs `.md` allowlisted for AEO surfaces

**Symptom:** `curl -sI https://<site>/llms.txt` returns 200 (good — `.txt` is in the default allowlist). `curl -sI https://<site>/en/news/<slug>.md` returns 200 but the body is `<!DOCTYPE html>` (the SPA shell).

**Why:** Amplify's default rewrite rule `</^[^.]+$|\.(?!(css|gif|ico|jpg|js|png|txt|svg|woff|woff2|ttf|map|json|xml|webmanifest)$)([^.]+$)/>` rewrites any path with an extension NOT in the allowlist. `.md` isn't in the default list.

**Fix:** Update the regex to add `md`: `…(?!(...|webmanifest|md)$)…`. Documented in [docs/AMPLIFY.md](AMPLIFY.md).

### 5e. `mktemp -t name` is BSD-only — fails on Linux CI

**Symptom:** Amplify build dies with `mktemp: too few X's in template 'hfga-tailwind'`.

**Fix:** Use the portable form: `mktemp "${TMPDIR:-/tmp}/<name>.XXXXXX"`. macOS BSD mktemp accepts it; GNU mktemp on Linux requires it.

### 5f. PATH is stripped inside subshells / functions

**Symptom:** Inside a `Bash` tool call, `curl: command not found` despite curl being on PATH.

**Fix:** First line of every script: `export PATH="/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:$PATH"`. Belt-and-suspenders for any harness that strips PATH.

### 5g. Article slugs must match live site EXACTLY (typos and all)

**Why:** Inbound links from search engines, social shares, and external citations all reference the existing slug. Any change is a 404 with associated SEO damage.

**Real example from HFGA:** the slug `corn-vs-hemp-fiber-economics-hemp-fiber-associatio` (missing trailing `n`) was preserved verbatim, with the typo, because that's what Google had indexed.

### 5h. Klaviyo form embed pattern (only if the source site uses Klaviyo)

The template ships a Klaviyo embed in [src/components/popup.rs](../src/components/popup.rs) and corresponding CSP allowlist entries because the seed client (HFGA) used it. **Most clients don't.** If the Phase 1c scan turned up no Klaviyo company-ID, take this path:

1. `rm src/components/popup.rs`
2. Remove `pub mod popup;` from [src/components/mod.rs](../src/components/mod.rs)
3. Remove the `KlaviyoForm` import + render call from any page that referenced it (typically `landing.rs`).
4. Remove `KLAVIYO_COMPANY_ID` + `KLAVIYO_FORM_ID` constants from [src/tracking.rs](../src/tracking.rs).
5. Edit [customHttp.yml](../customHttp.yml) CSP to drop `static.klaviyo.com`, `static-tracking.klaviyo.com`, `a.klaviyo.com`, `manage.kmail-lists.com`, and remove `https://manage.kmail-lists.com` from `form-action`.

If the source site DOES use Klaviyo, the HTML pattern to look for is:

```html
<script async src="https://static.klaviyo.com/onsite/js/klaviyo.js?company_id=ABCDEF"></script>
<div class="klaviyo-form-XYZ123">&nbsp;</div>
```

Capture both IDs:
- `company_id` → `tracking.rs` `KLAVIYO_COMPANY_ID`
- The class suffix after `klaviyo-form-` → `tracking.rs` `KLAVIYO_FORM_ID`

The `popup.rs` component renders `<div class="klaviyo-form-{KLAVIYO_FORM_ID}">` — Klaviyo's onsite JS finds the div and injects the form on first visit.

### 5i. Font portability — prefer Inter

We started HFGA with Kumbh Sans + Montserrat (matching the live HubSpot site). The Google Fonts CSS endpoint returned `.woff` for our user agent, not `.woff2` — `download-fonts.sh` couldn't find the right URLs. Switched to Inter via `cdn.jsdelivr.net/npm/@fontsource/inter` and it just worked.

Default to Inter for any new site unless the brand insists otherwise. One family covers display + body + numerics. Reliable CDN. Self-hosting fits in 5 woff2 files (~22KB each).

### 5j. Hero bg behind a fixed transparent header

**Pattern (in [tailwind.css](../tailwind.css)):**
- `.site-header` is `position: fixed`, transparent at top, `data-scrolled` toggles solid-with-glass via JS
- `main` has `padding-top: var(--header-height)`
- `.bg-mesh-hero` has `margin-top: calc(-1 * var(--header-height))` and `padding-top: var(--header-height)` — pulls hero up to viewport top, header overlays the mesh transparently

This is the cleanest way to get a "header floats over hero" look without z-index gymnastics.

### 5k. Mobile-only CSS line breaks

To break a heading onto two lines on desktop only:

```rust
h1 {
    "First half"
    br { class: "hidden md:inline" }
    " second half"
}
```

`<br>` with `display: none` is removed from rendering; with `display: inline` it forces a break. Mobile reads as one flowing block, desktop reads as two visual lines.

### 5l. H1 + `<br>` text concatenation (SEO/AEO trap)

When an H1 splits across multiple `<br>` boundaries — common for hero headlines that fan over 2–3 visual lines — make sure **every text node ends OR begins with a space character**. HTML parsers (which is what Google's crawler, AEO scrapers, and our `Phase 7` H1 audit all use to extract page text) ignore `<br>` for whitespace purposes. So:

```rust
// BAD — renders fine visually, but the extracted SEO/H1 text reads
//       "Built Transparently. Designed Affordably.Open To Everyone."
//       with no space between sentences across <br>.
h1 {
    "Built Transparently."
    br {}
    " Designed Affordably."
    br {}
    span { "Open To Everyone." }                  // ← no leading space → jam
}

// GOOD — same visual rendering, clean SEO/AEO text extraction.
h1 {
    "Built Transparently."
    br {}
    " Designed Affordably. "                      // ← trailing space
    br {}
    span { "Open To Everyone." }
}
```

The `<br>` only triggers a line break in CSS-aware renderers; for purposes of meta description, OG description, llms-full.txt, and any tool that strips tags before measuring text, the result is the same as if `<br>` weren't there. Catch this in the H1 audit (Phase 7).

### 5m. Amplify CI Python image lacks `tomllib` (3.11+) AND `tomli` (pip)

**Symptom:** Amplify build runs `dx bundle` + `prerender.sh` + `build-tailwind.sh` cleanly, then fails on `generate-aeo.py` with:

```
ModuleNotFoundError: No module named 'tomllib'
…
ModuleNotFoundError: No module named 'tomli'
!!! Build failed
```

**Why:** `tomllib` is Python 3.11+ stdlib; Amplify's Linux image (Amazon Linux 2023) ships **Python 3.10** as `python3`. The script's import-fallback chain `tomllib → tomli` fails because `tomli` isn't preinstalled either, and adding `pip install tomli` to `amplify.yml` is fragile (pip3-vs-python3 path mismatches, `--user` site-packages discovery, `dnf` permissions, etc.).

**Fix shipped in [scripts/generate-aeo.py](../scripts/generate-aeo.py):** an inline pure-Python TOML parser scoped to the article-front-matter shape we actually use (string keys, ISO-date strings, string arrays). Three-tier loader:

1. `tomllib` (Python 3.11+ stdlib) — used on dev machines and any CI with modern Python.
2. `tomli` (pip) — used if a future Amplify image ships it.
3. Inline parser — production CI fallback, **zero dependencies**, no pip install required.

If you ever expand the front-matter spec to use multi-line strings, integers, booleans, tables, or nested arrays, the inline parser will raise `ValueError: inline-toml: cannot parse line` — at which point either install `tomli` via pip OR extend the inline parser. Don't silently catch the error; the CI failure is a feature.

---

## Phase 6 — Deploy & DNS

### 6a. Amplify (current default)

1. Connect the GitHub repo
2. **Force the build spec** to use the repo's [amplify.yml](../amplify.yml) (Amplify auto-detects sometimes wrongly)
3. **Add the rewrite rule** with `.md` in the allowlist (see Phase 5d) — one console step, manual
4. [customHttp.yml](../customHttp.yml) is picked up automatically — no console step

### 6b. Custom domain (GoDaddy → CloudFront)

GoDaddy doesn't support ANAME / ALIAS records. Two options:

**A. Stay on GoDaddy (simplest):**
- CNAME `www` → `<distribution>.cloudfront.net`
- Forward apex (`@`) → `https://www.<domain>` via GoDaddy's Forwarding feature (NOT "with masking" — breaks SSL). 301 permanent.
- In Amplify, remove `@` from the subdomain list so it stops waiting for verification.

**B. Switch DNS to Route 53 (cleaner):**
- Create hosted zone in Route 53
- GoDaddy → Nameservers → Custom → paste the 4 NS values from Route 53
- Route 53 → A record (Alias = Yes) on apex AND `www`, both pointing at the CloudFront distribution
- Amplify SSL issues for both apex and www

Route 53 is the right answer for new client engagements. GoDaddy forwarding is the right answer when the client wants minimal DNS changes.

### 6c. SSL

Amplify auto-issues via ACM as soon as DNS verification passes. Wait 15–60 min after DNS records propagate.

---

## Phase 7 — Verify (curl checks)

Run these against the deployed site after DNS + SSL are green. **All must pass before declaring done.**

```bash
SITE="https://<client-domain>"

# 1. SSG worked — every route returns 200 with substantive HTML
for path in "/" "/page-1" "/page-2" "/blog" "/blog/post-1"; do
  size=$(curl -s -o /dev/null -w "%{http_code} %{size_download}" "${SITE}${path}/")
  printf "  %-40s  %s\n" "$path" "$size"
done
# Each should be 200 with ≥18KB body

# 2. Tailwind compiled (not raw)
curl -s "${SITE}/assets/tailwind-*.css" | head -c 200
# Should start with /*! tailwindcss v4 */ — NOT @import "tailwindcss"
curl -s "${SITE}/assets/tailwind-*.css" | grep -c '\.flex'
# Should be ≥1

# 3. Sitemap + robots
curl -sSI "${SITE}/sitemap.xml" | head -3   # 200, application/xml
curl -sSI "${SITE}/robots.txt"  | head -3   # 200, text/plain

# 4. AEO surfaces
curl -sSI "${SITE}/llms.txt"      | head -3 # 200, Content-Type: text/plain; charset=utf-8
curl -sSI "${SITE}/llms-full.txt" | head -3 # 200, Content-Type: text/plain; charset=utf-8
curl -sSI "${SITE}/blog/post-1.md" | head -3 # 200, Content-Type: text/markdown; charset=utf-8
curl -s   "${SITE}/llms.txt" | head -8       # spec shape: # H1, > pitch, ## Core

# 5. SEO meta tags + JSON-LD
curl -s "${SITE}/" | grep -oE '<title>[^<]+</title>'
curl -s "${SITE}/" | grep -oE '<meta name="description"[^>]+>'
curl -s "${SITE}/" | grep -oE '<link rel="canonical"[^>]+>'
curl -s "${SITE}/" | grep -c '"@type":"Organization"'

# 6. Article HTML has alternate Markdown link (AEO discovery)
curl -s "${SITE}/blog/post-1/" | grep -oE 'rel="alternate"[^>]+type="text/markdown"[^>]*'

# 7. Security headers
curl -sI "${SITE}/" | grep -E "(strict-transport-security|x-frame-options|content-security-policy)" | head -5

# 8. CSP allows wasm-unsafe-eval (mobile menu / any onclick depends on this)
curl -sI "${SITE}/" | grep -oE "wasm-unsafe-eval" | head -1

# 9. GA4 + tag manager visible in head
curl -s "${SITE}/" | grep -oE 'gtag/js\?id=G-[A-Z0-9]+' | head -1
```

### Mobile verification (manual)

Open the deployed site on a real phone (or Chrome DevTools mobile preview):

1. Tap hamburger → menu drops down with all links ✓
2. Email/lead-capture form renders mid-page (Klaviyo / Calendly / etc. — only if the client uses one) ✓
3. Brand logo readable ✓
4. Hero CTAs are tappable (≥44px tap target) ✓

If the hamburger does nothing, CSP is blocking WASM (Phase 5c).

### Phase 7.5 — Post-build audit (run before declaring done)

Three structured scans that catch the most common migration mistakes — each one a 30-second one-liner. Run them locally against `http://localhost:3000` (production preview) AFTER all the curl checks above pass.

#### Audit A: Sitemap parity vs. the live source site

```bash
SITE_OLD="https://<previous-cms-domain>"
SITE_NEW="http://localhost:3000"

# Extract every URL from the OLD site (handle Yoast multi-sitemap)
{
  curl -sSL "$SITE_OLD/post-sitemap.xml"
  curl -sSL "$SITE_OLD/page-sitemap.xml"
  curl -sSL "$SITE_OLD/sitemap.xml"
} | grep -oE '<loc>[^<]+</loc>' \
  | sed 's|<loc>||; s|</loc>||' \
  | sed -E "s|^https?://[^/]+||; s|/$||" \
  | sort -u > /tmp/old-urls.txt

# Extract every URL from the NEW site
curl -sSL "$SITE_NEW/sitemap.xml" \
  | grep -oE '<loc>[^<]+</loc>' \
  | sed 's|<loc>||; s|</loc>||' \
  | sed -E "s|^https?://[^/]+||; s|/$||" \
  | sort -u > /tmp/new-urls.txt

echo "--- ON OLD BUT NOT NEW (potential gaps) ---"
comm -23 /tmp/old-urls.txt /tmp/new-urls.txt

echo "--- ON NEW BUT NOT OLD (added pages) ---"
comm -13 /tmp/old-urls.txt /tmp/new-urls.txt
```

Every URL in the "ON OLD BUT NOT NEW" list must be either (a) explicitly in your "pages to skip" list from Phase 1b — document why in `docs/discovery-<client>.yml` — or (b) a bug. Trailing-slash differences (`/about-us/` vs `/about-us`) are fine; CloudFront and Amplify serve both forms.

#### Audit B: Tracker inventory matches what we shipped

```bash
SITE_OLD="https://<previous-cms-domain>"

echo "--- All third-party scripts on the LIVE site ---"
curl -sSL "$SITE_OLD" | grep -oE '<script[^>]+src="[^"]+"' \
  | grep -oE 'https?://[^"]+' | sort -u

echo "--- All inline tracker patterns on the LIVE site ---"
curl -sSL "$SITE_OLD" | grep -oiE \
  '(gtag|fbq|_hsq|_paq|clarity|hjEvent|intercomSettings|driftt|crisp|heap|smartlook|tealium|onetrust|cookiebot|chmln|pendo|amplitude|klaviyo)\(?'  \
  | sort -u

echo "--- All tracker IDs in the LIVE HTML ---"
curl -sSL "$SITE_OLD" | grep -oE '(G-[A-Z0-9]+|GTM-[A-Z0-9]+|UA-[0-9]+-[0-9]+)' \
  | sort -u
```

Every tracker in the output should match exactly what's in our `src/tracking.rs` — no missing trackers (we'd lose attribution / conversions) and no orphan trackers we never had (would be a security smell + CSP violation).

#### Audit C: H1 sanity

```bash
cd target/dx/<client>-website/release/web/public
# Every page should have exactly 1 H1 with full descriptive text.
python3 << 'PY'
import os
from html.parser import HTMLParser

class H1Grab(HTMLParser):
    def __init__(self):
        super().__init__()
        self.in_h1 = False
        self.bufs = []
        self.cur = []
    def handle_starttag(self, tag, attrs):
        if tag == 'h1': self.in_h1 = True; self.cur = []
    def handle_endtag(self, tag):
        if tag == 'h1' and self.in_h1:
            self.bufs.append(' '.join(''.join(self.cur).split()))
            self.in_h1 = False
    def handle_data(self, data):
        if self.in_h1: self.cur.append(data)

paths = sorted(p for r,_,fs in os.walk('.') for p in (os.path.join(r,f) for f in fs) if p.endswith('index.html'))
for p in paths:
    h = H1Grab(); h.feed(open(p, encoding='utf-8').read())
    route = p.removeprefix('./').removesuffix('index.html').rstrip('/') or '/'
    print(f"  H1={len(h.bufs):>1}  {route:<70}  {(h.bufs[0] if h.bufs else '(NO H1)')[:80]}")
PY
```

Look for: count != 1, empty H1 text, or text where two sentences run together (e.g. `"Affordably.Open To Everyone."` — see Phase 5l for the `<br>` spacing fix).

---

## Per-client customization checklist

Print and check off, one per client engagement:

```
RESET (before any per-client edits)
[ ] rm -rf content/articles/*.md
[ ] rm -rf assets/articles/*
[ ] rm -rf assets/brand/*
[ ] rm -f  README.md
[ ] rm -f  docs/{discovery,trackers}-*.* + docs/slug-map-*.csv + docs/source/*.md
[ ] rm previous-client page components from src/pages/ + remove from src/pages/mod.rs
[ ] rm src/components/popup.rs        if client doesn't use Klaviyo (and remove from mod.rs)
[ ] rm src/components/pricing_board.rs if client doesn't have a commodity-pricing widget
[ ] grep -rinE "<previous-client>" src/ scripts/ tailwind.css customHttp.yml — should be empty

BRAND
[ ] SITE_NAME / SITE_BASE in main.rs
[ ] Brand color in tailwind.css (--color-red-{6..12} + red-12 deepest)
[ ] Brand color in generate_icons.py BRAND_RED (or whatever you renamed it)
[ ] Real brand logo extracted from live site → assets/brand/<client>-logo-{light,dark}.png
[ ] Wordmark generation DISABLED in generate_icons.py (so a future run doesn't overwrite the real logo)
[ ] Favicon mark text in generate_icons.py rows = [...]  (or use SVG of real brand mark)
[ ] Run python3 generate_icons.py
[ ] Run ./scripts/download-fonts.sh (uses Inter by default — only override if brand demands)

TRACKING
[ ] GA4 ID in tracking.rs (or remove TrackingHead body if launching without analytics)
[ ] Klaviyo IDs in tracking.rs only if scan turned them up (else remove popup.rs)
[ ] CSP script-src + connect-src + frame-src in customHttp.yml — match the actual tracker list
[ ] CSP form-action — match the actual form provider (Klaviyo, Formspree, etc.)
[ ] Remove unused tracker hosts from CSP (security hygiene — narrower CSP is better)

PAGES + ROUTING
[ ] Stat counter values in stat_counters.rs (default_stats() at minimum; per-page overrides as needed)
[ ] Header nav labels in layout.rs Header + Dropdowns + MobileMenu
[ ] Footer columns + social links in layout.rs Footer
[ ] Per-page hero copy in pages/*.rs
[ ] Routable enum in main.rs matches all source URLs (Phase 1b sitemap output)
[ ] static_routes server fn in main.rs                     ┐
[ ] STATIC_ROUTES + PRIORITIES + CHANGEFREQS in            ├ all three must match
    scripts/generate-sitemap.sh                            │
[ ] ROUTES in scripts/prerender.sh                         ┘
[ ] PITCH + CORE_PAGES in scripts/generate-aeo.py
[ ] organization_jsonld() in seo.rs (name, url, logo path, sameAs, description)
[ ] Article SOP voice rules in README.md (3 primitives, outbound link allowlist)
[ ] Contact email + address — search for "hello@hfga" / "Newlab, Detroit" placeholder (whatever the seed had)

CONTENT
[ ] Migrated articles in content/articles/<slug>.md (slugs match old site EXACTLY, including typos)
[ ] Migrated hero images in assets/articles/<slug>/<keyword-bearing-name>
[ ] Every hero image is `file`-confirmed PNG/JPEG, not silent-404 HTML

LOCAL VERIFICATION
[ ] cargo check --features web --target wasm32-unknown-unknown (zero warnings)
[ ] cargo check --features server                              (zero warnings)
[ ] ./scripts/build-ssg.sh                                     (clean build, ALL routes 200)
[ ] All Phase 7 curl checks pass on local preview (port 3000)
[ ] Phase 7.5 audit A: sitemap parity (only deliberate-skip URLs in the gap list)
[ ] Phase 7.5 audit B: tracker inventory matches src/tracking.rs (no missing, no orphans)
[ ] Phase 7.5 audit C: every page has exactly 1 H1 with descriptive text + clean spacing

DEPLOY
[ ] Deploy to Amplify with rewrite rule (md allowlisted in the SPA-rewrite regex)
[ ] DNS records set (apex + www) — see Phase 6
[ ] Amplify SSL green for both apex + www
[ ] Phase 7 + 7.5 audits pass against production URL
[ ] Manual mobile test (hamburger opens, hero CTAs tappable ≥44px)
[ ] GSC verification + sitemap submitted
[ ] GA4 receiving pageviews (check real-time view)
```

---

## Migration provenance

For every client migration, save the following alongside the build for the audit trail. These also become the troubleshooting ammo if any URL stops working post-launch.

```
docs/
├── discovery-<client>.yml          # Phase 1a output — brand vars, fonts, colors
├── source-urls-<client>.txt        # Phase 1b output — every URL from old site
├── trackers-<client>.md            # Phase 1c output — kept vs dropped, with IDs
├── source/<slug>.md                # Phase 2a/2b — verbatim transcriptions
├── slug-map-<client>.csv           # Phase 2c — old URL → new slug, hero filename map
└── replicate-notes-<client>.md     # any client-specific gotchas (font swaps, slug typos preserved, etc.)
```

The discovery + source files protect future-you. If the client says "the old site had X — where is it now?" you can grep the source files in seconds.

---

## Quick-start command sequence

For an experienced operator who's done this before, the literal command sequence to go from `git clone` to "site loads in a browser":

```bash
# 1. Scaffold
git clone <hfga-rust> <client>-rust && cd <client>-rust
rm -rf content/articles/*.md assets/articles/* assets/brand/{hfga-logo*,favicon*,icon-*,apple-touch-*}

# 2. Per-client edits (use the find/replace map above)
$EDITOR src/main.rs tailwind.css generate_icons.py src/tracking.rs \
        src/components/layout.rs src/components/stat_counters.rs \
        scripts/generate-aeo.py scripts/generate-sitemap.sh scripts/prerender.sh \
        customHttp.yml src/seo.rs src/content.rs

# 3. Brand assets
python3 generate_icons.py
./scripts/download-fonts.sh

# 4. Migrate content
$EDITOR content/articles/<slug-1>.md  # paste verbatim transcription with TOML front matter
# (repeat per article)

# 5. Article hero images
mkdir -p assets/articles/<slug-1>
curl -sSL -o assets/articles/<slug-1>/<keyword-bearing-name>.png "https://<old-site>/.../hero.png"
# (repeat per article)

# 6. Verify locally
cargo check --features web --target wasm32-unknown-unknown
./scripts/build-ssg.sh
cd target/dx/<client>-website/release/web/public && python3 -m http.server 3000

# 7. Phase 7 curl checks against http://localhost:3000

# 8. git add / commit / push → Amplify builds and deploys
```
## Mandatory SEO and Website Validation

### SEO/AEO surfaces shipped by default

Every site shipped from this playbook gets these surfaces automatically — no per-client wiring beyond the brand copy. Knowing they exist saves time when a third-party SEO auditor (SEOptimer, ahrefs, etc.) flags a "missing" item that's actually live.

| Surface | Status | Generated by | Notes |
|---|---|---|---|
| `sitemap.xml` | ✓ deployed | `scripts/build-ssg.sh` step 5 | All static routes + every article |
| `robots.txt` | ✓ deployed | `scripts/build-ssg.sh` step 5 | Allow-all + sitemap pointer |
| `ads.txt` | ✓ deployed | `scripts/build-ssg.sh` step 5 | **Preset to "no sellers"** — comment-only file declaring no third party may sell ads on this domain. Prevents ad-fraud spoofing. To opt in to programmatic ads later, replace the heredoc body in the build script with IAB v1.1 lines (`<ssp>, <pub-id>, <relationship>, <tag>`). Most clients should leave as-is. |
| `/404.html` (+ `/404/index.html`) | ✓ deployed | `src/pages/not_found.rs` + `scripts/build-ssg.sh` | Branded "fun" custom 404 page with brand-voice copy, two CTAs (home + browse articles). Served with **HTTP 404** via the Amplify fallback rule (see [docs/AMPLIFY.md](AMPLIFY.md) step 2a). Real 404 vs. soft-404 redirect to `/` is better SEO and what auditors check for. |
| `llms.txt` + `llms-full.txt` | ✓ deployed | `scripts/build-ssg.sh` step 7 | Curated index + full content per llmstxt.org for AEO |
| `/learn/<slug>.md` | ✓ deployed | `scripts/build-ssg.sh` step 6 | Clean Markdown sibling of every article HTML page |
| **WebP-only image pipeline** | ✓ deployed | `scripts/generate-webp.sh` + `scripts/wrap-webp.py` (post-build) | WebP is the **canonical** image format on disk — no PNG/JPEG fallbacks ship. After prerender, `wrap-webp.py` rewrites every `<img src=".../X.png\|.jpg\|.jpeg">` in the built HTML to `.webp` (so author markup can keep referencing source-format names in front matter). `/assets/brand/**` PNGs are exempt — favicons and JSON-LD `logo` need PNG for crawler / OS-icon consumers. See **WebP image optimization** subsection below. |
| **WebM-only video pipeline** | ✓ standard pattern | `scripts/generate-webm.sh` + `src/components/video_hero.rs` (when present) | WebM (VP9) is the **canonical** hero/background-video format — no MP4 fallback ships. The `VideoBackground` component emits a single `<source type="video/webm">`; the poster is a `.webp`. Modern browsers (Chrome / Firefox / Edge / Safari 14.1+) cover ~99% of traffic. See **WebM video optimization** subsection below. |
| `Article` + `Organization` JSON-LD | ✓ deployed | `src/seo.rs` | Structured citation metadata |
| Open Graph + Twitter Cards | ✓ deployed | per-page `document::Meta` + `src/seo.rs` constants | Full spec (`og:image` + width/height/alt + `og:site_name` + `og:locale` + matching `twitter:` set) on every page. See **Open Graph / Twitter Card spec** below. |
| Canonical URLs | ✓ deployed | `src/seo.rs::canonical()` | Absolute URLs, set per-page |
| Hardened security headers | ✓ deployed | `customHttp.yml` | CSP, HSTS preload-ready, COOP/CORP, Permissions-Policy, X-Frame-Options DENY |

#### Open Graph / Twitter Card spec

Third-party SEO checkers (SEOptimer, ahrefs, Sitechecker, Meta Sharing Debugger, X Card Validator) all look for the same set of `og:*` + `twitter:*` meta tags. Skipping any of them — especially the `og:image` triad — causes shares on iMessage, Slack, Twitter, LinkedIn, Facebook, and Discord to render a bare preview instead of a card.

**Tags every page must emit:**

| Tag | Value | Notes |
|---|---|---|
| `og:type` | `"website"` (or `"article"` on article pages) | Drives platform classification. |
| `og:site_name` | Constant (e.g. `"<Client> Network"`) | Sub-label under title on FB/LinkedIn. |
| `og:locale` | `"en_US"` (BCP-47 region tag) | Required for FB language detection. |
| `og:url` | Absolute canonical URL | From `seo::canonical(path)`. |
| `og:title` | Per-page | Can shorten vs. `<title>` if share-friendly. |
| `og:description` | Per-page | 1–2 sentences. |
| `og:image` | **Absolute** URL (`https://...`, never `/...`) | Required — auditors will not accept relative. |
| `og:image:width` | Numeric string matching file | Auditors validate without fetching the image. |
| `og:image:height` | Numeric string matching file | Same. |
| `og:image:alt` | Descriptive text | Accessibility + image SEO. |
| `twitter:card` | `"summary_large_image"` | Wide card with prominent image. |
| `twitter:title` | Matches `og:title` | X reads these separately from OG. |
| `twitter:description` | Matches `og:description` | Same. |
| `twitter:image` | Matches `og:image` | Same. |

Article pages additionally emit `article:published_time` and `twitter:image:alt` (the article hero's `hero_alt`).

**Recommended image dimensions:**

| Surface | Dimensions | Notes |
|---|---|---|
| Static-page default (`/assets/brand/icon-512.png`) | 512 × 512 | Stopgap until a designed 1200×630 OG card ships. |
| Article hero (site standard) | **1210 × 786** | Heartland's article heroes are produced at 1210×786 (1.54:1). All major platforms (Facebook, X, LinkedIn, iMessage, Discord, Slack) render this well. The 1.91:1 (1200×630) Facebook recommendation is a target, not a hard floor — 1210×786 is in the platform-accepted range. |

≤ 5 MB file size per platform docs; in practice we keep WebP article heroes under 200 KB.

**Implementation pattern** (centralized in `src/seo.rs`, exposed via `SeoProps`):

The `Seo` component owns every OG/Twitter tag. Per-page values flow in through props with sensible defaults — non-article pages pass only the few props they need (`title`, `description`, `path`); the rest fall back to the brand-icon defaults declared as `const` in the same file.

```rust
// src/seo.rs — current pattern
pub const DEFAULT_OG_IMAGE_PATH:   &str = "/assets/brand/icon-512.png";
pub const DEFAULT_OG_IMAGE_WIDTH:  &str = "512";
pub const DEFAULT_OG_IMAGE_HEIGHT: &str = "512";
pub const DEFAULT_OG_IMAGE_ALT:    &str = "<Brand> — <one-line tagline>";
pub const OG_LOCALE:               &str = "en_US";

#[derive(Props, Clone, PartialEq)]
pub struct SeoProps {
    pub title: String,
    pub description: String,
    pub path: String,
    #[props(default = String::new())]                          pub image: String,
    #[props(default = String::from(DEFAULT_OG_IMAGE_WIDTH))]   pub image_width: String,
    #[props(default = String::from(DEFAULT_OG_IMAGE_HEIGHT))]  pub image_height: String,
    #[props(default = String::from(DEFAULT_OG_IMAGE_ALT))]     pub image_alt: String,
    #[props(default = String::from("website"))]                pub og_type: String,
}
```

`og:site_name` is hardcoded to `SITE_NAME` (defined in `src/main.rs`) — no per-client constant; one source of truth.

Non-article pages just call `Seo` with the minimum:

```rust
rsx! {
    Seo {
        title: "Why Imperium",
        description: "Cost-reducing, carbon-negative drop-in material …",
        path: "/why-imperium",
    }
    // … page body
}
```

Article pages override the image, dimensions, alt, and og:type, and emit `article:published_time` alongside:

```rust
rsx! {
    Seo {
        title: "{seo_title}",
        description: "{seo_description}",
        path: "{url_path}",
        image: "{hero}",          // /assets/articles/<slug>/<hero>.webp
        image_width: "1210",
        image_height: "786",
        image_alt: "{hero_alt}",
        og_type: "article",
    }
    document::Meta { property: "article:published_time", content: "{published}" }
}
```

**Stopgap pattern (no designed OG card yet):** the defaults above already point at the 512×512 PWA icon. Every SEO checker passes immediately. When design ships a 1200×630 `og-card.png`, swap the three constants — every non-article page picks it up automatically without touching individual pages.

**Per-client setup checklist (Phase 4 work):**

1. Set `DEFAULT_OG_IMAGE_ALT` in `src/seo.rs` to the client's brand tagline.
2. (Optional) When a designed 1200×630 card exists, drop it at `assets/brand/og-card.png` and update `DEFAULT_OG_IMAGE_PATH` + width/height constants. Otherwise the 512×512 stopgap is fine for launch.
3. Confirm every page in `src/pages/` calls `Seo {…}` — that single call emits the full required tag set. Anything bypassing `Seo` is the only failure mode for missing tags.

**Verifying post-deploy:**

```bash
SITE="https://<client-domain>"

# Confirm every required OG tag is present on the homepage
curl -s "$SITE/" \
  | grep -oE '<meta[^>]*(property|name)="(og|twitter)[^"]*"[^>]*>' \
  | sort -u

# Sanity-check the OG image resolves and file size is sane
curl -sI "$SITE/assets/brand/og-card.png" | head -5   # 200, < 5MB

# Validate via third-party tools after each meaningful copy/image change:
#   - Facebook Sharing Debugger: https://developers.facebook.com/tools/debug/
#   - X Card Validator:           https://cards-dev.twitter.com/validator
#   - LinkedIn Post Inspector:    https://www.linkedin.com/post-inspector/
```

**Common mistakes that cost a re-deploy cycle:**

- Relative `og:image` URL (`/assets/...`) — auditors fail this; use `default_og_image_url()` which prepends `SITE_ORIGIN`.
- Width/height as integer in the markup — `document::Meta { content: ... }` expects a string. Use `"1200"`, not `1200`.
- Forgetting `twitter:image` when `og:image` is present — X reads its own namespace and will fall back to the `summary` (small) card without it.
- Hard-coding `og:site_name` per page — drift is guaranteed. Always reference a single constant.

#### WebP image optimization

**WebP is the canonical image format** on this site — no PNG/JPEG fallbacks ship in production. Modern browsers (Chrome / Firefox / Edge / Safari 14+) cover ~99% of traffic; shipping duplicate raster formats just slowed page loads and risked duplicate URLs getting indexed by crawlers.

`/assets/brand/**` is the lone exception — favicons, the Organization JSON-LD `logo`, and the default `og:image` stay as PNG so OS-level icon consumers and stricter social crawlers (LinkedIn historically prefers PNG/JPEG) can read them.

**How it works:**

1. **Author drops a source image** (any of `.png` / `.jpg` / `.jpeg`) into `assets/articles/<slug>/` (or any other non-`brand/` location).
2. **Author runs** `./scripts/generate-webp.sh` — produces a `.webp` next to the source at quality 82 (visually indistinguishable; ~25–35% smaller than JPEG, 80–95% smaller than photo-encoded PNG).
3. **Author deletes the source raster** — once the `.webp` is verified visually, the `.png/.jpg/.jpeg` is no longer needed on disk. Commit only the `.webp`.
4. **Article front matter still references the source extension** — `hero_image = "plastic-pallets-2.png"` stays as-is. This is intentional: the post-build rewriter (step 5) handles the `.png → .webp` swap in HTML output, so authors don't have to think about it. (You can also write `.webp` directly in front matter — both work.)
5. **Build copies WebPs** — `build-ssg.sh` rsync excludes `*.png`, `*.jpg`, `*.jpeg`, and `*.mp4` outside `brand/**`, so only WebPs reach the deploy artifact.
6. **Post-build HTML rewriter** — `scripts/wrap-webp.py` walks every prerendered HTML page and rewrites `<img src="/assets/X.png|.jpg|.jpeg">` to `<img src="/assets/X.webp">`. `/assets/brand/**` references are exempted. Tags it can't resolve (no `.webp` on disk) are left untouched so the breakage surfaces visibly rather than silently 404ing.

The rewriter does **not** wrap images in `<picture>` elements — single-format direct `src` is cleaner, smaller, and avoids the `<picture>` failure mode where a matched `<source>` URL that 404s shows a broken image (browsers don't fall back to the inner `<img>` when the matched source fails).

**Per-client setup:**

- **Local dev:** `brew install webp` (mac) / `dnf install libwebp-tools` (Amazon Linux) / `apt-get install -y webp` (Ubuntu/Debian). Only needed once per author per machine, only for running `scripts/generate-webp.sh` when adding/editing images.
- **Build hosts (Amplify, Cloudflare, Netlify, Vercel, S3, etc.): no install needed.** WebPs are committed to git; the build just copies them.

**Author workflow when adding a new article image:**

```bash
# 1. Drop the source image into the article's asset dir
cp ~/Downloads/hero.png assets/articles/my-new-slug/hero.png

# 2. Generate the WebP, verify visually
./scripts/generate-webp.sh
open assets/articles/my-new-slug/hero.webp   # eyeball it

# 3. Delete the source (WebP is now canonical)
rm assets/articles/my-new-slug/hero.png

# 4. Verify locally — the rewriter swaps any .png references in HTML
./scripts/build-ssg.sh
# Look for "WebP rewrite pass: ... skipped (no .webp on disk): 0"

# 5. Commit the WebP
git add assets/articles/my-new-slug/hero.webp
git commit -m "feat(content): add hero image for my-new-slug"
```

**Verifying it worked (post-deploy):**

```bash
SITE="https://<client-domain>"
SLUG="<article-slug>"
IMAGE="<keyword-bearing-name>"

curl -sI "${SITE}/assets/articles/${SLUG}/${IMAGE}.webp" | head -3   # 200, image/webp
curl -sI "${SITE}/assets/articles/${SLUG}/${IMAGE}.png"  | head -3   # 404 — intentional, no fallback ships
curl -s  "${SITE}/sustainability-news/${SLUG}/" | grep -oE 'src="[^"]+\.webp"' | head -1
```

**Why not AVIF?** AVIF compresses ~50% better than WebP but encodes ~10× slower and Safari support is only solid on Apple Silicon / iOS 16+. WebP is the practical sweet spot today — universal modern-browser support, fast encode, dramatic savings vs. PNG/JPEG. AVIF could be layered on later via a second `<source>` ahead of the WebP, *if* the breakage-on-missing-source concern (above) is addressed via a build-time existence check.

#### WebM video optimization

**WebM (VP9) is the canonical hero/background-video format** — no MP4 fallback ships in production. Modern browsers (Chrome / Firefox / Edge / Safari 14.1+) cover ~99% of traffic; the duplicate MP4 was costing repo size and risking a duplicate-URL SEO smell for the < 1% of older Safari users.

Posters are WebP for the same reason. The `<video poster="…">` attribute accepts WebP on every browser that plays VP9 in the first place.

**How it works:**

1. **Author drops a source MP4** at `assets/videos/<slug>.mp4` (temporarily — the MP4 itself is **not** committed).
2. **Author extracts a poster frame** as `<slug>-poster.jpg` — first frame, JPEG q=2 — then converts it to `.webp` (same cwebp pass as WebP image flow).
3. **Author runs** `./scripts/generate-webm.sh` — produces `assets/videos/<slug>.webm` at CRF 32, no audio.
4. **Author deletes the source MP4 and the JPG poster** — only the `.webm` + `.webp` poster are commit-worthy.
5. **Build copies the WebMs and WebP posters** — rsync excludes `*.mp4` and `*.jpg`, so only the canonical formats reach the deploy artifact.
6. **Runtime serving** — `src/components/video_hero.rs` exports a `VideoBackground` component that emits:

   ```html
   <video autoplay muted loop playsinline preload="metadata"
          poster="/assets/videos/<slug>-poster.webp" aria-hidden="true">
     <source src="/assets/videos/<slug>.webm" type="video/webm">
   </video>
   ```

**Per-client setup:**

- **Local dev:** `brew install ffmpeg webp` (mac) / `dnf install ffmpeg libwebp-tools` (Amazon Linux 2023, EPEL) / `apt-get install -y ffmpeg webp` (Ubuntu/Debian). One-time, only needed to run the generators.
- **Build hosts:** **no install needed.** WebM + WebP files are committed to git; the build just copies them.

**The `scripts/generate-webm.sh` script:**

Idempotent — skips files where the `.webm` is already present and
newer than the source `.mp4`. Auto-discards outputs that turn out
larger than the source MP4 (see source-bitrate threshold note below).
Drop into `scripts/generate-webm.sh`, `chmod +x`, commit:

```bash
#!/usr/bin/env bash
# Generate VP9 .webm companions for every .mp4 in assets/videos/.
# Idempotent: skips up-to-date outputs.
# Auto-discards outputs that come out larger than source (already-optimized
# MP4 sources where VP9 can't win — see docs/replicate.md §"WebM video").
set -euo pipefail

command -v ffmpeg >/dev/null || {
  echo "ffmpeg not found. Install: brew install ffmpeg" >&2; exit 1; }

shopt -s nullglob
for mp4 in assets/videos/*.mp4; do
  webm="${mp4%.mp4}.webm"
  if [[ -f "$webm" && "$webm" -nt "$mp4" ]]; then
    echo "skip  $webm (up to date)"; continue
  fi
  echo "encode $mp4 → $webm"
  ffmpeg -y -i "$mp4" \
    -c:v libvpx-vp9 -crf 32 -b:v 0 -row-mt 1 -an \
    -deadline good -cpu-used 2 \
    "$webm"
  mp4_size=$(stat -f%z "$mp4" 2>/dev/null || stat -c%s "$mp4")
  webm_size=$(stat -f%z "$webm" 2>/dev/null || stat -c%s "$webm")
  if (( webm_size >= mp4_size )); then
    pct=$(( webm_size * 100 / mp4_size ))
    echo "  ⚠  webm is ${pct}% of mp4 — discarding (source already optimized)"
    rm -f "$webm"
  else
    saved=$(( (mp4_size - webm_size) * 100 / mp4_size ))
    echo "  ✓ saved ${saved}%"
  fi
done

echo "done. sizes:"
ls -lh assets/videos/*.{mp4,webm} 2>/dev/null | awk '{print $5"\t"$NF}'
```

**Author workflow when adding a new hero video:**

```bash
SLUG=landing

# 1. Drop the source MP4 (transient — used only to generate WebM + poster)
cp ~/Downloads/farm-aerial.mp4 assets/videos/${SLUG}.mp4

# 2. Extract a poster frame and convert to WebP
ffmpeg -y -i assets/videos/${SLUG}.mp4 -vframes 1 -q:v 2 \
  assets/videos/${SLUG}-poster.jpg
cwebp -quiet -q 82 assets/videos/${SLUG}-poster.jpg \
  -o assets/videos/${SLUG}-poster.webp

# 3. Generate the WebM (CRF 32, no audio)
./scripts/generate-webm.sh

# 4. Verify locally
./scripts/build-ssg.sh

# 5. Delete the transient MP4 + JPG poster — only canonical formats ship
rm assets/videos/${SLUG}.mp4 assets/videos/${SLUG}-poster.jpg

# 6. Commit just the WebM + WebP poster
git add assets/videos/${SLUG}.webm assets/videos/${SLUG}-poster.webp
git commit -m "feat(media): add ${SLUG} hero video"
```

**Verifying it worked (post-deploy):**

```bash
SITE="https://<client-domain>"
SLUG="landing"

curl -sI "${SITE}/assets/videos/${SLUG}.webm"        | head -3   # 200, video/webm
curl -sI "${SITE}/assets/videos/${SLUG}-poster.webp" | head -3   # 200, image/webp
curl -sI "${SITE}/assets/videos/${SLUG}.mp4"         | head -3   # 404 — intentional, no fallback ships
curl -s  "${SITE}/" | grep -oE '<source[^>]+video/webm' | head -1
```

**Encoder tuning notes:**

| Knob | Default | When to change |
|---|---|---|
| `-crf 32` | Sweet spot for muted decorative loops | Lower to 28 for foreground / detail-rich content; raise to 36 for very long backgrounds where filesize dominates |
| `-cpu-used 2` | Balance of speed vs. compression | Use `0` for one-shot best-quality (much slower); use `4` for fast iteration |
| `-an` | No audio | Remove if the video has narration the user is meant to unmute |
| Two-pass | Off (single-pass CRF is fine) | Switch to two-pass for >30 MB sources where another 5–10% savings matters |

**Source-bitrate threshold (important):** VP9's "30–60% smaller than H.264" applies to typical un-optimized source MP4s — anything ≥2 Mbps. For a hero MP4 already encoded at very low bitrate (≤1.5 Mbps, common when the source comes off a CMS like HubSpot/WordPress that already re-encoded it for web), CRF 32 VP9 will produce a *larger* file because the encode floor (motion vectors, container overhead, intra keyframes) exceeds what's left of the source budget. `generate-webm.sh` auto-discards any output ≥100% of source size — that's the script telling you the source is the problem, not the codec settings. Resolve by re-sourcing a higher-quality MP4 from the client and re-encoding.

We hit this on the carbon-farming-t-rust- migration in May 2026: 5 of 6 hero MP4s were already at 295–1297 kbps and VP9 transcodes came out larger.

**Why not AV1?** AV1 compresses ~30% better than VP9 but encodes **10–50× slower** even with `libsvtav1`, and Safari support is only solid on Apple Silicon Macs and iOS 17+. VP9 is the practical sweet spot today — universal modern-browser support, fast enough to encode in a pre-commit script, and the bandwidth savings already swamp any realistic Core Web Vitals threshold. AV1 could be layered on later via a second `<source>` ahead of the WebM, *if* the breakage-on-missing-source concern is addressed via a build-time existence check (same caveat as AVIF for images).

**Decorative hero vs. content video:** This SOP assumes hero/background
videos — autoplay, muted, looping, no controls, decorative. For
**content videos** (interview clips, product demos with audio, anything
the user actively watches) the same WebM-first ordering applies, but
keep audio (`-c:a libopus -b:a 96k` instead of `-an`) and consider
adaptive streaming (HLS / DASH) for clips longer than ~60s — at that
point a single big file is the wrong shape regardless of codec.

#### Image alt-text best practices (SEO + accessibility)

Alt text is read by screen readers, search engine crawlers, and
answer-engine indexers. Bad alt text fails accessibility audits AND
loses SEO value. Every image in this template's source code passes
the `alt` attribute through correctly; the **content** of that alt
string is per-client editorial work.

**Rules of the road:**

| Rule | Example (good) | Example (bad) |
|---|---|---|
| **1. Describe the IMAGE, not the article topic.** Alt text answers "what would a sighted user see?", not "what is this page about?" | `alt="Hands counting cash bills at a wooden table beside a notebook and coffee — the daily morning finance check ritual"` | `alt="Why a daily finance check matters"` (this is the article topic) |
| **2. Include the primary keyword naturally — once.** Don't keyword-stuff. | `alt="Smartphone showing all bank account balances on a single MATA dashboard screen"` | `alt="bank dashboard banking finance dashboard banking dashboard"` |
| **3. Keep it concise (under ~125 chars).** Screen readers read alt text in full. Long alt = exhausted listener. | `alt="Two-monitor desktop with MATA password vault open on the left and a code editor on the right"` | `alt="A picture of a guy at his desk with two monitors. The left one is showing the MATA app and on the right he is coding a Rust application that does..."` (too long) |
| **4. Decorative-only images get `alt=""` (empty but present) AND `aria-hidden="true"`.** Tells screen readers "skip me" without omitting the attribute (which would make some readers announce the filename). | Logo duplicate that flips on dark mode: `alt="" aria-hidden="true"` | Decorative spacer with no alt at all (browsers may announce the filename) |
| **5. Don't start with "Image of…" or "Picture of…".** Screen readers already announce "image" — leading with that doubles up. | `alt="Three smartphones running MATA, syncing peer-to-peer over Iroh"` | `alt="Image of three smartphones running MATA"` |
| **6. Tracking pixels and analytics noscript fallbacks always get `alt=""`.** They're not content. | Meta Pixel `<noscript>` img: `alt=""` | `alt="Tracking pixel"` (creates audible noise) |
| **7. Logos get the brand name — once per page.** If the same logo appears twice (e.g. mobile + desktop versions, or light + dark variants), only one has the brand alt; the other(s) get `alt="" aria-hidden="true"`. | Header logo: `alt="MATA"`. Dark-mode duplicate of the same logo: `alt="" aria-hidden="true"` | Both logos: `alt="MATA"` (announced twice by screen readers) |

**Where alt text lives in this codebase:**

| Image | Source of alt text | Per-article authoring |
|---|---|---|
| Article hero | `hero_alt` in TOML front matter (required field) | Yes — author writes per article |
| Inline article images | `![alt](src)` in markdown body | Yes — author writes per image |
| Header / footer logos | Hardcoded in `src/components/layout.rs` | No — set once at brand kit time |
| Tracking pixel noscripts | Hardcoded `alt=""` in `src/tracking.rs` | No — never touched |

**Phase-7 audit query** — verify no article has a topic-style alt or
empty `hero_alt`:

```bash
# All hero_alt values across articles:
grep -E "^hero_alt[[:space:]]*=" content/articles/*.md

# Flag heroes with title-style alt (article topic instead of image):
# Heuristic: alt that exactly matches the article title or starts with
# "Why" / "What is" / "How to" is suspect.
for art in content/articles/*.md; do
  title=$(grep -oE '^title[[:space:]]*=[[:space:]]*"[^"]*"' "$art" | sed 's/^title.*"\(.*\)"$/\1/')
  alt=$(grep -oE '^hero_alt[[:space:]]*=[[:space:]]*"[^"]*"' "$art" | sed 's/^hero_alt.*"\(.*\)"$/\1/')
  if [[ "$title" == "$alt" ]] || [[ "$alt" =~ ^(Why|What\ is|How\ to) ]]; then
    echo "⚠️  $art: alt looks like article topic, not image description"
  fi
done
```

#### Common SEO-auditor false-negatives (don't waste time chasing)

| Audit flag | Real status | Why the tool gets it wrong |
|---|---|---|
| **"No SPF record"** | SPF is on the apex (`mata.network`) — that's the only place it needs to be for email auth. Verify with `dig +short TXT <domain>`. | Some auditors check `www.<domain>` or fail to resolve via the right authoritative resolver. |
| **"Not using CSS media queries"** | Tailwind v4 emits 8+ `@media` rules per build (every `sm:` / `md:` / `lg:` / `xl:` / `2xl:` variant + `prefers-color-scheme` + `prefers-reduced-motion` + `hover:hover`). | Tailwind v4 uses modern range syntax `@media (width>=40rem)` instead of the older `@media (min-width: 40rem)`. Older parsers don't recognize the range form and report "no media queries." Verify with: `curl -s <site>/assets/tailwind-*.css \| grep -oE "@media[^{]*" \| sort -u`. |
| **"No 404 page"** (post-deploy if Amplify rule 2a wasn't added) | Custom 404 is built and prerendered, but Amplify falls back to its default error page until rule 2a is configured in the console. | The build emits the artifact; the routing rule is a one-time console step. Re-run the Phase 7 `curl -sI <site>/some-bogus-url` check — if it returns 200 or 302, rule 2a is missing. |

### Phase 7.5 — Post-build audit (run before declaring done)

Three structured scans that catch the most common migration mistakes — each one a 30-second one-liner. Run them locally against `http://localhost:3000` (production preview) AFTER all the curl checks above pass.

#### Audit A: Sitemap parity vs. the live source site

```bash
SITE_OLD="https://<previous-cms-domain>"
SITE_NEW="http://localhost:3000"

# Extract every URL from the OLD site (handle Yoast multi-sitemap)
{
  curl -sSL "$SITE_OLD/post-sitemap.xml"
  curl -sSL "$SITE_OLD/page-sitemap.xml"
  curl -sSL "$SITE_OLD/sitemap.xml"
} | grep -oE '<loc>[^<]+</loc>' \
  | sed 's|<loc>||; s|</loc>||' \
  | sed -E "s|^https?://[^/]+||; s|/$||" \
  | sort -u > /tmp/old-urls.txt

# Extract every URL from the NEW site
curl -sSL "$SITE_NEW/sitemap.xml" \
  | grep -oE '<loc>[^<]+</loc>' \
  | sed 's|<loc>||; s|</loc>||' \
  | sed -E "s|^https?://[^/]+||; s|/$||" \
  | sort -u > /tmp/new-urls.txt

echo "--- ON OLD BUT NOT NEW (potential gaps) ---"
comm -23 /tmp/old-urls.txt /tmp/new-urls.txt

echo "--- ON NEW BUT NOT OLD (added pages) ---"
comm -13 /tmp/old-urls.txt /tmp/new-urls.txt
```

Every URL in the "ON OLD BUT NOT NEW" list must be either (a) explicitly in your "pages to skip" list from Phase 1b — document why in `docs/discovery-<client>.yml` — or (b) a bug. Trailing-slash differences (`/about-us/` vs `/about-us`) are fine; CloudFront and Amplify serve both forms.

#### Audit B: Tracker inventory matches what we shipped

```bash
SITE_OLD="https://<previous-cms-domain>"

echo "--- All third-party scripts on the LIVE site ---"
curl -sSL "$SITE_OLD" | grep -oE '<script[^>]+src="[^"]+"' \
  | grep -oE 'https?://[^"]+' | sort -u

echo "--- All inline tracker patterns on the LIVE site ---"
curl -sSL "$SITE_OLD" | grep -oiE \
  '(gtag|fbq|_hsq|_paq|clarity|hjEvent|intercomSettings|driftt|crisp|heap|smartlook|tealium|onetrust|cookiebot|chmln|pendo|amplitude|klaviyo)\(?'  \
  | sort -u

echo "--- All tracker IDs in the LIVE HTML ---"
curl -sSL "$SITE_OLD" | grep -oE '(G-[A-Z0-9]+|GTM-[A-Z0-9]+|UA-[0-9]+-[0-9]+)' \
  | sort -u
```

Every tracker in the output should match exactly what's in our `src/tracking.rs` — no missing trackers (we'd lose attribution / conversions) and no orphan trackers we never had (would be a security smell + CSP violation).

#### Audit C: H1 sanity

```bash
cd target/dx/<client>-website/release/web/public
# Every page should have exactly 1 H1 with full descriptive text.
python3 << 'PY'
import os
from html.parser import HTMLParser

class H1Grab(HTMLParser):
    def __init__(self):
        super().__init__()
        self.in_h1 = False
        self.bufs = []
        self.cur = []
    def handle_starttag(self, tag, attrs):
        if tag == 'h1': self.in_h1 = True; self.cur = []
    def handle_endtag(self, tag):
        if tag == 'h1' and self.in_h1:
            self.bufs.append(' '.join(''.join(self.cur).split()))
            self.in_h1 = False
    def handle_data(self, data):
        if self.in_h1: self.cur.append(data)

paths = sorted(p for r,_,fs in os.walk('.') for p in (os.path.join(r,f) for f in fs) if p.endswith('index.html'))
for p in paths:
    h = H1Grab(); h.feed(open(p, encoding='utf-8').read())
    route = p.removeprefix('./').removesuffix('index.html').rstrip('/') or '/'
    print(f"  H1={len(h.bufs):>1}  {route:<70}  {(h.bufs[0] if h.bufs else '(NO H1)')[:80]}")
PY
```

Look for: count != 1, empty H1 text, or text where two sentences run together (e.g. `"Affordably.Open To Everyone."` — see Phase 5l for the `<br>` spacing fix).

---

## Per-client customization checklist

Print and check off, one per client engagement:

```
RESET (before any per-client edits)
[ ] rm -rf content/articles/*.md
[ ] rm -rf assets/articles/*
[ ] rm -rf assets/brand/*
[ ] rm -f  README.md
[ ] rm -f  docs/{discovery,trackers}-*.* + docs/slug-map-*.csv + docs/source/*.md
[ ] rm previous-client page components from src/pages/ + remove from src/pages/mod.rs
[ ] rm src/components/popup.rs        if client doesn't use Klaviyo (and remove from mod.rs)
[ ] rm src/components/pricing_board.rs if client doesn't have a commodity-pricing widget
[ ] grep -rinE "<previous-client>" src/ scripts/ tailwind.css customHttp.yml — should be empty

BRAND
[ ] SITE_NAME / SITE_BASE in main.rs
[ ] Brand color in tailwind.css (--color-red-{6..12} + red-12 deepest)
[ ] Brand color in generate_icons.py BRAND_RED (or whatever you renamed it)
[ ] Real brand logo extracted from live site → assets/brand/<client>-logo-{light,dark}.png
[ ] Wordmark generation DISABLED in generate_icons.py (so a future run doesn't overwrite the real logo)
[ ] Favicon mark text in generate_icons.py rows = [...]  (or use SVG of real brand mark)
[ ] Run python3 generate_icons.py
[ ] Run ./scripts/download-fonts.sh (uses Inter by default — only override if brand demands)

TRACKING
[ ] GA4 ID in tracking.rs (or remove TrackingHead body if launching without analytics)
[ ] Klaviyo IDs in tracking.rs only if scan turned them up (else remove popup.rs)
[ ] CSP script-src + connect-src + frame-src in customHttp.yml — match the actual tracker list
[ ] CSP form-action — match the actual form provider (Klaviyo, Formspree, etc.)
[ ] Remove unused tracker hosts from CSP (security hygiene — narrower CSP is better)

PAGES + ROUTING
[ ] Stat counter values in stat_counters.rs (default_stats() at minimum; per-page overrides as needed)
[ ] Header nav labels in layout.rs Header + Dropdowns + MobileMenu
[ ] Footer columns + social links in layout.rs Footer
[ ] Per-page hero copy in pages/*.rs
[ ] Routable enum in main.rs matches all source URLs (Phase 1b sitemap output)
[ ] static_routes server fn in main.rs                     ┐
[ ] STATIC_ROUTES + PRIORITIES + CHANGEFREQS in            ├ all three must match
    scripts/generate-sitemap.sh                            │
[ ] ROUTES in scripts/prerender.sh                         ┘
[ ] PITCH + CORE_PAGES in scripts/generate-aeo.py
[ ] organization_jsonld() in seo.rs (name, url, logo path, sameAs, description)
[ ] Article SOP voice rules in README.md (3 primitives, outbound link allowlist)
[ ] Contact email + address — search for "hello@hfga" / "Newlab, Detroit" placeholder (whatever the seed had)

CONTENT
[ ] Migrated articles in content/articles/<slug>.md (slugs match old site EXACTLY, including typos)
[ ] Migrated hero images in assets/articles/<slug>/<keyword-bearing-name>
[ ] Every hero image is `file`-confirmed PNG/JPEG, not silent-404 HTML
[ ] `./scripts/generate-webm.sh` was run on every new hero video (idempotent — auto-discards WebMs larger than source). Every video slug ships both `<slug>.webm` and `<slug>-poster.webp`; source MP4 + JPG are NOT committed.

LOCAL VERIFICATION
[ ] cargo check --features web --target wasm32-unknown-unknown (zero warnings)
[ ] cargo check --features server                              (zero warnings)
[ ] ./scripts/build-ssg.sh                                     (clean build, ALL routes 200)
[ ] All Phase 7 curl checks pass on local preview (port 3000)
[ ] Phase 7.5 audit A: sitemap parity (only deliberate-skip URLs in the gap list)
[ ] Phase 7.5 audit B: tracker inventory matches src/tracking.rs (no missing, no orphans)
[ ] Phase 7.5 audit C: every page has exactly 1 H1 with descriptive text + clean spacing

DEPLOY
[ ] Deploy to Amplify with rewrite rule (md allowlisted in the SPA-rewrite regex)
[ ] DNS records set (apex + www) — see Phase 6
[ ] Amplify SSL green for both apex + www
[ ] www ↔ apex 301 redirect rule live (matches canonical direction in HTML / sitemap)
[ ] Phase 7 + 7.5 audits pass against production URL
[ ] Manual mobile test (hamburger opens, hero CTAs tappable ≥44px)
[ ] GSC verification + sitemap submitted (use a **domain property**, not URL-prefix — see Phase 8)
[ ] GA4 receiving pageviews (check real-time view)

CONSOLE INTAKE (full detail in Phase 8 below)
[ ] docs/console-intake-<client>.md filled in (every field in the Phase 8 table)
[ ] GCP service account added as Viewer on the GA4 property
[ ] GCP service account added as user on the Search Console property
[ ] (optional) Bing Webmaster: service account added + sitemap submitted
[ ] GitHub Console PAT extended to include the new repo
[ ] sites.toml block appended in the Console repo + pushed
[ ] Post-enrollment verification (5 curls in Phase 8) all green
```

---

## Migration provenance

For every client migration, save the following alongside the build for the audit trail. These also become the troubleshooting ammo if any URL stops working post-launch.

```
docs/
├── discovery-<client>.yml          # Phase 1a output — brand vars, fonts, colors
├── source-urls-<client>.txt        # Phase 1b output — every URL from old site
├── trackers-<client>.md            # Phase 1c output — kept vs dropped, with IDs
├── source/<slug>.md                # Phase 2a/2b — verbatim transcriptions
├── slug-map-<client>.csv           # Phase 2c — old URL → new slug, hero filename map
├── replicate-notes-<client>.md     # any client-specific gotchas (font swaps, slug typos preserved, etc.)
└── console-intake-<client>.md      # Phase 8 output — every field needed to enroll this site in console.md
```

The discovery + source files protect future-you. If the client says "the old site had X — where is it now?" you can grep the source files in seconds.

---

## Phase 8 — Console intake (post-deploy)

Every site shipped from this playbook is meant to be enrolled in the unified **Console** — the single dashboard that monitors health, uptime, SEO, visitors, deploys, and content health across the full portfolio. Full design lives in [console.md](console.md).

Enrollment is one TOML block in the Console's `sites.toml` + a handful of one-time API grants. Do this **at deploy time** — half the data points only exist *after* the Amplify app exists, and half of them become guesswork a week later. Capture them now.

### 8a. Data to capture per site

Open `docs/console-intake-<client>.md` (alongside the other migration provenance files) and record:

| Field | Where to find it | Notes |
|---|---|---|
| `domain` | Apex you registered | No protocol, no trailing slash. e.g. `mata.network` |
| `display_name` | Brand name | Title-cased; appears in the Console grid |
| `canonical` | `https://<canonical-host>` | The host the site 301s *to* — apex or www. Must match `<link rel="canonical">` |
| `www_redirects` | `true` / `false` | Direction of the www ↔ apex 301 (set in Amplify rewrites — Phase 6) |
| `github_repo` | `<org>/<repo>` | What Amplify is wired to. `gh repo view --json nameWithOwner` |
| `amplify_app_id` | Amplify console URL | The `d########` slug — only visible in the URL: `console.aws.amazon.com/amplify/apps/<APP_ID>/...` |
| `cloudfront_dist_id` | Amplify → Domain management | Per-domain, not per-app. Multi-domain apps have one per canonical host |
| `ga4_property_id` | GA4 admin → Property Settings | Format `properties/##########` — **include the prefix** |
| `ga4_measurement_id` | Same place | `G-XXXXXXXXXX` — already wired in `src/tracking.rs` |
| `gtm_id` | If GTM is in use | `GTM-XXXXXXX` |
| `sc_property` | Search Console | Use the **domain property** form: `sc-domain:<domain>`. NOT the URL-prefix form |
| `bing_site` | Bing Webmaster (if enrolled) | `https://<canonical-host>/` |
| `klaviyo_company_id` | `src/tracking.rs` (if forms in use) | Already present from Phase 4 |
| `newsletter_provider` + `publication_id` | If newsletter present | Beehiiv publication_id is in the dashboard URL |
| `cf_zone_id` | Cloudflare (if behind CF or DNS-only on CF) | Zone overview page |
| `aws_region` | Amplify's region | `us-east-1` for our default |
| `domain_registrar` | Wherever the domain was bought | GoDaddy / Cloudflare / Namecheap — matters when registration expires |
| `dns_provider` | Where DNS records live | Often different from registrar (e.g. registered at GoDaddy, DNS at Cloudflare) |
| `expected_headers` | The headers `customHttp.yml` ships | Console validates each is present on every response |
| `launched_at` | Deploy date | ISO `YYYY-MM-DD` |

The same file is also where you note anything not in the columns above — Phase 5 gotchas hit during this client, custom Amplify settings, manual DNS oddities.

### 8b. One-time API grants

The Console reads upstream APIs using **one shared GCP service account + one AWS IAM user + one GitHub PAT**, all set up when the Console itself was deployed. For each new site, grant access *at the site level*:

1. **GA4 — add the GCP service account as a Viewer:**
   - GA4 admin → Property → Property Access Management → Add user.
   - Paste the service account email (looks like `console-reader@<project>.iam.gserviceaccount.com`).
   - Role: `Viewer`.
   - **Wait ~5 minutes** before the first API call — propagation is not instant.
2. **Search Console — add the service account as a user:**
   - Search Console → site → Settings → Users and permissions → Add user.
   - Paste the same service account email.
   - Permission: `Restricted` (full read access).
   - The domain must already be verified (DNS TXT during Phase 6). Service-account access doesn't bypass verification.
3. **Bing Webmaster — optional but free, covers DuckDuckGo + ChatGPT Search + ~5% organic:**
   - Webmaster → Settings → Users and Roles → Add User.
   - Paste the service account email, role `Read-only`.
   - Submit `sitemap.xml` while you're there.
4. **AWS Amplify — confirm the IAM user has account-wide read access:**
   - Console IAM user already has `amplify:Get*` / `amplify:List*` at account level (one-time setup).
   - No per-site grant needed if the new app is in the same AWS account.
5. **GitHub PAT — extend repo access:**
   - GitHub → Settings → Developer settings → Personal access tokens → fine-grained → edit the Console PAT.
   - Add the new repo under `Repository access`.
   - Required scopes: `Contents: read`, `Metadata: read`, `Actions: read`, `Dependabot alerts: read`.

### 8c. Things to remember (lessons from the field)

- **Service-account Viewer grants in GA4 don't propagate instantly.** First API call after granting will 403 with "User does not have any Google Analytics account." Wait 5 minutes; retry.
- **Search Console: domain property ≠ URL-prefix property.** Domain properties give one unified data set across apex + www + http + https. URL-prefix properties split each combination into its own surface. Always pick **domain property** during Phase 6 verification.
- **The Amplify app ID is not in the dashboard text** — it's only in the URL. Bookmark or copy it the day you create the app.
- **CloudFront distribution ID is per-canonical-host.** If your Amplify app serves `apex` AND `www` as separate hosts (rather than redirecting one), each has its own distribution.
- **Canonical host preference must agree in three places:** `<link rel="canonical">` in HTML, `<loc>` entries in `sitemap.xml`, and the Amplify `https://www.<domain>/<*>` redirect rule. The Console flags drift between any of the three.
- **DKIM / SPF / DMARC live on the apex.** The Console queries the apex; "www has no SPF" is correct behavior — don't chase it.
- **`customHttp.yml` headers don't apply to Amplify's default error page.** If `/some-bogus-url` returns Amplify's default 404 with no security headers, the 404-rewrite rule from Phase 6 isn't configured. Re-check before declaring the site Console-ready.
- **A redirect chain longer than two hops is a soft SEO regression.** `http://www.<domain>` → `https://www.<domain>` → `https://<domain>` is fine (protocol upgrade + canonical normalization). Apex → www → apex is a loop and the Console alerts on it.
- **The Console's `og:image` validator demands an absolute URL.** Match the spec in [README.md → Open Graph & Twitter Card standard](../README.md). Any new top-level page added post-launch must emit the full tag set or the Console flags it.
- **GitHub fine-grained PATs default to **deny-all** when a new repo is created.** Adding the new repo to the existing PAT is a separate step — easy to forget. Verify with `gh api /repos/<org>/<repo>/commits` using the PAT.
- **Klaviyo `company_id` is public** (it's already in `src/tracking.rs` and ships to the browser). The Klaviyo *private API key* is what the Console needs for list-size data and goes in the serverless layer's env, not in the site's repo.
- **`launched_at` matters for the Console.** SEO and visitor metrics from a brand-new domain look terrible relative to mature sites; the Console uses `launched_at` to scale "is this a regression?" thresholds.

### 8d. The TOML block to append to `sites.toml`

Once the data in §8a is captured and the grants in §8b are done, append this block to the Console repo's `sites.toml`:

```toml
[[sites]]
domain                    = "<domain>"
display_name              = "<Display Name>"
canonical                 = "https://<canonical-host>"
www_redirects             = true                                # or false, matching Phase 6
github_repo               = "<org>/<repo>"
amplify_app_id            = "d########"
cloudfront_dist_id        = "E########"
ga4_property_id           = "properties/##########"
ga4_measurement_id        = "G-XXXXXXXXXX"
sc_property               = "sc-domain:<domain>"
bing_site                 = "https://<canonical-host>/"
aws_region                = "us-east-1"
domain_registrar          = "<godaddy | cloudflare | namecheap | ...>"
dns_provider              = "<cloudflare | godaddy | route53 | ...>"
launched_at               = "YYYY-MM-DD"
expected_headers          = [
    "Content-Security-Policy",
    "Strict-Transport-Security",
    "X-Frame-Options",
    "X-Content-Type-Options",
    "Referrer-Policy",
    "Permissions-Policy",
]
# Optional — comment out the line entirely if not used:
# klaviyo_company_id      = "..."
# newsletter              = { provider = "beehiiv", publication_id = "..." }
# cf_zone_id              = "..."
# gtm_id                  = "GTM-..."
```

Commit this in the Console repo and push. The Amplify build runs (~3 min), the Console rebuilds, and the new site appears in the overview grid.

### 8e. Post-enrollment verification

From a machine with the Console bearer token, verify the new site is wired correctly end-to-end:

```bash
CONSOLE="https://console.<your-domain>"
TOKEN="<bearer>"
SITE="<domain>"

# 1. Enrolled and visible in the overview
curl -s -H "Authorization: Bearer $TOKEN" "$CONSOLE/api/sites" \
  | grep -q "\"$SITE\"" && echo "✓ enrolled"

# 2. Live uptime check returns 2xx
curl -s -H "Authorization: Bearer $TOKEN" "$CONSOLE/api/check/uptime?site=$SITE" \
  | jq '.status_code'        # → 200

# 3. Self-built SEO check passes (no missing OG, canonical, sitemap, etc.)
curl -s -H "Authorization: Bearer $TOKEN" "$CONSOLE/api/check/seo?site=$SITE" \
  | jq '.failures'           # → [] (empty array)

# 4. Security headers grade
curl -s -H "Authorization: Bearer $TOKEN" "$CONSOLE/api/check/security?site=$SITE" \
  | jq '.observatory_grade'  # → "A" or "A+"

# 5. GA4 reachable (returns sessions count; 0 is fine for a brand-new site)
curl -s -H "Authorization: Bearer $TOKEN" "$CONSOLE/api/ga4?site=$SITE&range=24h" \
  | jq '.sessions'           # → integer

# 6. Search Console reachable (data has a 2–3 day lag — may be 0 for new sites)
curl -s -H "Authorization: Bearer $TOKEN" "$CONSOLE/api/search-console?site=$SITE&range=28d" \
  | jq '.impressions'        # → integer
```

Any of these failing points to a specific intake step: 1 → not in `sites.toml`, 2 → DNS/redirect, 3 → page tags drift (cross-check [README.md → Open Graph & Twitter Card standard](../README.md)), 4 → `customHttp.yml` regression, 5 → GA4 grant not propagated or wrong `ga4_property_id`, 6 → Search Console user not added or wrong `sc_property` format.

### 8f. Cross-reference

- Full Console architecture, UI, and API table: [console.md](console.md)
- Required APIs list (authoritative): [console.md → Required APIs — centralized reference](console.md#required-apis--centralized-reference)
- OG tag spec the SEO validator enforces: [README.md → Open Graph & Twitter Card standard](../README.md)
- Security headers the Console validates: [infra/security-headers.md](../infra/security-headers.md)

---

## Quick-start command sequence

For an experienced operator who's done this before, the literal command sequence to go from `git clone` to "site loads in a browser":

```bash
# 1. Scaffold
git clone <hfga-rust> <client>-rust && cd <client>-rust
rm -rf content/articles/*.md assets/articles/* assets/brand/{hfga-logo*,favicon*,icon-*,apple-touch-*}

# 2. Per-client edits (use the find/replace map above)
$EDITOR src/main.rs tailwind.css generate_icons.py src/tracking.rs \
        src/components/layout.rs src/components/stat_counters.rs \
        scripts/generate-aeo.py scripts/generate-sitemap.sh scripts/prerender.sh \
        customHttp.yml src/seo.rs src/content.rs

# 3. Brand assets
python3 generate_icons.py
./scripts/download-fonts.sh

# 4. Migrate content
$EDITOR content/articles/<slug-1>.md  # paste verbatim transcription with TOML front matter
# (repeat per article)

# 5. Article hero images
mkdir -p assets/articles/<slug-1>
curl -sSL -o assets/articles/<slug-1>/<keyword-bearing-name>.png "https://<old-site>/.../hero.png"
# (repeat per article)

# 6. Verify locally
cargo check --features web --target wasm32-unknown-unknown
./scripts/build-ssg.sh
cd target/dx/<client>-website/release/web/public && python3 -m http.server 3000

# 7. Phase 7 curl checks against http://localhost:3000

# 8. git add / commit / push → Amplify builds and deploys

# 9. Phase 8 — Console intake (post-deploy)
#    Capture amplify_app_id, cloudfront_dist_id, ga4_property_id, sc_property → docs/console-intake-<client>.md
#    Grant GCP service account Viewer on GA4 + Search Console for this domain
#    Extend Console GitHub PAT to include this repo
#    Append [[sites]] block in the Console repo's sites.toml → push → site appears in dashboard
```

Target wall-clock for an experienced operator: **6–10 hours** for a migration with up to 10 articles + ~10 static pages, including content harvest. First-time operators expect ~20 hours — most of it in Phase 1 discovery. Add ~30 minutes for Phase 8 Console intake.

---

## Phase 9 — Multilingual rollout (optional)

The translation pipeline is decoupled from migration: you can ship an English-only site, then add languages later without re-touching anything else. Done with [`scripts/translate.py`](../scripts/translate.py), Anthropic's Claude Haiku 4.5, and the Anthropic Batch API.

### 9a. Pick languages

Edit `LANGUAGES` in [`scripts/translate.py`](../scripts/translate.py) AND the `Language` enum + `Language::ALL` in [`src/i18n.rs`](../src/i18n.rs). Both lists must stay in sync (manual — no codegen yet). Map client operating regions to BCP-47 codes:

```
US / UK / AU / etc.         → en (canonical, no prefix)
LatAm + Spain               → es
Germany / Austria           → de
France / Belgium / Lux      → fr
Italy                       → it
Brazil + Portugal           → pt
Netherlands                 → nl
Poland                      → pl
India (B2B)                 → en + hi
Bangladesh                  → bn
Pakistan                    → ur + pa
UAE / KSA                   → ar
China (mainland)            → zh-CN
Vietnam                     → vi
Japan                       → ja
Korea                       → ko
Turkey                      → tr
```

### 9b. Build & wire-up (one-time per repo)

Already in place for heartland-rust — for new sites mirror these files:

- [`build.rs`](../build.rs) — walks `content/articles/<lang>/` subdirs, emits `(slug, lang, src)` manifest tuples.
- [`src/content.rs`](../src/content.rs) — adds `find_lang()` (English fallback), `recent_lang()`, `translations_for()`.
- [`src/main.rs`](../src/main.rs) — `#[route("/:lang/sustainability-news/:slug")] LangArticle` + `#[route("/:lang/sustainability-news")] LangNews`.
- [`src/pages/article.rs`](../src/pages/article.rs) + [`src/pages/news.rs`](../src/pages/news.rs) — `LangArticle` / `LangNews` thin wrappers around shared `*_Inner` components.
- [`src/seo.rs`](../src/seo.rs) — `HreflangAlternates` component for `<link rel="alternate" hreflang="...">` tags.
- [`scripts/prerender.sh`](../scripts/prerender.sh) — enumerates `/<lang>/...` routes; post-processes `<html>` to inject `lang=` + `dir=`.
- [`scripts/generate-sitemap.sh`](../scripts/generate-sitemap.sh) — emits `/<lang>/sitemap_index.xml` per language, advertises them in `robots.txt`.

### 9c. Translation commands

```bash
# One-time setup
pip install -r scripts/requirements.txt
export ANTHROPIC_API_KEY=sk-ant-...

# First-time bulk run — real-time, ~$3/lang, ~15 min/lang at concurrency 5.
./scripts/translate.py --lang pt

# Cheaper batch path — ~$1.50/lang, results in minutes-to-hours.
./scripts/translate.py --batch --lang pt    # submit
./scripts/translate.py --batch-poll <id>    # download + auto-scan + auto-retry

# Full multi-language batch — about $25 for 17 languages × ~228 routes.
./scripts/translate.py --batch              # all unfilled langs/files

# Quality audit (line-overlap + non-ASCII heuristics, auto-retry on suspects).
./scripts/translate.py --audit              # everything
./scripts/translate.py --audit --lang ja    # one language
```

### 9d. Quality gates worth knowing about

Haiku 4.5 occasionally returns a partial translation — frontmatter + inline link text translated but body paragraphs stay English. Rate on heartland-rust's PT run was 2/178 ≈ 1.1%. Caught by two heuristics in the script:

- **Line overlap** — >25% of body lines verbatim in the English source (the strongest signal).
- **Non-ASCII ratio** — body >800 chars with <0.5% non-ASCII (works for every target language since each has accents, CJK, or non-Latin script).

`--batch-poll` runs this scan automatically after download and auto-retries with Haiku → Sonnet 4.6 escalation. The `--audit` mode runs the same scan on a fixed corpus.

### 9e. Cost reference (Haiku 4.5 + Batch API)

| Volume | Real-time | Batch (50% off) |
|---|---|---|
| 178 articles × 1 language | ~$3 | ~$1.50 |
| 178 articles × 17 languages | ~$51 | ~$25 |
| Incremental per new article × 17 langs | ~$0.20 | ~$0.10 |

### 9f. CI automation

[`.github/workflows/translate.yml`](../.github/workflows/translate.yml) — triggers on any push that touches `content/articles/*.md`, runs `translate.py`, commits results back to `main` with `[skip ci]`. Requires `ANTHROPIC_API_KEY` set as a repo secret. Manual trigger via GitHub Actions UI also supported (with `--force` and `--lang` overrides).

### 9g. GSC reflection

Existing GSC sitemap submissions stay valid: our build emits `sitemap_index.xml` (matching Yoast's filename) at both the apex and each `/<lang>/` path. If the client had per-language sitemaps submitted to GSC under WordPress + Yoast, they continue working unchanged.

---

*Last updated from learnings on the HFGA migration (2026-05). Update this file after every client engagement with anything you wish you'd known on day one.*
