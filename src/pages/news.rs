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

    function apply() {
      var q = (input.value || '').trim().toLowerCase();
      if (clear) clear.style.display = q ? '' : 'none';
      if (!q) {
        for (var i = 0; i < cards.length; i++) cards[i].style.display = '';
        status.textContent = total + ' articles';
        if (empty) empty.style.display = 'none';
        grid.style.display = '';
        return;
      }
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

    input.addEventListener('input', apply);
    if (clear) clear.addEventListener('click', function () {
      input.value = '';
      apply();
      input.focus();
    });
    apply();
  }
  if (document.readyState !== 'loading') init();
  else document.addEventListener('DOMContentLoaded', init);
})();
"#;

#[component]
pub fn News() -> Element {
    let articles = content::all();
    let total = articles.len();

    rsx! {
        Seo {
            title: "Sustainability News & Articles",
            description: "Heartland's library of articles on industrial hemp, sustainable plastics, regenerative agriculture, supply-chain decarbonization, and the future of material innovation.",
            path: "/sustainability-news",
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

    rsx! {
        Link {
            to: Route::Article { slug: slug.clone() },
            class: "group block surface-glass overflow-hidden hover:translate-y-[-2px] transition-transform animate-fade-in-up",
            style: "animation-delay: {index * 30}ms",
            "data-search": "{haystack}",
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
        }
    }
}
