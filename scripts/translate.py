#!/usr/bin/env python3
"""
Translate Heartland's English markdown content into all supported languages
using the Anthropic Claude API (Haiku 4.5).

Inputs:  content/articles/*.md     (English source of truth)
Outputs: content/articles/<lang>/*.md   (translated, mirror structure)

How it works:
  1. Enumerate English source files.
  2. For each (file, language) pair:
       - Hash the source content (sha256).
       - If the cached hash for that pair matches → skip (already translated).
       - Otherwise call Claude with a translation prompt, save the output,
         and update the cache.
  3. Cache lives at `.translation-cache.json` in the repo root.

Usage:
    export ANTHROPIC_API_KEY=sk-ant-...
    pip install -r scripts/requirements.txt    # one time

    ./scripts/translate.py                     # translate everything missing/changed
    ./scripts/translate.py --lang es           # only Spanish
    ./scripts/translate.py --file content/articles/heartland-raises-seed-capital.md
    ./scripts/translate.py --force             # ignore cache, re-translate all
    ./scripts/translate.py --dry-run           # list what would be translated, no API calls
    ./scripts/translate.py --concurrency 10    # tune parallelism (default 5)

Future-content workflow:
  Any time `content/articles/*.md` changes (new article or edits), re-run
  the script. Unchanged files are skipped via cache; only new/changed
  pairs get translated. Wire into CI for full automation:

      # .github/workflows/translate.yml
      - run: ./scripts/translate.py
      - run: git add content/articles && git commit -m "Re-translate" ...
"""

from __future__ import annotations

import argparse
import concurrent.futures
import hashlib
import json
import os
import re
import sys
from pathlib import Path

try:
    import anthropic
except ImportError:
    print("Missing dep: pip install -r scripts/requirements.txt", file=sys.stderr)
    sys.exit(1)


# ----------------------------------------------------------------------------
# Configuration
# ----------------------------------------------------------------------------

REPO_ROOT = Path(__file__).resolve().parent.parent
ARTICLES_DIR = REPO_ROOT / "content" / "articles"
CACHE_FILE = REPO_ROOT / ".translation-cache.json"

# Translation model. Haiku 4.5 is plenty for this — it's faster, cheaper,
# and indistinguishable from Sonnet/Opus on this task for the language pairs
# we care about.
MODEL = "claude-haiku-4-5-20251001"

# Target languages. Must stay in sync with `Language::ALL` in src/i18n.rs.
LANGUAGES: dict[str, str] = {
    "ar":    "Arabic",
    "bn":    "Bengali",
    "de":    "German",
    "es":    "Spanish",
    "fr":    "French",
    "hi":    "Hindi",
    "it":    "Italian",
    "ja":    "Japanese",
    "ko":    "Korean",
    "nl":    "Dutch",
    "pa":    "Punjabi (Gurmukhi script)",
    "pl":    "Polish",
    "pt":    "Portuguese",
    "tr":    "Turkish",
    "ur":    "Urdu",
    "vi":    "Vietnamese",
    "zh-CN": "Chinese (Simplified)",
}

# Brand / product names that must NEVER be translated. Add to this list as
# new products launch.
GLOSSARY = [
    "Heartland",
    "Heartland Industries",
    "Imperium",
    "Imperium Masterbatch",
    "Imperium Filled Resin",
    "Imperium Filler",
    "Imperium Fibers",
    "Imperium Animal Feed",
    "Imperium Pork Feed",
    "Imperium Cattle Feed",
    "Imperium Chicken Feed",
    "Imperium Spin-Ready White Fiber",
    "Imperium Yarn",
    "Imperium Fabric",
    "Imperium Graphene",
    "Detroit",
    "Michigan",
    "PolyHemp",
]


# ----------------------------------------------------------------------------
# Cache
# ----------------------------------------------------------------------------

def load_cache() -> dict:
    if not CACHE_FILE.exists():
        return {}
    try:
        return json.loads(CACHE_FILE.read_text())
    except json.JSONDecodeError:
        print(f"warning: {CACHE_FILE} is corrupted, starting fresh", file=sys.stderr)
        return {}


def save_cache(cache: dict) -> None:
    CACHE_FILE.write_text(json.dumps(cache, indent=2, sort_keys=True) + "\n")


def content_hash(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()[:16]


# ----------------------------------------------------------------------------
# Translation
# ----------------------------------------------------------------------------

def build_system_prompt(target_lang_name: str) -> str:
    """The system prompt is the same for every file → cache it for ~90%
    discount on subsequent calls (prompt caching).
    """
    glossary_lines = "\n".join(f"  - {term}" for term in GLOSSARY)
    return f"""You are translating marketing and technical content for Heartland Industries, an industrial hemp materials company. Translate the user's English markdown into {target_lang_name}.

RULES (all are critical):

1. PRESERVE ALL MARKDOWN SYNTAX exactly:
   - Headers: `#`, `##`, `###`
   - Lists: `-`, `*`, `1.`
   - Links: `[text](url)` — translate the text, leave the URL unchanged
   - Bold/italic: `**bold**`, `*italic*`
   - Code: `` `inline` `` and ```fenced blocks```
   - Blockquotes: `>`
   - Horizontal rules: `---`
   - Images: `![alt](url)` — translate ONLY the alt text, never the URL

2. PRESERVE THE TOML FRONTMATTER at the top of the file (between `+++` lines):
   - Translate the VALUES for: `title`, `excerpt`, `hero_alt`
   - Do NOT translate or modify: keys (left side of `=`), slugs, image filenames, dates (`published_at`), `author`, `categories`, `tags`
   - Tags and categories stay in English (they're URL slugs)

3. NEVER TRANSLATE these brand / product / proper names — keep them EXACTLY as written in English:
{glossary_lines}

4. NEVER add explanations, preambles, code fences around the whole output, or anything else. Output ONLY the translated markdown, starting with `+++` and ending where the source ends.

5. Match the SOURCE'S register and tone: business-professional, somewhat technical, marketing-aware. Don't make the translation more formal than the English; don't make it more casual either.

6. Use the TARGET LANGUAGE'S natural conventions for punctuation, quotation marks, and number formatting (e.g. `1,234` vs `1.234` for European languages, full-width punctuation for CJK).

7. If the source markdown contains HTML (rare), preserve all HTML tags and only translate text content inside them.
"""


def translate_one(
    client: anthropic.Anthropic,
    source_text: str,
    target_lang_code: str,
    target_lang_name: str,
) -> str:
    """Send one file to Claude, return the translated content.

    Retries on 429 (rate limit) with exponential backoff that honors the
    `retry-after` header when present.
    """
    import time
    system = build_system_prompt(target_lang_name)
    max_retries = 6
    backoff = 5.0  # seconds, doubles each retry

    for attempt in range(max_retries):
        try:
            response = client.messages.create(
                model=MODEL,
                max_tokens=8192,
                system=[{
                    "type": "text",
                    "text": system,
                    "cache_control": {"type": "ephemeral"},
                }],
                messages=[{
                    "role": "user",
                    "content": source_text,
                }],
            )
            out = response.content[0].text
            out = re.sub(r"^```(?:markdown|md)?\s*\n", "", out)
            out = re.sub(r"\n```\s*$", "", out)
            return out
        except anthropic.RateLimitError as e:
            # Honor retry-after if the API provides it.
            retry_after = backoff
            try:
                hdr = e.response.headers.get("retry-after") if e.response else None
                if hdr:
                    retry_after = float(hdr)
            except Exception:
                pass
            if attempt == max_retries - 1:
                raise
            time.sleep(retry_after)
            backoff *= 2
        except anthropic.APIStatusError as e:
            # Transient 5xx: also retry with backoff.
            if 500 <= getattr(e, "status_code", 0) < 600 and attempt < max_retries - 1:
                time.sleep(backoff)
                backoff *= 2
                continue
            raise

    raise RuntimeError("unreachable")


# ----------------------------------------------------------------------------
# File enumeration + planning
# ----------------------------------------------------------------------------

def english_articles() -> list[Path]:
    if not ARTICLES_DIR.exists():
        return []
    # Skip language subdirectories — we only translate from the top-level English source.
    return sorted(
        p for p in ARTICLES_DIR.glob("*.md")
        if p.is_file() and p.parent == ARTICLES_DIR
    )


def output_path(source: Path, lang_code: str) -> Path:
    return ARTICLES_DIR / lang_code / source.name


def cache_key(source: Path, lang_code: str) -> str:
    rel = source.relative_to(REPO_ROOT).as_posix()
    return f"{rel}::{lang_code}"


# ----------------------------------------------------------------------------
# Main
# ----------------------------------------------------------------------------

def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--lang", action="append", default=None,
                        help="Only translate to this language code (repeatable). Default: all 17 target langs.")
    parser.add_argument("--file", action="append", default=None,
                        help="Only translate this specific source file path (repeatable).")
    parser.add_argument("--force", action="store_true",
                        help="Ignore cache; re-translate everything in scope.")
    parser.add_argument("--dry-run", action="store_true",
                        help="List what would be translated; make no API calls.")
    parser.add_argument("--concurrency", type=int, default=5,
                        help="Max concurrent API calls. Default 5.")
    args = parser.parse_args()

    if not args.dry_run and not os.environ.get("ANTHROPIC_API_KEY"):
        print("ANTHROPIC_API_KEY not set. Export it before running (or use --dry-run).", file=sys.stderr)
        return 2

    # Build the work list.
    target_langs: dict[str, str] = (
        {code: LANGUAGES[code] for code in args.lang} if args.lang else dict(LANGUAGES)
    )
    unknown = [c for c in (args.lang or []) if c not in LANGUAGES]
    if unknown:
        print(f"Unknown language code(s): {unknown}", file=sys.stderr)
        return 2

    if args.file:
        sources = [Path(f).resolve() for f in args.file]
        for s in sources:
            if not s.exists():
                print(f"File not found: {s}", file=sys.stderr)
                return 2
    else:
        sources = english_articles()

    cache = load_cache()
    work: list[tuple[Path, str, str]] = []  # (source, lang_code, lang_name)

    for source in sources:
        try:
            text = source.read_text()
        except Exception as e:
            print(f"skip {source} (read failed: {e})", file=sys.stderr)
            continue

        src_hash = content_hash(text)
        for lang_code, lang_name in target_langs.items():
            out_path = output_path(source, lang_code)
            key = cache_key(source, lang_code)
            cached_hash = cache.get(key)
            if not args.force and cached_hash == src_hash and out_path.exists():
                continue
            work.append((source, lang_code, lang_name))

    print(f"Sources: {len(sources)}   Languages: {len(target_langs)}   Pairs to translate: {len(work)}")

    if args.dry_run:
        for source, lang_code, _ in work[:20]:
            print(f"  would translate: {source.relative_to(REPO_ROOT)} -> {lang_code}")
        if len(work) > 20:
            print(f"  ... and {len(work) - 20} more")
        return 0

    if not work:
        print("Nothing to do.")
        return 0

    client = anthropic.Anthropic()
    done = 0
    failed: list[tuple[Path, str, str]] = []

    def do_one(item):
        source, lang_code, lang_name = item
        text = source.read_text()
        try:
            translated = translate_one(client, text, lang_code, lang_name)
        except Exception as e:
            return (item, None, str(e))
        return (item, translated, None)

    with concurrent.futures.ThreadPoolExecutor(max_workers=args.concurrency) as pool:
        futures = [pool.submit(do_one, w) for w in work]
        for fut in concurrent.futures.as_completed(futures):
            item, translated, err = fut.result()
            source, lang_code, _ = item
            if err:
                failed.append((source, lang_code, err))
                print(f"  FAIL  {source.name} -> {lang_code}: {err}", file=sys.stderr)
                continue

            out_path = output_path(source, lang_code)
            out_path.parent.mkdir(parents=True, exist_ok=True)
            out_path.write_text(translated)

            src_hash = content_hash(source.read_text())
            cache[cache_key(source, lang_code)] = src_hash
            done += 1
            if done % 10 == 0 or done == len(work):
                save_cache(cache)
                print(f"  done {done}/{len(work)}   (last: {source.name} -> {lang_code})")

    save_cache(cache)
    print(f"\nTranslated {done} pair(s). Failed: {len(failed)}.")
    if failed:
        print("Failed pairs (re-run to retry, or use --force):")
        for source, lang_code, err in failed[:20]:
            print(f"  {source.name} -> {lang_code}: {err}")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
