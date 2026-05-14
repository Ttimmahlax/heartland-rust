#!/usr/bin/env bash
# Generate VP9 .webm companions for every .mp4 in assets/videos/.
# Idempotent: skips up-to-date outputs.
# Auto-discards outputs that come out larger than source (already-optimized
# MP4 sources where VP9 can't win — see docs/replicate.md §"WebM video").
set -euo pipefail
export PATH="/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:$PATH"

command -v ffmpeg >/dev/null || {
  echo "ffmpeg not found. Install: brew install ffmpeg" >&2; exit 1; }

cd "$(dirname "$0")/.."

shopt -s nullglob
for mp4 in assets/videos/*.mp4; do
  webm="${mp4%.mp4}.webm"
  if [[ -f "$webm" && "$webm" -nt "$mp4" ]]; then
    echo "skip  $webm (up to date)"
    continue
  fi
  echo "encode $mp4 → $webm"
  ffmpeg -y -i "$mp4" \
    -c:v libvpx-vp9 -crf 32 -b:v 0 -row-mt 1 -an \
    -deadline good -cpu-used 2 \
    "$webm"
  mp4_size=$(stat -f%z "$mp4" 2>/dev/null || stat -c%s "$mp4")
  webm_size=$(stat -f%z "$webm" 2>/dev/null || stat -c%s "$webm")
  if (( webm_size >= mp4_size )); then
    pct=$(( webm_size * 100 / mp4_size ))
    echo "  warn  webm is ${pct}% of mp4 — discarding (source already optimized)"
    rm -f "$webm"
  else
    saved=$(( (mp4_size - webm_size) * 100 / mp4_size ))
    echo "  ok    saved ${saved}%"
  fi
done

echo "done. sizes:"
ls -lh assets/videos/*.{mp4,webm} 2>/dev/null | awk '{print $5"\t"$NF}'
