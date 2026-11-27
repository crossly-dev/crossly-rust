#!/usr/bin/env node
/**
 * Finish one generated Rust crate after assemble.mjs (which vendored src/client.rs, src/webhooks.rs and
 * src/webhooks_http.rs into it):
 *
 *   node codegen/integrations/extras/rust/finish.mjs <seller|buyer>      (run by codegen/ci/check-rust.sh)
 *
 *   1. Cargo.toml: crates.io metadata the 5.4.0 template omits (description and license are required to publish),
 *      the extras' dependencies, and the optional `axum` / `actix-web` features. The generated [dependencies]
 *      block is kept verbatim between markers.
 *   2. src/lib.rs declares the vendored modules.
 *   3. configuration.rs gains `retry: client::RetryPolicy`; the default User-Agent becomes crossly-rust/<version>.
 *   4. Every generated call sends through `client::execute` (retries, Retry-After, Idempotency-Key) instead of
 *      `client.execute`: one line per function, counted, so a template change fails loudly instead of silently
 *      dropping the retry layer.
 *   5. src/paged.rs: `impl client::Paged` for every `{ data: Vec<T>, pagination }` response model.
 *
 * Idempotent: ci.yml runs it again over the committed tree and must find nothing to change.
 */
import { copyFileSync, existsSync, readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { basename, dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const dir = resolve(process.argv[2] ?? '');
const api = basename(dir);
const die = (m) => {
  console.error(`finish.mjs (${api}): ${m}`);
  process.exit(1);
};
if (!['seller', 'buyer'].includes(api) || !existsSync(join(dir, 'Cargo.toml'))) die(`${dir} is not a generated crate`);
// The vendor list again (assemble.mjs copies it after generation): ci.yml checks the committed tree against the
// CURRENT hand-written files, and a tree generated before they existed has none.
const CODEGEN = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..', '..');
for (const v of JSON.parse(readFileSync(join(CODEGEN, 'repo.json'), 'utf8')).vendor ?? []) {
  if (v.to.startsWith(`${api}/`)) copyFileSync(join(CODEGEN, 'integrations', v.from), join(dir, v.to.slice(api.length + 1)));
}
for (const f of ['client.rs', 'webhooks.rs', 'webhooks_http.rs']) {
  if (!existsSync(join(dir, 'src', f))) die(`src/${f} was not vendored (codegen/repo.json vendor list)`);
}
const put = (rel, text) => {
  const f = join(dir, rel);
  if (!existsSync(f) || readFileSync(f, 'utf8') !== text) writeFileSync(f, text);
};

// 1. Cargo.toml
const cargo = readFileSync(join(dir, 'Cargo.toml'), 'utf8');
const field = (k) => cargo.match(new RegExp(`^${k}\\s*=\\s*"([^"]*)"`, 'm'))?.[1] ?? die(`no ${k} in Cargo.toml`);
const name = field('name');
const version = field('version');
const BEGIN = '# --- generated dependencies (openapi-generator) ---';
const END = '# --- end generated dependencies ---';
let generatedDeps;
if (cargo.includes(BEGIN)) {
  generatedDeps = cargo.slice(cargo.indexOf(BEGIN) + BEGIN.length, cargo.indexOf(END)).trim();
} else {
  const start = cargo.indexOf('[dependencies]');
  if (start === -1) die('no [dependencies] in Cargo.toml');
  const devAt = cargo.indexOf('[dev-dependencies]', start);
  generatedDeps = cargo.slice(start, devAt === -1 ? undefined : devAt).trim();
}
const what =
  api === 'seller'
    ? 'your store: listings, crossposting, orders, inventory, analytics'
    : 'shopping: cart, orders, wishlists, offers, cashback';
put(
  'Cargo.toml',
  `[package]
name = "${name}"
version = "${version}"
authors = ["Crossly"]
edition = "2018"
rust-version = "1.75"
description = "Official Crossly ${api} API client (${what}), with webhook verification."
license = "MIT"
repository = "https://github.com/crossly-dev/crossly-rust"
homepage = "https://crossly.net/docs"
documentation = "https://docs.rs/${name}"
readme = "README.md"
keywords = ["crossly", "reselling", "ecommerce", "marketplace", "api"]
categories = ["api-bindings", "web-programming::http-client"]
exclude = ["docs/", ".openapi-generator/", ".openapi-generator-ignore", ".travis.yml", "git_push.sh", "target/"]

[package.metadata.docs.rs]
all-features = true

${BEGIN}
${generatedDeps}
${END}

[dependencies.hmac]
version = "0.12"

[dependencies.sha2]
version = "0.10"

[dependencies.tokio]
version = "1"
features = ["time"]

[dependencies.futures-util]
version = "0.3"
default-features = false
features = ["alloc"]

[dependencies.axum]
version = "0.8"
optional = true
default-features = false

[dependencies.actix-web]
version = "4"
optional = true
default-features = false

[features]
# Webhook extractors: crossly::webhooks_http::axum_extract / actix_extract.
axum = ["dep:axum"]
actix-web = ["dep:actix-web"]

[dev-dependencies.tokio]
version = "1"
features = ["macros", "rt-multi-thread"]
`,
);

// 2. src/lib.rs
const MODS = ['pub mod client;', 'pub mod webhooks;', 'pub mod webhooks_http;', 'mod paged;'];
let lib = readFileSync(join(dir, 'src', 'lib.rs'), 'utf8');
for (const m of MODS) if (!lib.includes(m)) lib = `${lib.trimEnd()}\n${m}\n`;
put('src/lib.rs', lib);

// 3. configuration.rs
const cfgRel = 'src/apis/configuration.rs';
let cfg = readFileSync(join(dir, cfgRel), 'utf8');
if (!cfg.includes('pub retry: crate::client::RetryPolicy')) {
  const before = cfg;
  cfg = cfg
    .replace(/(\n(\s*)pub api_key: Option<ApiKey>,\n)/, '$1$2/// Retries, backoff and idempotency keys for every call (crate::client).\n$2pub retry: crate::client::RetryPolicy,\n')
    .replace(/(\n(\s*)api_key: None,\n)/, '$1$2retry: crate::client::RetryPolicy::default(),\n');
  if ((cfg.match(/RetryPolicy/g) ?? []).length !== 2 || cfg === before) die(`could not add the retry field to ${cfgRel}`);
}
cfg = cfg.replace(/user_agent: Some\("OpenAPI-Generator\/[^"]*"\.to_owned\(\)\)/, 'user_agent: Some(crate::client::user_agent())');
if (!cfg.includes('Some(crate::client::user_agent())')) die(`default user_agent not found in ${cfgRel}`);
put(cfgRel, cfg);

// 4. every call through client::execute
const SEND = 'local_var_client.execute(local_var_req).await?';
const OURS = 'crate::client::execute(local_var_configuration, local_var_req).await?';
let sites = 0;
for (const f of readdirSync(join(dir, 'src', 'apis')).filter((n) => n.endsWith('_api.rs'))) {
  const p = join(dir, 'src', 'apis', f);
  const t = readFileSync(p, 'utf8');
  sites += t.split(OURS).length - 1 + t.split(SEND).length - 1;
  if (t.includes(SEND)) writeFileSync(p, t.replaceAll(SEND, OURS));
  if (/\.execute\(local_var_req\)/.test(readFileSync(p, 'utf8').replaceAll(OURS, ''))) die(`${f}: a send that bypasses client::execute`);
}
if (sites === 0) die('no generated call sites found (template changed?)');

const walk = (d) =>
  readdirSync(d).flatMap((n) => {
    const p = join(d, n);
    return statSync(p).isDirectory() ? walk(p) : p.endsWith('.rs') ? [p] : [];
  });

// 4b. A parameter declared `anyOf: [string, array]` (listListings' `tag`) is typed AnyOfstringarray, which the
// 5.4.0 generator never declares. Alias each missing one in models/mod.rs: String where a string is allowed (the
// API takes a comma-separated list), serde_json::Value otherwise. A no-op once the generator handles it.
{
  const used = new Set();
  const declared = new Set();
  for (const f of walk(join(dir, 'src'))) {
    const t = readFileSync(f, 'utf8');
    for (const m of t.matchAll(/\b((?:AnyOf|OneOf)[A-Za-z0-9]+)\b/g)) used.add(m[1]);
    for (const m of t.matchAll(/pub (?:struct|enum|type) ((?:AnyOf|OneOf)[A-Za-z0-9]+)\b/g)) declared.add(m[1]);
  }
  const missing = [...used].filter((n) => !declared.has(n)).sort();
  if (missing.length) {
    const modRs = join(dir, 'src', 'models', 'mod.rs');
    const aliases = missing
      .map((n) =>
        /string/.test(n)
          ? `/// A string; where the API accepts a list, pass it comma-separated.\npub type ${n} = String;\n`
          : `/// Any JSON value.\npub type ${n} = serde_json::Value;\n`,
      )
      .join('');
    writeFileSync(modRs, `${readFileSync(modRs, 'utf8').trimEnd()}\n\n// Declared by finish.mjs: types the generator references but never writes.\n${aliases}`);
    console.log(`declared ${missing.length} undeclared anyOf/oneOf type(s): ${missing.join(', ')}`);
  }
}

// 5. src/paged.rs
const impls = [];
for (const f of walk(join(dir, 'src', 'models')).sort()) {
  const t = readFileSync(f, 'utf8');
  for (const m of t.matchAll(/pub struct (\w+) \{([\s\S]*?)\n\}/g)) {
    const data = m[2].match(/\n\s*pub data: Vec<([^\n]+)>,\n/);
    if (!data || !/\n\s*pub pagination: Option<Box<crate::models::Pagination>>,\n/.test(m[2])) continue;
    impls.push(
      `impl Paged for crate::models::${m[1]} {\n    type Item = ${data[1]};\n` +
        `    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {\n` +
        `        (self.data, self.pagination.map(|p| *p))\n    }\n}\n`,
    );
  }
}
if (impls.length === 0) die('no paged response models found');
put(
  'src/paged.rs',
  `// Written by codegen/integrations/extras/rust/finish.mjs: every { data, pagination } response model.\n` +
    `use crate::client::Paged;\n\n${impls.join('\n')}`,
);
console.log(`finished ${api} (${name} ${version}): ${sites} calls through client::execute, ${impls.length} paged models`);
