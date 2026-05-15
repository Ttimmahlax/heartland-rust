#!/usr/bin/env bash
# Generate WebP companions for every raster image under assets/.
# Per docs/replicate.md §"WebP image optimization": every PNG / JPG /
# JPEG should ship with a sibling `.webp` of the same basename so the
# Picture component (or downstream <picture> markup) can serve the
# smaller file to capable browsers (~97% of traffic today).
#
# Idempotent — skips outputs where the `.webp` is already present and
# newer than the source. Auto-discards outputs that come out larger
# than the source (rare, but happens with already-optimized PNGs that
# carry transparency or very small raster art).
#
# Skips:
#   - SVG / GIF (already vector / animated)
#   - WebP companions of WebP files (no-op)
#   - assets/videos/*-poster.jpg — those are video posters, not page
#     content; serving them as WebP wouldn't shrink LCP.
set -euo pipefail
export PATH="/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:$PATH"

command -v cwebp >/dev/null || {
  echo "cwebp not found. Install: brew install webp" >&2; exit 1; }

cd "$(dirname "$0")/.."

QUALITY="${QUALITY:-82}"
ENCODED=0
SKIPPED=0
DISCARDED=0

# macOS ships Bash 3.2 (no globstar). Use find for portable recursion.
while IFS= read -r -d '' src; do
  # Skip video posters
  case "$src" in
    assets/videos/*) continue ;;
  esac

  stem="${src%.*}"
  out="${stem}.webp"
  if [[ -f "$out" && "$out" -nt "$src" ]]; then
    SKIPPED=$((SKIPPED + 1))
    continue
  fi

  # Encode. Use lossy q=82 for jpg/jpeg, higher-alpha-quality for png.
  # (macOS Bash 3.2 has no ${var,,} lower-case — match both cases instead.)
  case "$src" in
    *.png|*.PNG)
      cwebp -quiet -q "$QUALITY" -alpha_q 100 "$src" -o "$out" 2>/dev/null
      ;;
    *)
      cwebp -quiet -q "$QUALITY" "$src" -o "$out" 2>/dev/null
      ;;
  esac

  src_size=$(stat -f%z "$src" 2>/dev/null || stat -c%s "$src")
  webp_size=$(stat -f%z "$out" 2>/dev/null || stat -c%s "$out")
  if (( webp_size >= src_size )); then
    DISCARDED=$((DISCARDED + 1))
    rm -f "$out"
    continue
  fi
  ENCODED=$((ENCODED + 1))
done < <(find assets -type f \( -iname '*.png' -o -iname '*.jpg' -o -iname '*.jpeg' \) -print0)

echo "WebP encode pass:"
echo "  encoded:   $ENCODED"
echo "  skipped:   $SKIPPED  (companion already up to date)"
echo "  discarded: $DISCARDED  (output ≥ source size)"
