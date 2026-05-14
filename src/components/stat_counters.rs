//! 4-stat row used on every marketing page.
//!
//! Default numbers come from heartland.io's published metrics: acres
//! contracted, supply-chain emissions reductions, automotive parts shipped,
//! and tons of CO2e offset.

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StatCountersProps {
    #[props(default = default_stats())]
    pub stats: Vec<(String, String)>,
}

pub fn default_stats() -> Vec<(String, String)> {
    vec![
        ("12,000+".into(), "Acres of Imperium Farmed".into()),
        ("30%".into(), "Cost Reduction vs. Talc".into()),
        ("90%".into(), "Less CO₂ vs. Glass Fiber".into()),
        ("11".into(), "States Growing Imperium".into()),
    ]
}

pub fn product_stats() -> Vec<(String, String)> {
    vec![
        ("$0.50/lb".into(), "Imperium Filler Cost".into()),
        ("Up to 40%".into(), "Loading in Polypropylene".into()),
        ("-90%".into(), "vs Glass Fiber Emissions".into()),
        ("Drop-In".into(), "Existing Compounding Lines".into()),
    ]
}

pub fn farm_stats() -> Vec<(String, String)> {
    vec![
        ("11".into(), "US States in Network".into()),
        ("12,000+".into(), "Acres Contracted".into()),
        ("21%".into(), "Avg. Premium to Farmers".into()),
        ("Hemp4Soil".into(), "USDA Grant Recipient".into()),
    ]
}

#[component]
pub fn StatCounters(props: StatCountersProps) -> Element {
    rsx! {
        section { class: "container-content py-12",
            div { class: "grid grid-cols-2 md:grid-cols-4 gap-4 md:gap-6",
                for (i, (value, label)) in props.stats.into_iter().enumerate() {
                    div {
                        key: "{i}",
                        class: "surface-glass p-6 text-center animate-scale-in",
                        style: "animation-delay: {i * 60}ms",
                        div {
                            class: "text-3xl md:text-4xl font-display font-extrabold text-[color:var(--color-accent)]",
                            "{value}"
                        }
                        div {
                            class: "mt-1 text-sm text-[color:var(--color-fg-muted)]",
                            "{label}"
                        }
                    }
                }
            }
        }
    }
}
