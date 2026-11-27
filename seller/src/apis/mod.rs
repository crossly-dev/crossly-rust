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
pub mod app_managed_listings_api;
pub mod apps_api;
pub mod automation_api;
pub mod billing_api;
pub mod books_api;
pub mod brand_drops_api;
pub mod branding_api;
pub mod breaks_api;
pub mod bulk_batches_api;
pub mod buyer_api;
pub mod cbx_api;
pub mod cosm_api;
pub mod catalog_api;
pub mod channel_inventory_api;
pub mod comp_watchlists_api;
pub mod connections_api;
pub mod conversations_api;
pub mod creator_api;
pub mod creator_rewards_api;
pub mod creator_studio_api;
pub mod crossly_wallet_api;
pub mod customers_api;
pub mod dao_api;
pub mod default_api;
pub mod developer_apps_api;
pub mod discounts_api;
pub mod drops_api;
pub mod embeds_api;
pub mod field_rules_api;
pub mod funding_api;
pub mod grading_api;
pub mod group_buys_api;
pub mod imports_api;
pub mod inbox_api;
pub mod integrations_api;
pub mod inventory_api;
pub mod listings_api;
pub mod lookbooks_api;
pub mod magic_api;
pub mod mobile_api;
pub mod nft_market_api;
pub mod neighbourhoods_api;
pub mod network_api;
pub mod offers_api;
pub mod orders_api;
pub mod pat_api;
pub mod payout_api;
pub mod policy_presets_api;
pub mod price_rules_api;
pub mod product_feeds_api;
pub mod product_sets_api;
pub mod production_api;
pub mod profile_api;
pub mod reference_api;
pub mod restock_prompts_api;
pub mod returns_api;
pub mod sales_api;
pub mod sandbox_api;
pub mod saved_views_api;
pub mod scan_api;
pub mod search_api;
pub mod source_sync_api;
pub mod sourcing_api;
pub mod stalls_api;
pub mod storefront_api;
pub mod storefronts_api;
pub mod tcg_api;
pub mod tcg_creators_api;
pub mod tax_api;
pub mod taxonomy_api;
pub mod team_api;
pub mod templates_api;
pub mod toolbox_api;
pub mod trade_block_api;
pub mod vault_markets_api;
pub mod wallet_api;
pub mod web3_api;
pub mod webhooks_api;
pub mod wholesale_api;
pub mod workflows_api;
pub mod world_api;
pub mod worlds_api;
pub mod worlds_ads_api;
pub mod worlds_badges_api;
pub mod worlds_data_api;
pub mod worlds_engagement_api;
pub mod worlds_gallery_api;
pub mod worlds_http_api;
pub mod worlds_money_api;
pub mod worlds_packages_api;
pub mod worlds_rewards_api;
pub mod worlds_studio_api;
pub mod worlds_team_api;

pub mod configuration;
