//! Verify a Crossly webhook.
//!
//! ```text
//! Crossly-Signature: t=<unix seconds>,v1=<hex HMAC-SHA256>
//! ```
//!
//! signed over `format!("{t}.{raw_body}")` with the endpoint's signing secret.
//!
//! Three ways to get this wrong, all silent:
//!
//! 1. Verifying a re-serialised body. `serde_json` round-trips reorder keys and
//!    reformat numbers, so genuine payloads fail and the usual fix is to stop
//!    verifying. In axum take `body: Bytes`; in actix, `web::Bytes`.
//! 2. Comparing with `==`. Slice equality short-circuits on the first differing
//!    byte. The comparison here accumulates over the whole input instead.
//! 3. Ignoring the timestamp. Without it a captured request replays forever.
//!    The timestamp is INSIDE the signed message, so it cannot be edited.
//!
//! Depends on `hmac`, `sha2` and `serde_json` — all already in the generated
//! client's tree.

use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

pub const DEFAULT_TOLERANCE_SECONDS: i64 = 300;

/// Why a webhook did not verify.
///
/// A caller should distinguish these — a forged request and a clock problem
/// want different responses and different alerts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationError {
    MalformedHeader(String),
    BadSignature(String),
    TimestampOutOfTolerance(String),
    MissingSecret,
}

impl VerificationError {
    /// Stable machine-readable code, matching the other language ports.
    pub fn reason(&self) -> &'static str {
        match self {
            Self::MalformedHeader(_) => "malformed_header",
            Self::BadSignature(_) => "bad_signature",
            Self::TimestampOutOfTolerance(_) => "timestamp_out_of_tolerance",
            Self::MissingSecret => "missing_secret",
        }
    }
}

impl std::fmt::Display for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MalformedHeader(m)
            | Self::BadSignature(m)
            | Self::TimestampOutOfTolerance(m) => write!(f, "{m}"),
            Self::MissingSecret => write!(f, "A webhook signing secret is required."),
        }
    }
}

impl std::error::Error for VerificationError {}

/// Options. `Default` gives the 300s tolerance and the system clock.
#[derive(Debug, Clone, Copy)]
pub struct VerifyOptions {
    pub tolerance_seconds: i64,
    /// Override the clock, for tests.
    pub now: Option<i64>,
}

impl Default for VerifyOptions {
    fn default() -> Self {
        Self { tolerance_seconds: DEFAULT_TOLERANCE_SECONDS, now: None }
    }
}

/// Pull `t` and `v1` out of the header.
///
/// Field-wise rather than one regex, so a future `v2=` alongside `v1=` does not
/// break existing verifiers — the entire reason the scheme is versioned.
fn parse_signature_header(header: &str) -> Option<(i64, &str)> {
    let mut t: Option<i64> = None;
    let mut v1: Option<&str> = None;

    for part in header.split(',') {
        let (key, value) = part.split_once('=')?;
        match key.trim() {
            "t" => t = Some(value.trim().parse().ok()?),
            "v1" => v1 = Some(value.trim()),
            _ => {}
        }
    }

    match (t, v1) {
        (Some(t), Some(v1)) if !v1.is_empty() => Some((t, v1)),
        _ => None,
    }
}

/// Constant-time compare over the whole input.
///
/// `==` on slices short-circuits, and the timing difference is enough to forge
/// a signature given enough attempts. Unequal lengths still walk the longer
/// input rather than returning early.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    let len = a.len().max(b.len());
    let mut diff = (a.len() ^ b.len()) as u8;
    for i in 0..len {
        diff |= a.get(i).copied().unwrap_or(0) ^ b.get(i).copied().unwrap_or(0);
    }
    diff == 0
}

fn hmac_hex(secret: &str, message: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC accepts a key of any length");
    mac.update(message.as_bytes());
    mac.finalize().into_bytes().iter().map(|b| format!("{b:02x}")).collect()
}

/// Verify a webhook and return the raw body.
///
/// Returns the body rather than a parsed struct so this has no opinion about
/// your event types — deserialise it yourself once it is trusted.
///
/// Returns a `Result` rather than a bool so `#[must_use]` on `Result` makes an
/// ignored verification a compiler warning rather than a silent acceptance.
pub fn verify<'a>(
    raw_body: &'a [u8],
    signature_header: Option<&str>,
    secret: &str,
    options: VerifyOptions,
) -> Result<&'a str, VerificationError> {
    if secret.is_empty() {
        return Err(VerificationError::MissingSecret);
    }

    let header = signature_header.filter(|h| !h.is_empty()).ok_or_else(|| {
        VerificationError::MalformedHeader("No Crossly-Signature header on the request.".into())
    })?;

    let (timestamp, provided) = parse_signature_header(header).ok_or_else(|| {
        let preview: String = header.chars().take(60).collect();
        VerificationError::MalformedHeader(format!(
            r#"Could not parse Crossly-Signature: expected "t=<unix>,v1=<hex>", got "{preview}"."#
        ))
    })?;

    let body = std::str::from_utf8(raw_body).map_err(|_| {
        VerificationError::BadSignature("Body is not valid UTF-8.".into())
    })?;

    let expected = hmac_hex(secret, &format!("{timestamp}.{body}"));

    if !constant_time_eq(expected.as_bytes(), provided.as_bytes()) {
        return Err(VerificationError::BadSignature(
            "Signature did not match. If genuine payloads are failing, you are almost certainly \
             verifying a re-serialised body — pass the bytes you read off the wire."
                .into(),
        ));
    }

    // Freshness AFTER the signature, so an attacker learns nothing about
    // timestamps without already holding a valid signature.
    let current = options.now.unwrap_or_else(|| {
        SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
    });
    let drift = (current - timestamp).abs();
    if drift > options.tolerance_seconds {
        return Err(VerificationError::TimestampOutOfTolerance(format!(
            "Timestamp is {drift}s away from now (tolerance {}s). This is a replay guard — \
             if it fires on live traffic, check your server clock.",
            options.tolerance_seconds
        )));
    }

    Ok(body)
}
