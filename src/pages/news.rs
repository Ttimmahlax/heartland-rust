use dioxus::prelude::*;

use crate::content;
use crate::seo::Seo;
use crate::Route;

/// Inline JS that drives the article search filter. Runs as soon as the page
/// loads — no WASM hydration required, so typing in the search box filters
/// the visible cards instantly. Each article card carries a `data-search`
/// attribute containing its lowercase title + excerpt + tags + slug; the
/// script walks the cards on every keystroke, hides non-matches, and
/// updates the result counter + "no matches" message.
const SEARCH_JS: &str = r#"
(function () {
  // Initial visible cards on page load. The rest get display:none and
  // reveal in batches as the user scrolls (Intersection Observer on a
  // sentinel placed at the end of the currently-visible window).
  var INITIAL = 15;
  var BATCH   = 15;

  function init() {
    var input  = document.getElementById('news-search-input');
    var grid   = document.getElementById('news-grid');
    var clear  = document.getElementById('news-search-clear');
    var status = document.getElementById('news-search-status');
    var empty  = document.getElementById('news-search-empty');
    if (!input || !grid || !status) { setTimeout(init, 50); return; }

    var cards = Array.prototype.slice.call(
      grid.querySelectorAll('[data-search]')
    );
    var total = cards.length;
    var visibleCount = Math.min(INITIAL, total);
    var searching = false;

    // Hide everything past the initial window. If JS isn't running the
    // cards just stay visible — that's the graceful no-JS fallback.
    function applyLazy() {
      for (var i = 0; i < cards.length; i++) {
        cards[i].style.display = (i < visibleCount) ? '' : 'none';
      }
    }

    function revealMore() {
      if (visibleCount >= total) return false;
      var next = Math.min(visibleCount + BATCH, total);
      for (var i = visibleCount; i < next; i++) cards[i].style.display = '';
      visibleCount = next;
      return visibleCount < total;
    }

    // Auto-reveal the next batch when the user scrolls near the bottom of
    // the current visible window. The sentinel is the last currently-visible
    // card; once it enters the viewport we reveal another BATCH.
    var observer = ('IntersectionObserver' in window)
      ? new IntersectionObserver(function (entries) {
          if (searching) return;
          for (var k = 0; k < entries.length; k++) {
            if (entries[k].isIntersecting) {
              observer.unobserve(entries[k].target);
              var more = revealMore();
              if (more) observeSentinel();
            }
          }
        }, { rootMargin: '600px 0px' })
      : null;

    function observeSentinel() {
      if (!observer || visibleCount === 0 || visibleCount >= total) return;
      observer.observe(cards[visibleCount - 1]);
    }

    function applySearch() {
      var q = (input.value || '').trim().toLowerCase();
      if (clear) clear.style.display = q ? '' : 'none';

      if (!q) {
        // Empty search → re-apply lazy state (collapse back to initial window
        // unless the user already revealed more by scrolling).
        searching = false;
        applyLazy();
        observeSentinel();
        status.textContent = total + ' articles';
        if (empty) empty.style.display = 'none';
        grid.style.display = '';
        return;
      }

      // During search, ignore the lazy state — every card is a candidate.
      searching = true;
      if (observer) observer.disconnect();
      var shown = 0;
      for (var j = 0; j < cards.length; j++) {
        var hit = cards[j].getAttribute('data-search').indexOf(q) !== -1;
        cards[j].style.display = hit ? '' : 'none';
        if (hit) shown++;
      }
      if (shown === 0) {
        status.textContent = 'No articles match "' + input.value + '". Try a different keyword.';
        if (empty) empty.style.display = '';
        grid.style.display = 'none';
      } else {
        status.textContent = shown === 1
          ? '1 match for "' + input.value + '"'
          : shown + ' matches for "' + input.value + '"';
        if (empty) empty.style.display = 'none';
        grid.style.display = '';
      }
    }

    // Initial render: apply lazy, wire up listeners, hook up the observer.
    applyLazy();
    observeSentinel();
    status.textContent = total + ' articles';

    input.addEventListener('input', applySearch);
    if (clear) clear.addEventListener('click', function () {
      input.value = '';
      applySearch();
      input.focus();
    });
  }
  if (document.readyState !== 'loading') init();
  else document.addEventListener('DOMContentLoaded', init);
})();
"#;

#[component]
pub fn News() -> Element {
    NewsInner(NewsInnerProps { lang: "en".to_string() })
}

/// Translated news index. Lists articles available in the requested language
/// (with English fallback for slugs that don't have a translation yet).
#[component]
pub fn LangNews(lang: String) -> Element {
    NewsInner(NewsInnerProps { lang })
}

#[derive(Props, Clone, PartialEq)]
struct NewsInnerProps {
    lang: String,
}

#[component]
fn NewsInner(props: NewsInnerProps) -> Element {
    let lang = props.lang;
    // Pick the article in the requested language; fall back to English for
    // any slug that doesn't have a translation yet. Each slug appears once.
    let articles: Vec<&'static content::Article> = {
        let mut by_slug: std::collections::BTreeMap<&str, &'static content::Article> =
            std::collections::BTreeMap::new();
        // English first (baseline).
        for a in content::all().iter().filter(|a| a.lang == "en") {
            by_slug.insert(a.slug.as_str(), a);
        }
        // Translated entries override the English fallback.
        if lang != "en" {
            for a in content::all().iter().filter(|a| a.lang == lang) {
                by_slug.insert(a.slug.as_str(), a);
            }
        }
        let mut v: Vec<&'static content::Article> = by_slug.into_values().collect();
        v.sort_by(|a, b| b.front.published_at.cmp(&a.front.published_at));
        v
    };
    let total = articles.len();

    let path = if lang == "en" {
        "/sustainability-news".to_string()
    } else {
        format!("/{}/sustainability-news", lang)
    };

    rsx! {
        Seo {
            title: "Sustainability News & Articles",
            description: "Heartland's library of articles on industrial hemp, sustainable plastics, regenerative agriculture, supply-chain decarbonization, and the future of material innovation.",
            path: "{path}",
        }

        section { class: "bg-mesh-hero section-soft-bottom",
            div { class: "container-content py-20 md:py-24 text-center",
                p { class: "text-sm uppercase tracking-[0.2em] text-[color:var(--color-accent)] mb-4",
                    "Library"
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold leading-tight max-w-3xl mx-auto",
                    "Sustainability News"
                    br {}
                    span { class: "text-gradient-red", "& Yapping" }
                }
                p { class: "mt-6 max-w-2xl mx-auto text-lg text-[color:var(--color-fg-muted)]",
                    "Long-form coverage of how Imperium reshapes plastic compounding, automotive supply chains, regenerative agronomy, and industrial decarbonization."
                }
            }
        }

        section { class: "container-content pt-8 md:pt-12",
            div { class: "max-w-2xl mx-auto",
                label { class: "block",
                    span { class: "sr-only", "Search articles" }
                    div { class: "relative",
                        span { class: "pointer-events-none absolute left-4 top-1/2 -translate-y-1/2 text-[color:var(--color-fg-muted)]",
                            svg { width: "18", height: "18", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                circle { cx: "11", cy: "11", r: "8" }
                                line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                            }
                        }
                        input {
                            id: "news-search-input",
                            r#type: "search",
                            placeholder: "Search articles by title, topic, or keyword…",
                            aria_label: "Search articles",
                            autocomplete: "off",
                            spellcheck: "false",
                            class: "w-full pl-12 pr-12 py-3 rounded-full border border-[color:var(--color-border)] bg-[color:var(--color-surface)] text-[color:var(--color-fg)] placeholder:text-[color:var(--color-fg-muted)] focus:outline-none focus:border-[color:var(--color-accent)] focus:ring-2 focus:ring-[color:var(--color-accent-quiet)] transition",
                        }
                        button {
                            id: "news-search-clear",
                            r#type: "button",
                            class: "absolute right-3 top-1/2 -translate-y-1/2 inline-flex items-center justify-center w-7 h-7 rounded-full hover:bg-[color:var(--color-accent-quiet)] text-[color:var(--color-fg-muted)] hover:text-[color:var(--color-accent)]",
                            style: "display: none",
                            aria_label: "Clear search",
                            svg { width: "14", height: "14", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round",
                                line { x1: "18", y1: "6", x2: "6", y2: "18" }
                                line { x1: "6", y1: "6", x2: "18", y2: "18" }
                            }
                        }
                    }
                }
                p {
                    id: "news-search-status",
                    class: "mt-3 text-center text-sm text-[color:var(--color-fg-muted)]",
                    "{total} articles"
                }
            }
        }

        section { class: "container-content pb-16 pt-8",
            if articles.is_empty() {
                div { class: "text-center text-[color:var(--color-fg-muted)] py-12",
                    "Articles are being migrated from heartland.io. Check back soon."
                }
            } else {
                div {
                    id: "news-grid",
                    class: "grid gap-6 md:grid-cols-2 lg:grid-cols-3",
                    for (i, article) in articles.iter().enumerate() {
                        ArticleCard {
                            key: "{article.slug}",
                            index: i,
                            lang: lang.clone(),
                            slug: article.slug.clone(),
                            title: article.front.title.clone(),
                            excerpt: article.front.excerpt.clone(),
                            hero: article.hero_path(),
                            tags: article.front.tags.clone(),
                        }
                    }
                }
                div {
                    id: "news-search-empty",
                    class: "text-center text-[color:var(--color-fg-muted)] py-12",
                    style: "display: none",
                    "No articles match your search. Try a different keyword."
                }
            }
        }

        document::Script { "{SEARCH_JS}" }
    }
}

#[component]
fn ArticleCard(
    index: usize,
    lang: String,
    slug: String,
    title: String,
    excerpt: String,
    hero: String,
    tags: Vec<String>,
) -> Element {
    // Build a lowercase haystack that the inline JS search will match
    // against on every keystroke. Includes title, excerpt, slug, and all
    // tags so any of those substrings counts as a hit.
    let haystack = {
        let mut s = String::with_capacity(title.len() + excerpt.len() + slug.len() + 64);
        s.push_str(&title.to_lowercase());
        s.push(' ');
        s.push_str(&excerpt.to_lowercase());
        s.push(' ');
        s.push_str(&slug.to_lowercase());
        for t in &tags {
            s.push(' ');
            s.push_str(&t.to_lowercase());
        }
        s
    };

    let card_class = "group block surface-glass overflow-hidden hover:translate-y-[-2px] transition-transform animate-fade-in-up";
    let card_style = format!("animation-delay: {}ms", index * 30);
    let body = rsx! {
        div { class: "aspect-[16/9] overflow-hidden bg-[color:var(--color-surface)]",
            img {
                src: "{hero}",
                alt: "{title}",
                loading: "lazy",
                class: "w-full h-full object-cover transition-transform duration-300 group-hover:scale-105",
            }
        }
        div { class: "p-5",
            if !tags.is_empty() {
                div { class: "flex flex-wrap gap-2 mb-2",
                    for tag in tags.iter().take(3) {
                        span {
                            key: "{tag}",
                            class: "text-xs px-2 py-0.5 rounded-full bg-[color:var(--color-accent-quiet)] text-[color:var(--color-accent)]",
                            "{tag}"
                        }
                    }
                }
            }
            h2 { class: "font-display font-semibold text-lg leading-snug group-hover:text-[color:var(--color-accent)]",
                "{title}"
            }
            if !excerpt.is_empty() {
                p { class: "mt-2 text-sm text-[color:var(--color-fg-muted)] line-clamp-3",
                    "{excerpt}"
                }
            }
        }
    };

    if lang == "en" {
        rsx! {
            Link {
                to: Route::Article { slug: slug.clone() },
                class: "{card_class}",
                style: "{card_style}",
                "data-search": "{haystack}",
                {body}
            }
        }
    } else {
        rsx! {
            Link {
                to: Route::LangArticle { lang: lang.clone(), slug: slug.clone() },
                class: "{card_class}",
                style: "{card_style}",
                "data-search": "{haystack}",
                {body}
            }
        }
    }
}
