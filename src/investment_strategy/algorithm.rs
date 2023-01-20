use crate::alpaca_trading_client::client::Client;
use uuid::{Uuid};

pub struct BuySharesStatusVec {
    pub shares: Vec<String>,
    pub prices: Vec<f32>,
    pub status: Vec<bool>,
    pub order_ids: Vec<Uuid>
}

impl<'a> BuySharesStatusVec {
    pub fn push(&mut self, share: String, order_amount : f32, order_id: Uuid, status: bool) {
        self.shares.push(share);
        self.prices.push(order_amount);
        self.status.push(status);
        self.order_ids.push(order_id);
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
        shares: Vec::new(), // maybe a vector is not required because we know the number of shares to buy
        prices: Vec::new(),
        status: Vec::new(),
        order_ids: Vec::new(),
    };
    for share_symbol in shares_to_buy {
        let res = match trading_client
            .open_position(&share_symbol, &money_spent_per_company)
            .await?
        {
            Some(buy_order) => {
                buy_status.push(
                    buy_order.symbol.to_string(),
                    money_spent_per_company,
                    Uuid::parse_str(&buy_order.id.to_string()).unwrap(), // TODO remove this and use match
                    true,
                );
            }
            None => {
                buy_status.push(
                    share_symbol.to_string(),
                    money_spent_per_company,
                    Uuid::new_v4(), // since the position was not opened, we generate a random guid for now
                    false,
                );
            }
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
