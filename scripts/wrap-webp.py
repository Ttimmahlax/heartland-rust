#!/usr/bin/env python3
"""Post-build HTML rewriter — rewrites every <img src="/assets/...{png|jpg|jpeg}">
to point at the .webp companion directly.

No <picture> fallback wrapper: modern browsers cover ~99% of traffic, the
JPG/PNG sources are not shipped in the deploy artifact, and we don't want
duplicate raster URLs getting indexed by SEO crawlers.

Runs from build-ssg.sh AFTER prerender + AFTER the assets/ → public/assets/
copy, so the .webp companion is guaranteed to exist on disk at the path
the rewritten markup will reference.

Exemptions (kept as the original format):
  - /assets/brand/**  — favicons + logo PNGs referenced by <link rel="icon">,
    og:image, and the Organization JSON-LD `logo` field. Crawlers and OS-level
    icon consumers want PNG for those surfaces, so we don't rewrite them.

Skips:
  - <img> with a non-/assets/ src (external CDNs, data URIs, etc.)
  - <img> with no WebP companion on disk (graceful — leaves markup as-is
    so the broken state surfaces visibly rather than silently 404ing)
"""
from __future__ import annotations
import argparse, sys
from pathlib import Path
import re

IMG_RE = re.compile(
    r'(<img\b[^>]*\bsrc=")(/assets/[^"]+)\.(png|jpg|jpeg|PNG|JPG|JPEG)("[^>]*/?>)',
    re.I,
)


def rewrite(html: str, public_dir: Path) -> tuple[str, int, int]:
    """Returns (new_html, rewritten_count, skipped_no_webp)."""
    rewritten = 0
    skipped = 0

    def sub(m: re.Match) -> str:
        nonlocal rewritten, skipped
        prefix, stem, _ext, suffix = m.group(1), m.group(2), m.group(3), m.group(4)
        if stem.lower().startswith("/assets/brand/"):
            return m.group(0)
        webp_url = f"{stem}.webp"
        webp_disk = public_dir / webp_url.lstrip("/")
        if not webp_disk.exists():
            skipped += 1
            return m.group(0)
        rewritten += 1
        return f"{prefix}{webp_url}{suffix}"

    new_html = IMG_RE.sub(sub, html)
    return new_html, rewritten, skipped


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("public_dir", help="prerendered output dir")
    args = ap.parse_args()
    pub = Path(args.public_dir)
    if not pub.is_dir():
        print(f"FAIL: {pub} is not a directory", file=sys.stderr)
        sys.exit(1)

    total_rewritten = 0
    total_skipped = 0
    files_touched = 0
    for html_path in pub.rglob("*.html"):
        text = html_path.read_text(encoding="utf-8", errors="replace")
        new_text, rewritten, skipped = rewrite(text, pub)
        if rewritten:
            html_path.write_text(new_text, encoding="utf-8")
            files_touched += 1
        total_rewritten += rewritten
        total_skipped += skipped

    print("WebP rewrite pass:")
    print(f"  files touched: {files_touched}")
    print(f"  <img> rewritten: {total_rewritten}")
    print(f"  skipped (no .webp on disk): {total_skipped}")


if __name__ == "__main__":
    main()
