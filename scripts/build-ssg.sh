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

echo "==> Copying assets/ → $OUT/assets"
mkdir -p "$OUT/assets"
cp -R assets/. "$OUT/assets/" 2>/dev/null || true

echo "==> Generating sitemap.xml + robots.txt"
./scripts/generate-sitemap.sh "$OUT"

echo "==> Generating AEO surfaces (llms.txt, llms-full.txt, /sustainability-news/<slug>.md)"
python3 ./scripts/generate-aeo.py "$OUT"

echo
echo "Build complete."
echo "Preview:"
echo "  cd $OUT && python3 -m http.server 3000"
