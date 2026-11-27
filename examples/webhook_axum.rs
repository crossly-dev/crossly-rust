//! A Crossly webhook receiver on axum. The extractor verifies the signature over the exact body bytes and
//! answers an unverified delivery with a 400 before `hook` runs.
//!
//!     CROSSLY_WEBHOOK_SECRET=... cargo run --bin webhook_axum
//!     # then point a webhook endpoint (Settings → Webhooks) at http://<host>:8080/webhooks/crossly
use axum::{http::StatusCode, routing::post, Router};
use crossly::webhooks_http::{axum_extract::VerifiedWebhook, WebhookSecret};

async fn hook(VerifiedWebhook(event): VerifiedWebhook) -> StatusCode {
    println!("{} {}: {}", event.id, event.event_type, event.data);
    // Answer fast; Crossly retries anything that is not a 2xx.
    StatusCode::NO_CONTENT
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = WebhookSecret(std::env::var("CROSSLY_WEBHOOK_SECRET")?);
    let app = Router::new().route("/webhooks/crossly", post(hook)).with_state(secret);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
