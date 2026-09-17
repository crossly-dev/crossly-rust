use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod ai_api;
pub mod account_api;
pub mod accounts_api;
pub mod activity_api;
pub mod ads_api;
pub mod analytics_api;
pub mod automation_api;
pub mod billing_api;
pub mod cbx_api;
pub mod catalog_api;
pub mod comp_watchlists_api;
pub mod connections_api;
pub mod customers_api;
pub mod default_api;
pub mod embeds_api;
pub mod imports_api;
pub mod inbox_api;
pub mod integrations_api;
pub mod inventory_api;
pub mod listings_api;
pub mod magic_api;
pub mod mobile_api;
pub mod network_api;
pub mod offers_api;
pub mod orders_api;
pub mod pat_api;
pub mod payout_api;
pub mod policy_presets_api;
pub mod profile_api;
pub mod reference_api;
pub mod restock_prompts_api;
pub mod returns_api;
pub mod sales_api;
pub mod saved_views_api;
pub mod sourcing_api;
pub mod tax_api;
pub mod taxonomy_api;
pub mod team_api;
pub mod templates_api;
pub mod webhooks_api;
pub mod workflows_api;

pub mod configuration;
