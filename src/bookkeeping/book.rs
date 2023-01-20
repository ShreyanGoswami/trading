use crate::alpaca_trading_client::client::Client;
use crate::investment_strategy::buy_shares;
use crate::investment_strategy::sell_shares;
use crate::investment_strategy::BuySharesStatusVec;

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

pub struct Order<'a> {
    order_id: Uuid
    shares: &'a Vec<String>,
    amount_paid_per_share: f32,
    quantity: &'a Vec<f32>,
    unrealized_pl: Vec<f32>,
    order_status: OrderStatus,
    order_status_per_share: &'a Vec<bool>
}

impl Order {

    pub async fn execute_buy_order(trading_client: &Client, shares_to_buy: &'a Vec<String>, total_money: &'a f32) -> Result<(), Box<dyn std::error::Error> {
        let buy_status = buy_shares(trading_client, shares_to_buy, total_money).await?;
        // TODO generate a report of the buy order
        println!("Order report");
        // TODO present the transaction as a single order with all the shares that have been bought -> should include the share symbol, the price of each share and the total shares bought for that share
        println!("In case any shares were not bought, please use the order id to execute the transaction again");
        Ok(())
    }

    pub async fn execute_sell_order(&self, trading_client: &Client) {
        // sells all the shares in the order that have been bough
        sell_shares(trading_client, &self.shares, &self.quantity).await?;
    }
}