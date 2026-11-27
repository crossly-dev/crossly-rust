# Crossly Rust SDK

[![ci](https://github.com/crossly-dev/crossly-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/crossly-dev/crossly-rust/actions/workflows/ci.yml) [![release](https://github.com/crossly-dev/crossly-rust/actions/workflows/release.yml/badge.svg)](https://github.com/crossly-dev/crossly-rust/actions/workflows/release.yml) [![crates.io](https://img.shields.io/crates/v/crossly)](https://crates.io/crates/crossly) [![license](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

Official Rust clients for the [Crossly](https://crossly.net) API: list once and sell across every marketplace, read
orders, manage inventory, and receive signed webhooks. Generated from the public OpenAPI specs and released
automatically when they change, with signed build provenance.

## Install

| client | for | install |
|---|---|---|
| seller | your store: listings, crossposting, orders, analytics | `cargo add crossly` |
| buyer | shopping: cart, orders, wishlists, offers, cashback | `cargo add crossly_buyer` |

## Two clients, two credentials

| client | credential |
|---|---|
| seller | a Personal Access Token (`crossly_pat_…`, Settings → Developer) or a seller OAuth token |
| buyer | a buyer OAuth token (`crossly_oat_…`) with `buyer:*` scopes |

They are not interchangeable: a seller token is refused by the buyer API and the reverse, even for the same
Crossly account. A 401 from a call that looks right usually means the other client's token.

## First call

```rust
use crossly::apis::listings_api;
use crossly::client::CrosslyError;

// 30 s timeout, retries on 429/5xx with backoff (honouring Retry-After), an Idempotency-Key on
// every POST/PATCH, User-Agent crossly-rust/<version>.
let config = crossly::client::config(std::env::var("CROSSLY_TOKEN")?); // crossly_pat_…

// page, limit, platform, status, sku, updated_since, kind, tag, still_live
let page = listings_api::list_listings(&config, Some(1), Some(25), None, None, None, None, None, None, None)
    .await
    .map_err(CrosslyError::from)?;
for item in page.data {
    println!("{} {} {}", item.id, item.title.as_deref().unwrap_or(""), item.status);
}
```

List calls return `{ data, pagination }` and page with `page` and `limit`. Money fields named `…_cents` are
integer cents. A failed call returns `crossly::apis::Error`; convert it with `CrosslyError::from` (or `?` into a `CrosslyError`) for `status`, `code`, `message`, `details` and the `correlation_id` support can trace.

## Webhooks

Crossly signs every webhook with `Crossly-Signature: t=<unix>,v1=<hex HMAC-SHA256 of "t.body">`. The verifier
ships in the seller and buyer packages and checks the signature in constant time, rejects a timestamp more than
five minutes old, and returns the parsed event:

```rust
use crossly::webhooks_http::verify_event;

// body: the exact bytes received (axum `Bytes`, actix `web::Bytes`), before any JSON parsing
match verify_event(&body, signature_header, &std::env::var("CROSSLY_WEBHOOK_SECRET")?) {
    Ok(event) => println!("{} {}", event.event_type, event.id),
    Err(rejected) => return reply(rejected.status(), rejected.body()), // 400 {"error":{"code":…}}
}
```

Verify the raw bytes, not a re-serialised body. `webhooks/` holds the same verifier on its own, with the shared
test vectors every language's port is checked against on each build.

## Client helpers

Every generated function sends through `crossly::client::execute`, so the policy on
`Configuration::retry` applies to all of them: 429 is always retried, 5xx and connection errors are retried
for reads and for POST/PATCH carrying an `Idempotency-Key` (added automatically; the API replays the first
response to a repeated key for 24 hours), with full-jitter exponential backoff or exactly the `Retry-After`
the server sent (up to `max_retry_after`, 60 s, beyond which the response is returned).

```rust
use std::time::Duration;
use crossly::apis::listings_api;
use crossly::client::{paginate, ConfigBuilder, RetryPolicy, StreamExt};

let config = ConfigBuilder::new(token)
    .timeout(Duration::from_secs(10))
    .retry(RetryPolicy { max_retries: 5, ..RetryPolicy::default() })
    .build();

// Every row of a list endpoint, a page at a time, as a Stream:
let mut rows = paginate(100, |page, limit| {
    listings_api::list_listings(&config, Some(page), Some(limit), None, None, None, None, None, None, None)
});
while let Some(row) = rows.next().await {
    println!("{}", row?.id);
}
```

The buyer crate (`crossly_buyer`) has the same `client` and `webhooks_http` modules.

## Webhook extractors

Features `axum` (0.8) and `actix-web` (4) add extractors that read the raw body, verify the signature, and
reject an unverified delivery with a 400 `{"error":{"code":"<reason>"}}` before your handler runs:

```toml
crossly = { version = "0.2", features = ["axum"] }   # or "actix-web"
```

```rust
// axum: the secret lives in (or is FromRef of) the router state
use crossly::webhooks_http::{axum_extract::VerifiedWebhook, WebhookSecret};

async fn hook(VerifiedWebhook(event): VerifiedWebhook) -> axum::http::StatusCode {
    println!("{} {}", event.event_type, event.id);
    axum::http::StatusCode::NO_CONTENT
}
let app = axum::Router::new()
    .route("/webhooks/crossly", axum::routing::post(hook))
    .with_state(WebhookSecret(std::env::var("CROSSLY_WEBHOOK_SECRET")?));

// actix-web: App::new().app_data(web::Data::new(WebhookSecret(secret)))
use crossly::webhooks_http::actix_extract::VerifiedWebhook;
async fn hook(VerifiedWebhook(event): VerifiedWebhook) -> actix_web::HttpResponse {
    actix_web::HttpResponse::NoContent().finish()
}
```

[examples/](examples/) has a runnable lister and an axum receiver.

## Where things are

| client | API classes | models | API reference pages |
|---|---|---|---|
| seller | `seller/src/apis/` | `seller/src/models/<area>/` | `seller/docs/` |
| buyer | `buyer/src/apis/` | `buyer/src/models/<area>/` | `buyer/docs/` |

One class per API group (`OrdersApi`, `ListingsApi`, …), one model per type, the
models grouped one folder per product area (`vault`, `world`, `order`, …). A type
several endpoints share is ONE model, named after where it first appears
(`GetOrderResponse`, `ListOrdersItem`) or, for small shared shapes such as `{ ok }`,
after its fields (`OkResponse`). Each client README lists every method.

## Versions and changelog

Semantic versioning. While the version is 0.x, a release that removes an operation or a model bumps the minor
version; anything else bumps the patch. Every [CHANGELOG](CHANGELOG.md) entry lists the operations and models
that were added, changed or removed in the specs it was generated from.

## Verifying a release

Every release artifact is built on GitHub-hosted runners by [release.yml](.github/workflows/release.yml) and
carries a signed SLSA provenance attestation:

```sh
gh attestation verify <downloaded-file> --repo crossly-dev/crossly-rust
```

## Contributing

`seller/`, `buyer/` and `webhooks/` are generated, so a pull request that edits them would be overwritten by the
next release. Open an issue instead: the fix goes into the API spec or the generator, and every language picks it
up. [CONTRIBUTING.md](CONTRIBUTING.md) explains how to regenerate locally.

## More

- API reference and guides: https://crossly.net/docs
- Other languages: https://github.com/crossly-dev
- TypeScript (Node and browsers): [`@crossly/sdk`](https://github.com/crossly-dev/crossly-sdk); command line: [`@crossly/cli`](https://github.com/crossly-dev/crossly-cli); AI agents: [`@crossly/mcp`](https://github.com/crossly-dev/crossly-mcp)

## License

[MIT](LICENSE)
