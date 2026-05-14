# Amplify deployment

The site builds via AWS Amplify, GitHub-connected. Push to `main` → Amplify
runs [amplify.yml](../amplify.yml) → static output is uploaded.

## One-time console steps

After connecting the repo, configure these in the Amplify console:

### 1. Rewrite rule for SSG sub-paths

Add a single rewrite under **App settings → Rewrites and redirects**:

| Source address | Target address | Type |
| --- | --- | --- |
| `</^[^.]+$|\.(?!(css|gif|ico|jpg|js|png|txt|svg|woff|woff2|ttf|map|json|xml|webmanifest|md)$)([^.]+$)/>` | `/index.html` | `200 (Rewrite)` |

Then add a more specific rule for SSG sub-paths so `/learn/foo/` resolves to its own `index.html`:

| Source | Target | Type |
| --- | --- | --- |
| `/<*>` | `/<*>/index.html` | `200 (Rewrite)` |

### 2. Custom HTTP headers

Amplify reads [customHttp.yml](../customHttp.yml) automatically — no console step.

### 3. Build settings

The console may detect a different build spec; force it to use the repo's `amplify.yml`.

### Note on `.md` extensions

The rewrite rule above includes `md` in the extension allowlist so that AEO surfaces (`/en/news/<slug>.md`) are served as files and not rewritten to `/index.html`. If you ever regenerate the rule from defaults, make sure `md` stays in the list.

## Environment variables

None required. Klaviyo company ID is public-side and lives in `tracking.rs`.

## Cache strategy

- `*.html`: `max-age=0, must-revalidate` — every revisit revalidates so deploys propagate.
- `/assets/**`, `/wasm/**`: `max-age=31536000, immutable` — fingerprinted, safe to cache forever.

## Local preview of the production bundle

```bash
./scripts/build-ssg.sh
cd target/dx/hfga-website/release/web/public && python3 -m http.server 3000
```
