mod alpaca_trading_client;
use alpaca_trading_client::client::Client;

mod investment_strategy;
use investment_strategy::algorithm::buy_shares;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = match env::var("ALPACA_PAPER_API_KEY") {
        Ok(v) => v,
        Err(e) => panic!("ALPACA api key not set")
    };
    let api_secret = match env::var("ALPACA_PAPER_API_SECRET") {
        Ok(v) => v,
        Err(e) => panic!("ALPACA secret api key not set")
    };
    println!("{} {}", api_key, api_secret);
    let trading_client = Client::init_client(
        &"https://paper-api.alpaca.markets/v2",
        &api_key,
        &api_secret,
    );
    let mut shares_to_buy = Vec::new();
    shares_to_buy.push(String::from("AMZN"));
    shares_to_buy.push(String::from("GOOG"));
    shares_to_buy.push(String::from("APPL"));
    shares_to_buy.push(String::from("qwer"));
    let money_to_trade: f32 = 100.0;
    // buy_sell_amazon().await?;

    Ok(())
    // read stock input from user

    // configure client

    // call API to buy the share

    // test out the API when share does not exist

    // call API to sell a share that I own

    // call API to sell a share that does not exist

    // call API to sell a share that I don't own
}

// async fn buy_sell_amazon() -> Result<(), Box<dyn std::error::Error>> {
//     println!("Checking balance");
//     let trading_client = Client::init_client(
//         &"https://paper-api.alpaca.markets/v2",
//         &"PK9LN9JUKKAASLKF2JSA",
//         &"MCJ7DAm7uw6gvSYb3DL3XeUAVbxCYfnjFarEiliX",
//     );
//     trading_client.check_balance().await?;
//     println!("Getting stock info for Amazon");
//     let stock_symbol = String::from("AMZN");
//     let quantity = 1.00;

//     println!("Attempting to buy shares");
//     trading_client
//         .open_position(&stock_symbol, &quantity)
//         .await?;
//     println!("Bought shares");

//     println!("Selling the shares");
//     trading_client
//         .close_position(&stock_symbol, &quantity)
//         .await?;
//     println!("Sold shares");

//     Ok(())
// }
