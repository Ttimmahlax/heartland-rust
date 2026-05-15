use dioxus::prelude::*;

use crate::components::markdown::Markdown;
use crate::content;
use crate::seo::{article_jsonld, HreflangAlternates, Seo};
use crate::Route;

#[component]
pub fn Article(slug: String) -> Element {
    ArticleInner(ArticleInnerProps { slug, lang: "en".to_string() })
}

/// Translated article. Same content shape, served at /<lang>/sustainability-news/<slug>.
/// Falls back to English if no translation exists for the given slug.
#[component]
pub fn LangArticle(lang: String, slug: String) -> Element {
    ArticleInner(ArticleInnerProps { slug, lang })
}

#[derive(Props, Clone, PartialEq)]
struct ArticleInnerProps {
    slug: String,
    lang: String,
}

#[component]
fn ArticleInner(props: ArticleInnerProps) -> Element {
    let ArticleInnerProps { slug, lang } = props;
    let article = content::find_lang(&lang, &slug);

    let Some(article) = article else {
        return rsx! { NotFound {} };
    };

    // The URL path reflects the route the user hit, not the article's
    // canonical English path. SEO + canonical-link logic should reference
    // the locale-prefixed URL so Google sees a stable per-language identity.
    let url_path = if lang == "en" {
        article.url_path()
    } else {
        format!("/{}{}", lang, article.url_path())
    };
    let hero = article.hero_path();
    let title = article.front.title.clone();
    let seo_title = article.seo_title();
    let seo_description = article.seo_description();
    let author = article.front.author.clone();
    let published = article.front.published_at.clone();
    let tags = article.front.tags.clone();
    let body = article.body.clone();
    let alt = article.front.hero_alt.clone();

    let jsonld = article_jsonld(
        &title,
        &seo_description,
        &url_path,
        &hero,
        &published,
        &author,
    );
    let md_url = format!("{}.md", url_path);
    let english_path = article.url_path();
    let available_langs: Vec<String> = content::translations_for(&slug)
        .iter()
        .map(|s| s.to_string())
        .collect();

    rsx! {
        Seo {
            title: "{seo_title}",
            description: "{seo_description}",
            path: "{url_path}",
            image: "{hero}",
            image_width: "1210",
            image_height: "786",
            image_alt: "{alt}",
            og_type: "article",
        }
        HreflangAlternates {
            english_path: english_path,
            available_langs: available_langs,
        }
        document::Meta { property: "article:published_time", content: "{published}" }
        document::Link {
            rel: "alternate",
            r#type: "text/markdown",
            href: "{md_url}",
            title: "Markdown version (for AI / answer engines)",
        }
        document::Script { r#type: "application/ld+json", "{jsonld}" }

        article { class: "container-content py-12 md:py-16 px-6 md:px-12 lg:px-20 max-w-3xl",

            nav { class: "flex items-center gap-2 text-sm text-[color:var(--color-fg-muted)] mb-6",
                Link { to: Route::News {}, class: "hover:text-[color:var(--color-accent)]", "News" }
                span { "/" }
                for t in tags.iter() {
                    span {
                        key: "{t}",
                        class: "px-2 py-0.5 rounded-full bg-[color:var(--color-accent-quiet)] text-[color:var(--color-accent)] text-xs",
                        "{t}"
                    }
                }
            }

            h1 { class: "text-3xl md:text-5xl font-extrabold leading-tight mb-4", "{title}" }

            div { class: "flex items-center gap-4 text-sm text-[color:var(--color-fg-muted)] mb-8",
                em { "Signed by {author}" }
                span { "•" }
                time { datetime: "{published}", "{published}" }
            }

            figure { class: "mb-8",
                img {
                    src: "{hero}",
                    alt: "{alt}",
                    class: "w-full rounded-xl",
                    loading: "eager",
                }
            }

            Markdown { source: body }

            div { class: "mt-12 pt-8 border-t border-[color:var(--color-border)] flex items-center justify-between text-sm",
                Link { to: Route::News {}, class: "hover:text-[color:var(--color-accent)]", "← All articles" }
                Link { to: Route::WhyImperium {}, class: "btn-accent-gradient text-sm", "Why Imperium" }
            }
        }
    }
}

#[component]
fn NotFound() -> Element {
    rsx! {
        Seo {
            title: "Article Not Found",
            description: "The requested article could not be found.",
            path: "/sustainability-news",
        }
        section { class: "container-content py-24 text-center",
            h1 { class: "text-4xl font-bold mb-4", "Article not found" }
            p { class: "text-[color:var(--color-fg-muted)] mb-6",
                "This URL does not match any of our published articles."
            }
            Link { to: Route::News {}, class: "btn-accent-gradient", "Back to articles" }
        }
    }
}
