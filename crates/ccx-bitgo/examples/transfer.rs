use ccx_bitgo::prelude::*;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

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
    let rate_limiter = RateLimiter::spawn();

    let client = {
        let client = reqwest::Client::new();
        let config = config::testing();

        BitGoClient::new(client, config)
    };

    let fee = transfer::FeeEstimate::builder()
        .coin("tton")
        .build()
        .throttle(&rate_limiter)
        .await?
        .send(&client)
        .await?
        .into_payload();

    dbg!(&fee);

    let fee = transfer::FeeEstimate::builder()
        .coin("teth")
        .build()
        .throttle(&rate_limiter)
        .await?
        .send(&client)
        .await?
        .into_payload();

    dbg!(&fee);

    Ok(())
}
