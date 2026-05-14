#!/usr/bin/env bash
# Self-host Inter via fontsource.org's direct woff2 CDN. Inter is a single
# family covering body + display weights — widely cached, easy to fetch.
set -euo pipefail

cd "$(dirname "$0")/.."
mkdir -p assets/fonts

CDN="https://cdn.jsdelivr.net/npm/@fontsource/inter@5.0.16/files"

fetch_face() {
  local weight="$1"
  local outfile="$2"
  local url="${CDN}/inter-latin-${weight}-normal.woff2"
  echo "==> $outfile  ($url)"
  curl -sSL --fail -o "assets/fonts/$outfile" "$url"
}

fetch_face "400" "inter-400.woff2"
fetch_face "500" "inter-500.woff2"
fetch_face "600" "inter-600.woff2"
fetch_face "700" "inter-700.woff2"
fetch_face "800" "inter-800.woff2"

echo "Done. Files in assets/fonts/."
ls -la assets/fonts/
