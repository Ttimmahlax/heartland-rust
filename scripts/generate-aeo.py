#!/usr/bin/env python3
"""Generate AEO (Answer Engine Optimization) surfaces for Heartland Industries.

For each article in `content/articles/*.md`, emit:
  - <OUT>/sustainability-news/<slug>.md   — Markdown sibling of the HTML page

For the site as a whole, emit:
  - <OUT>/llms.txt        — curated index per https://llmstxt.org spec
  - <OUT>/llms-full.txt   — every article body concatenated for one-shot ingestion

Usage:
  ./scripts/generate-aeo.py <out_dir>
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

try:
    import tomllib as _tomllib  # type: ignore  # py3.11+
    _toml_loads = _tomllib.loads
except ImportError:
    try:
        import tomli as _tomllib  # type: ignore
        _toml_loads = _tomllib.loads
    except ImportError:
        _tomllib = None
        _toml_loads = None


def _inline_toml_loads(src: str) -> dict:
    """Minimal TOML parser scoped to the article-front-matter format."""
    out: dict = {}
    str_rx = r'"((?:\\.|[^"\\])*)"'
    line_rx = re.compile(rf'^([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(?:{str_rx}|\[(.*)\])\s*$')
    for raw in src.splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        m = line_rx.match(line)
        if not m:
            raise ValueError(f"inline-toml: cannot parse line: {raw!r}")
        key, single, array = m.group(1), m.group(2), m.group(3)
        if single is not None:
            out[key] = bytes(single, "utf-8").decode("unicode_escape")
        else:
            items = re.findall(str_rx, array or "")
            out[key] = [bytes(s, "utf-8").decode("unicode_escape") for s in items]
    return out


def toml_loads(src: str) -> dict:
    if _toml_loads is not None:
        return _toml_loads(src)
    return _inline_toml_loads(src)


BASE_URL = "https://heartland.io"

PITCH = (
    "Heartland Industries is a material science company helping manufacturers "
    "exceed their cost-reduction goals while reducing their emissions. Our "
    "flagship product, Imperium, is engineered industrial hemp that drops into "
    "existing compounding lines as a direct substitute for talc, calcium "
    "carbonate, and (at high loadings) glass fiber — lower cost AND lower "
    "carbon. 12,000+ acres farmed across 11 US states. Detroit headquartered. "
    "Verified LCA. Same equipment. Better economics."
)

CORE_PAGES = [
    ("/", "Heartland Industries — The Future of Material Innovation",
     "High performance, cost-reducing, carbon-negative materials for plastic compounders, automotive OEMs, packaging molders, building-products manufacturers, and textile mills."),
    ("/why-imperium", "Why Imperium",
     "Imperium is engineered industrial hemp — drop-in for talc, CaCO3, and glass fiber. Cuts cost AND embedded carbon in the same swap. Runs on existing compounding equipment."),
    ("/imperium-masterbatch", "Imperium Masterbatch",
     "Pre-dispersed hemp-filler concentrate for direct let-down in PP, PE, PVC, and PA6. Eliminates the dispersion and feed-handling penalties of dry filler."),
    ("/imperium-filled-resin", "Imperium Filled Resin — Performance Plastics",
     "Pre-compounded Imperium-filled resin shipped as pellets ready for injection molding or extrusion. Skip the compounding step entirely."),
    ("/imperium-filler", "Imperium Filler",
     "Dry milled hemp filler in supersack or bulk. The most direct drop-in for talc and calcium carbonate. Cost-competitive, carbon-negative."),
    ("/imperium-fibers", "Imperium Textile Fiber",
     "Soft American hemp fiber for spin-ready white yarn, polyhemp, hemp-lyocell, hemp-cotton, hemp-linen, hemp-silk and hemp-wool blends. No microplastic shed, no port risk."),
    ("/imperium-animal-feed", "Imperium Animal Feed",
     "Hemp-derived feed for pork, cattle, chicken, and aquaculture. High-protein, high-omega, drop-in compatible with TMR and pelleting processes."),
    ("/sustainable-plastic-compounding", "Sustainable Plastic Compounding",
     "Plastic compounders use Imperium to cut cost and embedded carbon in PP, PE, PVC, PA6 and engineered thermoplastics — no line modifications required."),
    ("/automotive", "Automotive — Imperium-Reinforced Plastics",
     "Imperium-reinforced plastics for automotive interior, under-hood, and exterior trim. Continental winner, Magna×BASF Altair Enlighten finalist."),
    ("/sustainable-packaging", "Sustainable Packaging — Carbon-Neutral Pallets and Bins",
     "Carbon-neutral pallets, returnable bins, industrial crates. Imperium-inside HDPE formulations in every standard footprint."),
    ("/sustainable-building-materials", "Sustainable Building Materials",
     "Imperium-filled WPC decking, hempcrete blocks, hemp-reinforced WPC, hemp-additive concrete. Construction products built around American hemp."),
    ("/sustainable-rubber-additives", "Sustainable Rubber Additives",
     "Hemp-meal additives that partially replace carbon black in tire, conveyor-belt, floor-tile, and vibration-damping rubber compounds."),
    ("/sustainable-concrete-additives", "Sustainable Concrete Additives",
     "Hemp-fiber and hemp-hurd additives for crack resistance, lower carbon intensity, and improved thermal performance in mid-strength concretes."),
    ("/sustainable-asphalt-additives", "Sustainable Asphalt Additives",
     "Hemp-pulp asphalt modifier — lower binder demand, improved cold-temperature behavior, reduced thermal cracking on road and parking-lot applications."),
    ("/sustainable-paper-additives", "Sustainable Paper Additives",
     "Hemp pulp partial substitute for hardwood kraft. Lower water + carbon footprint, softwood-class fiber length, available for mid-grade paperboard and tissue."),
    ("/marine", "Marine Applications",
     "Imperium-reinforced plastics for boat hulls, decking, dock structures, and aquaculture gear — corrosion-immune, lower carbon than imported FRP."),
    ("/government", "Government Programs",
     "USDA Hemp4Soil grant recipient. MBDA + KDM partnership. State DOT hemp-asphalt and hemp-concrete pilots."),
    ("/engineering-earth", "Engineering Earth — Regenerative Agronomy",
     "Heartland's regenerative agronomy program. Industrial hemp in rotation with corn and soybean, cover-crop integration, soil-health measurement, third-party verified carbon sequestration."),
    ("/e-books", "Heartland E-Books",
     "Long-form downloadable guides on industrial hemp, sustainable materials, regenerative agriculture, and the future of manufacturing."),
    ("/whitepapers", "Heartland White Papers",
     "Technical white papers covering hemp-reinforced polymers, LCA methodology, carbon disclosures, and Scope 3 supply-chain economics."),
    ("/natural-fiber-research", "Natural Fiber Research",
     "Curated academic and industry literature on natural-fiber composites, hemp agronomy, and the LCA methodologies underpinning bio-based materials."),
    ("/frequently-asked-questions", "FAQ — Imperium and Industrial Hemp",
     "Common questions on Imperium pricing vs. talc, drop-in compatibility, carbon-negative claims, supply security, and how to start a program with Heartland."),
    ("/heartland-team", "The Heartland Team",
     "John Ely (CEO), Tim Almond (Chairman & COO), Eric Austermann (VP Engineering), Markus von Graf, Roger Blackwell, Robby Dameron, Deborah LaBelle."),
    ("/heartland-farmers", "Heartland's Farmer Network",
     "11 US states, 12,000+ acres of contracted industrial hemp. Premium contracts, agronomic support, and a guaranteed buyer."),
    ("/green-packaging-initiative", "Green Packaging Initiative",
     "Multi-brand commitment to swap fossil-derived industrial packaging for Imperium-inside HDPE alternatives. Pallets, bins, crates."),
    ("/lca", "Imperium Farming LCA",
     "First published life-cycle analysis for industrial hemp fiber used as a carbon-negative additive. Farm-to-filler boundary, third-party verified."),
    ("/about", "About Heartland Industries",
     "Detroit-headquartered material science company. We help manufacturers exceed cost-reduction goals while reducing emissions."),
    ("/contact", "Contact Heartland",
     "Email Hello@heartland.io. Engineering, sample requests, farmer applications, partnership inquiries routed within one business day."),
    ("/sustainability-news", "Sustainability News & Articles",
     "Heartland's full library — every article in chronological order."),
]


def parse_article(path: Path):
    text = path.read_text(encoding="utf-8").lstrip("﻿")
    if not text.startswith("+++"):
        return None
    end = text.find("\n+++", 3)
    if end == -1:
        return None
    front_str = text[3:end]
    body = text[end + 4 :].lstrip("\n\r")
    try:
        front = toml_loads(front_str)
    except Exception as e:
        print(f"  WARN: front-matter error in {path}: {e}", file=sys.stderr)
        return None
    return {"slug": path.stem, "front": front, "body": body}


def article_meta_line(front: dict) -> str:
    bits = []
    if front.get("published_at"):
        bits.append(f"Published {front['published_at']}")
    if front.get("author"):
        bits.append(f"by {front['author']}")
    if front.get("tags"):
        bits.append("tags: " + ", ".join(front["tags"]))
    return " · ".join(bits)


def render_article_md(article: dict) -> str:
    f = article["front"]
    slug = article["slug"]
    parts = [f"# {f['title']}", "", f"> {f['excerpt']}", ""]
    meta = article_meta_line(f)
    if meta:
        parts.extend([f"*{meta}*", ""])
    parts.extend([
        "---",
        "",
        article["body"].rstrip(),
        "",
        "---",
        "",
        f"Source: {BASE_URL}/sustainability-news/{slug}",
        f"HTML version: {BASE_URL}/sustainability-news/{slug}",
        "",
    ])
    return "\n".join(parts)


def render_llms_txt(articles: list[dict]) -> str:
    lines = ["# Heartland Industries", "", f"> {PITCH}", "", "## Core", ""]
    for path, title, desc in CORE_PAGES:
        lines.append(f"- [{title}]({BASE_URL}{path}): {desc}")
    lines.append("")
    lines.append("## Articles")
    lines.append("")
    for a in articles:
        f = a["front"]
        lines.append(
            f"- [{f['title']}]({BASE_URL}/sustainability-news/{a['slug']}.md): {f['excerpt']}"
        )
    lines.extend([
        "",
        "## Optional",
        "",
        f"- [Sitemap (XML)]({BASE_URL}/sitemap.xml): All indexed URLs with last-modified dates.",
        f"- [Robots]({BASE_URL}/robots.txt): Crawl directives.",
        f"- [Full content corpus]({BASE_URL}/llms-full.txt): Every article body concatenated — one fetch for full ingestion.",
        "",
    ])
    return "\n".join(lines)


def render_llms_full_txt(articles: list[dict]) -> str:
    lines = [
        "# Heartland Industries — Full Content Corpus",
        "",
        f"> {PITCH}",
        "",
        f"Source: {BASE_URL}",
        "Spec: https://llmstxt.org",
        "",
        f"This file concatenates every published article body verbatim. {len(articles)} articles included.",
        "",
        "---",
        "",
    ]
    for a in articles:
        f = a["front"]
        lines.append(f"# {f['title']}")
        lines.append("")
        lines.append(f"> {f['excerpt']}")
        lines.append("")
        meta = article_meta_line(f)
        if meta:
            lines.extend([f"*{meta}*", ""])
        lines.extend([
            f"Source: {BASE_URL}/sustainability-news/{a['slug']}",
            "",
            a["body"].rstrip(),
            "",
            "---",
            "",
        ])
    return "\n".join(lines)


def main() -> None:
    if len(sys.argv) < 2:
        print("Usage: generate-aeo.py <out_dir>", file=sys.stderr)
        sys.exit(1)

    out_dir = Path(sys.argv[1])
    articles_dir = Path("content/articles")

    if not articles_dir.is_dir():
        print(f"FAIL: {articles_dir} not found", file=sys.stderr)
        sys.exit(1)

    articles = []
    for path in sorted(articles_dir.glob("*.md")):
        a = parse_article(path)
        if a:
            articles.append(a)

    articles.sort(
        key=lambda a: a["front"].get("published_at", ""),
        reverse=True,
    )

    news_dir = out_dir / "sustainability-news"
    news_dir.mkdir(parents=True, exist_ok=True)
    for a in articles:
        out_path = news_dir / f"{a['slug']}.md"
        out_path.write_text(render_article_md(a), encoding="utf-8")
    print(f"  wrote {len(articles)} per-article .md siblings under {news_dir}")

    llms_path = out_dir / "llms.txt"
    llms_path.write_text(render_llms_txt(articles), encoding="utf-8")
    print(f"  wrote {llms_path}  ({llms_path.stat().st_size} bytes)")

    llms_full_path = out_dir / "llms-full.txt"
    llms_full_path.write_text(render_llms_full_txt(articles), encoding="utf-8")
    print(f"  wrote {llms_full_path}  ({llms_full_path.stat().st_size} bytes)")


if __name__ == "__main__":
    main()
