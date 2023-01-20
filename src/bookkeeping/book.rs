use crate::alpaca_trading_client::client::Client;
use crate::investment_strategy::algorithm::{buy_shares, sell_shares, BuySharesStatusVec};

use uuid::Uuid;

enum OrderStatus {
    OrderIncomplete,
    OrderExecuted,
    OrderActive,
    OrderSold,
}

enum ShareStatus {
    Purchased,
    FailedToPurchase,
    Sold,
}

pub struct Portfolio {
    portfolio_id: Uuid,
    order_ids: Vec<Uuid>,
    status: Vec<bool>
}

struct Order {
    pub share_symbol: String,
    pub share_quantity: f32,
    pub share_price_bought: f32,
    pub share_price_sold: Option<f32>,
}

impl From<BuySharesStatusVec> for Portfolio {
    fn from(buy_shares_status: BuySharesStatusVec) -> Self {
        Portfolio {
            portfolio_id: Uuid::new_v4(),
            order_ids: buy_shares_status.order_ids,
            status: buy_shares_status.status,
        }
    }
}

impl<'a> Portfolio {

    pub async fn execute_buy_order(trading_client: &Client, shares_to_buy: &'a Vec<String>, total_money: &'a f32) -> Result<Portfolio, Box<dyn std::error::Error>> {
        let buy_status: BuySharesStatusVec = buy_shares(trading_client, shares_to_buy, total_money).await?;
        // TODO store all the order ids in a file
        let portfolio = Portfolio::from(buy_status);
        println!("Order report");
        // TODO present the transaction as a single order with all the shares that have been bought -> should include the share symbol, the price of each share and the total shares bought for that share
        println!("In case any shares were not bought, please use the order id to execute the transaction again");
        Ok(portfolio)
    }

    // pub async fn execute_sell_order(trading_client: &Client, portfolio_id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
    //     // sells all the shares in the order that have been bough
    //     let orders = get_order(trading_client, portfolio_id).await?;
    //     // 
    // }

    // pub async fn get_order(trading_client: &Client, portfolio_id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
    //     // pull out the order from storage
    //     // for each id in the order, get the information using the get_orders API
    //     let portfolio = get_portfolio(portfolio_id);
    //     for order_id in &portfolio.order_ids {
    //         let order = trading_client.get_order(&order_id);
    //         // convert to a viewable order
    //     }
    //     Ok(())
    // }

    // fn get_portfolio(portfolio_id: &Uuid) -> Portfolio {
        
    // }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn it_creates_portfolio() {
        panic!("Something went wrong");
    }
}