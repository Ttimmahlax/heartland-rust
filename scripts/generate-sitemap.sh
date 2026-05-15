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
  "/"                                                  # 1.0 weekly
  "/why-imperium"                                      # 0.9 monthly
  "/imperium-masterbatch"                              # 0.8
  "/imperium-filled-resin"                             # 0.8
  "/imperium-filler"                                   # 0.8
  "/imperium-fibers"                                   # 0.8
  "/imperium-animal-feed"                              # 0.7
  "/imperium-pork-feed"                                # 0.6
  "/imperium-cattle-feed"                              # 0.6
  "/imperium-chicken-feed"                             # 0.6
  "/imperium-spin-ready-white-fiber"                   # 0.6
  "/imperium-yarn"                                     # 0.6
  "/imperium-fabric"                                   # 0.6
  "/imperium-graphene"                                 # 0.6
  "/sustainable-plastic-compounding"                   # 0.8
  "/automotive"                                        # 0.8
  "/sustainable-packaging"                             # 0.8
  "/carbon-neutral-packaging-with-imperium-inside"     # 0.7
  "/sustainable-building-materials"                    # 0.8
  "/sustainable-rubber-additives"                      # 0.7
  "/sustainable-concrete-additives"                    # 0.7
  "/sustainable-asphalt-additives"                     # 0.7
  "/sustainable-paper-additives"                       # 0.7
  "/sustainable-foam"                                  # 0.6
  "/marine"                                            # 0.7
  "/government"                                        # 0.7
  "/hemp-fiber-and-hurd"                               # 0.6
  "/wood-products"                                     # 0.5
  "/plastic-additives"                                 # 0.6
  "/case-studies"                                      # 0.6
  "/usda"                                              # 0.6
  "/engineering-earth"                                 # 0.7
  "/e-books"                                           # 0.6
  "/heartland-e-books"                                 # 0.5
  "/whitepapers"                                       # 0.6
  "/natural-fiber-research"                            # 0.7
  "/frequently-asked-questions"                        # 0.8
  "/portfolios"                                        # 0.6
  "/heartland-team"                                    # 0.7
  "/heartland-farmers"                                 # 0.7
  "/green-packaging-initiative"                        # 0.7
  "/lca"                                               # 0.7
  "/about"                                             # 0.7
  "/contact"                                           # 0.8
  "/sustainability-news"                               # 0.9 weekly
)

# Priorities + changefreqs aligned with the array above.
PRIORITIES=(
  "1.0"
  "0.9"
  "0.8" "0.8" "0.8" "0.8" "0.7"
  "0.6" "0.6" "0.6" "0.6" "0.6" "0.6" "0.6"
  "0.8" "0.8" "0.8" "0.7" "0.8"
  "0.7" "0.7" "0.7" "0.7" "0.6"
  "0.7" "0.7"
  "0.6" "0.5" "0.6" "0.6" "0.6"
  "0.7" "0.6" "0.5" "0.6" "0.7" "0.8" "0.6"
  "0.7" "0.7" "0.7" "0.7" "0.7" "0.8"
  "0.9"
)

CHANGEFREQS=(
  "weekly"
  "monthly"
  "monthly" "monthly" "monthly" "monthly" "monthly"
  "monthly" "monthly" "monthly" "monthly" "monthly" "monthly" "monthly"
  "monthly" "monthly" "monthly" "monthly" "monthly"
  "monthly" "monthly" "monthly" "monthly" "monthly"
  "monthly" "monthly"
  "monthly" "monthly" "monthly" "monthly" "monthly"
  "monthly" "monthly" "monthly" "monthly" "monthly" "monthly" "monthly"
  "monthly" "monthly" "monthly" "monthly" "monthly" "monthly"
  "weekly"
)

SITEMAP="$OUT_DIR/sitemap_index.xml"

{
  echo '<?xml version="1.0" encoding="UTF-8"?>'
  echo '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">'

  for i in "${!STATIC_ROUTES[@]}"; do
    path="${STATIC_ROUTES[$i]}"
    pri="${PRIORITIES[$i]:-0.6}"
    cf="${CHANGEFREQS[$i]:-monthly}"
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

  # Category archives
  CATEGORY_SLUGS=$(
    grep -h '^categories = \[' content/articles/*.md 2>/dev/null \
      | grep -oE '"[^"]+"' | tr -d '"' | sort -u
  )
  for slug in $CATEGORY_SLUGS; do
    printf '  <url>\n    <loc>%s/sustainability-news/category/%s</loc>\n    <lastmod>%s</lastmod>\n    <changefreq>weekly</changefreq>\n    <priority>0.5</priority>\n  </url>\n' \
      "$BASE" "$slug" "$TODAY"
  done

  # Tag archives
  TAG_SLUGS=$(
    grep -h '^tags = \[' content/articles/*.md 2>/dev/null \
      | grep -oE '"[^"]+"' | tr -d '"' | sort -u
  )
  for slug in $TAG_SLUGS; do
    printf '  <url>\n    <loc>%s/sustainability-news/tag/%s</loc>\n    <lastmod>%s</lastmod>\n    <changefreq>weekly</changefreq>\n    <priority>0.4</priority>\n  </url>\n' \
      "$BASE" "$slug" "$TODAY"
  done

  # Portfolio items (slugs baked into the Rust manifest)
  if [ -f src/pages/portfolio_item.rs ]; then
    PORTFOLIO_SLUGS=$(
      grep -oE 'slug: "[^"]+"' src/pages/portfolio_item.rs 2>/dev/null \
        | sed 's/slug: "//; s/"$//'
    )
    for slug in $PORTFOLIO_SLUGS; do
      printf '  <url>\n    <loc>%s/sustainability-news/portfolio/%s</loc>\n    <lastmod>%s</lastmod>\n    <changefreq>monthly</changefreq>\n    <priority>0.5</priority>\n  </url>\n' \
        "$BASE" "$slug" "$TODAY"
    done
  fi

  echo '</urlset>'
} > "$SITEMAP"

# --- Per-language sitemaps ---------------------------------------------------
# For every translated language directory we have (content/articles/<lang>/),
# emit /<lang>/sitemap.xml with the article URLs for that language. Google
# Search Console submissions reference these paths.
if [ -d content/articles ]; then
  for langdir in content/articles/*/; do
    [ -d "$langdir" ] || continue
    lang=$(basename "$langdir")
    case "$lang" in .*|_*) continue ;; esac

    LANG_DIR="$OUT_DIR/${lang}"
    mkdir -p "$LANG_DIR"
    LANG_SITEMAP="$LANG_DIR/sitemap_index.xml"
    {
      echo '<?xml version="1.0" encoding="UTF-8"?>'
      echo '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">'
      # Per-language news index page.
      printf '  <url>\n    <loc>%s/%s/sustainability-news</loc>\n    <lastmod>%s</lastmod>\n    <changefreq>weekly</changefreq>\n    <priority>0.7</priority>\n  </url>\n' \
        "$BASE" "$lang" "$TODAY"
      for f in "$langdir"*.md; do
        [ -e "$f" ] || continue
        slug=$(basename "$f" .md)
        date=$(grep -E '^published_at\s*=\s*' "$f" | head -1 | sed -E 's/.*"([0-9]{4}-[0-9]{2}-[0-9]{2}).*"/\1/' || echo "$TODAY")
        [ -z "$date" ] && date="$TODAY"
        printf '  <url>\n    <loc>%s/%s/sustainability-news/%s</loc>\n    <lastmod>%s</lastmod>\n    <changefreq>monthly</changefreq>\n    <priority>0.6</priority>\n  </url>\n' \
          "$BASE" "$lang" "$slug" "$date"
      done
      echo '</urlset>'
    } > "$LANG_SITEMAP"
    echo "Wrote $LANG_SITEMAP"
  done
fi

cat > "$OUT_DIR/robots.txt" <<EOF
User-agent: *
Allow: /
Disallow: /api/

Sitemap: ${BASE}/sitemap_index.xml
EOF
# Add a Sitemap: directive for each per-language sitemap (lets crawlers
# discover them without explicit GSC submission, though GSC submission is
# still the canonical source of truth).
if [ -d content/articles ]; then
  for langdir in content/articles/*/; do
    [ -d "$langdir" ] || continue
    lang=$(basename "$langdir")
    case "$lang" in .*|_*) continue ;; esac
    echo "Sitemap: ${BASE}/${lang}/sitemap_index.xml" >> "$OUT_DIR/robots.txt"
  done
fi

cat > "$OUT_DIR/ads.txt" <<EOF
# Heartland Industries — no programmatic ads run on this domain.
# This file exists per the IAB ads.txt spec to prevent ad-fraud spoofing.
# To opt in to programmatic ads later, replace this file with valid
# <ssp>, <pub-id>, <relationship>, <tag> records.
EOF

echo "Wrote $SITEMAP"
echo "Wrote $OUT_DIR/robots.txt"
echo "Wrote $OUT_DIR/ads.txt"
