use trading::alpaca_trading_client::client::Client;
use trading::bookkeeping::book::Portfolio;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = match env::var("ALPACA_PAPER_API_KEY") {
        Ok(v) => v,
        Err(e) => panic!("ALPACA api key not set"),
    };
    let api_secret = match env::var("ALPACA_PAPER_API_SECRET") {
        Ok(v) => v,
        Err(e) => panic!("ALPACA secret api key not set"),
    };
    println!("{} {}", api_key, api_secret);
    let trading_client = Client::init_client(
        &"https://paper-api.alpaca.markets/v2",
        &api_key,
        &api_secret,
    );
    let mut shares_to_buy = Vec::new();
    shares_to_buy.push(String::from("AMZN"));
    // shares_to_buy.push(String::from("O"));
    shares_to_buy.push(String::from("AAPL"));
    let money_to_trade: f32 = 100.0;
    let portfolio = Portfolio::execute_buy_order(&trading_client, &shares_to_buy, &money_to_trade).await?;
    Ok(())
    // read stock input from user

    // configure client

    // call API to buy the share

    // test out the API when share does not exist

    // call API to sell a share that I own

    // call API to sell a share that does not exist

    // call API to sell a share that I don't own
}
