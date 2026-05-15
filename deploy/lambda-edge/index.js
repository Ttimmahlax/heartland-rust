// heartland-gtranslate-router (Lambda@Edge, us-east-1, Node.js 22.x)
//
// Attached to CloudFront as an Origin Request handler.
// For /<lang>/<path>, rewrites the request origin to the GTranslate
// translation network. Everything else passes through to the default
// origin (Amplify).
//
// Deploy: copy this code into the Lambda console, publish a new version,
// then attach the versioned ARN to the CloudFront distribution's default
// behavior under "Function associations → Origin request".
//
// LANGS must stay in sync with `Language::ALL` in src/i18n.rs.

'use strict';

const crypto = require('crypto');

const LANGS = new Set([
  'ar', 'bn', 'de', 'es', 'fr', 'hi', 'it', 'ja', 'ko',
  'nl', 'pa', 'pl', 'pt', 'tr', 'ur', 'vi', 'zh-CN',
]);

// GTranslate translation-server pool. Matches the vendor gtranslate.php
// pool so cache behavior is consistent with the prior WP-plugin setup.
const GT_SERVERS = [
  'van', 'kars', 'sis', 'dvin', 'ani', 'evn',
  'vagh', 'step', 'sis', 'tigr', 'ani', 'van',
];

function gtServerFor(hostname) {
  const stripped = hostname.replace(/^www\./, '');
  const md5 = crypto.createHash('md5').update(stripped).digest('hex');
  const idx = parseInt(md5.slice(0, 5), 16) % GT_SERVERS.length;
  return `${GT_SERVERS[idx]}.tdn.gtranslate.net`;
}

exports.handler = async (event) => {
  const request = event.Records[0].cf.request;
  const host =
    (request.headers.host && request.headers.host[0] && request.headers.host[0].value) ||
    'heartland.io';

  // Match /<lang>/<rest>. Supports zh-CN (two-letter base + optional region).
  const m = request.uri.match(/^\/([a-z]{2}(?:-[A-Z]{2})?)(\/.*)?$/);
  if (!m) return request;

  const lang = m[1];
  if (!LANGS.has(lang)) return request;

  const gtServer = gtServerFor(host);

  request.origin = {
    custom: {
      domainName: gtServer,
      port: 443,
      protocol: 'https',
      path: '',
      sslProtocols: ['TLSv1.2'],
      readTimeout: 30,
      keepaliveTimeout: 5,
      customHeaders: {},
    },
  };
  request.headers.host = [{ key: 'host', value: gtServer }];

  // Keep the language prefix in the URI — GTranslate's network keys
  // translation by the original request path.
  return request;
};
