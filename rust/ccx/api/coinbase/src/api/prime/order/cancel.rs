use std::str::FromStr;

use crate::api::prime::prelude::*;
use crate::api::prime::AccountPortfolioOrder;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioCancelOrderResponse {
    pub id: Uuid,
}

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbaseSigner,
    S: Unpin + 'static,
{
    /// Get Order by Order ID.
    ///
    /// Retrieve an order by order ID.
    ///
    /// * `portfolio_id` - The ID of the portfolio under which the order was placed.
    /// * `order_id` - The order ID generated by Coinbase upon order submission.
    ///
    /// [https://docs.cloud.coinbase.com/prime/reference/primerestapi_cancelorder]
    pub fn cancel_order(
        &self,
        portfolio_id: Uuid,
        order_id: Uuid,
    ) -> CoinbaseResult<Task<AccountPortfolioCancelOrderResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/orders/{order_id}/cancel");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .post(&endpoint)?
                    .signed(timestamp)?
                    .request_body(())?,
            )
            .cost(RL_PORTFOLIO_KEY, 1)
            .send())
    }
}
