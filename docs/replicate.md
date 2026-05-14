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

Target wall-clock for an experienced operator: **6–10 hours** for a migration with up to 10 articles + ~10 static pages, including content harvest. First-time operators expect ~20 hours — most of it in Phase 1 discovery.

---

*Last updated from learnings on the HFGA migration (2026-05). Update this file after every client engagement with anything you wish you'd known on day one.*
