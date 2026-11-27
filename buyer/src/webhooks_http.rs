//! Webhook receiving for web frameworks, over [`crate::webhooks::verify`].
//!
//! - always: [`verify_event`] (bytes + header + secret → [`WebhookEvent`]);
//! - feature `axum`: [`axum_extract::VerifiedWebhook`], an extractor;
//! - feature `actix-web`: [`actix_extract::VerifiedWebhook`], an extractor.
//!
//! Both extractors read the exact body bytes, verify `Crossly-Signature`, and reject an unverified
//! delivery with a 400 `{"error":{"code":"<reason>"}}` before the handler runs.
//!
//! Hand-written, vendored by codegen (edit it in the Crossly monorepo: tools/sdk-repo/extras/rust/).

use crate::webhooks::{verify, VerificationError, VerifyOptions};

/// The header Crossly signs deliveries with.
pub const SIGNATURE_HEADER: &str = "crossly-signature";

/// The endpoint's signing secret (Settings → Webhooks), for the extractors to find in app state.
#[derive(Clone)]
pub struct WebhookSecret(pub String);

impl std::fmt::Debug for WebhookSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("WebhookSecret(…)")
    }
}

/// The envelope Crossly sends. `data` stays JSON: deserialise it into the event's own type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub created: String,
    #[serde(default)]
    pub data: serde_json::Value,
}

/// Why a delivery was refused.
#[derive(Debug)]
pub enum WebhookRejection {
    Verification(VerificationError),
    /// Verified, but the body is not a webhook envelope.
    Body(String),
}

impl WebhookRejection {
    /// The stable machine code, matching the other language ports.
    pub fn code(&self) -> &'static str {
        match self {
            Self::Verification(e) => e.reason(),
            Self::Body(_) => "malformed_body",
        }
    }
    /// 500 for a missing secret (a server misconfiguration), 400 otherwise.
    pub fn status(&self) -> u16 {
        match self {
            Self::Verification(VerificationError::MissingSecret) => 500,
            _ => 400,
        }
    }
    /// `{"error":{"code":"…","message":"…"}}`
    pub fn body(&self) -> String {
        serde_json::json!({ "error": { "code": self.code(), "message": self.to_string() } }).to_string()
    }
}

impl std::fmt::Display for WebhookRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Verification(e) => write!(f, "{e}"),
            Self::Body(m) => write!(f, "{m}"),
        }
    }
}

impl std::error::Error for WebhookRejection {}

/// Verify a delivery over its exact body bytes and parse the envelope.
pub fn verify_event(body: &[u8], signature: Option<&str>, secret: &str) -> Result<WebhookEvent, WebhookRejection> {
    let raw = verify(body, signature, secret, VerifyOptions::default()).map_err(WebhookRejection::Verification)?;
    serde_json::from_str(raw).map_err(|e| WebhookRejection::Body(format!("verified, but not a webhook envelope: {e}")))
}

/// axum (0.8) extractor. Put a [`WebhookSecret`] in the router state (or make it `FromRef` your state):
///
/// ```ignore
/// use crossly::webhooks_http::{axum_extract::VerifiedWebhook, WebhookSecret};
/// async fn hook(VerifiedWebhook(event): VerifiedWebhook) -> axum::http::StatusCode {
///     println!("{} {}", event.event_type, event.id);
///     axum::http::StatusCode::NO_CONTENT
/// }
/// let app = axum::Router::new()
///     .route("/webhooks/crossly", axum::routing::post(hook))
///     .with_state(WebhookSecret(std::env::var("CROSSLY_WEBHOOK_SECRET")?));
/// ```
#[cfg(feature = "axum")]
pub mod axum_extract {
    use super::{verify_event, WebhookEvent, WebhookRejection, WebhookSecret, SIGNATURE_HEADER};
    use ::axum::body::Bytes;
    use ::axum::extract::{FromRef, FromRequest, Request};
    use ::axum::http::{header, StatusCode};
    use ::axum::response::{IntoResponse, Response};

    /// A verified delivery. The extractor consumes the body, so it must be the handler's last argument.
    #[derive(Debug, Clone)]
    pub struct VerifiedWebhook(pub WebhookEvent);

    impl IntoResponse for WebhookRejection {
        fn into_response(self) -> Response {
            let status = StatusCode::from_u16(self.status()).unwrap_or(StatusCode::BAD_REQUEST);
            (status, [(header::CONTENT_TYPE, "application/json")], self.body()).into_response()
        }
    }

    impl<S> FromRequest<S> for VerifiedWebhook
    where
        S: Send + Sync,
        WebhookSecret: FromRef<S>,
    {
        type Rejection = Response;

        async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
            let secret = WebhookSecret::from_ref(state);
            let signature = req.headers().get(SIGNATURE_HEADER).and_then(|v| v.to_str().ok()).map(str::to_owned);
            let body = Bytes::from_request(req, state).await.map_err(IntoResponse::into_response)?;
            verify_event(&body, signature.as_deref(), &secret.0)
                .map(VerifiedWebhook)
                .map_err(IntoResponse::into_response)
        }
    }
}

/// actix-web (4) extractor. Register the secret with `App::new().app_data(web::Data::new(WebhookSecret(…)))`:
///
/// ```ignore
/// use crossly::webhooks_http::{actix_extract::VerifiedWebhook, WebhookSecret};
/// async fn hook(VerifiedWebhook(event): VerifiedWebhook) -> actix_web::HttpResponse {
///     println!("{} {}", event.event_type, event.id);
///     actix_web::HttpResponse::NoContent().finish()
/// }
/// ```
#[cfg(feature = "actix-web")]
pub mod actix_extract {
    use super::{verify_event, WebhookEvent, WebhookRejection, WebhookSecret, SIGNATURE_HEADER};
    use ::actix_web::dev::Payload;
    use ::actix_web::http::StatusCode;
    use ::actix_web::{web, FromRequest, HttpRequest, HttpResponse, ResponseError};
    use futures_util::future::LocalBoxFuture;

    /// A verified delivery.
    #[derive(Debug, Clone)]
    pub struct VerifiedWebhook(pub WebhookEvent);

    impl ResponseError for WebhookRejection {
        fn status_code(&self) -> StatusCode {
            StatusCode::from_u16(self.status()).unwrap_or(StatusCode::BAD_REQUEST)
        }
        fn error_response(&self) -> HttpResponse {
            HttpResponse::build(self.status_code()).content_type("application/json").body(self.body())
        }
    }

    impl FromRequest for VerifiedWebhook {
        type Error = ::actix_web::Error;
        type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

        fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
            let req = req.clone();
            let body = web::Bytes::from_request(&req, payload);
            Box::pin(async move {
                let body = body.await?;
                let secret = req
                    .app_data::<web::Data<WebhookSecret>>()
                    .map(|s| s.0.clone())
                    .or_else(|| req.app_data::<WebhookSecret>().map(|s| s.0.clone()))
                    .unwrap_or_default();
                let signature = req.headers().get(SIGNATURE_HEADER).and_then(|v| v.to_str().ok());
                Ok(VerifiedWebhook(verify_event(&body, signature, &secret)?))
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hmac::{Hmac, Mac};

    fn sign(secret: &str, body: &str, t: i64) -> String {
        let mut mac = Hmac::<sha2::Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(format!("{t}.{body}").as_bytes());
        let hex: String = mac.finalize().into_bytes().iter().map(|b| format!("{b:02x}")).collect();
        format!("t={t},v1={hex}")
    }

    #[test]
    fn verifies_and_parses() {
        let body = r#"{"id":"evt_1","type":"listing.sold","created":"2026-01-01T00:00:00Z","data":{"x":1}}"#;
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
        let ev = verify_event(body.as_bytes(), Some(&sign("s", body, now)), "s").unwrap();
        assert_eq!(ev.event_type, "listing.sold");
        let bad = verify_event(format!("{body} ").as_bytes(), Some(&sign("s", body, now)), "s").unwrap_err();
        assert_eq!(bad.code(), "bad_signature");
        assert_eq!(bad.status(), 400);
        assert!(bad.body().contains("bad_signature"));
        assert_eq!(verify_event(body.as_bytes(), None, "s").unwrap_err().code(), "malformed_header");
        assert_eq!(verify_event(body.as_bytes(), Some("t=1,v1=00"), "").unwrap_err().status(), 500);
    }
}
