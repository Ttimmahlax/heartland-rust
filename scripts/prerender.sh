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
  "/404"
)

# Append each migrated article slug
if [ -d content/articles ]; then
  for f in content/articles/*.md; do
    [ -e "$f" ] || continue
    slug=$(basename "$f" .md)
    ROUTES+=("/sustainability-news/${slug}")
  done
fi

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
done

# Also write the literal 404.html that Amplify's rewrite rule serves
if [ -f "$PUBLIC_DIR/404/index.html" ]; then
  cp "$PUBLIC_DIR/404/index.html" "$PUBLIC_DIR/404.html"
fi

echo "Done. Pre-rendered HTML written under $PUBLIC_DIR/."
