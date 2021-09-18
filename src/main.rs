use serde::{Serialize, Deserialize};
use futures::{future, FutureExt};
use nipper::Document;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    origin: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://www.coingecko.com/en/coins/solana/historical_data/usd#panel").await?;

    // assert_eq!(res.url().as_str(), "hhttps://www.coingecko.com/get");

    println!("Status: {}", res.status());

    let res = res.text().await?;

    // println!("Body:\n\n{}", body);

    let document = Document::from(res.as_str());

    println!("Solana Historical Data");

    document.select("tbody").iter().for_each(|coin_data| {
        let info = coin_data.select("tr");
        println!("{}", info.text());
    });

    let futures = vec![sol_info().boxed()];
    let _results = future::join_all(futures).await;

    Ok(())
}

async fn sol_info() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://www.coingecko.com/en/coins/solana/news").await?;

    println!("Status: {}", res.status());

    let res = res.text().await?;

    let document = Document::from(res.as_str());

    println!("Solana News and Links");

    document.select("article").iter().for_each(|coin_data| {
        let info = coin_data.select("h2");
        let href = coin_data.select("a");
        println!("{}", info.text());
        println!("{}", href.attr("href").unwrap());
        println!();
    });

    Ok(())
}

// CSV function --->