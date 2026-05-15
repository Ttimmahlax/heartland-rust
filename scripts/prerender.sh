#!/usr/bin/env bash
# Manually pre-render every route by starting the dx-built server, hitting
# each path, and saving the response HTML to <path>/index.html.
#
# Why this exists: dx 0.7.7's `--ssg` flag does not always wire up against the
# `#[server]` static_routes endpoint correctly. This script does the same job
# deterministically.
#
# Inputs:
#   $1: server binary path (default: target/dx/heartland-website/release/web/server)
#   $2: public output dir   (default: target/dx/heartland-website/release/web/public)
set -euo pipefail
export PATH="/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:$PATH"

cd "$(dirname "$0")/.."

SERVER_BIN="${1:-target/dx/heartland-website/release/web/server}"
PUBLIC_DIR="${2:-target/dx/heartland-website/release/web/public}"
PORT="${PORT:-8089}"

if [ ! -x "$SERVER_BIN" ]; then
  echo "FAIL: server binary not found at $SERVER_BIN" >&2
  exit 1
fi

# Static routes — keep in sync with `Route` enum in src/main.rs and
# STATIC_ROUTES in scripts/generate-sitemap.sh.
ROUTES=(
  "/"
  "/why-imperium"
  "/imperium-masterbatch"
  "/imperium-filled-resin"
  "/imperium-filler"
  "/imperium-fibers"
  "/imperium-animal-feed"
  "/imperium-pork-feed"
  "/imperium-cattle-feed"
  "/imperium-chicken-feed"
  "/imperium-spin-ready-white-fiber"
  "/imperium-yarn"
  "/imperium-fabric"
  "/imperium-graphene"
  "/sustainable-plastic-compounding"
  "/automotive"
  "/sustainable-packaging"
  "/carbon-neutral-packaging-with-imperium-inside"
  "/sustainable-building-materials"
  "/sustainable-rubber-additives"
  "/sustainable-concrete-additives"
  "/sustainable-asphalt-additives"
  "/sustainable-paper-additives"
  "/sustainable-foam"
  "/marine"
  "/government"
  "/hemp-fiber-and-hurd"
  "/wood-products"
  "/plastic-additives"
  "/case-studies"
  "/usda"
  "/engineering-earth"
  "/e-books"
  "/heartland-e-books"
  "/whitepapers"
  "/natural-fiber-research"
  "/frequently-asked-questions"
  "/portfolios"
  "/heartland-team"
  "/heartland-farmers"
  "/green-packaging-initiative"
  "/lca"
  "/about"
  "/contact"
  "/sustainability-news"
  "/404"
)

# Append each migrated article slug (English).
if [ -d content/articles ]; then
  for f in content/articles/*.md; do
    [ -e "$f" ] || continue
    slug=$(basename "$f" .md)
    ROUTES+=("/sustainability-news/${slug}")
  done
fi

# Append the translated article routes for every language we have markdown for.
# Each non-English language dir under content/articles/ becomes a set of routes
# at /<lang>/sustainability-news/<slug>, plus a /<lang>/sustainability-news index.
# Article components fall back to English when a specific translation is missing,
# so it's safe to enumerate by directory.
if [ -d content/articles ]; then
  for langdir in content/articles/*/; do
    [ -d "$langdir" ] || continue
    lang=$(basename "$langdir")
    # Skip dot/underscore dirs (none today, but defensive).
    case "$lang" in .*|_*) continue ;; esac
    # Per-language news index.
    ROUTES+=("/${lang}/sustainability-news")
    for f in "$langdir"*.md; do
      [ -e "$f" ] || continue
      slug=$(basename "$f" .md)
      ROUTES+=("/${lang}/sustainability-news/${slug}")
    done
  done
fi

# Append every category archive page (one per unique slug across all articles)
CATEGORY_SLUGS=$(
  grep -h '^categories = \[' content/articles/*.md 2>/dev/null \
    | grep -oE '"[^"]+"' | tr -d '"' | sort -u
)
for slug in $CATEGORY_SLUGS; do
  ROUTES+=("/sustainability-news/category/${slug}")
done

# Append every tag archive page
TAG_SLUGS=$(
  grep -h '^tags = \[' content/articles/*.md 2>/dev/null \
    | grep -oE '"[^"]+"' | tr -d '"' | sort -u
)
for slug in $TAG_SLUGS; do
  ROUTES+=("/sustainability-news/tag/${slug}")
done

# Append every portfolio item (slugs baked into src/pages/portfolio_item.rs)
PORTFOLIO_SLUGS=$(
  grep -oE 'slug: "[^"]+"' src/pages/portfolio_item.rs 2>/dev/null \
    | sed 's/slug: "//; s/"$//'
)
for slug in $PORTFOLIO_SLUGS; do
  ROUTES+=("/sustainability-news/portfolio/${slug}")
done

# Start server in background
echo "==> Starting server: $SERVER_BIN on :$PORT"
PORT="$PORT" IP="127.0.0.1" "$SERVER_BIN" >/tmp/heartland-prerender.log 2>&1 &
SERVER_PID=$!
trap 'kill $SERVER_PID 2>/dev/null || true; wait $SERVER_PID 2>/dev/null || true' EXIT

# Wait for it to be ready
for i in $(seq 1 30); do
  if curl -fsS -o /dev/null "http://127.0.0.1:$PORT/" 2>/dev/null; then
    break
  fi
  sleep 0.2
done

if ! curl -fsS -o /dev/null "http://127.0.0.1:$PORT/" 2>/dev/null; then
  echo "FAIL: server didn't come up on :$PORT" >&2
  cat /tmp/heartland-prerender.log >&2 || true
  exit 1
fi

echo "==> Pre-rendering ${#ROUTES[@]} routes"

mkdir -p "$PUBLIC_DIR"

for route in "${ROUTES[@]}"; do
  if [ "$route" = "/" ]; then
    out="$PUBLIC_DIR/index.html"
  else
    out_dir="$PUBLIC_DIR${route}"
    mkdir -p "$out_dir"
    out="$out_dir/index.html"
  fi

  status=$(curl -s -o "$out" -w "%{http_code}" "http://127.0.0.1:$PORT${route}")
  size=$(wc -c < "$out" | tr -d ' ')
  printf "  %-3s  %-65s  %s bytes\n" "$status" "$route" "$size"

  if [ "$status" != "200" ]; then
    echo "WARN: $route returned HTTP $status" >&2
  fi

  # Patch the <html> tag with lang + dir for translated routes. Dioxus SSR
  # emits a bare `<html>` (no attributes), so we inject them post-render.
  # English routes get `lang="en"` and `dir="ltr"`; /<lang>/... gets the
  # matching BCP-47 code + dir.
  html_lang="en"
  html_dir="ltr"
  case "$route" in
    /*/*)
      maybe_lang="${route#/}"
      maybe_lang="${maybe_lang%%/*}"
      if [[ "$maybe_lang" =~ ^[a-z]{2}(-[A-Z]{2})?$ ]]; then
        html_lang="$maybe_lang"
        case "$maybe_lang" in ar|ur) html_dir="rtl" ;; esac
      fi
      ;;
  esac
  # macOS sed needs -i ''; GNU sed needs -i without the arg. Try GNU first.
  if ! sed -i "s|<html>|<html lang=\"${html_lang}\" dir=\"${html_dir}\">|" "$out" 2>/dev/null; then
    sed -i '' "s|<html>|<html lang=\"${html_lang}\" dir=\"${html_dir}\">|" "$out"
  fi
done

# Also write the literal 404.html that Amplify's rewrite rule serves
if [ -f "$PUBLIC_DIR/404/index.html" ]; then
  cp "$PUBLIC_DIR/404/index.html" "$PUBLIC_DIR/404.html"
fi

echo "Done. Pre-rendered HTML written under $PUBLIC_DIR/."
