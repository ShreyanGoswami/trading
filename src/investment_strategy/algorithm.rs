use crate::alpaca_trading_client::client::BuyOrder;
use crate::alpaca_trading_client::client::Client;

pub struct BuySharesStatusVec {
    shares_to_buy: Vec<String>,
    quantity: Vec<f32>,
    status: Vec<bool>,
    share_price: Vec<f32>,
}

impl<'a> BuySharesStatusVec {
    pub fn push(&mut self, share: String, share_quantity: f32, share_price: f32, status: bool) {
        self.shares_to_buy.push(share);
        self.quantity.push(share_quantity);
        self.share_price.push(share_price);
        self.status.push(status);
    }
}

pub async fn buy_shares(
    trading_client: &Client,
    shares_to_buy: &Vec<String>,
    total_money: &f32,
) -> Result<BuySharesStatusVec, Box<dyn std::error::Error>> {
    // amount to be spent on each share
    let money_spent_per_company = total_money / shares_to_buy.len() as f32;
    println!(
        "Total money - {} money to be spent per share {}",
        total_money, money_spent_per_company
    );
    let mut buy_status = BuySharesStatusVec {
        shares_to_buy: Vec::new(), // maybe a vector is not required because we know the number of shares to buy
        status: Vec::new(),
        quantity: Vec::new(),
        share_price: Vec::new(),
    };
    for share_symbol in shares_to_buy {
        println!(
            "Attempting to purchase {} for amount {}",
            share_symbol, money_spent_per_company
        );
        let res = match trading_client
            .open_position(&share_symbol, &money_spent_per_company)
            .await?
        {
            Some(buy_order) => {
                buy_status.push(
                    buy_order.share_symbol.to_string(),
                    *buy_order.share_quantity,
                    *buy_order.share_price,
                    buy_order.status,
                );
            }
            None => {}
        };
    }
    Ok(buy_status)
}

// Given a list of shares and their quantity sell all of them
pub async fn sell_shares(
    trading_client: &Client,
    shares_to_sell: &Vec<String>,
    quantity_of_shares: &Vec<f32>,
) -> Result<f32, Box<dyn std::error::Error>> {
    let mut failures: Vec<String> = Vec::new();
    for shares in shares_to_sell.iter().zip(quantity_of_shares.iter()) {
        let (share_symbol, quantity) = shares;
        if !trading_client
            .close_position(&share_symbol, &quantity)
            .await?
        {
            failures.push(share_symbol.clone());
        }
    }
    if failures.len() > 0 {
        println!("WARNING: Unable to close positions on the following shares");
        for share_symbol in &failures {
            println!("{}", share_symbol);
        }
    }
    Ok(1.00) // TODO calculate total profit/loss
}
