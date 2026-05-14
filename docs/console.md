# Console — Unified Health / SEO / Visitor Dashboard

This document is the plan for a single dashboard ("the Console") that monitors all sites built from [replicate.md](replicate.md) — health, uptime, SEO, visitors, deploys, and content health — in one place. It is scoped to **free or build-it-yourself** integrations only; nothing here requires paid SaaS.

The Console is itself a Dioxus + Rust + WASM site, deployed via the same SSG-on-Amplify pipeline as the sites it monitors. Real-time data comes from a small Rust serverless layer the WASM frontend calls at page-load and on button-press.

---

## Goals

1. **One pane of glass** — every site's health, traffic, SEO score, deploy status, and content inventory visible in one view, side-by-side.
2. **Real-time on demand** — page refresh or button-press triggers a live check. No "data was true 6 hours ago" surprises.
3. **Stays in stack** — 100% Dioxus + Rust + WASM, SSG output on Amplify, no Node/Python/Ruby anywhere in the runtime.
4. **Free tier only** — every integration in this plan is either free indefinitely, free at our usage volume, or built ourselves in Rust.
5. **Per-site replicability** — adding a 6th site is editing one TOML row, not writing new code.

---

## Architecture

```
┌──────────────────────────────────────────────────────────────────────┐
│  Console SSG site (Dioxus + WASM)        deployed on Amplify         │
│  console.mata.network                                                │
│                                                                      │
│   ┌─ Static shell (prerendered)                                      │
│   │    sites grid, tabs, layout, brand                               │
│   │                                                                  │
│   └─ WASM hydration:                                                 │
│        on load + on refresh button                                   │
│             │                                                        │
│             ▼ fetch() with auth token                                │
└─────────────│────────────────────────────────────────────────────────┘
              │
              │  HTTPS + bearer token
              │
┌─────────────▼────────────────────────────────────────────────────────┐
│  Rust serverless layer (Lambda Function URLs OR Cloudflare Workers)  │
│                                                                      │
│   GET /api/sites                  → list + cached snapshot           │
│   GET /api/check/uptime           → live, ~500ms                     │
│   GET /api/check/seo              → live, ~1–2s                      │
│   GET /api/check/perf             → live via PSI, ~10–30s            │
│   GET /api/check/security         → live via Observatory, ~5s        │
│   GET /api/ga4                    → cached, daily refresh            │
│   GET /api/search-console         → cached, daily refresh            │
│   GET /api/deploys                → cached, 5-min refresh            │
│   GET /api/alerts                 → derived, cached 5-min            │
│                                                                      │
│   Cron Lambda (EventBridge or CF Cron Trigger) refreshes caches      │
└─────────────│────────────────────────────────────────────────────────┘
              │
              │  reqwest + serde_json + service-account JWTs
              │
┌─────────────▼────────────────────────────────────────────────────────┐
│  Upstream APIs    +    Cache storage                                 │
│  (see Required APIs section below)    (S3 JSON OR CF KV/R2)          │
└──────────────────────────────────────────────────────────────────────┘
```

### Why a serverless layer (and not just static)

The Console must surface live data on demand. Three things break under pure SSG:

1. **Page refresh → live uptime check** — can't run from build time, has to execute when the user asks.
2. **Cross-origin API calls with secrets** — GA4 / Search Console / Amplify need server-side credentials we cannot ship to the browser.
3. **Rate-limited upstream APIs** — Search Console caps at a few req/sec; we cannot let every page-load hit it directly.

The serverless layer solves all three: it owns the credentials, it runs the live checks, and it caches the slow upstreams.

### Why SSG for the frontend (and not Dioxus fullstack)

Keeping the shell static means:

- Same Amplify deploy pipeline as the 5 sites it monitors — no new infra type.
- WASM hydrates the live data; the static HTML loads instantly even before the API calls return.
- Security headers (CSP, HSTS) come for free from the existing `customHttp.yml` pattern.
- Auth lives entirely client-side (one token in `localStorage`, sent as a bearer header), no session store.

---

## Live vs. cached data

| Check | Trigger | Cache TTL | Why |
|---|---|---|---|
| Uptime / status code | Page load + button | 0 (always live) | The whole point — needs to be true *now* |
| SSL cert expiry | Page load + button | 1h | Doesn't change between checks |
| DNS records | Page load + button | 1h | Same |
| Security headers (Observatory) | Page load + button | 1h | Doesn't change without a deploy |
| Self-built SEO checker (OG/canonical/sitemap) | Page load + button | 0 (live) | Cheap, our own code |
| PageSpeed Insights (Core Web Vitals) | Manual button only | 24h | 25k/day quota; expensive to compute |
| GA4 visitors | Page load | 1h (24h for ranges > 7d) | Rate-limited, daily data doesn't move minute-to-minute |
| Search Console | Page load | 24h | Data is only published with a 2–3 day lag anyway |
| Amplify deploys | Page load | 5min | Slow-changing |
| Alerts feed | Page load | 5min | Derived from above |

Cache layer: **S3 JSON files** (if AWS Lambda) or **Cloudflare KV** (if CF Workers). Each cache entry is `{ site, check, fetched_at, payload }`. A cron Lambda refreshes the heavy entries on a schedule independent of user requests.

---

## Repo & deploy layout

Single new repo, `console-website-rust` (cloned from one of the existing 5 sites, same pattern as [replicate.md](replicate.md) prescribes):

```
console-website-rust/
  src/
    main.rs                  router + LaunchBuilder
    pages/
      overview.rs            /              all sites grid
      site_detail.rs         /<site>        per-site drill-down
      site_health.rs         /<site>/health
      site_perf.rs           /<site>/perf
      site_seo.rs            /<site>/seo
      site_visitors.rs       /<site>/visitors
      site_alerts.rs         /<site>/alerts
      settings.rs            /settings       config + auth
      apis.rs                /settings/apis  central API credentials view
    components/
      site_card.rs           one row in the overview grid
      kpi_tile.rs            number + label + spark + delta
      uptime_history.rs      24h heatmap
      lighthouse_gauge.rs    LCP / INP / CLS dials
      alert_chip.rs          red/amber/green pill
      refresh_button.rs      triggers fetch + spinner
    api.rs                   fetch wrappers around /api/* endpoints
    config.rs                parses sites.toml at compile time
    auth.rs                  token storage + bearer header injection
  sites.toml                 the 5 sites to monitor (see Site config schema)
  api/                       Rust serverless source
    Cargo.toml
    src/
      handler.rs             one binary, one entry point, route on path
      checks/
        uptime.rs
        ssl.rs
        dns.rs
        seo.rs               our own SEO validator
        observatory.rs       Mozilla Observatory wrapper
        psi.rs               PageSpeed Insights wrapper
      upstreams/
        ga4.rs               GA4 Data API client
        search_console.rs    Search Console API client
        amplify.rs           aws-sdk-amplify wrapper
      cache.rs               S3 or KV read/write
      auth.rs                bearer token verification
  infra/
    lambda.tf  or  wrangler.toml   deployment for the serverless layer
    cron.tf                         schedule definitions for cache refresh
  scripts/
    build-ssg.sh             same as the 5 sites
    deploy-api.sh            cargo lambda deploy   OR   wrangler deploy
  customHttp.yml             same headers pattern as other sites
  amplify.yml                same build spec
```

The frontend ships exactly like the other 5 sites. The `api/` directory is a separate cargo workspace member deployed independently to the serverless host.

---

## UI tabs

### Overview (`/`)
5-site grid. One row per site. Columns:

| Site | Status | Uptime 24h | LCP | SEO score | Visitors 24h | Last deploy | Alerts |
|---|---|---|---|---|---|---|---|

Each row clicks through to the site detail.

### Site detail (`/<site>`)
Header with site name + canonical URL + "Refresh all" button + last-checked timestamp.
Sub-tabs:

- **Health** — status, response time, SSL cert (days remaining), DNS records, redirect chain (www ↔ apex), security headers grade, uptime history (24h heatmap).
- **Performance** — Core Web Vitals (LCP / INP / CLS) field + lab, bundle size budget (WASM, CSS, total), Lighthouse score trend.
- **SEO** — self-built checks (canonical, OG full set, sitemap reachable, robots.txt valid, H1 count per page, broken internal links), Search Console (top queries, impressions, clicks, CTR, average position, indexing coverage, sitemap submission status), and content inventory (article count, days since last publish, articles below 800 words, articles missing alt text — sourced from `/llms-full.txt`) with per-article "Edit on GitHub" deep-links for fast typo fixes.
- **Visitors** — GA4: realtime active users, 24h / 7d / 30d sessions, top pages, top sources, country breakdown (world map), device split, conversion events.
- **Alerts** — derived rollup: cert expiring < 30 days, domain expiring < 60 days, uptime incidents in last 24h, SEO score drop > 10%, broken link found, missing OG tag on a new page.

### Settings (`/settings`)
- **Sites** — add/remove sites (edits `sites.toml` via PR to the console repo).
- **APIs** — see central API panel below.
- **Auth** — bearer token used by frontend → API layer.
- **Notifications** — webhook URL for alerts (Slack / Discord / email-via-Lambda).

### Central APIs panel (`/settings/apis`)
Single screen listing every upstream API, with status indicator (configured / unconfigured / failing) and a "Test connection" button. **This is the centralized API tab the rest of this document references**; the full machine-readable list is in the next section.

---

## Features (free / build-yourself only)

### Health

- Status code + response time + redirect chain — Rust `reqwest`, on demand.
- SSL certificate expiry — `rustls` socket peek, on demand.
- DNS A/AAAA/CNAME/MX/TXT lookup — `hickory-resolver`, on demand.
- DKIM / SPF / DMARC presence + format validation — built on top of `hickory-resolver`.
- Security headers grade — Mozilla Observatory API.
- HSTS preload status — `https://hstspreload.org/api/v2/status?domain=` (free).
- TLS configuration grade — SSL Labs API (free, rate-limited; cache result 24h).
- Uptime history — store every `/api/check/uptime` response in S3 (`uptime/<site>/<yyyy-mm-dd>.jsonl`), render last 24h as a heatmap.
- Domain registration expiry — RDAP query (`https://rdap.org/domain/<domain>`).

### Performance

- Core Web Vitals — PageSpeed Insights API (free, 25k/day).
- Bundle size budget — parse the deployed `target/.../public/` listing (or hit Amplify's build artifacts); render WASM / CSS / total bytes as gauges.
- Lighthouse score trend — sample PSI daily, persist `lighthouse/<site>/<yyyy-mm-dd>.json` in S3, plot in the UI.
- Image weight per article — walk `public/articles/<slug>/` listings via the deployed site index.

### SEO

- Self-built validator running on every check:
  - Canonical tag present, absolute, matches request URL
  - Full OG + Twitter tag set per [README — Open Graph & Twitter Card standard](../README.md)
  - `og:image` resolves with 2xx
  - `sitemap.xml` reachable, valid XML, every `<loc>` returns 2xx
  - `robots.txt` reachable, references sitemap
  - `<title>` length 30–60 chars, `<meta description>` length 120–160 chars
  - Exactly one `<h1>` per page
  - No mixed-content or insecure resource references
- Broken internal link crawler — `lychee` crate, on-demand button (slow).
- Schema.org JSON-LD validity — Google Rich Results Test API (free).
- Search Console — clicks, impressions, CTR, position per query/page; indexing coverage; sitemap submission state; mobile usability issues.
- Bing Webmaster Tools — same metrics for Bing (free, often-ignored 5–10% of traffic).

### Visitors

- GA4 Data API — realtime active users (last 30 min), sessions / pageviews / users at 24h / 7d / 30d windows, top pages, top traffic sources, country breakdown, device categories, conversion events.
- Cloudflare Web Analytics API — overlay if a site is behind CF; useful for verifying GA4 numbers against server-side counts.
- Referrer breakdown — derived from GA4 source/medium dimension.

### Deploys & content health

- AWS Amplify API — last 5 deploys with status + duration + commit SHA + deployer. Powers the "Last deploy" column on the overview grid and the deploy history card on the Health tab.
- Content inventory — for each site, hit `/llms-full.txt` (we already ship this — see [README — AEO & LLM Discoverability](../README.md)) and compute: article count, latest publish date, days since latest publish, total word count. Surfaced on the SEO tab.
- Edit-in-GitHub link per article — deep-link `github.com/<repo>/edit/main/content/articles/<slug>.md` so a typo fix is one click. Static URL, no API required.

### Social listening (free)

- Reddit search API (`https://www.reddit.com/search.json?q=mata.network`) — anonymous, rate-limited but free.
- Bluesky search API (`https://api.bsky.app/xrpc/app.bsky.feed.searchPosts`) — free, anonymous, no auth.
- Mastodon search API — needs an instance, anonymous queries allowed.
- HackerNews Algolia search (`https://hn.algolia.com/api/v1/search?query=`) — free.

### Newsletter / forms (free with platform plan)

- Beehiiv API — subscribers, opens, clicks per post.
- Klaviyo API — list size, recent form submissions.

### Alerts (derived, no API)

Rules engine running on cached data. Each rule emits zero or one alert per `/api/alerts` response:

- `cert_expiring` — SSL cert expires in < 30 days
- `domain_expiring` — registration expires in < 60 days
- `downtime` — any `/api/check/uptime` returned non-2xx in last 24h
- `seo_drop` — Search Console clicks ↓ > 30% week-over-week
- `lcp_regression` — LCP > 2.5s after being < 2.5s last week
- `broken_link` — internal 4xx/5xx found by crawler
- `missing_og` — any page missing `og:image`
- `redirect_chain_loop` — uptime check found a redirect loop or > 3 hops
- `mixed_content` — HTTP resource on an HTTPS page
- `sitemap_drift` — sitemap URL count changed by > 20% since last check

---

## Required APIs — centralized reference

This table is the source of truth for the Console's upstream integrations. Every entry is free at our usage volume.

| # | API | Purpose | Auth | Rate limit | Rust client | Setup |
|---|---|---|---|---|---|---|
| 1 | **Google Analytics 4 Data API** (`analyticsdata.googleapis.com`) | Sessions, users, top pages, sources, countries, events | Service account JSON, scope `analytics.readonly` | 25k req/day/project | `reqwest` + `yup-oauth2` | GCP project → enable API → create service account → grant `Viewer` on each GA4 property |
| 2 | **Google Search Console API** (`searchconsole.googleapis.com`) | Queries, impressions, clicks, CTR, position, indexing | Same service account, scope `webmasters.readonly` | ~1.2k req/min | `reqwest` + `yup-oauth2` | Add the service account email as a "Viewer" user in Search Console for each domain property |
| 3 | **Google PageSpeed Insights API** (`pagespeedonline.googleapis.com/pagespeedonline/v5/runPagespeed`) | Core Web Vitals (lab + field), Lighthouse score | API key (URL query param) | 25k req/day | `reqwest` | GCP project → API key → restrict to PSI |
| 4 | **Google Rich Results Test API** (unofficial; or use the [structured-data-testing-tool](https://github.com/google/structured-data-testing-tool) parser locally) | Validate JSON-LD against Google's parser | None for the unofficial; can also validate locally with a Rust schema crate | n/a | local Rust impl + cached fallback | No external setup needed if running locally |
| 5 | **Mozilla Observatory API** (`http-observatory.security.mozilla.org/api/v1/analyze`) | Security headers grade (CSP, HSTS, XFO, etc.) | None | ~1 req/min/host | `reqwest` | None |
| 6 | **SSL Labs API** (`api.ssllabs.com/api/v3/analyze`) | TLS configuration grade | None | Polite — ~1 req/host every few hours; cache 24h | `reqwest` | None |
| 7 | **HSTS Preload Status** (`hstspreload.org/api/v2/status`) | HSTS preload eligibility | None | Generous | `reqwest` | None |
| 8 | **RDAP** (`rdap.org/domain/<domain>` or registry-specific) | Domain registration expiry | None | Generous | `reqwest` | None |
| 9 | **AWS Amplify API** (`amplify.<region>.amazonaws.com`) | Build status, deploy history, app metadata | IAM user with `amplify:Get*`, `amplify:List*` (read-only) | High | `aws-sdk-amplify` | IAM user → narrow policy → access key + secret in serverless env |
| 10 | **AWS S3 / Cloudflare KV** | Cache storage for slow upstreams + uptime history | IAM / API token | High | `aws-sdk-s3` or `worker-kv` | Bucket / KV namespace + read/write IAM |
| 11 | **Bing Webmaster Tools API** (`ssl.bing.com/webmaster/api.svc/json`) | Bing search analytics, crawl issues | API key | Generous | `reqwest` | Bing Webmaster → Settings → API access → generate key |
| 12 | **Cloudflare Web Analytics API** (`api.cloudflare.com/client/v4/zones/<zone>/analytics/dashboard`) | Server-side pageviews, top hosts, top paths | API token | High | `reqwest` | Cloudflare → My Profile → API Tokens → Analytics: Read |
| 13 | **UptimeRobot API** (`api.uptimerobot.com/v2/getMonitors`) — *optional, layered alerting only* | Sub-minute pings + alert webhooks | API key | 10 req/min | `reqwest` | UptimeRobot → My Settings → API |
| 14 | **Better Stack API** (`uptime.betterstack.com/api/v2/monitors`) — *optional alternative to #13* | Same | Bearer token | Generous | `reqwest` | Better Stack → API tokens |
| 15 | **Beehiiv API** (`api.beehiiv.com/v2`) — *if newsletter present* | Subscribers, opens, clicks | API key | Generous on free plan | `reqwest` | Beehiiv → Integrations → API |
| 16 | **Klaviyo API** (`a.klaviyo.com/api`) — *if forms present* | List size, recent submissions | API key (revision-pinned) | Generous | `reqwest` | Klaviyo → Account → Settings → API Keys |
| 17 | **Reddit JSON API** (`reddit.com/search.json`) | Brand mentions | None (anonymous) | ~60 req/min | `reqwest` | None |
| 18 | **Bluesky API** (`api.bsky.app/xrpc/app.bsky.feed.searchPosts`) | Brand mentions | None | Generous | `reqwest` | None |
| 19 | **HackerNews Algolia API** (`hn.algolia.com/api/v1/search`) | Brand mentions | None | Generous | `reqwest` | None |
| 20 | **`hickory-resolver`** (Rust crate, not a remote API) | DNS lookup, DKIM/SPF/DMARC validation | n/a | Local | `hickory-resolver` | `cargo add hickory-resolver` |
| 21 | **`rustls` / `tokio-rustls`** (Rust crate) | SSL cert inspection | n/a | Local | `rustls` | `cargo add rustls tokio-rustls` |
| 22 | **`lychee`** (Rust crate) | Broken link crawling | n/a | Local | `lychee_lib` | `cargo add lychee-lib` |

### Auth secrets summary

The serverless layer needs exactly these environment variables (or KV secrets):

| Env var | Source |
|---|---|
| `GOOGLE_SERVICE_ACCOUNT_JSON` | GCP service account JSON (covers #1, #2) |
| `GOOGLE_API_KEY` | PageSpeed Insights key (#3) |
| `AWS_ACCESS_KEY_ID` + `AWS_SECRET_ACCESS_KEY` | IAM user (#9, #10) — or use IAM role if Lambda |
| `BING_WEBMASTER_KEY` | Bing Webmaster API key (#11) — optional |
| `CLOUDFLARE_API_TOKEN` | Cloudflare token (#12) — optional |
| `UPTIMEROBOT_KEY` *or* `BETTERSTACK_TOKEN` | Optional alerting (#13 / #14) |
| `BEEHIIV_KEY`, `KLAVIYO_KEY` | Optional (#15, #16) |
| `CONSOLE_BEARER_TOKEN` | The single token the frontend sends to authenticate; rotates by redeploying the Lambda env |

---

## Site configuration schema

`sites.toml` at the repo root is the per-site config. Adding a 6th site is one block:

```toml
[[sites]]
domain          = "mata.network"
display_name    = "MATA Network"
canonical       = "https://mata.network"
www_redirects   = true                                    # expected www → apex 301
github_repo     = "matanetwork/mata-website-rust"
amplify_app_id  = "d1u80cmofj0jmo"                        # for deploy history
ga4_property_id = "properties/123456789"
sc_property     = "sc-domain:mata.network"
bing_site       = "https://mata.network/"
newsletter      = { provider = "beehiiv", publication_id = "..." }   # optional
expected_headers = ["Content-Security-Policy", "Strict-Transport-Security", "X-Frame-Options"]

[[sites]]
domain          = "hfga.io"
display_name    = "Hemp Fiber & Grain Association"
# ...
```

The Console parses this at compile time via `build.rs` (same pattern as article loading in [build.rs](../build.rs)), so the static shell knows the site list without an API call.

---

## Auth model

**Interim — Phases 1–5 ship with this:**

- **Frontend → serverless layer:** single bearer token, stored in `localStorage` after a one-time paste from a settings page or env. Sent as `Authorization: Bearer <token>` on every `/api/*` request.
- **Token validation:** serverless handler compares to `CONSOLE_BEARER_TOKEN` env var, constant-time compare.
- **No user accounts.** Single-operator dashboard. A second operator gets the same token.
- **Rotation:** redeploy the serverless env with a new token; paste it once in the dashboard.
- **CORS:** allow only `https://console.mata.network` as the origin.
- **Rate-limit per token:** soft cap of ~60 req/min/token in the handler to prevent runaway WASM bugs from exhausting upstream quotas.

If a public/read-only mode is wanted later, expose a separate `/api/public/*` namespace with no live checks and cached-only data, no auth.

The shared-bearer model is intentionally a stopgap. It scales to ~2–3 trusted operators and breaks down past that — no per-operator identity, no scoped access, no signed audit trail, no clean revocation when someone leaves. The follow-on is the next section.

---

## Future — MATA KMS-backed access control

Once MATA's KMS Phase 2 ships, the Console replaces the shared bearer token with **per-operator authentication via the MATA sovereign identity stack**. This is a planned Phase 6 (see Build phases), gated on specific KMS milestones — *not* part of the MVP. Capturing the plan here so the MVP design doesn't paint us into a corner.

### Background — what MATA KMS gives us

The full design lives in [mata-master/docs/plans](file:///Users/talmond/mata-master/docs/plans). What matters for the Console:

- **Three-role model.** *User* owns the keys, *MATA* notarizes ("yes this DID is real"), *Verifier* (= the Console serverless layer, in this case) checks signatures with no MATA contract required. MATA never holds private keys.
- **Per-user DID.** Every authenticated MATA user has `did:mata:<user_uuid>` with two keypairs: `#root` (ECDSA P-256, signing) and `#encryption` (ECDH P-256, inbound encryption). Self-signed today (M2-Structural, shipped 2026-04-28); MATA-attested once Phase 2 Milestone 1 ships.
- **Capability certificates** (Phase 2 Milestone 4). Signed delegations of the form *"DID X is authorized to perform scopes Y on resource Z until time T,"* signed by an admin's `#root`. Standard format, third-party-verifiable, expire automatically.
- **Three-layer revocation** (Phase 2 Milestone 3). Key expiry → revocation list → attestation withdrawal. Pull an operator's access by adding their DID to the revocation list.
- **OIDC provider** (Phase 2 Milestone 4). "Sign in with MATA" works on any OIDC-capable service, including this Console.
- **Subordinate keys.** Operators can derive a Console-scoped subordinate key from their vault key — `#console-admin` — without exposing `#root`. Compromise of the scoped key only loses Console access.

### How this maps to Console operations

| Console concept | MATA KMS primitive |
|---|---|
| Operator identity | `did:mata:<uuid>` — fetched from the Console's allowlist, verified per request |
| Sign in | "Sign in with MATA" via the KMS OIDC endpoint (Phase 2 M4); returns an ID token containing the operator's DID |
| Authorization | Capability certificate signed by an admin DID — embeds scopes (`sites:read`, `sites:write`, `alerts:ack`, `apis:rotate`) and expiry |
| Privileged action audit | Operator's `#console-admin` key signs the request payload; serverless layer stores `{ payload, signature, did, timestamp }` as an append-only log in S3/KV |
| Operator removal | Add the operator's DID to the Console's revocation list (which mirrors the MATA-wide list); serverless layer denies the next request |
| Multi-operator | Each operator has their own DID; no shared secret to rotate when one leaves |
| Federated verification | Console's verifier is the `mata-identity-verify` crate (Phase 2 M1 deliverable) — `cargo add` + two cached HTTP fetches; no MATA API contract needed |

### Concrete login flow (post-Phase 2)

1. Operator visits `https://console.mata.network` for the first time.
2. Clicks **"Sign in with MATA"** → redirects to `https://my.mata.network/oidc/authorize?client_id=console&scope=did&redirect_uri=...` (Phase 2 M4 endpoint).
3. MATA app authenticates the operator with their existing WebAuthn + passphrase flow (see [mata-master/AUTH_FLOW.md](file:///Users/talmond/mata-master/AUTH_FLOW.md)) — Console never sees the user's vault key.
4. MATA redirects back with an ID token: `{ sub: "did:mata:<uuid>", aud: "console", iat, exp, ... }`, signed by MATA's attestation key.
5. Console's serverless layer verifies the token against `/.well-known/mata-pki-roots.json` (Phase 2 M1 artifact) and looks the DID up in `console-acl.toml` (next section).
6. Console issues a short-lived session JWT scoped to the matched capability cert. The session JWT is what every subsequent `/api/*` request carries — no need to re-do the OIDC dance per call.

### `console-acl.toml` — access control list

Replaces the single `CONSOLE_BEARER_TOKEN` env var. Lives in the Console repo, edited by PR:

```toml
[[operators]]
did             = "did:mata:11111111-2222-3333-4444-555555555555"
display_name    = "Tim"
role            = "admin"                    # admin | editor | reader
scopes          = ["*"]                      # full access
expires_at      = "2027-01-01T00:00:00Z"     # capability-cert expiry
added_by        = "self"
added_at        = "2026-05-12T00:00:00Z"

[[operators]]
did             = "did:mata:66666666-7777-8888-9999-aaaaaaaaaaaa"
display_name    = "Contractor"
role            = "reader"
scopes          = ["sites:read", "checks:run", "alerts:read"]
expires_at      = "2026-08-01T00:00:00Z"     # 90-day contractor window
added_by        = "did:mata:11111111-..."    # admin who issued the cert
added_at        = "2026-05-12T00:00:00Z"
```

The serverless layer reads this at cold-start, refreshes every 5 min, and consults the MATA revocation list (Phase 2 M3) on every request to drop revoked DIDs in-flight.

### Per-action signing (audit trail)

For destructive actions (rotate an API key, edit `sites.toml`, acknowledge an alert), the WASM frontend asks the operator's MATA wallet to sign the action payload with their `#console-admin` key before sending:

```
POST /api/admin/rotate-api-key
Authorization: Bearer <session-jwt>
X-Mata-Action-Sig: base64(ecdsa_p256(sha256("rotate-api-key|github|2026-05-12T12:34:56Z")))
```

Server verifies the signature against the operator's DID document. Every audited action lands in an append-only `s3://console-audit/<yyyy>/<mm>/<dd>.jsonl` log. Tamper-evident; signed by the actual operator, not the Console itself.

### Scopes the Console should define

| Scope | What it permits |
|---|---|
| `sites:read` | View overview grid + site detail tabs |
| `sites:write` | Edit `sites.toml`, enroll/remove sites |
| `checks:run` | Press refresh / run live checks |
| `alerts:read` | View the alerts feed |
| `alerts:ack` | Acknowledge / silence alerts |
| `apis:read` | View central APIs panel (status only, no secrets) |
| `apis:rotate` | Rotate upstream API keys / tokens |
| `audit:read` | View signed-action log |
| `acl:write` | Add/remove operators (admin-only) |

`role = "admin"` maps to `scopes = ["*"]`; `editor` covers everything except `acl:write` and `apis:rotate`; `reader` is the bottom three. Custom roles via explicit scope arrays.

### What's still gated (don't promise these before they ship)

| Console feature | Blocked by KMS milestone | Workaround until then |
|---|---|---|
| "Sign in with MATA" button | Phase 2 M1 (attestation infrastructure) + Phase 2 M4 (OIDC endpoint) | Bearer token |
| Verifier library (`mata-identity-verify`) | Phase 2 M1 (verifier crate completion) | Vendor a stub that only validates self-signed DIDs |
| Capability certificates | Phase 2 M4 | Roles hard-coded in `console-acl.toml` |
| Revocation list | Phase 2 M3 | Manual removal from `console-acl.toml` + redeploy |
| Per-operator subordinate keys | Phase 2 M4 (subordinate-key derivation UI) | Operator's `#root` signs directly (slightly weaker isolation) |

In short: the Console **can** ship a "DIDs in `console-acl.toml`" first cut on top of just Phase 1 (M2-Structural, already shipped). Sign-in is still the bearer token at that stage, but every action is signed by the operator's `#root`, and the audit trail becomes real. Full OIDC + scoped caps fall in once Phase 2 M1 + M4 land.

### Why this is a good fit for the Console specifically

- **Tech-stack alignment.** Same Rust, same WASM, same Dioxus, same ECDSA P-256 the Console already uses elsewhere. No new crypto stack.
- **Brand alignment.** The Console monitors MATA's brand sites; eating our own KMS as the access layer is the strongest possible demo to partners.
- **No SaaS dependency.** Auth0/Okta/Clerk would all work, but they're paid + opinionated + custodial — exactly the failure modes MATA is built to replace. Using MATA KMS keeps the Console on the [Tech stack: Dioxus + Rust + WASM only] constraint.
- **Reference integration.** Phase 2 Milestone 5 calls for "at least 3 reference services accept Sign in with MATA at GA." The Console is the canonical first one.

---

## Build phases

### Phase 1 — MVP (week 1)

Goal: replace any "is the site up?" anxiety + see SEO/uptime regressions before the user does.

- New repo cloned from one of the 5 sites
- Overview grid + Health tab only
- Endpoints: `/api/sites`, `/api/check/uptime`, `/api/check/seo`, `/api/check/security`
- APIs wired: #5 (Observatory), #6 (SSL Labs), #8 (RDAP), #21 (`hickory-resolver`), #22 (`rustls`)
- Storage: S3 JSON only for uptime history
- Auth: bearer token
- Deploy: Lambda Function URLs + Amplify SSG

### Phase 2 — Visitors + SEO depth (week 2)

- Visitors tab (GA4)
- SEO tab (Search Console + self-built validator + broken link crawler)
- Performance tab (PageSpeed Insights button)
- APIs wired: #1 (GA4), #2 (Search Console), #3 (PSI), #4 (Rich Results), #23 (`lychee`)
- Cron Lambda for daily GA4 + SC refresh

### Phase 3 — Activity + alerts (week 3)

- Activity tab (GitHub + Amplify + content inventory)
- Alerts tab + rules engine
- Notifications webhook (Slack/Discord/email)
- APIs wired: #9 (GitHub), #10 (Amplify)

### Phase 4 — Bells & whistles (week 4+)

- World map visitor viz (GA4 country → SVG)
- Lighthouse trend chart (PSI sampled daily)
- Social listening cards (Reddit / Bluesky / HN)
- Newsletter stats (Beehiiv)
- Bing parity (Bing Webmaster)
- CF Web Analytics overlay (if any site moves behind CF)
- APIs wired: #12, #13, #16, #17, #18, #19, #20

### Phase 5 — Optional alerting layer

- UptimeRobot or Better Stack for sub-minute paging
- Console reads their API to surface incidents but does NOT replace them as the alerting source
- APIs wired: #14 or #15

### Phase 6 — MATA KMS-backed access control (gated on `mata-master` Phase 2)

Replaces the shared bearer token with per-operator authentication via MATA's sovereign identity stack. See the **Future — MATA KMS-backed access control** section above for the full design.

Sub-phases mirror the upstream KMS milestones:

- **6a — DID allowlist (immediately, on top of KMS Phase 1 M2-Structural which is already shipped):** Add `console-acl.toml`, swap the bearer token for "session JWT issued after operator pastes their DID + signs a challenge with their `#root` key." Per-action signing + audit log live from here on.
- **6b — Verifier library:** Replace the hand-rolled DID-document parser with [`mata-identity-verify`](file:///Users/talmond/mata-master/docs/plans/phase%202%20KMS%20implementation%20mission.md) once it ships. Gates on Phase 2 M1.
- **6c — "Sign in with MATA":** Wire the OIDC button on `/login` → MATA's `/oidc/authorize` endpoint. Gates on Phase 2 M4.
- **6d — Capability certificates:** Replace hard-coded role → scope mapping with capability certs signed by admin DIDs. Gates on Phase 2 M4.
- **6e — Live revocation:** Subscribe to MATA's revocation list; deny revoked DIDs in-flight. Gates on Phase 2 M3.
- **6f — Subordinate-key derivation UI:** Let operators issue a Console-scoped `#console-admin` subordinate key from their wallet rather than signing directly with `#root`. Gates on Phase 2 M4 UI work.

---

## Open decisions

These need a call before Phase 1 starts:

1. **Serverless host: AWS Lambda (Function URLs) vs. Cloudflare Workers.** Lambda matches the existing AWS footprint; Workers gives effectively zero cold-start and free-tier-friendlier pricing. Both fully support Rust.
2. **Cache storage: S3 vs. CF KV vs. CF R2.** Tied to #1.
3. **Console subdomain: `console.mata.network` vs. dedicated `.io` / `.dev`.** Subdomain keeps brand cohesion; dedicated domain makes the auth boundary cleaner.
4. **Read-only public mirror?** If yes, design `/api/public/*` from day 1 with no auth + cached-only.
5. **Cron host:** EventBridge Scheduler (if Lambda) vs. CF Cron Triggers (if Workers) — same answer as #1.
6. **Where the bearer token lives:** `localStorage` (simple) vs. session-only `sessionStorage` (re-paste on every tab) vs. HTTP-only cookie set by a sign-in route (more infra, more secure).
7. **Alerts destination:** Slack webhook, Discord webhook, email-via-SES, or all three with toggles per rule.
8. **Phase 6 trigger:** ship Phase 6a (DID allowlist on top of already-shipped KMS M2-Structural) opportunistically alongside MVP, or hold all KMS work until Phase 2 M1 + M4 land in `mata-master`? Risk of doing 6a early is rework when capability certs land; benefit is per-operator audit trail from day one.

---

## What the Console explicitly does *not* do

To stay in scope and free-tier:

- **Not a CMS.** No editor UI, no draft management, no preview — articles still live in markdown in each site's repo. Edit-in-GitHub links cover the typo-fix path.
- **Not an analytics platform.** GA4 / Search Console / Cloudflare own the raw event store; the Console aggregates and surfaces.
- **Not a paging system.** UptimeRobot / Better Stack page the operator; the Console shows the current state.
- **Not a multi-tenant SaaS.** Single-operator dashboard in the MVP; two operators share one token. Per-operator identity with scoped access lands in Phase 6 once MATA KMS Phase 2 ships (see **Future — MATA KMS-backed access control**).
- **No paid integrations.** Ahrefs / Semrush / Moz / DataForSEO / Pingdom / SimilarWeb — out of scope. If they're worth adding later, that's a Phase 6+ decision and a separate doc.

---

## Reference — files this depends on

- [replicate.md](replicate.md) — the playbook the Console itself is built from.
- [replicate.md → Phase 8 — Console intake](replicate.md#phase-8--console-intake-post-deploy) — the per-site intake checklist that feeds `sites.toml`.
- [README.md — Open Graph & Twitter Card standard](../README.md) — the SEO spec the validator checks against.
- [README.md — AEO & LLM Discoverability](../README.md) — `/llms-full.txt` is the easy content-inventory hook.
- [customHttp.yml](../customHttp.yml) — the security-headers spec the Console expects every monitored site to ship.
- [src/seo.rs](../src/seo.rs) — the canonical/OG/JSON-LD source the Console's self-built SEO validator mirrors.

**Cross-repo references — MATA KMS (for the Phase 6 access-control work):**

- [`mata-master` / AUTH_FLOW.md](file:///Users/talmond/mata-master/AUTH_FLOW.md) — the WebAuthn + passphrase + vault-key flow that "Sign in with MATA" wraps.
- [`mata-master` / docs/plans/(done) phase 1 KMS implementation mission.md](file:///Users/talmond/mata-master/docs/plans/) — what's shipped (M1, M2-Structural, M3-Structural, M4). The Console can use M2-Structural primitives today.
- [`mata-master` / docs/plans/phase 2 KMS implementation mission.md](file:///Users/talmond/mata-master/docs/plans/) — attestation infrastructure (M1), inbox UI (M2), revocation & rotation (M3), SSO/OIDC + capability certs (M4), ecosystem SDKs (M5), advanced (M6). The Console's Phase 6 sub-phases (6a–6f) map onto these milestones.
- [`mata-master` / docs/business cases/KMS platform value and business cases.md](file:///Users/talmond/mata-master/docs/business%20cases/) — the embassy metaphor + activation guide; useful framing when explaining KMS-based access to a non-MATA stakeholder.
- [`mata-master` / docs/audits/m4-kms-conformance.md](file:///Users/talmond/mata-master/docs/audits/) + [runbooks/kms-conformance-checklist.md](file:///Users/talmond/mata-master/docs/runbooks/) — the conformance bar the Console's KMS integration must clear before going live.
