#!/usr/bin/env bash
# Emit sitemap.xml + robots.txt + ads.txt into the SSG output directory.
# Usage:
#   ./scripts/generate-sitemap.sh [output_dir]
# Default output_dir is target/dx/heartland-website/release/web/public.
set -euo pipefail
export PATH="/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:$PATH"

cd "$(dirname "$0")/.."

OUT_DIR="${1:-target/dx/heartland-website/release/web/public}"
BASE="https://heartland.io"
TODAY=$(date -u +%Y-%m-%d)

mkdir -p "$OUT_DIR"

# IMPORTANT: keep this list in sync with the Route enum in src/main.rs and the
# ROUTES array in scripts/prerender.sh and the CORE_PAGES list in
# scripts/generate-aeo.py. See docs/replicate.md §"The trio of route lists."
STATIC_ROUTES=(
  "/"
  "/why-imperium"
  "/imperium-masterbatch"
  "/imperium-filled-resin"
  "/imperium-filler"
  "/imperium-fibers"
  "/imperium-animal-feed"
  "/sustainable-plastic-compounding"
  "/automotive"
  "/sustainable-packaging"
  "/sustainable-building-materials"
  "/sustainable-rubber-additives"
  "/sustainable-concrete-additives"
  "/sustainable-asphalt-additives"
  "/sustainable-paper-additives"
  "/marine"
  "/government"
  "/engineering-earth"
  "/e-books"
  "/whitepapers"
  "/natural-fiber-research"
  "/frequently-asked-questions"
  "/heartland-team"
  "/heartland-farmers"
  "/green-packaging-initiative"
  "/lca"
  "/about"
  "/contact"
  "/sustainability-news"
)

PRIORITIES=(
  "1.0" "0.9" "0.8" "0.8" "0.8" "0.8" "0.7"
  "0.8" "0.8" "0.8" "0.8" "0.7" "0.7" "0.7" "0.7" "0.7" "0.7"
  "0.7" "0.6" "0.6" "0.7" "0.8"
  "0.7" "0.7" "0.7" "0.7" "0.7" "0.8"
  "0.9"
)

CHANGEFREQS=(
  "weekly" "monthly" "monthly" "monthly" "monthly" "monthly" "monthly"
  "monthly" "monthly" "monthly" "monthly" "monthly" "monthly" "monthly" "monthly" "monthly" "monthly"
  "monthly" "monthly" "monthly" "monthly" "monthly"
  "monthly" "monthly" "monthly" "monthly" "monthly" "monthly"
  "weekly"
)

SITEMAP="$OUT_DIR/sitemap.xml"

{
  echo '<?xml version="1.0" encoding="UTF-8"?>'
  echo '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">'

  for i in "${!STATIC_ROUTES[@]}"; do
    path="${STATIC_ROUTES[$i]}"
    pri="${PRIORITIES[$i]}"
    cf="${CHANGEFREQS[$i]}"
    printf '  <url>\n    <loc>%s%s</loc>\n    <lastmod>%s</lastmod>\n    <changefreq>%s</changefreq>\n    <priority>%s</priority>\n  </url>\n' \
      "$BASE" "$path" "$TODAY" "$cf" "$pri"
  done

  if [ -d content/articles ]; then
    for f in content/articles/*.md; do
      [ -e "$f" ] || continue
      slug=$(basename "$f" .md)
      date=$(grep -E '^published_at\s*=\s*' "$f" | head -1 | sed -E 's/.*"([0-9]{4}-[0-9]{2}-[0-9]{2}).*"/\1/' || echo "$TODAY")
      [ -z "$date" ] && date="$TODAY"
      printf '  <url>\n    <loc>%s/sustainability-news/%s</loc>\n    <lastmod>%s</lastmod>\n    <changefreq>monthly</changefreq>\n    <priority>0.6</priority>\n  </url>\n' \
        "$BASE" "$slug" "$date"
    done
  fi

  echo '</urlset>'
} > "$SITEMAP"

cat > "$OUT_DIR/robots.txt" <<EOF
User-agent: *
Allow: /
Disallow: /api/

Sitemap: ${BASE}/sitemap.xml
EOF

cat > "$OUT_DIR/ads.txt" <<EOF
# Heartland Industries — no programmatic ads run on this domain.
# This file exists per the IAB ads.txt spec to prevent ad-fraud spoofing.
# To opt in to programmatic ads later, replace this file with valid
# <ssp>, <pub-id>, <relationship>, <tag> records.
EOF

echo "Wrote $SITEMAP"
echo "Wrote $OUT_DIR/robots.txt"
echo "Wrote $OUT_DIR/ads.txt"
