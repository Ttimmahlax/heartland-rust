# Article format

Each markdown file in this directory becomes one route at `/en/news/<slug>/`.
The slug is the filename stem.

```toml
+++
title = "Headline that contains the primary keyword"
excerpt = "140–180 chars. Used as <meta description>, OG description, and the card preview."
hero_image = "your-image-filename.jpg"
hero_alt = "Description of what the user sees, not what the article is about."
published_at = "2025-03-31"
author = "Henry the Hemp Fiber Expert"   # optional; defaults to "HFGA"
tags = ["hemp-fiber", "economics"]        # optional
seo_title = "Override <title> if needed"  # optional
seo_description = "Override <meta description>" # optional
+++
```

The body that follows the second `+++` is rendered through a CommonMark walker — no
raw HTML. Place inline images under `assets/articles/<slug>/`, reference them as
`/assets/articles/<slug>/<file>` in the markdown.

See the SOP in the root [README.md](../../README.md#sop-for-new-articles--cms) for the
full structure rules + SEO checklist.
