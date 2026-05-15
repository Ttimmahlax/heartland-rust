  // heartland.io — Cloudflare Worker (language router)
  //
  // Routes /<lang>/<path> requests through GTranslate's translation network.
  // Everything else passes through to the origin (Amplify Hosting, reached via
  // the proxied CNAME record for heartland.io).
  //
  // LANGS must stay in sync with `Language::ALL` in src/i18n.rs.
  //
  // Deploy:
  //   1. Cloudflare dashboard → Workers & Pages → Create Worker
  //   2. Paste this file into the editor
  //   3. Save & Deploy
  //   4. Triggers → add custom domain or route: `heartland.io/*` and `www.heartland.io/*`

  const LANGS = new Set([
    'ar', 'bn', 'de', 'es', 'fr', 'hi', 'it', 'ja', 'ko',
    'nl', 'pa', 'pl', 'pt', 'tr', 'ur', 'vi', 'zh-CN',
  ]);

  // GTranslate translation-server pool. md5("heartland.io") → idx 10 of 12 → "ani".
  // Pinned here because Workers' crypto.subtle doesn't implement MD5.
  // If the production hostname changes, recompute (see deploy/lambda-edge/index.mjs
  // gtServerFor()) and update this constant.
  const GT_SERVER = 'ani.tdn.gtranslate.net';

  export default {
    async fetch(request) {
      const url = new URL(request.url);

      // Match /<lang>/<rest>. Supports zh-CN (two-letter base + optional region).
      const m = url.pathname.match(/^\/([a-z]{2}(?:-[A-Z]{2})?)(\/.*)?$/);
      if (!m || !LANGS.has(m[1])) {
        // English or unknown-prefix path — passthrough to the Amplify origin.
        // Cloudflare's subrequests skip the Worker route, so this goes
        // directly to the CNAME target (Amplify Hosting) without looping.
        return fetch(request);
      }

      // Translated path — proxy to GTranslate's network.
      //
      // Three things to know about this call (matches GTranslate's PHP addon):
      //
      // 1. We use HTTP (not HTTPS) because *.tdn.gtranslate.net presents a
      //    self-signed cert. Cloudflare Workers' fetch doesn't support custom
      //    CA bundles, so HTTPS would 526. The CF→GT hop is unencrypted, but
      //    it's all public marketing content. User-facing TLS is unaffected.
      //
      // 2. We STRIP the language prefix from the URL. GTranslate's TDN servers
      //    don't expect the lang in the path.
      //
      // 3. We set Host to `<lang>.heartland.io`. The PHP addon does this
      //    exact thing (gtranslate.php line 71: `$host = $glang . '.' . ...`).
      //    GTranslate identifies BOTH the source site AND the target language
      //    from this single Host header.
      const lang = m[1];
      const rest = m[2] || '/';
      const gtUrl = `http://${GT_SERVER}${rest}${url.search}`;

      // Build the upstream headers. Mirror what the PHP addon does:
      //   - Strip CF-* headers (the addon explicitly removes these so they
      //     don't confuse GT's request fingerprinting).
      //   - Set Host to `<lang>.heartland.io`.
      //   - Add `X-GT-Viewer-IP` with the real client IP — GT uses this to
      //     distinguish authenticated origin requests from anonymous demos.
      //     Without it, GT returns the source content un-translated and
      //     stamped with `X-Robots-Tag: noindex,nofollow,noarchive,nosnippet`.
      const headers = new Headers();
      for (const [k, v] of request.headers) {
        if (k.toLowerCase().startsWith('cf-')) continue;
        headers.set(k, v);
      }
      headers.set('host', `${lang}.heartland.io`);
      headers.set('accept-encoding', 'gzip');

      const viewerIp =
        request.headers.get('cf-connecting-ip') ||
        request.headers.get('x-real-ip') ||
        (request.headers.get('x-forwarded-for') || '').split(',')[0].trim() ||
        '0.0.0.0';
      headers.set('x-gt-viewer-ip', viewerIp);

      const xff = request.headers.get('x-forwarded-for');
      if (xff) headers.set('x-gt-forwarded-for', xff);

      return fetch(gtUrl, {
        method: request.method,
        headers,
        body: ['GET', 'HEAD'].includes(request.method) ? undefined : request.body,
        redirect: 'manual',
      });
    },
  };
