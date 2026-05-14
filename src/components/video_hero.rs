//! Hero background video — autoplay, muted, looped, native `<video>`.
//!
//! Replaces heartland.io's Revslider `<rs-bgvideo>` element with a standards-
//! compliant native `<video>` that needs no JS or external plugin. WebM-first
//! `<source>` ordering so capable browsers (Chrome/Firefox/Edge + Safari 14+)
//! load the smaller VP9 file; older Safari falls back to MP4. Both files +
//! the JPEG poster live under [assets/videos/](../../assets/videos/) and ship
//! verbatim in the deploy artifact.

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct VideoBackgroundProps {
    /// Filename stem under `/assets/videos/` (without extension).
    /// e.g. `"landing"` → expects `landing.webm`, `landing.mp4`, `landing-poster.jpg`.
    #[props(default = String::from("landing"))]
    pub slug: String,
}

#[component]
pub fn VideoBackground(props: VideoBackgroundProps) -> Element {
    let slug = props.slug;
    let webm = format!("/assets/videos/{slug}.webm");
    let mp4 = format!("/assets/videos/{slug}.mp4");
    let poster = format!("/assets/videos/{slug}-poster.jpg");

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
            source { src: "{mp4}",  r#type: "video/mp4" }
        }
    }
}
