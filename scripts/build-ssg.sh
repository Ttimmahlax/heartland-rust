#!/usr/bin/env bash
# End-to-end SSG build: enumerate routes, prerender every one, copy assets.
set -euo pipefail

cd "$(dirname "$0")/.."

if ! command -v dx >/dev/null 2>&1; then
  echo "dx (dioxus-cli) not found. Install with:"
  echo "  cargo install dioxus-cli --version 0.7.7 --locked"
  exit 1
fi

echo "==> dx bundle --platform web --release"
dx bundle --platform web --release

OUT="target/dx/heartland-website/release/web/public"
SERVER_BIN="target/dx/heartland-website/release/web/server"

echo "==> Pre-rendering every route via the server binary"
./scripts/prerender.sh "$SERVER_BIN" "$OUT"

echo "==> Compiling Tailwind v4 CSS"
./scripts/build-tailwind.sh "$OUT"

echo "==> Copying assets/ → $OUT/assets (skipping raw raster/MP4 sources)"
mkdir -p "$OUT/assets"
# Only ship optimized formats — .webp/.webm are canonical, .svg/.gif/.ico
# pass through unchanged. Source .png/.jpg/.jpeg/.mp4 are dev-only and
# deliberately excluded so SEO crawlers can't index duplicate raster URLs.
#
# Exception: assets/brand/** keeps its PNGs — favicons, apple-touch-icon,
# Organization JSON-LD logo, and og:image URLs all need PNG for crawler /
# OS-icon-consumer compatibility. wrap-webp.py has the matching exemption.
if command -v rsync >/dev/null 2>&1; then
  rsync -a \
    --include 'brand/***' \
    --exclude '*.png' --exclude '*.PNG' \
    --exclude '*.jpg' --exclude '*.JPG' \
    --exclude '*.jpeg' --exclude '*.JPEG' \
    --exclude '*.mp4' --exclude '*.MP4' \
    assets/ "$OUT/assets/"
else
  # Portable fallback: copy everything, the find-delete below prunes rasters.
  cp -a assets/. "$OUT/assets/"
fi

# Sweep stale rasters/MP4s left behind by prior builds (rsync above doesn't
# --delete, since other build steps write tailwind*.css into $OUT/assets/).
# brand/** is preserved — it's the only place we ship PNG (favicons + OG logo).
# Everything else ships as WebP — including og:image and twitter:image surfaces,
# which modern crawlers (Twitter since 2020, Facebook, LinkedIn, Discord, Slack,
# iMessage) all accept.
find "$OUT/assets" -type f \( \
    -iname '*.png' -o -iname '*.jpg' -o -iname '*.jpeg' -o -iname '*.mp4' \
  \) -not -path "$OUT/assets/brand/*" -delete

echo "==> Rewriting <img src> to .webp directly (no <picture> fallback)"
python3 ./scripts/wrap-webp.py "$OUT"

echo "==> Generating sitemap.xml + robots.txt"
./scripts/generate-sitemap.sh "$OUT"

echo "==> Generating AEO surfaces (llms.txt, llms-full.txt, /sustainability-news/<slug>.md)"
python3 ./scripts/generate-aeo.py "$OUT"

echo
echo "Build complete."
echo "Preview:"
echo "  cd $OUT && python3 -m http.server 3000"
