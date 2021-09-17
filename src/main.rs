use futures::{future, FutureExt};
use nipper::Document;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://www.coingecko.com/en/coins/solana/historical_data/usd#panel").await?;

    println!("Status: {}", res.status());

    let res = res.text().await?;

    // println!("Body:\n\n{}", body);

    println!("Solana");

    let document = Document::from(res.as_str());

    document.select("tbody").iter().for_each(|coin_data| {
        let info = coin_data.select("tr");
        println!("{}", info.text());
    });

    let futures = vec![btc_data().boxed(), eth_data().boxed()];
    let _results = future::join_all(futures).await;

    Ok(())
}

async fn btc_data() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://www.coingecko.com/en/coins/solana/historical_data/usd#panel").await?;

    println!("Status: {}", res.status());

    let res = res.text().await?;

    let document = Document::from(res.as_str());

    println!("Bitcoin");

    document.select("tbody").iter().for_each(|coin_data| {
        let info = coin_data.select("tr");
        println!("{}", info.text());
        println!();
    });

    Ok(())
}

async fn eth_data() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://www.coingecko.com/en/coins/solana/historical_data/usd#panel").await?;

    println!("Status: {}", res.status());

    let res = res.text().await?;

    let document = Document::from(res.as_str());

    println!("Ethereum");

    document.select("tbody").iter().for_each(|coin_data| {
        let info = coin_data.select("tr");
        println!("{}", info.text());
        println!();
    });

    Ok(())
}

// CSV function --->