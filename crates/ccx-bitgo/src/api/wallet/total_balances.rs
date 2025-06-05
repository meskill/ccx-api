use macro_rules_attribute::apply;

use crate::prelude::Coin;
use crate::proto::{Request, Response, SignedRequest};
use crate::types::derive::{Request, Response};
use crate::types::rate_limits::RateLimitType;

/// Request to list total balances across all wallets
#[apply(Request)]
pub struct TotalBalances {
    /// Include balances for all subtokens (i.e. ERC20 Tokens, Stellar Tokens)
    all_tokens: Option<bool>,

    /// Filter by coin
    coin: Option<Vec<Coin>>,

    /// Filter by deleted state
    deleted: Option<bool>,

    /// Filter by enterprise ID
    enterprise: Option<Vec<String>>,

    /// Exclude tokens with zero balance.
    /// When allTokens is passed, then every token supported by BitGo will be included, regardless of balance.
    exclude_empty_balances: Option<bool>,

    /// Skip spendable balance calculations and only include confirmed balances.
    exclude_spendable_balances: Option<bool>,

    /// Whether balances of linked custodial wallets should be included
    expand_custodial_wallet: Option<bool>,

    /// Filter by wallet ID
    id: Option<Vec<String>>,

    /// Include `stakingBalanceString` and `rewardBalanceString` properties for each staking wallet.
    include_staking_balances: Option<bool>,

    /// Filter by label substring
    label_contains: Option<String>,

    /// Do not include pending withdrawals when determining total balances.
    /// By default, pending withdrawals will be considered.
    skip_pending_txs: Option<bool>,

    /// Exclude transfer counts when calculating total balances.
    skip_tx_counts: Option<bool>,

    /// Filter by wallet type
    #[serde(rename = "type")]
    wallet_type: Option<Vec<String>>,
}

/// Lightning Balance information
#[apply(Response)]
pub struct LightningBalance {
    /// Balance in string representation
    pub balance_string: Option<String>,
    /// Confirmed balance in string representation
    pub confirmed_balance_string: Option<String>,
}

/// Wallet Balance information
#[apply(Response)]
pub struct WalletBalance {
    /// Coin symbol (e.g., "btc")
    pub coin: Coin,

    /// String representation of the balance in base units
    pub balance_string: String,

    /// String representation of the confirmed balance in base units
    pub confirmed_balance_string: String,

    /// String representation of the spendable balance in base units
    pub spendable_balance_string: Option<String>,

    /// The staked balance in base units (only if includeStakingBalances=true)
    pub staking_balance_string: Option<String>,

    /// The staking reward balance in base units (only if includeStakingBalances=true)
    pub reward_balance_string: Option<String>,

    /// Lightning Balances
    pub offchain: Option<LightningBalance>,
}

/// Response for TotalBalances request
#[apply(Response)]
pub struct TotalBalancesResponse {
    /// List of balances by coin
    pub balances: Vec<WalletBalance>,
}

impl Response for TotalBalancesResponse {}

impl Request for TotalBalances {
    type Response = TotalBalancesResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        "/api/v2/wallet/balances".into()
    }
}

impl SignedRequest for TotalBalances {}
