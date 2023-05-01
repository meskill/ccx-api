use chrono::Utc;
use uuid::Uuid;

use super::BalanceType;
use super::PortfolioBalance;
use crate::api::prime::PrimeApi;
use crate::api::prime::RL_PORTFOLIO_KEY;
use crate::client::Task;
use crate::CoinbaseResult;

pub type GetBalanceResponse = PortfolioBalance;

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// Get Account Balance.
    ///
    /// Retrieve all cash balances, net of pending withdrawals.
    ///
    /// * `portfolio_id` - The portfolio ID.
    /// * `symbols` - A list of symbols by which to filter the response.
    /// * `balance_type` - A type by which to filter balances.
    ///
    /// [https://docs.cloud.coinbase.com/prime/reference/primerestapi_getposttradecredit]
    pub fn get_portfolio_balances(
        &self,
        portfolio_id: Uuid,
        symbols: Option<String>,
        balance_type: Option<BalanceType>,
    ) -> CoinbaseResult<Task<GetBalanceResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/balances");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(&endpoint)?
                    .try_query_arg("symbols", &symbols)?
                    .try_query_arg("balance_type", &balance_type)?
                    .signed(timestamp)?
                    .request_body(())?,
            )
            .cost(RL_PORTFOLIO_KEY, 1)
            .send())
    }
}
