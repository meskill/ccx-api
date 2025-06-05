use ccx_bitgo::prelude::*;
use ccx_bitgo::types::order::{FundingType, OrderSide};
use chrono::Utc;
use envconfig::Envconfig;
use rust_decimal_macros::dec;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug, Envconfig)]
struct EnvConfig {
    #[envconfig(from = "EXAMPLE_BITGO_API_TOKEN")]
    api_token: String,
}

const ACCOUNT_ID: &str = "682776ff2b4bacca1470a4ca9a4b34b1";

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_err| "info,ccx_bitgo=trace,ccx_lib=trace".into()),
        )
        .init();

    let credential = {
        let config = match EnvConfig::init_from_env() {
            Ok(config) => config,
            Err(err) => {
                tracing::error!("{err}");
                std::process::exit(1);
            }
        };
        BitGoCredential::new(config.api_token)
    };
    let rate_limiter = RateLimiter::spawn();

    let client = {
        let client = reqwest::Client::new();
        let config = config::testing();

        BitGoClient::new(client, config)
    };

    let current_user = trade::CurrentUser::builder()
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(current_user);

    // Get account balance information
    let account_balance = trade::AccountBalance::builder()
        .account_id(ACCOUNT_ID)
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(&account_balance);

    let currencies = trade::ListCurrencies::builder()
        .account_id(ACCOUNT_ID)
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(currencies.currencies.first());

    // Get the list of products available for trading
    let products = trade::ListProducts::builder()
        .account_id(ACCOUNT_ID)
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(&products.products.first());

    let order_book = trade::OrderBook::builder()
        .account_id(ACCOUNT_ID)
        .product("TBTC-TUSD*")
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(order_book.asks.first(), order_book.bids.first());

    let orders = trade::ListOrders::builder()
        .account_id(ACCOUNT_ID)
        .date_gte(Utc::now() - chrono::Duration::minutes(1))
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(orders.orders.first());

    // Test orders should be ok to create, especially
    // considering this is limit order with low-probability chances to be fulfilled
    let new_order = trade::PlaceOrder::builder()
        .account_id(ACCOUNT_ID)
        .product("TBTC-USDC*")
        .side(OrderSide::Buy)
        .funding_type(FundingType::Funded)
        .quantity(dec!(0.1))
        .quantity_currency("TBTC")
        .order_type(trade::PlaceOrderType::Limit(
            trade::LimitOrder::builder()
                .limit_price(dec!(100))
                .duration(1u32)
                .build(),
        ))
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(&new_order);

    let order_info = trade::GetOrder::builder()
        .account_id(ACCOUNT_ID)
        .order_id(new_order.id)
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(order_info);

    let cancel_result = trade::CancelOrder::builder()
        .account_id(ACCOUNT_ID)
        .order_id(new_order.id)
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(&cancel_result);

    let orders = trade::ListOrders::builder()
        .account_id(ACCOUNT_ID)
        .date_gte(Utc::now() - chrono::Duration::minutes(1))
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(orders.orders.first());

    Ok(())
}
