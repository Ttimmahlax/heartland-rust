//! Language / i18n primitives.
//!
//! Translated pages live at `/<prefix>/...`; English uses an empty prefix.
//! Prefixes match what GTranslate currently serves so SEO maps over at cutover.

use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Arabic,
    Bengali,
    Chinese,
    Dutch,
    French,
    German,
    Hindi,
    Italian,
    Japanese,
    Korean,
    Polish,
    Portuguese,
    Punjabi,
    Spanish,
    Turkish,
    Urdu,
    Vietnamese,
}

impl Language {
    pub const ALL: &'static [Language] = &[
        Language::English,
        Language::Arabic,
        Language::Bengali,
        Language::Chinese,
        Language::Dutch,
        Language::French,
        Language::German,
        Language::Hindi,
        Language::Italian,
        Language::Japanese,
        Language::Korean,
        Language::Polish,
        Language::Portuguese,
        Language::Punjabi,
        Language::Spanish,
        Language::Turkish,
        Language::Urdu,
        Language::Vietnamese,
    ];

    pub const DEFAULT: Language = Language::English;

    /// URL path prefix. English has none.
    pub fn url_prefix(self) -> &'static str {
        match self {
            Language::English => "",
            Language::Arabic => "ar",
            Language::Bengali => "bn",
            Language::Chinese => "zh-CN",
            Language::Dutch => "nl",
            Language::French => "fr",
            Language::German => "de",
            Language::Hindi => "hi",
            Language::Italian => "it",
            Language::Japanese => "ja",
            Language::Korean => "ko",
            Language::Polish => "pl",
            Language::Portuguese => "pt",
            Language::Punjabi => "pa",
            Language::Spanish => "es",
            Language::Turkish => "tr",
            Language::Urdu => "ur",
            Language::Vietnamese => "vi",
        }
    }

    /// BCP 47 tag for `<html lang>` and `hreflang`.
    pub fn hreflang(self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Arabic => "ar",
            Language::Bengali => "bn",
            Language::Chinese => "zh-CN",
            Language::Dutch => "nl",
            Language::French => "fr",
            Language::German => "de",
            Language::Hindi => "hi",
            Language::Italian => "it",
            Language::Japanese => "ja",
            Language::Korean => "ko",
            Language::Polish => "pl",
            Language::Portuguese => "pt",
            Language::Punjabi => "pa",
            Language::Spanish => "es",
            Language::Turkish => "tr",
            Language::Urdu => "ur",
            Language::Vietnamese => "vi",
        }
    }

    /// Name in the language's own script — for the language picker UI.
    pub fn native_name(self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Arabic => "العربية",
            Language::Bengali => "বাংলা",
            Language::Chinese => "中文",
            Language::Dutch => "Nederlands",
            Language::French => "Français",
            Language::German => "Deutsch",
            Language::Hindi => "हिन्दी",
            Language::Italian => "Italiano",
            Language::Japanese => "日本語",
            Language::Korean => "한국어",
            Language::Polish => "Polski",
            Language::Portuguese => "Português",
            Language::Punjabi => "ਪੰਜਾਬੀ",
            Language::Spanish => "Español",
            Language::Turkish => "Türkçe",
            Language::Urdu => "اردو",
            Language::Vietnamese => "Tiếng Việt",
        }
    }

    /// Text direction. RTL for Arabic and Urdu.
    pub fn dir(self) -> &'static str {
        match self {
            Language::Arabic | Language::Urdu => "rtl",
            _ => "ltr",
        }
    }

    /// Split a request path into (language, remaining-path). Unknown prefixes
    /// or no prefix → English plus the original path.
    pub fn from_path(path: &str) -> (Language, &str) {
        let trimmed = path.strip_prefix('/').unwrap_or(path);
        let (head, rest_with_slash) = match trimmed.find('/') {
            Some(i) => (&trimmed[..i], &path[i + 1..]),
            None => (trimmed, ""),
        };
        for lang in Self::ALL.iter().copied() {
            if !lang.url_prefix().is_empty() && lang.url_prefix() == head {
                return (lang, rest_with_slash);
            }
        }
        (Language::DEFAULT, path)
    }

    /// Build a translated URL from an English path.
    /// Example: `Language::Spanish.translated_path("/about")` → `/es/about`.
    pub fn translated_path(self, english_path: &str) -> String {
        let prefix = self.url_prefix();
        if prefix.is_empty() {
            english_path.to_string()
        } else {
            let trimmed = english_path.trim_start_matches('/');
            if trimmed.is_empty() {
                format!("/{}/", prefix)
            } else {
                format!("/{}/{}", prefix, trimmed)
            }
        }
    }
}

impl FromStr for Language {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let s = s.to_ascii_lowercase();
        for lang in Self::ALL {
            if lang.url_prefix().eq_ignore_ascii_case(&s)
                || lang.hreflang().eq_ignore_ascii_case(&s)
            {
                return Ok(*lang);
            }
        }
        // BCP 47 base-subtag fallback: "zh" → "zh-CN".
        for lang in Self::ALL {
            if let Some(base) = lang.hreflang().split('-').next() {
                if base.eq_ignore_ascii_case(&s) {
                    return Ok(*lang);
                }
            }
        }
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_english() {
        assert_eq!(Language::DEFAULT, Language::English);
        assert_eq!(Language::English.url_prefix(), "");
    }

    #[test]
    fn from_path_parses_prefix() {
        assert_eq!(Language::from_path("/es/about"), (Language::Spanish, "/about"));
        assert_eq!(Language::from_path("/zh-CN/"), (Language::Chinese, "/"));
        assert_eq!(Language::from_path("/about"), (Language::English, "/about"));
        assert_eq!(
            Language::from_path("/unknown/thing"),
            (Language::English, "/unknown/thing")
        );
    }

    #[test]
    fn translated_path_round_trips() {
        for lang in Language::ALL.iter().copied() {
            let path = lang.translated_path("/about");
            let (parsed, rest) = Language::from_path(&path);
            assert_eq!(parsed, lang);
            assert_eq!(rest, "/about");
        }
    }

    #[test]
    fn from_str_accepts_prefix_or_hreflang() {
        assert_eq!("es".parse::<Language>().unwrap(), Language::Spanish);
        assert_eq!("zh".parse::<Language>().unwrap(), Language::Chinese);
        assert_eq!("zh-CN".parse::<Language>().unwrap(), Language::Chinese);
        assert_eq!("ZH-cn".parse::<Language>().unwrap(), Language::Chinese);
        assert!("xx".parse::<Language>().is_err());
    }

    #[test]
    fn arabic_and_urdu_are_rtl() {
        assert_eq!(Language::Arabic.dir(), "rtl");
        assert_eq!(Language::Urdu.dir(), "rtl");
        assert_eq!(Language::English.dir(), "ltr");
    }
}
