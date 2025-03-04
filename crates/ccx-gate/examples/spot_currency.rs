use ccx_gate::prelude::*;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = {
        let client = reqwest::Client::new();
        let config = config::production();

        GateClient::new(client, config)
    };
    let all_currencies = spot::AllCurrencies.send(&client).await?.into_payload();

    println!("len of all currencies: {}", all_currencies.len());

    let btc_currency = spot::Currency::new("BTC")
        .send(&client)
        .await?
        .into_payload();

    dbg!(btc_currency);

    let all_currency_pairs = spot::AllCurrencyPairs.send(&client).await?.into_payload();

    println!("len of all currency pairs: {}", all_currency_pairs.len());

    let currency_pair = spot::CurrencyPair::new("BTC", "USDT")
        .send(&client)
        .await?
        .into_payload();

    dbg!(currency_pair);

    Ok(())
}
