//! Hero background video — autoplay, muted, looped, native `<video>`.
//!
//! WebM-only (VP9) with a WebP poster. Modern browsers (Chrome/Firefox/Edge +
//! Safari 14+) cover ~99% of traffic; legacy fallbacks were dropped to keep
//! the deploy lean and avoid duplicate raster files getting indexed.

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct VideoBackgroundProps {
    /// Filename stem under `/assets/videos/` (without extension).
    /// e.g. `"landing"` → expects `landing.webm` + `landing-poster.webp`.
    #[props(default = String::from("landing"))]
    pub slug: String,
}

#[component]
pub fn VideoBackground(props: VideoBackgroundProps) -> Element {
    let slug = props.slug;
    let webm = format!("/assets/videos/{slug}.webm");
    let poster = format!("/assets/videos/{slug}-poster.webp");

    rsx! {
        video {
            class: "video-hero-bg",
            autoplay: true,
            muted: true,
            r#loop: true,
            playsinline: true,
            preload: "metadata",
            poster: "{poster}",
            aria_hidden: "true",
            tabindex: "-1",
            source { src: "{webm}", r#type: "video/webm" }
        }
    }
}
