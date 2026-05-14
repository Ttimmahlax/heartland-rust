#!/usr/bin/env bash
# Compile Tailwind v4 CSS and replace every tailwind*.css file dx wrote into
# public/assets/. dx 0.7.7 ships the source `@import "tailwindcss"` CSS
# unprocessed — the compiler is needed to actually generate utility classes.
#
# Strategy:
#   1. Use locally-installed `tailwindcss` binary if present (dev machines).
#   2. Otherwise, download the platform-specific standalone binary from
#      Tailwind's GitHub releases and cache it under $HOME/.cache.
#
# Why standalone over `npx @tailwindcss/cli`: in v4, the CLI needs to resolve
# `@import "tailwindcss"` against the `tailwindcss` package, and npx-installed
# packages aren't always findable from the project root (Amplify CI hits
# "Can't resolve 'tailwindcss'"). The standalone binary bundles everything.
set -euo pipefail

export PATH="/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:$PATH"

cd "$(dirname "$0")/.."

OUT="${1:-target/dx/heartland-website/release/web/public}"
SRC="tailwind.css"
TW_VERSION="${TW_VERSION:-v4.2.4}"

TMP_BASE=$(mktemp "${TMPDIR:-/tmp}/heartland-tailwind.XXXXXX")
TMP="${TMP_BASE}.css"
trap 'rm -f "$TMP" "$TMP_BASE"' EXIT

detect_platform() {
  local os arch
  os=$(uname -s | tr '[:upper:]' '[:lower:]')
  case "$os" in
    darwin) os=macos ;;
    linux)  os=linux  ;;
    *) echo "FAIL: unsupported OS '$os'" >&2; return 1 ;;
  esac
  arch=$(uname -m)
  case "$arch" in
    x86_64|amd64)   arch=x64   ;;
    arm64|aarch64)  arch=arm64 ;;
    *) echo "FAIL: unsupported arch '$arch'" >&2; return 1 ;;
  esac
  echo "tailwindcss-${os}-${arch}"
}

resolve_compiler() {
  if command -v tailwindcss >/dev/null 2>&1; then
    command -v tailwindcss
    return 0
  fi
  local platform url cache_dir bin_path
  platform=$(detect_platform) || return 1
  cache_dir="${HOME:-/tmp}/.cache/heartland-tailwindcss"
  mkdir -p "$cache_dir"
  bin_path="${cache_dir}/${platform}-${TW_VERSION}"
  if [ ! -x "$bin_path" ]; then
    url="https://github.com/tailwindlabs/tailwindcss/releases/download/${TW_VERSION}/${platform}"
    echo "==> Downloading tailwindcss standalone: $url" >&2
    curl -sSL --fail -o "$bin_path" "$url"
    chmod +x "$bin_path"
  fi
  echo "$bin_path"
}

COMPILER=$(resolve_compiler)
if [ -z "$COMPILER" ]; then
  echo "FAIL: could not resolve tailwindcss compiler" >&2
  exit 1
fi

echo "==> $COMPILER -i $SRC -o $TMP --minify"
"$COMPILER" -i "$SRC" -o "$TMP" --minify

if [ ! -s "$TMP" ]; then
  echo "FAIL: compiled CSS at $TMP is empty" >&2
  exit 1
fi

mkdir -p "$OUT/assets"
replaced=0
for css in "$OUT"/assets/tailwind*.css; do
  [ -f "$css" ] || continue
  cp "$TMP" "$css"
  replaced=$((replaced + 1))
  printf "  replaced  %s  (%s bytes)\n" "$css" "$(wc -c < "$css" | tr -d ' ')"
done

if [ "$replaced" -eq 0 ]; then
  cp "$TMP" "$OUT/assets/tailwind.css"
  echo "  wrote     $OUT/assets/tailwind.css"
fi
