//! Reusable 4-step zigzag supply-chain timeline.
//!
//! Used on the landing page ("Throughout The Supply Chain") and on
//! `/why-imperium` ("Your Supply Chain Partner"). Each consumer composes
//! their own 4 SupplyChainStep instances with their own copy.

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum SupplyIcon {
    Tractor,
    Blender,
    Gears,
    Store,
}

#[component]
pub fn SupplyChainStep(
    number: u8,
    icon: SupplyIcon,
    heading: &'static str,
    body: &'static str,
    align_right: bool,
) -> Element {
    // On desktop: alternate the card to the left or right of the center rail.
    // On mobile: every card is full-width.
    let outer_grid = "grid md:grid-cols-2 gap-6 md:gap-12 mb-12 md:mb-16 items-center";
    let card_col = if align_right { "md:col-start-2" } else { "md:col-start-1" };
    let card_align = if align_right { "md:text-left" } else { "md:text-right" };
    let icon_align = if align_right { "md:justify-start" } else { "md:justify-end" };

    rsx! {
        div { class: "{outer_grid} animate-fade-in-up",
            div { class: "{card_col} surface-glass p-6 md:p-8 rounded-xl shadow-lg {card_align}",
                div { class: "flex items-center gap-4 mb-4 {icon_align}",
                    if align_right {
                        StepNumber { number }
                        IconBadge { icon: icon.clone() }
                    } else {
                        IconBadge { icon: icon.clone() }
                        StepNumber { number }
                    }
                }
                h3 { class: "text-xl md:text-2xl font-display font-bold mb-3", "{heading}" }
                p { class: "text-[color:var(--color-fg-muted)] leading-relaxed", "{body}" }
            }
        }
    }
}

#[component]
fn StepNumber(number: u8) -> Element {
    rsx! {
        div { class: "w-10 h-10 md:w-12 md:h-12 rounded-full bg-[color:var(--color-accent)] text-white font-display font-extrabold flex items-center justify-center text-lg md:text-xl shadow-md",
            "{number}"
        }
    }
}

#[component]
fn IconBadge(icon: SupplyIcon) -> Element {
    rsx! {
        div { class: "w-12 h-12 md:w-14 md:h-14 rounded-full bg-[color:var(--color-accent-quiet)] flex items-center justify-center text-[color:var(--color-accent)]",
            match icon {
                SupplyIcon::Tractor => rsx! {
                    svg { width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        circle { cx: "7", cy: "18", r: "3" }
                        circle { cx: "16", cy: "18", r: "3" }
                        path { d: "M3 18h1M11 18h2M19 18h2" }
                        path { d: "M5 15V8h6l3 5h5v5" }
                        path { d: "M11 8l3 5" }
                    }
                },
                SupplyIcon::Blender => rsx! {
                    svg { width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        path { d: "M2 20h20" }
                        path { d: "M4 20V10l5 4V10l5 4V6l5 14" }
                        rect { x: "6", y: "14", width: "2", height: "3" }
                        rect { x: "11", y: "14", width: "2", height: "3" }
                        rect { x: "16", y: "14", width: "2", height: "3" }
                    }
                },
                SupplyIcon::Gears => rsx! {
                    svg { width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        circle { cx: "12", cy: "12", r: "3" }
                        path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" }
                    }
                },
                SupplyIcon::Store => rsx! {
                    svg { width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        path { d: "M3 9l1.5-5h15L21 9" }
                        path { d: "M3 9v11h18V9" }
                        path { d: "M3 9c0 1.66 1.34 3 3 3s3-1.34 3-3" }
                        path { d: "M9 9c0 1.66 1.34 3 3 3s3-1.34 3-3" }
                        path { d: "M15 9c0 1.66 1.34 3 3 3s3-1.34 3-3" }
                        path { d: "M10 20v-5h4v5" }
                    }
                },
            }
        }
    }
}
