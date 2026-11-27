//! Client ergonomics over the generated `apis`: retries with backoff that honour
//! `Retry-After`, an `Idempotency-Key` on every POST/PATCH, timeouts, a
//! `crossly-rust/<version>` User-Agent, a typed [`CrosslyError`], and
//! [`paginate`] over any list endpoint.
//!
//! Hand-written, vendored by codegen (edit it in the Crossly monorepo:
//! tools/sdk-repo/extras/rust/). Every generated call goes through [`execute`],
//! so the retry policy on [`Configuration::retry`](crate::apis::configuration::Configuration)
//! applies to all of them.
//!
//! ```ignore
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let config = crossly::client::config(std::env::var("CROSSLY_TOKEN")?);
//! # Ok(()) }
//! ```

use std::collections::hash_map::RandomState;
use std::collections::VecDeque;
use std::future::Future;
use std::hash::{BuildHasher, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::apis::configuration::Configuration;

pub use futures_util::stream::{LocalBoxStream, StreamExt, TryStreamExt};

/// This crate's version, from Cargo.toml.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The User-Agent [`config`] sets.
pub fn user_agent() -> String {
    format!("crossly-rust/{VERSION}")
}

/// How generated calls retry. Lives on `Configuration::retry`.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Retries after the first attempt (0 disables retrying).
    pub max_retries: u32,
    /// First backoff ceiling; each retry doubles it (full jitter), up to `max_backoff`.
    pub base_backoff: Duration,
    pub max_backoff: Duration,
    /// A `Retry-After` longer than this is returned to the caller instead of slept through.
    pub max_retry_after: Duration,
    /// Add an `Idempotency-Key` to POST and PATCH requests that do not carry one. The API replays the
    /// first response to a repeated key for 24 hours, so a retried write is applied once.
    pub idempotency_keys: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_backoff: Duration::from_millis(500),
            max_backoff: Duration::from_secs(8),
            max_retry_after: Duration::from_secs(60),
            idempotency_keys: true,
        }
    }
}

impl RetryPolicy {
    /// No retries (and no automatic idempotency keys).
    pub fn none() -> Self {
        Self { max_retries: 0, idempotency_keys: false, ..Self::default() }
    }
}

/// A `Configuration` for `token` (a `crossly_pat_…` Personal Access Token or an OAuth token) with a
/// 30 s timeout, the default [`RetryPolicy`] and the `crossly-rust/<version>` User-Agent.
pub fn config(token: impl Into<String>) -> Configuration {
    ConfigBuilder::new(token).build()
}

/// Builder for a `Configuration` with non-default settings.
#[derive(Debug, Clone)]
pub struct ConfigBuilder {
    token: String,
    base_path: Option<String>,
    timeout: Duration,
    retry: RetryPolicy,
}

impl ConfigBuilder {
    pub fn new(token: impl Into<String>) -> Self {
        Self { token: token.into(), base_path: None, timeout: Duration::from_secs(30), retry: RetryPolicy::default() }
    }
    /// Another server than https://crossly.net/api.
    pub fn base_path(mut self, base: impl Into<String>) -> Self {
        self.base_path = Some(base.into().trim_end_matches('/').to_owned());
        self
    }
    /// Per-attempt timeout, including reading the body.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    pub fn retry(mut self, retry: RetryPolicy) -> Self {
        self.retry = retry;
        self
    }
    pub fn build(self) -> Configuration {
        let mut c = Configuration::new();
        if let Some(base) = self.base_path {
            c.base_path = base;
        }
        c.user_agent = Some(user_agent());
        c.bearer_access_token = Some(self.token.clone());
        c.oauth_access_token = Some(self.token);
        c.client = reqwest::Client::builder()
            .timeout(self.timeout)
            .build()
            .expect("a reqwest client with only a timeout set always builds");
        c.retry = self.retry;
        c
    }
}

/// A random key for the `Idempotency-Key` header.
pub fn new_idempotency_key() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let mut out = String::with_capacity(32);
    for _ in 0..2 {
        let mut h = RandomState::new().build_hasher();
        h.write_u64(COUNTER.fetch_add(1, Ordering::Relaxed));
        h.write_u128(SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0));
        out.push_str(&format!("{:016x}", h.finish()));
    }
    out
}

/// `Retry-After` as a duration: delta-seconds or an HTTP date (RFC 9110).
pub fn retry_after(headers: &reqwest::header::HeaderMap) -> Option<Duration> {
    let v = headers.get(reqwest::header::RETRY_AFTER)?.to_str().ok()?.trim();
    if let Ok(secs) = v.parse::<f64>() {
        return if secs >= 0.0 { Some(Duration::from_secs_f64(secs)) } else { None };
    }
    let at = httpdate_to_unix(v)?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs() as i64;
    Some(Duration::from_secs((at - now).max(0) as u64))
}

/// IMF-fixdate ("Sun, 06 Nov 1994 08:49:37 GMT") to unix seconds; the only form servers send now.
fn httpdate_to_unix(s: &str) -> Option<i64> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() != 6 || parts[5] != "GMT" {
        return None;
    }
    let day: i64 = parts[1].parse().ok()?;
    let month = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]
        .iter()
        .position(|m| *m == parts[2])? as i64
        + 1;
    let year: i64 = parts[3].parse().ok()?;
    let hms: Vec<i64> = parts[4].split(':').map(|x| x.parse().ok()).collect::<Option<Vec<_>>>()?;
    if hms.len() != 3 {
        return None;
    }
    // Days from civil (Howard Hinnant's algorithm).
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (month + 9) % 12;
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    Some(days * 86_400 + hms[0] * 3600 + hms[1] * 60 + hms[2])
}

fn jitter(ceiling: Duration) -> Duration {
    let mut h = RandomState::new().build_hasher();
    h.write_u128(SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0));
    let nanos = ceiling.as_nanos().max(1) as u64;
    Duration::from_nanos(h.finish() % nanos + 1)
}

fn backoff(policy: &RetryPolicy, attempt: u32) -> Duration {
    let ceiling = policy.base_backoff.saturating_mul(1u32 << attempt.min(16)).min(policy.max_backoff);
    jitter(ceiling)
}

fn retryable_status(status: reqwest::StatusCode) -> bool {
    status == reqwest::StatusCode::TOO_MANY_REQUESTS
        || (status.is_server_error() && status != reqwest::StatusCode::NOT_IMPLEMENTED)
}

/// Send one request with the configuration's [`RetryPolicy`]. Every generated API function calls this in
/// place of `client.execute`, so it is rarely needed directly.
///
/// Retried: 429 always (the API refused before doing anything); 5xx and connection errors for reads, and
/// for POST/PATCH only when they carry an `Idempotency-Key` (added automatically unless disabled). A body
/// that cannot be cloned (a stream) is sent once.
pub async fn execute(configuration: &Configuration, mut request: reqwest::Request) -> Result<reqwest::Response, reqwest::Error> {
    let policy = &configuration.retry;
    let write = matches!(*request.method(), reqwest::Method::POST | reqwest::Method::PATCH);
    if write && policy.idempotency_keys && !request.headers().contains_key("idempotency-key") {
        if let Ok(v) = reqwest::header::HeaderValue::from_str(&new_idempotency_key()) {
            request.headers_mut().insert("idempotency-key", v);
        }
    }
    let keyed = request.headers().contains_key("idempotency-key");
    let mut attempt: u32 = 0;
    loop {
        let again = if attempt < policy.max_retries { request.try_clone() } else { None };
        let result = configuration.client.execute(request).await;
        let next = match again {
            Some(next) => next,
            None => return result,
        };
        let delay = match &result {
            Ok(resp) => {
                let status = resp.status();
                if !retryable_status(status) || (write && !keyed && status != reqwest::StatusCode::TOO_MANY_REQUESTS) {
                    return result;
                }
                match retry_after(resp.headers()) {
                    Some(wait) if wait > policy.max_retry_after => return result,
                    Some(wait) => wait,
                    None => backoff(policy, attempt),
                }
            }
            Err(e) => {
                if (write && !keyed) || e.is_builder() || e.is_redirect() || e.is_decode() {
                    return result;
                }
                backoff(policy, attempt)
            }
        };
        drop(result);
        tokio::time::sleep(delay).await;
        request = next;
        attempt += 1;
    }
}

/// A failed call, decoded from the API's error envelope
/// `{ "error": { "code", "message", "details" }, "correlationId" }`.
///
/// `code` is the stable key to match on (`not_found`, `rate_limited`, `idempotency_key_reused`, …);
/// `correlation_id` is what Crossly support traces. Convert with `?` or `.map_err(CrosslyError::from)`.
#[derive(Debug, Clone, PartialEq)]
pub struct CrosslyError {
    /// HTTP status; `None` when the request never got a response (connection, timeout, decoding).
    pub status: Option<u16>,
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub correlation_id: Option<String>,
    /// The raw response body, when there was one.
    pub body: String,
}

impl std::fmt::Display for CrosslyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status {
            Some(s) => write!(f, "crossly: {s} {}: {}", self.code, self.message)?,
            None => write!(f, "crossly: {}: {}", self.code, self.message)?,
        }
        if let Some(id) = &self.correlation_id {
            write!(f, " (correlationId {id})")?;
        }
        Ok(())
    }
}

impl std::error::Error for CrosslyError {}

impl CrosslyError {
    /// Parse an error response body.
    pub fn from_body(status: u16, body: &str) -> Self {
        let v: serde_json::Value = serde_json::from_str(body).unwrap_or(serde_json::Value::Null);
        let err = v.get("error");
        let text = |k: &str| err.and_then(|e| e.get(k)).and_then(|x| x.as_str()).map(str::to_owned);
        let reason = reqwest::StatusCode::from_u16(status).ok().and_then(|s| s.canonical_reason()).unwrap_or("error");
        Self {
            status: Some(status),
            code: text("code").unwrap_or_else(|| format!("http_{status}")),
            message: text("message").unwrap_or_else(|| reason.to_owned()),
            details: err.and_then(|e| e.get("details")).filter(|d| !d.is_null()).cloned(),
            correlation_id: v.get("correlationId").and_then(|x| x.as_str()).map(str::to_owned),
            body: body.to_owned(),
        }
    }
}

impl<T> From<crate::apis::Error<T>> for CrosslyError {
    fn from(e: crate::apis::Error<T>) -> Self {
        let other = |code: &str, message: String| Self {
            status: None,
            code: code.to_owned(),
            message,
            details: None,
            correlation_id: None,
            body: String::new(),
        };
        match e {
            crate::apis::Error::ResponseError(rc) => Self::from_body(rc.status.as_u16(), &rc.content),
            crate::apis::Error::Reqwest(e) if e.is_timeout() => other("timeout", e.to_string()),
            crate::apis::Error::Reqwest(e) => {
                let mut out = other("network_error", e.to_string());
                out.status = e.status().map(|s| s.as_u16());
                out
            }
            crate::apis::Error::Serde(e) => other("decode_error", e.to_string()),
            crate::apis::Error::Io(e) => other("io_error", e.to_string()),
        }
    }
}

/// A list response: its rows and its pagination block. Implemented for every generated
/// `{ data, pagination }` response model.
pub trait Paged {
    type Item;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>);
}

fn is_last_page(rows: usize, p: Option<&crate::models::Pagination>, page: i32, limit: i32) -> bool {
    let p = match p {
        // No pagination block: the endpoint returned everything at once.
        None => return true,
        Some(p) => p,
    };
    if rows == 0 {
        return true;
    }
    if let Some(total_pages) = p.total_pages {
        return page >= total_pages;
    }
    if let Some(total) = p.total {
        return i64::from(page) * i64::from(limit) >= i64::from(total);
    }
    (rows as i64) < i64::from(if p.limit > 0 { p.limit } else { limit })
}

/// Every row of a list endpoint as a stream, fetching each page when the stream reaches it. `fetch`
/// gets `(page, limit)`; the stream ends after the last page, or after yielding the first error.
///
/// ```ignore
/// # async fn run(config: crossly::apis::configuration::Configuration) -> Result<(), crossly::client::CrosslyError> {
/// use crossly::client::{paginate, StreamExt};
/// let mut rows = paginate(100, |page, limit| crossly::apis::listings_api::list_listings(&config, Some(page), Some(limit), None, None, None));
/// while let Some(row) = rows.next().await {
///     let row = row?;
///     println!("{}", row.id);
/// }
/// # Ok(()) }
/// ```
pub fn paginate<'a, R, E, F, Fut>(limit: i32, fetch: F) -> LocalBoxStream<'a, Result<R::Item, CrosslyError>>
where
    R: Paged + 'a,
    R::Item: 'a,
    E: Into<CrosslyError> + 'a,
    F: FnMut(i32, i32) -> Fut + 'a,
    Fut: Future<Output = Result<R, E>> + 'a,
{
    struct State<F, I> {
        fetch: F,
        page: i32,
        limit: i32,
        rows: VecDeque<I>,
        done: bool,
    }
    let limit = if limit > 0 { limit } else { 100 };
    let start = State { fetch, page: 1, limit, rows: VecDeque::new(), done: false };
    futures_util::stream::unfold(start, |mut st| async move {
        loop {
            if let Some(row) = st.rows.pop_front() {
                return Some((Ok(row), st));
            }
            if st.done {
                return None;
            }
            match (st.fetch)(st.page, st.limit).await {
                Err(e) => {
                    st.done = true;
                    return Some((Err(e.into()), st));
                }
                Ok(resp) => {
                    let (rows, p) = resp.into_page();
                    st.done = is_last_page(rows.len(), p.as_ref(), st.page, st.limit);
                    st.page += 1;
                    st.rows.extend(rows);
                }
            }
        }
    })
    .boxed_local()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_error_envelope() {
        let e = CrosslyError::from_body(
            404,
            r#"{"error":{"code":"not_found","message":"No such listing","details":{"id":"x"}},"correlationId":"corr_1"}"#,
        );
        assert_eq!(e.code, "not_found");
        assert_eq!(e.correlation_id.as_deref(), Some("corr_1"));
        assert_eq!(e.details, Some(serde_json::json!({"id": "x"})));
        let plain = CrosslyError::from_body(502, "<html>bad gateway</html>");
        assert_eq!(plain.code, "http_502");
    }

    #[test]
    fn reads_retry_after() {
        let mut h = reqwest::header::HeaderMap::new();
        h.insert(reqwest::header::RETRY_AFTER, "2".parse().unwrap());
        assert_eq!(retry_after(&h), Some(Duration::from_secs(2)));
        assert_eq!(httpdate_to_unix("Sun, 06 Nov 1994 08:49:37 GMT"), Some(784_111_777));
    }

    #[test]
    fn last_page_rules() {
        let mut p = crate::models::Pagination::new(1, 2);
        assert!(!is_last_page(2, Some(&p), 1, 2));
        assert!(is_last_page(1, Some(&p), 1, 2));
        p.total = Some(4);
        assert!(!is_last_page(2, Some(&p), 1, 2));
        assert!(is_last_page(2, Some(&p), 2, 2));
        p.total_pages = Some(3);
        assert!(!is_last_page(2, Some(&p), 2, 2));
        assert!(is_last_page(2, None, 1, 2));
    }

    /// A one-connection-per-request HTTP server answering `responses` in turn; returns each request's
    /// lowercased `idempotency-key` and `user-agent` lines.
    fn serve(responses: Vec<&'static str>) -> (String, std::thread::JoinHandle<Vec<(Option<String>, Option<String>)>>) {
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let handle = std::thread::spawn(move || {
            let mut seen = Vec::new();
            for resp in responses {
                let (mut s, _) = listener.accept().unwrap();
                let mut buf = vec![0u8; 16384];
                let mut n = 0;
                while !String::from_utf8_lossy(&buf[..n]).contains("\r\n\r\n") {
                    n += s.read(&mut buf[n..]).unwrap();
                }
                let req = String::from_utf8_lossy(&buf[..n]).to_lowercase();
                let header = |name: &str| req.lines().find(|l| l.starts_with(name)).map(|l| l.trim().to_owned());
                seen.push((header("idempotency-key:"), header("user-agent:")));
                s.write_all(resp.as_bytes()).unwrap();
            }
            seen
        });
        (base, handle)
    }

    const OK: &str = "HTTP/1.1 200 OK\r\ncontent-length: 2\r\nconnection: close\r\n\r\n{}";

    #[tokio::test]
    async fn retries_429_and_503_with_one_idempotency_key() {
        let (base, server) = serve(vec![
            "HTTP/1.1 429 Too Many Requests\r\nretry-after: 0\r\ncontent-length: 0\r\nconnection: close\r\n\r\n",
            "HTTP/1.1 503 Service Unavailable\r\ncontent-length: 0\r\nconnection: close\r\n\r\n",
            OK,
        ]);
        let retry = RetryPolicy { base_backoff: Duration::from_millis(1), ..RetryPolicy::default() };
        let cfg = ConfigBuilder::new("crossly_pat_test").base_path(base.clone()).retry(retry).build();
        let req = cfg.client.post(format!("{base}/v1/x")).header("user-agent", user_agent()).body("{}").build().unwrap();
        let resp = execute(&cfg, req).await.unwrap();
        assert_eq!(resp.status(), 200);
        let seen = server.join().unwrap();
        assert_eq!(seen.len(), 3);
        assert!(seen[0].0.is_some(), "POST got no idempotency key");
        assert!(seen.iter().all(|s| s.0 == seen[0].0), "keys differ across retries: {seen:?}");
        assert_eq!(seen[0].1.as_deref(), Some(format!("user-agent: crossly-rust/{VERSION}").as_str()));
    }

    #[tokio::test]
    async fn returns_a_long_retry_after_instead_of_sleeping() {
        let (base, server) = serve(vec!["HTTP/1.1 429 Too Many Requests\r\nretry-after: 3600\r\ncontent-length: 0\r\nconnection: close\r\n\r\n"]);
        let cfg = ConfigBuilder::new("t").base_path(base.clone()).build();
        let resp = execute(&cfg, cfg.client.get(format!("{base}/v1/x")).build().unwrap()).await.unwrap();
        assert_eq!(resp.status(), 429);
        assert_eq!(server.join().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn does_not_retry_a_404() {
        let (base, server) = serve(vec!["HTTP/1.1 404 Not Found\r\ncontent-length: 0\r\nconnection: close\r\n\r\n"]);
        let cfg = ConfigBuilder::new("t").base_path(base.clone()).build();
        let resp = execute(&cfg, cfg.client.get(format!("{base}/v1/x")).build().unwrap()).await.unwrap();
        assert_eq!(resp.status(), 404);
        assert_eq!(server.join().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn paginates_until_the_last_page() {
        #[derive(Default)]
        struct Resp(Vec<i32>, Option<crate::models::Pagination>);
        impl Paged for Resp {
            type Item = i32;
            fn into_page(self) -> (Vec<i32>, Option<crate::models::Pagination>) {
                (self.0, self.1)
            }
        }
        let calls = std::cell::Cell::new(0);
        let rows: Vec<i32> = paginate(2, |page, limit| {
            calls.set(calls.get() + 1);
            async move {
                let all = [1, 2, 3, 4, 5];
                let lo = ((page - 1) * limit) as usize;
                let hi = (lo + limit as usize).min(all.len());
                let mut p = crate::models::Pagination::new(page, limit);
                p.total = Some(5);
                Ok::<_, CrosslyError>(Resp(all[lo..hi].to_vec(), Some(p)))
            }
        })
        .try_collect()
        .await
        .unwrap();
        assert_eq!(rows, vec![1, 2, 3, 4, 5]);
        assert_eq!(calls.get(), 3);
    }

    #[test]
    fn idempotency_keys_differ() {
        assert_ne!(new_idempotency_key(), new_idempotency_key());
        assert_eq!(new_idempotency_key().len(), 32);
    }
}
