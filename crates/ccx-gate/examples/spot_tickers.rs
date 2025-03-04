use ccx_gate::prelude::*;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = {
        let client = reqwest::Client::new();
        let config = config::production();

        GateClient::new(client, config)
    };

    let tickers = spot::SpotTickers::builder()
        .currency_pair("BTC_USDT")
        .build()
        .send(&client)
        .await?
        .into_payload();

    dbg!(tickers);

    Ok(())
}
