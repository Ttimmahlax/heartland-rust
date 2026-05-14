//! pulldown-cmark walker → safe Dioxus elements (no `dangerous_inner_html`).
//!
//! Raw HTML in markdown is **dropped silently** — content authors should not
//! be embedding raw tags. Output is wrapped in `.prose-article` styling.

use dioxus::prelude::*;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Element {
        tag: &'static str,
        attrs: Vec<(&'static str, String)>,
        children: Vec<Node>,
    },
    Text(String),
    Br,
    Hr,
}

pub fn parse(src: &str) -> Vec<Node> {
    let opts = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH;
    let parser = Parser::new_ext(src, opts);

    let mut stack: Vec<Node> = vec![Node::Element {
        tag: "root",
        attrs: vec![],
        children: vec![],
    }];

    let push_child = |stack: &mut Vec<Node>, child: Node| {
        if let Some(Node::Element { children, .. }) = stack.last_mut() {
            children.push(child);
        }
    };

    for event in parser {
        match event {
            Event::Start(t) => {
                let (tag, attrs) = open_tag(&t);
                stack.push(Node::Element {
                    tag,
                    attrs,
                    children: vec![],
                });
            }
            Event::End(_) => {
                if stack.len() > 1 {
                    if let Some(top) = stack.pop() {
                        push_child(&mut stack, top);
                    }
                }
            }
            Event::Text(s) => push_child(&mut stack, Node::Text(s.to_string())),
            Event::Code(s) => push_child(
                &mut stack,
                Node::Element {
                    tag: "code",
                    attrs: vec![],
                    children: vec![Node::Text(s.to_string())],
                },
            ),
            Event::Html(_) | Event::InlineHtml(_) => {}
            Event::SoftBreak => push_child(&mut stack, Node::Text(" ".into())),
            Event::HardBreak => push_child(&mut stack, Node::Br),
            Event::Rule => push_child(&mut stack, Node::Hr),
            Event::TaskListMarker(_) | Event::FootnoteReference(_) => {}
            Event::InlineMath(_) | Event::DisplayMath(_) => {}
        }
    }

    if let Some(Node::Element { children, .. }) = stack.into_iter().next() {
        children
    } else {
        Vec::new()
    }
}

fn open_tag(t: &Tag) -> (&'static str, Vec<(&'static str, String)>) {
    match t {
        Tag::Paragraph => ("p", vec![]),
        Tag::Heading { level, .. } => {
            let tag = match level {
                HeadingLevel::H1 => "h1",
                HeadingLevel::H2 => "h2",
                HeadingLevel::H3 => "h3",
                HeadingLevel::H4 => "h4",
                HeadingLevel::H5 => "h5",
                HeadingLevel::H6 => "h6",
            };
            (tag, vec![])
        }
        Tag::BlockQuote(_) => ("blockquote", vec![]),
        Tag::CodeBlock(_) => ("pre", vec![]),
        Tag::List(Some(_)) => ("ol", vec![]),
        Tag::List(None) => ("ul", vec![]),
        Tag::Item => ("li", vec![]),
        Tag::Emphasis => ("em", vec![]),
        Tag::Strong => ("strong", vec![]),
        Tag::Strikethrough => ("s", vec![]),
        Tag::Link {
            dest_url, title, ..
        } => {
            let mut a = vec![("href", dest_url.to_string())];
            if !title.is_empty() {
                a.push(("title", title.to_string()));
            }
            ("a", a)
        }
        Tag::Image {
            dest_url, title, ..
        } => {
            let mut a = vec![("src", dest_url.to_string())];
            if !title.is_empty() {
                a.push(("alt", title.to_string()));
            }
            ("img", a)
        }
        Tag::Table(_) => ("table", vec![]),
        Tag::TableHead => ("thead", vec![]),
        Tag::TableRow => ("tr", vec![]),
        Tag::TableCell => ("td", vec![]),
        _ => ("span", vec![]),
    }
}

fn attr<'a>(attrs: &'a [(&'static str, String)], key: &str) -> &'a str {
    attrs
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| v.as_str())
        .unwrap_or("")
}

fn render(node: &Node) -> Element {
    match node {
        Node::Text(s) => rsx! { "{s}" },
        Node::Br => rsx! { br {} },
        Node::Hr => rsx! { hr {} },
        Node::Element {
            tag,
            attrs,
            children,
        } => render_element(tag, attrs, children),
    }
}

fn render_children(children: &[Node]) -> Element {
    let kids: Vec<Element> = children.iter().map(render).collect();
    rsx! { {kids.into_iter()} }
}

fn render_element(
    tag: &'static str,
    attrs: &[(&'static str, String)],
    children: &[Node],
) -> Element {
    match tag {
        "p" => rsx! { p { {render_children(children)} } },
        "h1" => rsx! { h1 { {render_children(children)} } },
        "h2" => rsx! { h2 { {render_children(children)} } },
        "h3" => rsx! { h3 { {render_children(children)} } },
        "h4" => rsx! { h4 { {render_children(children)} } },
        "h5" => rsx! { h5 { {render_children(children)} } },
        "h6" => rsx! { h6 { {render_children(children)} } },
        "ul" => rsx! { ul { {render_children(children)} } },
        "ol" => rsx! { ol { {render_children(children)} } },
        "li" => rsx! { li { {render_children(children)} } },
        "em" => rsx! { em { {render_children(children)} } },
        "strong" => rsx! { strong { {render_children(children)} } },
        "s" => rsx! { s { {render_children(children)} } },
        "code" => rsx! { code { {render_children(children)} } },
        "pre" => rsx! { pre { {render_children(children)} } },
        "blockquote" => rsx! { blockquote { {render_children(children)} } },
        "table" => rsx! { table { {render_children(children)} } },
        "thead" => rsx! { thead { {render_children(children)} } },
        "tbody" => rsx! { tbody { {render_children(children)} } },
        "tr" => rsx! { tr { {render_children(children)} } },
        "th" => rsx! { th { {render_children(children)} } },
        "td" => rsx! { td { {render_children(children)} } },
        "a" => {
            let href = attr(attrs, "href").to_string();
            let title = attr(attrs, "title").to_string();
            let is_external = href.starts_with("http") || href.starts_with("//");
            rsx! {
                a {
                    href: "{href}",
                    title: "{title}",
                    rel: if is_external { "noopener noreferrer" } else { "" },
                    target: if is_external { "_blank" } else { "" },
                    {render_children(children)}
                }
            }
        }
        "img" => {
            let src = attr(attrs, "src").to_string();
            let alt = attr(attrs, "alt").to_string();
            rsx! { img { src: "{src}", alt: "{alt}", loading: "lazy" } }
        }
        _ => rsx! { span { {render_children(children)} } },
    }
}

#[component]
pub fn Markdown(source: String) -> Element {
    let nodes = parse(&source);
    rsx! {
        div { class: "prose-article",
            {nodes.iter().map(|n| render(n)).collect::<Vec<Element>>().into_iter()}
        }
    }
}
