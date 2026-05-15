#!/usr/bin/env bash
# Inventory all articles: slug, URL, H1 (title), H2/H3 headings.
# Also cross-checks asset directories and hero_image existence.

set -uo pipefail

cd "$(dirname "$0")/.."

OUT="${1:-articles-index.md}"
ARTICLES_DIR="content/articles"
ASSETS_DIR="assets/articles"

total=0
missing_assets=0
missing_hero=0
no_title=0

{
  echo "# Article inventory"
  echo ""
  echo "Source: \`content/articles/*.md\` (TOML frontmatter + markdown body)."
  echo "Route pattern: \`/sustainability-news/:slug\`."
  echo ""
  echo "---"
  echo ""
} > "$OUT"

# Per-article detail goes to a temp; we'll write summary first then append.
DETAIL=$(mktemp)
trap 'rm -f "$DETAIL"' EXIT

for f in "$ARTICLES_DIR"/*.md; do
  slug="$(basename "$f" .md)"
  total=$((total + 1))

  # Extract frontmatter (between first two +++ lines).
  fm="$(awk '/^\+\+\+$/{c++; next} c==1' "$f")"
  title="$(printf '%s\n' "$fm" | sed -n 's/^title = "\(.*\)"$/\1/p' | head -1)"
  hero_image="$(printf '%s\n' "$fm" | sed -n 's/^hero_image = "\(.*\)"$/\1/p' | head -1)"

  [ -z "$title" ] && { title="(MISSING)"; no_title=$((no_title + 1)); }

  # Body = everything after second +++
  body="$(awk '/^\+\+\+$/{c++; next} c>=2' "$f")"
  h2s="$(printf '%s\n' "$body" | grep -E '^## [^#]' | sed 's/^## //')"
  h3s="$(printf '%s\n' "$body" | grep -E '^### [^#]' | sed 's/^### //')"
  h2_count=$(printf '%s\n' "$h2s" | grep -c . || true)
  h3_count=$(printf '%s\n' "$h3s" | grep -c . || true)

  # Cross-checks
  assets_ok="yes"
  hero_ok="yes"
  if [ ! -d "$ASSETS_DIR/$slug" ]; then
    assets_ok="MISSING"
    missing_assets=$((missing_assets + 1))
  fi
  if [ -n "$hero_image" ] && [ ! -f "$ASSETS_DIR/$slug/$hero_image" ]; then
    hero_ok="MISSING ($hero_image)"
    missing_hero=$((missing_hero + 1))
  fi

  {
    echo "### $title"
    echo ""
    echo "- **slug**: \`$slug\`"
    echo "- **url**: \`/sustainability-news/$slug\`"
    echo "- **assets dir**: $assets_ok"
    echo "- **hero image**: $hero_ok"
    if [ "$h2_count" -gt 0 ]; then
      echo "- **H2** ($h2_count):"
      printf '%s\n' "$h2s" | sed 's/^/  - /'
    else
      echo "- **H2**: (none)"
    fi
    if [ "$h3_count" -gt 0 ]; then
      echo "- **H3** ($h3_count):"
      printf '%s\n' "$h3s" | sed 's/^/  - /'
    fi
    echo ""
  } >> "$DETAIL"
done

{
  echo "## Summary"
  echo ""
  echo "- Total articles: **$total**"
  echo "- Articles missing assets/ directory: **$missing_assets**"
  echo "- Articles whose hero_image file is missing: **$missing_hero**"
  echo "- Articles with no title: **$no_title**"
  echo ""
  echo "## Articles"
  echo ""
} >> "$OUT"

cat "$DETAIL" >> "$OUT"

echo "Wrote $OUT"
echo "total=$total missing_assets=$missing_assets missing_hero=$missing_hero no_title=$no_title"
