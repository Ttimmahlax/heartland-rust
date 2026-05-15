#!/usr/bin/env python3
"""Post-build HTML rewriter — wraps every <img src="/assets/...{png|jpg|jpeg}">
in a <picture><source srcset="*.webp" type="image/webp"/>…</picture> block
so capable browsers (~97% of traffic) load the smaller WebP companion.

Run from build-ssg.sh AFTER prerender + AFTER the assets/ → public/assets/
copy, so the webp companion is guaranteed to exist on disk at the path
the rewritten markup will reference.

Skips:
  - <img> tags inside an existing <picture>
  - <img> with a non-/assets/ src (external CDNs, data URIs, etc.)
  - <img> with no WebP companion on disk (graceful — leaves markup as-is)
"""
from __future__ import annotations
import argparse, os, re, sys
from pathlib import Path

# <img ... src="/assets/x.png" ...>  →
# <picture><source srcset="/assets/x.webp" type="image/webp"/><img ...></picture>
IMG_RE = re.compile(
    r'(<img\b[^>]*\bsrc="(/assets/[^"]+\.(?:png|jpg|jpeg|PNG|JPG|JPEG))"[^>]*/?>)',
    re.I,
)

# Skip images that are already inside a <picture> wrapper. We detect this
# by scanning backwards for a recent <picture> open without a corresponding
# close. Done structurally below.

def webp_for(asset_path: str, public_dir: Path) -> Path:
    stem, _ = os.path.splitext(asset_path)
    return public_dir / stem.lstrip("/").replace("/", os.sep) + ".webp" \
        if False else (public_dir / (stem.lstrip("/") + ".webp"))


def wrap_one(html: str, public_dir: Path) -> tuple[str, int, int]:
    """Returns (new_html, wrapped_count, skipped_no_webp)."""
    wrapped = 0
    skipped_no_webp = 0
    out_parts: list[str] = []
    cursor = 0
    open_picture_depth = 0
    # Find positions where <picture> opens / closes so we can avoid
    # double-wrapping. Simple state machine.
    # Tag scanner — yields (kind, start, end) for: picture-open, picture-close, img
    tag_re = re.compile(
        r'(<picture\b[^>]*>)|(</picture>)|'
        r'(<img\b[^>]*\bsrc="(/assets/[^"]+\.(?:png|jpg|jpeg|PNG|JPG|JPEG))"[^>]*/?>)',
        re.I,
    )
    for m in tag_re.finditer(html):
        out_parts.append(html[cursor:m.start()])
        cursor = m.end()
        if m.group(1):  # <picture>
            open_picture_depth += 1
            out_parts.append(m.group(0))
            continue
        if m.group(2):  # </picture>
            if open_picture_depth > 0:
                open_picture_depth -= 1
            out_parts.append(m.group(0))
            continue
        # img tag
        img_tag = m.group(3)
        src = m.group(4)
        if open_picture_depth > 0:
            # already inside picture, leave alone
            out_parts.append(img_tag)
            continue
        # Lookup .webp on disk
        webp_path = src.rsplit(".", 1)[0] + ".webp"
        webp_disk = public_dir / webp_path.lstrip("/")
        if not webp_disk.exists():
            skipped_no_webp += 1
            out_parts.append(img_tag)
            continue
        # Wrap in <picture>
        out_parts.append(
            f'<picture><source srcset="{webp_path}" type="image/webp"/>{img_tag}</picture>'
        )
        wrapped += 1
    out_parts.append(html[cursor:])
    return "".join(out_parts), wrapped, skipped_no_webp


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("public_dir", help="prerendered output dir")
    args = ap.parse_args()
    pub = Path(args.public_dir)
    if not pub.is_dir():
        print(f"FAIL: {pub} is not a directory", file=sys.stderr)
        sys.exit(1)

    total_wrapped = 0
    total_skipped = 0
    files_touched = 0
    for html_path in pub.rglob("*.html"):
        text = html_path.read_text(encoding="utf-8", errors="replace")
        new_text, wrapped, skipped = wrap_one(text, pub)
        if wrapped:
            html_path.write_text(new_text, encoding="utf-8")
            files_touched += 1
        total_wrapped += wrapped
        total_skipped += skipped

    print(f"WebP wrap pass:")
    print(f"  files touched: {files_touched}")
    print(f"  <img> wrapped: {total_wrapped}")
    print(f"  skipped (no .webp on disk): {total_skipped}")


if __name__ == "__main__":
    main()
