#!/usr/bin/env python3
"""Generate favicons + PWA icons for Heartland.

Mark: thick white "H" on brand red #ad2929, 22% rounded corners.
Outputs into assets/brand/.

Wordmark logos are NOT generated here — heartland-logo-{light,dark}.png are
real brand assets extracted from the live site per replicate.md Phase 1d. To
refresh from the live site, re-run:

  curl -sSL -o /tmp/h-color.webp https://heartland.io/wp-content/uploads/2022/08/Heartland-MAIN-LOGO.png
  curl -sSL -o /tmp/h-white.webp https://heartland.io/wp-content/uploads/2022/08/Heartland-White-Logo.png
  python3 -c "from PIL import Image; \
      Image.open('/tmp/h-color.webp').convert('RGBA').save('assets/brand/heartland-logo-light.png','PNG',optimize=True); \
      Image.open('/tmp/h-white.webp').convert('RGBA').save('assets/brand/heartland-logo-dark.png','PNG',optimize=True)"
"""
from pathlib import Path

try:
    from PIL import Image, ImageDraw, ImageFont
except ImportError as exc:
    raise SystemExit("Pillow required: pip install Pillow") from exc

BRAND_RED = (0xad, 0x29, 0x29)
WHITE = (0xff, 0xff, 0xff)
ROUND_RATIO = 0.22
STROKE_WIDTH_RATIO = 0.0

OUT = Path(__file__).parent / "assets" / "brand"
OUT.mkdir(parents=True, exist_ok=True)


def rounded_rect_mask(size: int) -> Image.Image:
    mask = Image.new("L", (size, size), 0)
    draw = ImageDraw.Draw(mask)
    radius = int(size * ROUND_RATIO)
    draw.rounded_rectangle((0, 0, size - 1, size - 1), radius=radius, fill=255)
    return mask


def find_font(size: int) -> ImageFont.FreeTypeFont:
    candidates = [
        "/System/Library/Fonts/Supplemental/Arial Black.ttf",
        "/System/Library/Fonts/Supplemental/Arial Bold.ttf",
        "/System/Library/Fonts/HelveticaNeue.ttc",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf",
    ]
    for path in candidates:
        if Path(path).exists():
            return ImageFont.truetype(path, size=size)
    return ImageFont.load_default()


def render_icon(size: int) -> Image.Image:
    canvas = Image.new("RGBA", (size, size), BRAND_RED + (255,))
    draw = ImageDraw.Draw(canvas)

    # Single-letter "H" centered on brand red.
    font_size = int(size * 0.74)
    font = find_font(font_size)
    stroke = int(size * STROKE_WIDTH_RATIO)

    text = "H"
    bbox = draw.textbbox((0, 0), text, font=font, stroke_width=stroke)
    w = bbox[2] - bbox[0]
    h = bbox[3] - bbox[1]
    x = (size - w) / 2 - bbox[0]
    y = (size - h) / 2 - bbox[1]
    draw.text(
        (x, y),
        text,
        fill=WHITE,
        font=font,
        stroke_width=stroke,
        stroke_fill=WHITE,
    )

    mask = rounded_rect_mask(size)
    output = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    output.paste(canvas, (0, 0), mask)
    return output


def main() -> None:
    sizes = {
        "favicon-32.png": 32,
        "favicon-192.png": 192,
        "icon-512.png": 512,
        "apple-touch-icon-180.png": 180,
    }
    for name, size in sizes.items():
        img = render_icon(size)
        out = OUT / name
        img.save(out)
        print(f"  wrote {out}  ({size}×{size})")

    ico_imgs = [render_icon(s) for s in (16, 32, 48)]
    ico_path = OUT / "favicon.ico"
    ico_imgs[0].save(ico_path, sizes=[(16, 16), (32, 32), (48, 48)])
    print(f"  wrote {ico_path}  (16/32/48 bundle)")

    svg = (
        '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">'
        '<rect width="64" height="64" rx="14" ry="14" fill="#ad2929"/>'
        '<text x="50%" y="54%" text-anchor="middle" '
        'font-family="Inter, Arial, sans-serif" font-weight="900" '
        'font-size="46" fill="#ffffff" dominant-baseline="middle" '
        'letter-spacing="-1">H</text>'
        '</svg>'
    )
    svg_path = OUT / "favicon.svg"
    svg_path.write_text(svg, encoding="utf-8")
    print(f"  wrote {svg_path}")

    print()
    print("Wordmark generation INTENTIONALLY DISABLED — the real Heartland")
    print("wordmark PNGs (heartland-logo-{light,dark}.png) come from the live")
    print("site. See module docstring for the refresh commands.")


if __name__ == "__main__":
    main()
