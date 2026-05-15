// heartland.io — Cloudflare Worker (auto-language redirect)
//
// On every request, if the user's browser prefers a language we support
// AND the path they're requesting has a translated version, 302-redirect
// them to the /<lang>/<path> equivalent. Bookmark a cookie so subsequent
// visits don't re-trigger the redirect.
//
// Safeguards:
//   - Skip bots (Googlebot etc.) so search engines crawl English at English URLs.
//   - Skip paths already prefixed with a language.
//   - Skip paths that don't have translations yet (allowlist).
//   - Cookie-based opt-in: once redirected, the cookie remembers their choice.
//     A user can clear cookies or set `lang_preference=en` (e.g. via a UI we
//     might add later) to escape.
//
// LANGS must stay in sync with `Language::ALL` in src/i18n.rs.

const LANGS = new Set([
  'ar', 'bn', 'de', 'es', 'fr', 'hi', 'it', 'ja', 'ko',
  'nl', 'pa', 'pl', 'pt', 'tr', 'ur', 'vi', 'zh-CN',
]);

// Bot detection. Conservative — better to occasionally fail to redirect a
// legitimate user (they can still navigate manually) than to redirect a
// crawler and pollute Google's index with wrong-language signals.
const BOT_REGEX =
  /\b(googlebot|bingbot|baiduspider|yandex|duckduck|slurp|facebookexternalhit|twitterbot|linkedinbot|whatsapp|telegrambot|discordbot|applebot|petalbot|semrushbot|ahrefsbot|mj12bot|dotbot|crawler|spider)\b/i;

/// Paths that have translated versions on Amplify. Only requests matching
/// these get redirected; other paths stay English. As we translate more of
/// the site (page components, etc.) we expand this list.
function isTranslatablePath(pathname) {
  return pathname === '/'
    ? false   // home page isn't translated yet; redirecting / → /es/ would 404
    : pathname.startsWith('/sustainability-news/');
}

/// Parse the Accept-Language header, return the best-matching code we
/// support, or null. Honors `;q=` quality weights and falls back from
/// `es-MX` → `es`; also maps `zh` / `zh-TW` / `zh-HK` to `zh-CN` since
/// that's the only Chinese variant we ship.
function pickLang(acceptLanguage) {
  if (!acceptLanguage) return null;
  const prefs = acceptLanguage
    .split(',')
    .map((seg) => {
      const [tag, ...params] = seg.trim().split(';');
      const qParam = params.find((p) => p.trim().startsWith('q='));
      const q = qParam ? parseFloat(qParam.trim().slice(2)) : 1.0;
      return { tag: tag.trim(), q: isNaN(q) ? 1.0 : q };
    })
    .sort((a, b) => b.q - a.q);

  for (const { tag } of prefs) {
    if (LANGS.has(tag)) return tag;
    const base = tag.split('-')[0];
    if (LANGS.has(base)) return base;
    if (base === 'zh' && LANGS.has('zh-CN')) return 'zh-CN';
  }
  return null;
}

function getCookie(request, name) {
  const header = request.headers.get('cookie') || '';
  const m = header.match(new RegExp('(?:^|; )' + name + '=([^;]*)'));
  return m ? decodeURIComponent(m[1]) : null;
}

function buildRedirect(targetUrl, setLangCookie) {
  const headers = new Headers({ Location: targetUrl });
  if (setLangCookie) {
    // 180 days. SameSite=Lax so the cookie survives cross-site link clicks
    // (e.g. arriving via a Google search result) but not third-party iframes.
    headers.set(
      'Set-Cookie',
      `lang_preference=${setLangCookie}; Path=/; Max-Age=15552000; SameSite=Lax; Secure`,
    );
  }
  return new Response(null, { status: 302, headers });
}

export default {
  async fetch(request) {
    const url = new URL(request.url);

    // 1. Already on a language path? Let it through unchanged.
    if (/^\/[a-z]{2}(-[A-Z]{2})?\//.test(url.pathname)) {
      return fetch(request);
    }

    // 2. Bot? Never redirect.
    const ua = request.headers.get('user-agent') || '';
    if (BOT_REGEX.test(ua)) {
      return fetch(request);
    }

    // 3. Sticky cookie wins. Once a user has been auto-redirected (or
    //    explicitly opted in to a language), respect it on every visit
    //    until they clear cookies or override.
    const cookieLang = getCookie(request, 'lang_preference');
    if (cookieLang === 'en') {
      return fetch(request);
    }
    if (cookieLang && LANGS.has(cookieLang) && isTranslatablePath(url.pathname)) {
      return buildRedirect(`/${cookieLang}${url.pathname}${url.search}`, null);
    }

    // 4. First-visit auto-detect from Accept-Language.
    if (!isTranslatablePath(url.pathname)) {
      return fetch(request);
    }
    const detected = pickLang(request.headers.get('accept-language'));
    if (!detected || detected === 'en') {
      return fetch(request);
    }

    return buildRedirect(
      `/${detected}${url.pathname}${url.search}`,
      detected, // set the cookie so subsequent visits skip the detection
    );
  },
};
