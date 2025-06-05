use macro_rules_attribute::apply;
use uuid::Uuid;

use crate::types::derive::Request;
use crate::types::derive::Response;
use crate::proto::{Request, Response, SignedRequest};
use crate::types;
use crate::types::rate_limits::RateLimitType;

#[apply(Request)]
pub struct ListProducts {
    /// The id of the trading account to retrieve
    #[serde(skip)]
    account_id: String,
}

/// Product information
#[apply(Response)]
pub struct Product {
    /// Unique identifier for the product
    pub id: Uuid,
    /// Product name e.g. BTC-USD
    pub name: types::product::Product,
    /// Base currency ID
    pub base_currency_id: Uuid,
    /// Quote currency ID
    pub quote_currency_id: Uuid,
    /// Base currency symbol (e.g., "BTC")
    pub base_currency: String,
    /// Quote currency symbol (e.g., "USD")
    pub quote_currency: String,
    /// Minimum size for base currency
    pub base_min_size: String,
    /// Maximum size for base currency
    pub base_max_size: Option<String>,
    /// Increment for base currency
    pub base_increment: Option<String>,
    /// Minimum size for quote currency
    pub quote_min_size: Option<String>,
    /// Increment for quote currency
    pub quote_increment: String,
    /// True if trading is disabled for this product
    pub is_trade_disabled: bool,
    /// True if margin trading is supported for this product
    pub is_margin_trade_supported: bool,
}

#[apply(Response)]
pub struct ListProductsResponse {
    /// Array of available products
    #[serde(rename = "data")]
    pub products: Vec<Product>,
}

impl Response for ListProductsResponse {}

impl Request for ListProducts {
    type Response = ListProductsResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        let account_id = &self.account_id;

        format!("/api/prime/trading/v1/accounts/{account_id}/products").into()
    }
}

impl SignedRequest for ListProducts {}
