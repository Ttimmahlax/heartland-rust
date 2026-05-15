# Deployment — heartland.io on AWS Amplify + CloudFront + Lambda@Edge

The production stack that replaces SiteGround/WordPress.

- **Amplify Hosting** serves the heartland-rust static build (English + all routes
  defined in [`src/main.rs`](../src/main.rs))
- **CloudFront** is the public front door (TLS termination, edge cache)
- **Lambda@Edge** routes `/<lang>/<path>` requests through the GTranslate
  translation network instead of Amplify
- **Cloudflare** handles DNS (no proxy — gray cloud)
- **GTranslate** hosts translations at `*.tdn.gtranslate.net` (the proxy upstream
  is the same service the WordPress plugin currently uses)

---

## Architecture

```
                          ┌────────────────────────────┐
                          │   user types heartland.io  │
                          └─────────────┬──────────────┘
                                        │
                                        ▼
                         ┌─────────────────────────────┐
                         │   Cloudflare DNS            │
                         │   heartland.io →            │
                         │     d1abc234.cloudfront.net │   DNS-only (gray cloud)
                         └─────────────┬───────────────┘
                                       │
                                       ▼
                       ┌───────────────────────────────┐
                       │   AWS CloudFront distribution │
                       │   (yours, us-east-1)          │
                       │                               │
                       │   • TLS: ACM cert             │
                       │   • Single behavior           │
                       │   • Lambda@Edge attached      │
                       │     as Origin Request handler │
                       └────┬────────────────────┬─────┘
                            │                    │
            request URI is  │                    │  request URI is
            /<lang>/<path>  │                    │  anything else
            (Lambda rewrites│                    │  (default origin)
             origin → GT,   │                    │
             strips lang)   │                    │
                            ▼                    ▼
              ┌────────────────────────┐  ┌──────────────────────────┐
              │ <server>.tdn.          │  │  AWS Amplify Hosting     │
              │   gtranslate.net       │  │  main.d1234abcd          │
              │ (translation upstream) │  │   .amplifyapp.com        │
              └────────────────────────┘  │  (heartland-rust build)  │
                                          └──────────────────────────┘
```

## Active languages (18)

`ar`, `bn`, `de`, `es`, `fr`, `hi`, `it`, `ja`, `ko`, `nl`, `pa`, `pl`, `pt`,
`tr`, `ur`, `vi`, `zh-CN`. English is the bare path (no prefix).

Source of truth: [`src/i18n.rs`](../src/i18n.rs) `Language::ALL`. **Any change
to the language list must update both the Rust module and the Lambda function
below.**

## Cost (rough monthly)

| Service | Cost |
|---|---|
| Amplify Hosting | $0–5 (free tier covers most static-site traffic) |
| CloudFront | $1–5 (free tier covers first 1 TB/mo) |
| Lambda@Edge | $0–1 (free tier covers most invocations) |
| ACM certificate | Free |
| Cloudflare DNS | Free |
| GTranslate plan | Unchanged from today |
| **AWS total** | **~$5/mo** |

Cheaper than Lightsail, no server to maintain, real edge CDN.

---

## Prerequisites

Before starting:

- [ ] AWS account with admin access (or IAM user with permissions for Amplify,
      Lambda, IAM, CloudFront, ACM)
- [ ] Cloudflare account managing `heartland.io` DNS
- [ ] GitHub repo (`heartland-rust`) connected to your AWS account, or willing
      to grant Amplify access during setup
- [ ] GTranslate dashboard access (to update origin server later)
- [ ] AWS CLI installed locally and configured (`aws configure`) — optional but
      recommended for the Lambda deploy step
- [ ] Local `dx bundle --release --platform web` produces a working artifact
      (verify before relying on Amplify to build it)

---

## Phase 1 — Amplify hosts the heartland-rust build

**Goal:** every push to `main` builds the Rust app and publishes a static origin URL.

### 1.1 Create the Amplify app

1. Amplify console → **Create new app** → **Host a web app** → connect GitHub
2. Pick `heartland-rust`, branch `main`
3. **Do not** add a custom domain in Amplify. CloudFront is the front door — Amplify just needs to be reachable at its default `*.amplifyapp.com` URL

### 1.2 Configure the build

Add `amplify.yml` at the repo root (Amplify reads this on every build):

```yaml
version: 1
frontend:
  phases:
    preBuild:
      commands:
        - curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
        - source "$HOME/.cargo/env"
        - rustup target add wasm32-unknown-unknown
        - cargo install dioxus-cli --version 0.6 --locked
    build:
      commands:
        - source "$HOME/.cargo/env"
        - dx bundle --release --platform web
  artifacts:
    baseDirectory: target/dx/heartland-website/release/web/public
    files:
      - '**/*'
  cache:
    paths:
      - $HOME/.cargo/**/*
      - target/**/*
```

> The `baseDirectory` path is where Dioxus 0.6 writes the bundle. Verify locally
> with `dx bundle --release --platform web` before committing — Dioxus has changed
> output paths between versions.

### 1.3 Record the Amplify domain

After the first successful build, note the URL Amplify gives you. Something like:

```
main.d2xxxxxxxxxxxx.amplifyapp.com
```

CloudFront will use this as its primary origin.

### 1.4 Verify the build works

Hit the Amplify URL directly. The home page should load. Try a few routes:

```
https://main.d2xxxxxxxxxxxx.amplifyapp.com/
https://main.d2xxxxxxxxxxxx.amplifyapp.com/about
https://main.d2xxxxxxxxxxxx.amplifyapp.com/sustainability-news/heartland-raises-seed-capital
```

If any 404, fix in heartland-rust before continuing. Once CloudFront is in
front, those 404s will look like production outages.

---

## Phase 2 — ACM certificate (us-east-1)

CloudFront requires its certificate in **us-east-1**, regardless of where the
rest of your AWS resources live.

1. ACM console → make sure region is **N. Virginia (us-east-1)**
2. **Request certificate** → Public → domain names:
   - `heartland.io`
   - `www.heartland.io`
3. Validation method: **DNS validation**
4. ACM gives you a CNAME for each domain — something like:
   - Name: `_abc123.heartland.io`
   - Value: `_def456.acmvalidations.aws`
5. Add each CNAME to Cloudflare DNS with **proxy OFF (gray cloud)** — orange
   cloud (proxy on) breaks ACM validation because Cloudflare rewrites the
   record values
6. Wait 5–30 min. Certificate status flips from `Pending validation` → `Issued`

---

## Phase 3 — Lambda@Edge function

The brain of the routing. ~70 lines of Node.js.

### 3.1 Create the IAM role

Lambda console → IAM (or directly in the IAM console):

1. Create role → trusted entity = AWS service → Lambda
2. After creation, edit trust policy to add `edgelambda.amazonaws.com` as a
   second principal:

   ```json
   {
     "Version": "2012-10-17",
     "Statement": [{
       "Effect": "Allow",
       "Principal": {
         "Service": ["lambda.amazonaws.com", "edgelambda.amazonaws.com"]
       },
       "Action": "sts:AssumeRole"
     }]
   }
   ```

3. Attach managed policy `AWSLambdaBasicExecutionRole` (for CloudWatch logs)
4. Name the role `heartland-edge-router-role`

### 3.2 Create the function

1. Lambda console → **us-east-1** → Create function
2. Author from scratch:
   - Name: `heartland-gtranslate-router`
   - Runtime: **Node.js 22.x**
   - Architecture: x86_64
   - Permissions: Use the role from 3.1

### 3.3 Function code

Replace the default `index.mjs` (or `index.js`) with:

```javascript
'use strict';

// Active languages — must match src/i18n.rs Language::ALL.
const LANGS = new Set([
  'ar', 'bn', 'de', 'es', 'fr', 'hi', 'it', 'ja', 'ko',
  'nl', 'pa', 'pl', 'pt', 'tr', 'ur', 'vi', 'zh-CN',
]);

// GTranslate translation-server pool. Pick one deterministically by hostname
// hash — matches the logic in vendor gtranslate.php so cache behavior is
// consistent with what the proxy did under WordPress.
const GT_SERVERS = [
  'van', 'kars', 'sis', 'dvin', 'ani', 'evn',
  'vagh', 'step', 'sis', 'tigr', 'ani', 'van',
];

const crypto = require('crypto');
function gtServerFor(hostname) {
  const stripped = hostname.replace(/^www\./, '');
  const md5 = crypto.createHash('md5').update(stripped).digest('hex');
  const idx = parseInt(md5.slice(0, 5), 16) % GT_SERVERS.length;
  return `${GT_SERVERS[idx]}.tdn.gtranslate.net`;
}

exports.handler = async (event) => {
  const request = event.Records[0].cf.request;
  const host = (request.headers.host && request.headers.host[0]?.value) || 'heartland.io';

  // Match /<lang>/<rest>. Support zh-CN which contains a hyphen.
  const m = request.uri.match(/^\/([a-z]{2}(?:-[A-Z]{2})?)(\/.*)?$/);
  if (!m) return request;

  const lang = m[1];
  const rest = m[2] || '/';

  if (!LANGS.has(lang)) return request;

  // Rewrite the request to go to the GTranslate origin.
  request.origin = {
    custom: {
      domainName: gtServerFor(host),
      port: 443,
      protocol: 'https',
      path: '',
      sslProtocols: ['TLSv1.2'],
      readTimeout: 30,
      keepaliveTimeout: 5,
      customHeaders: {},
    },
  };
  request.headers.host = [{ key: 'host', value: gtServerFor(host) }];
  // GTranslate's network keys translation by the original page path,
  // so we keep the lang in the URI but route to its server.
  request.uri = `/${lang}${rest}`;

  return request;
};
```

> The Lambda strips nothing from the URI — GTranslate's network expects
> `/<lang>/<path>` in the request to know which language to render. The
> rewrite is purely at the *origin* level: we change which server gets the
> request, not what URL the server sees.

### 3.4 Publish a version

Lambda@Edge cannot use `$LATEST`. You must publish numbered versions:

- In the function page → **Actions** → **Publish new version** → description e.g. "initial"
- Note the ARN: `arn:aws:lambda:us-east-1:<account-id>:function:heartland-gtranslate-router:1`

Every time you change the function code, publish a new version and update the
CloudFront association.

---

## Phase 4 — CloudFront distribution

### 4.1 Create distribution

CloudFront console → **Create distribution**.

**Origins (add two):**

| Setting | Amplify origin | GTranslate origin |
|---|---|---|
| Origin domain | `main.d2xxxxxxxxxxxx.amplifyapp.com` | `tdn.gtranslate.net` *(placeholder; Lambda overrides)* |
| Protocol | HTTPS only | HTTPS only |
| Origin path | *(leave blank)* | *(leave blank)* |
| Origin ID | `amplify-origin` | `gtranslate-origin` |

The GTranslate origin's actual hostname is rewritten by the Lambda on every
request, so the literal value here doesn't matter much — just needs to be a
valid HTTPS endpoint so CloudFront accepts the config.

### 4.2 Default cache behavior

- Path pattern: Default (*)
- Origin: `amplify-origin`
- Viewer protocol policy: Redirect HTTP to HTTPS
- Allowed HTTP methods: GET, HEAD, OPTIONS
- Cache policy: `CachingOptimized`
- Origin request policy: `AllViewer` (forwards all headers; needed so the
  Lambda sees `Host` to compute the right GT server)
- **Function associations** → Origin Request:
  - Function type: Lambda@Edge
  - ARN: paste the versioned ARN from Phase 3.4
  - Include body: No

### 4.3 Settings

- Price class: `Use only North America and Europe` (cheapest; bump to all
  edges if you need APAC/SA performance and the cost difference is acceptable)
- Alternate domain names (CNAMEs):
  - `heartland.io`
  - `www.heartland.io`
- SSL certificate: pick the ACM cert from Phase 2
- Security policy: TLSv1.2_2021
- Supported HTTP versions: HTTP/2 + HTTP/3
- Default root object: `index.html`
- Standard logging: enable if you want to retain access logs (small S3 cost)
- IPv6: enabled

### 4.4 Wait for deployment

CloudFront takes 5–15 min to propagate the new distribution to all edges.
Status in the console will flip from `Deploying` to `Enabled`.

Note the CloudFront domain — `d1abc234.cloudfront.net`. You'll DNS to it next.

---

## Phase 5 — Cloudflare DNS

### 5.1 Test against the CloudFront default domain first

Before touching production DNS, verify the whole stack works against the
CloudFront URL directly. See Phase 6.

### 5.2 Switch DNS records

Cloudflare dashboard → `heartland.io` zone → DNS:

1. Delete or note-and-keep the existing A/CNAME records that point at
   SiteGround (don't delete email-related MX/SPF/DKIM records — those are
   independent)
2. Add two CNAMEs:

   | Name | Value | Proxy | TTL |
   |---|---|---|---|
   | `heartland.io` | `d1abc234.cloudfront.net` | **Off (gray cloud)** | 300 |
   | `www.heartland.io` | `d1abc234.cloudfront.net` | **Off (gray cloud)** | 300 |

3. Apex CNAME works on Cloudflare via CNAME flattening (on by default).

### 5.3 Why proxy OFF (gray cloud)?

Stacking Cloudflare's proxy in front of CloudFront has almost no upside:

- **Double CDN, double cache.** Two different caching layers with different
  TTLs and invalidation paths. When you push an update, you flush both — and
  Cloudflare's free tier has limits on purges.
- **Double TLS.** Extra handshake adds latency.
- **Header rewrites.** Cloudflare in proxy mode rewrites the `Host` header in
  ways that can confuse the Lambda's hostname logic.
- **DDoS / WAF.** Both Cloudflare and CloudFront already do this.

Cloudflare DNS-only (gray cloud) means Cloudflare just resolves the name and
gets out of the way. Traffic goes directly to CloudFront, which handles the
real edge work.

### 5.4 TTL strategy

- Cutover day: 300s (5 min) so rollback is fast
- After 1 week stable: bump to `Auto` (Cloudflare picks reasonable values)

---

## Phase 6 — Validation

### 6.1 Test against CloudFront before DNS cutover

```bash
# English pass-through to Amplify
curl -sSI -H "Host: heartland.io" https://d1abc234.cloudfront.net/ | head -10
curl -sSI -H "Host: heartland.io" https://d1abc234.cloudfront.net/about | head -10
curl -sSI -H "Host: heartland.io" https://d1abc234.cloudfront.net/sustainability-news/heartland-raises-seed-capital | head -10

# Translated path → Lambda → GTranslate
curl -sSL -H "Host: heartland.io" https://d1abc234.cloudfront.net/es/ | head -50
# Expect: HTML body, content in Spanish, content-language: es header
curl -sSL -H "Host: heartland.io" https://d1abc234.cloudfront.net/zh-CN/ | head -50
# Expect: HTML body, Chinese content
curl -sSL -H "Host: heartland.io" https://d1abc234.cloudfront.net/ar/ | head -50
# Expect: HTML body, Arabic content (RTL)

# Unknown lang prefix falls through to SPA (not the proxy)
curl -sSI -H "Host: heartland.io" https://d1abc234.cloudfront.net/xx/about | head -10
# Expect: served from Amplify origin (SPA fallback renders /xx/about as a 404 page)
```

Critical that you include `-H "Host: heartland.io"` on every request — the
Lambda uses the Host header to decide which GTranslate server hashes to,
and Amplify origin behavior may depend on it too.

### 6.2 After DNS cutover

```bash
# Same tests, but against the real domain
curl -sSI https://heartland.io/ | head -10
curl -sSI https://heartland.io/es/ | head -10
curl -sSI https://heartland.io/zh-CN/ | head -10
```

### 6.3 Monitoring (first 48h)

- **CloudWatch Logs** → Lambda function `heartland-gtranslate-router` →
  watch for errors and unexpected URL patterns
- **CloudFront metrics** → 4xx rate, 5xx rate, total requests
- **Amplify hosting metrics** → request volume, build status
- **GTranslate dashboard** → page-view counts per language, cache hit ratio
- **Google Search Console** → coverage report (any sudden uptick in errors)

---

## GTranslate dashboard updates

Once CloudFront is live (Phase 4 complete), update GTranslate's configuration
before flipping DNS. **Order matters** — the new origin must be in place
before traffic starts flowing through CloudFront to GTranslate.

1. **Origin server** → change from WordPress URL to `main.d2xxxxxxxxxxxx.amplifyapp.com`
2. **Languages**:
   - Add: `zh-CN`
   - Remove: `hu`, `id`, `lo`, `my`, `ne`, `no`, `ru`, `sd`, `sv`, `tl`, `uk`
     (11 dead)
   - Remove if still listed: `fi`, `si`, `sw` (3 stale)
3. **Plugin mode** → switch off if there's a toggle (we're not using the
   WordPress plugin anymore; everything's done via the Lambda + proxy)
4. **Hreflang & `<html lang>` injection** → ON
5. **`<title>` and `<meta description>` translation** → ON

---

## Cutover sequence (the actual deployment day)

1. ✅ Phases 1–4 set up over the prior days. None of this affects production
   yet — CloudFront is just sitting there with no DNS pointing at it.
2. ✅ Validation per Phase 6.1 passes for all 18 languages
3. **Update GTranslate dashboard** (origin → Amplify, prune languages)
4. **Cloudflare DNS cutover** — change `heartland.io` and `www.heartland.io`
   CNAMEs to CloudFront, proxy off, TTL 300
5. **Monitor for 30 min.** Watch the Lambda logs, CloudFront metrics, and
   manually verify ~10 pages including translated ones
6. **Monitor for 48 hours.** Daily checks of GSC, GTranslate dashboard,
   Amplify metrics
7. **Delete SiteGround.** Cancel the hosting plan. Save the wp-content backup
   somewhere (e.g., S3 in a `legacy-wp-backup` bucket) for at least 6 months
   in case anything obscure is missing

---

## Rollback plan

If anything breaks during cutover:

1. **DNS rollback** — in Cloudflare, revert the CNAMEs to the old SiteGround
   values. With TTL 300, propagation is ~5 min.
2. **Amplify rollback** — Amplify keeps build history. Roll back to the last
   known-good build via the console.
3. **GTranslate rollback** — re-point origin to WordPress in the GTranslate
   dashboard.

Keep SiteGround running for at least 48 hours after DNS cutover so rollback is
real.

---

## Troubleshooting

### "Translated pages return 502 / 504"

The Lambda rewrites the origin to a GTranslate server. If that server is
unreachable or rate-limiting:

- Check CloudWatch logs for the Lambda — any errors?
- Curl the GTranslate server directly: `curl -sSI https://ani.tdn.gtranslate.net/es/`
  (replace `ani` with whichever your hostname hashes to)
- GTranslate support is usually responsive — they can move you to a less-loaded
  server pool

### "All pages 404 / 500"

- CloudFront origin pointing at the wrong Amplify URL? Check Origin Domain in
  the distribution config
- ACM cert not yet validated? CloudFront refuses to serve without it
- Amplify build failed? Check the Amplify console build log

### "Lambda function not invoking"

- Confirm the Lambda is associated with the distribution's **Origin Request**
  event (not Viewer Request)
- Confirm you used the **versioned** ARN, not `$LATEST`
- Lambda@Edge updates take 5–15 min to propagate after publish — wait before
  declaring failure

### "Cloudflare validation records won't validate"

- Make sure the validation CNAMEs have **proxy off (gray cloud)**. Orange cloud
  breaks them.
- Cloudflare's API/DNS UI strips the trailing `.heartland.io` from CNAME
  values. The ACM record value is what you keep — just make sure you copied
  the value field correctly without adding or stripping a trailing dot.

### "Cloudflare DNS won't allow apex CNAME"

- Check that **CNAME Flattening** is enabled in the zone settings. It's on by
  default for free plans but worth verifying.

### "Some translated pages render with broken layout"

- GTranslate caches translated pages at the edge for ~24h. After major DOM
  changes, request a cache purge via their dashboard.
- For specific pages with persistent layout issues, the cause is usually
  inline JS that runs before translation completes. Fix in the Rust source.

---

## Files in this folder

| File | Purpose | Used by |
|---|---|---|
| `README.md` | This document | Humans |
| `lambda-edge/index.js` *(TBD)* | Lambda@Edge source (Phase 3.3 code as a file) | Lambda console / `aws lambda update-function-code` |
| `cloudfront-distribution.json` *(TBD)* | CloudFront distribution config for `aws cloudfront create-distribution` | One-shot AWS CLI bootstrap |

### Stale — leftover from the abandoned Lightsail plan

These files were created when we briefly considered self-hosting the PHP proxy
on a Lightsail VPS. We pivoted to Amplify+Lambda@Edge before deploying. They
can be deleted; kept here only in case you ever need to fall back to a
PHP-based proxy.

- `.htaccess` — Apache rewrite rules
- `gtranslate-proxy/gtranslate.php` — vendor proxy script
- `gtranslate-proxy/config.php` — proxy config
- `gtranslate-proxy/cacert.pem` — CA bundle

---

## Cross-references

- Language module in heartland-rust: [`src/i18n.rs`](../src/i18n.rs) —
  must stay in sync with the `LANGS` set in the Lambda
- Route definitions: [`src/main.rs`](../src/main.rs)
- Article inventory (URL parity check, pre-cutover): [`articles-index.md`](../articles-index.md)
- Article inventory script: [`scripts/article_inventory.sh`](../scripts/article_inventory.sh)
