use scraper::{Html, Selector};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let body = get_bitches().await.unwrap();

    let fragment = Html::parse_fragment(&body);
    let selector = Selector::parse(".pip-temp-price-module__price").unwrap();

    let price_div = fragment.select(&selector).next().unwrap();
    let price_text = price_div.text().collect::<Vec<_>>();

    println!("{:#?}", price_text);
}

async fn get_bitches() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.ikea.com/fi/fi/p/blahaj-pehmolelu-hai-30373588/")
        .await?
        .text()
        .await?;

    Ok(resp)
}
