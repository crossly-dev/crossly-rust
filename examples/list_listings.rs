//! Print every listing in your Crossly store, a page at a time, with retries, backoff and typed errors.
//!
//!     CROSSLY_TOKEN=crossly_pat_... cargo run --bin list_listings
use crossly::apis::listings_api;
use crossly::client::{self, paginate, CrosslyError, StreamExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = client::config(std::env::var("CROSSLY_TOKEN")?);

    let mut rows = paginate(100, |page, limit| {
        // page, limit, platform, status, sku, updated_since, kind, tag, still_live
        listings_api::list_listings(&config, Some(page), Some(limit), None, None, None, None, None, None, None)
    });
    while let Some(row) = rows.next().await {
        match row {
            Ok(item) => println!("{}  {:<10}  {}", item.id, item.status, item.title.as_deref().unwrap_or("")),
            Err(CrosslyError { code, correlation_id, .. }) if code == "unauthorized" => {
                eprintln!("CROSSLY_TOKEN was refused (correlationId {correlation_id:?})");
                std::process::exit(1);
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
