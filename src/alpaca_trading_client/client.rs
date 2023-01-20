use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO models need to be moved out
#[derive(Serialize, Deserialize)]
struct AccountInfo {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "account_number")]
    account_number: String,

    #[serde(rename = "status")]
    status: String,

    #[serde(rename = "crypto_status")]
    crypto_status: String,

    #[serde(rename = "currency")]
    currency: String,

    #[serde(rename = "buying_power")]
    buying_power: String,

    #[serde(rename = "regt_buying_power")]
    regt_buying_power: String,

    #[serde(rename = "daytrading_buying_power")]
    daytrading_buying_power: String,

    #[serde(rename = "effective_buying_power")]
    effective_buying_power: String,

    #[serde(rename = "non_marginable_buying_power")]
    non_marginable_buying_power: String,

    #[serde(rename = "bod_dtbp")]
    bod_dtbp: String,

    #[serde(rename = "cash")]
    cash: String,

    #[serde(rename = "accrued_fees")]
    accrued_fees: String,

    #[serde(rename = "pending_transfer_in")]
    pending_transfer_in: String,

    #[serde(rename = "portfolio_value")]
    portfolio_value: String,

    #[serde(rename = "pattern_day_trader")]
    pattern_day_trader: bool,

    #[serde(rename = "trading_blocked")]
    trading_blocked: bool,

    #[serde(rename = "transfers_blocked")]
    transfers_blocked: bool,

    #[serde(rename = "account_blocked")]
    account_blocked: bool,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "trade_suspended_by_user")]
    trade_suspended_by_user: bool,

    #[serde(rename = "multiplier")]
    multiplier: String,

    #[serde(rename = "shorting_enabled")]
    shorting_enabled: bool,

    #[serde(rename = "equity")]
    equity: String,

    #[serde(rename = "last_equity")]
    last_equity: String,

    #[serde(rename = "long_market_value")]
    long_market_value: String,

    #[serde(rename = "short_market_value")]
    short_market_value: String,

    #[serde(rename = "position_market_value")]
    position_market_value: String,

    #[serde(rename = "initial_margin")]
    initial_margin: String,

    #[serde(rename = "maintenance_margin")]
    maintenance_margin: String,

    #[serde(rename = "last_maintenance_margin")]
    last_maintenance_margin: String,

    #[serde(rename = "sma")]
    sma: String,

    #[serde(rename = "daytrade_count")]
    daytrade_count: i64,

    #[serde(rename = "balance_asof")]
    balance_asof: String,

    #[serde(rename = "crypto_tier")]
    crypto_tier: i64,
}

#[derive(Serialize, Deserialize)]
pub struct AssetInfo {
    pub id: String,
    pub class: String,
    pub exchange: String,
    pub symbol: String,
    pub name: String,
    pub status: String,
    pub tradable: bool,
    pub marginable: bool,
    #[serde(rename = "maintenance_margin_requirement")]
    pub maintenance_margin_requirement: i64,
    pub shortable: bool,
    #[serde(rename = "easy_to_borrow")]
    pub easy_to_borrow: bool,
    pub fractionable: bool,
}

#[derive(Serialize, Deserialize)]
struct OpenPostionRequest {
    pub symbol: String,
    pub notional: f32,
    pub side: String,
    #[serde(rename = "type")]
    pub market_type: String,
    pub time_in_force: String,
}

#[derive(Serialize, Deserialize)]
pub struct OpenPositionResponse {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "client_order_id")]
    client_order_id: String,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,

    #[serde(rename = "submitted_at")]
    submitted_at: String,

    #[serde(rename = "filled_at")]
    filled_at: Option<serde_json::Value>,

    #[serde(rename = "expired_at")]
    expired_at: Option<serde_json::Value>,

    #[serde(rename = "canceled_at")]
    canceled_at: Option<serde_json::Value>,

    #[serde(rename = "failed_at")]
    failed_at: Option<serde_json::Value>,

    #[serde(rename = "replaced_at")]
    replaced_at: Option<serde_json::Value>,

    #[serde(rename = "replaced_by")]
    replaced_by: Option<serde_json::Value>,

    #[serde(rename = "replaces")]
    replaces: Option<serde_json::Value>,

    #[serde(rename = "asset_id")]
    asset_id: String,

    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "asset_class")]
    asset_class: String,

    #[serde(rename = "notional")]
    notional: Option<serde_json::Value>,

    #[serde(rename = "qty")]
    qty: Option<serde_json::Value>,

    #[serde(rename = "filled_qty")]
    filled_qty: Option<serde_json::Value>,

    #[serde(rename = "filled_avg_price")]
    filled_avg_price: Option<serde_json::Value>,

    #[serde(rename = "order_class")]
    order_class: String,

    #[serde(rename = "order_type")]
    order_type: String,

    #[serde(rename = "type")]
    welcome7_type: String,

    #[serde(rename = "side")]
    side: String,

    #[serde(rename = "time_in_force")]
    time_in_force: String,

    #[serde(rename = "limit_price")]
    limit_price: Option<serde_json::Value>,

    #[serde(rename = "stop_price")]
    stop_price: Option<serde_json::Value>,

    #[serde(rename = "status")]
    status: String,

    #[serde(rename = "extended_hours")]
    extended_hours: bool,

    #[serde(rename = "legs")]
    legs: Option<serde_json::Value>,

    #[serde(rename = "trail_percent")]
    trail_percent: Option<serde_json::Value>,

    #[serde(rename = "trail_price")]
    trail_price: Option<serde_json::Value>,

    #[serde(rename = "hwm")]
    hwm: Option<serde_json::Value>,

    #[serde(rename = "subtag")]
    subtag: Option<serde_json::Value>,

    #[serde(rename = "source")]
    source: Option<serde_json::Value>,
}

pub struct Client {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    api_secret: String,
}

impl Client {
    // Trading API reference - https://alpaca.markets/docs/api-references/trading-api/

    pub fn init_client<'a>(base_url: &'a str, api_key: &'a str, api_secret: &'a str) -> Client {
        let client = reqwest::Client::new();

        Client {
            client: client,
            base_url: String::from(base_url),
            api_key: String::from(api_key),
            api_secret: String::from(api_secret),
        }
    }

    pub async fn check_balance(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _resp = self
            .client
            .get(format!("{}{}", self.base_url, "/account"))
            .headers(self.get_headers())
            .send()
            .await?
            .json::<AccountInfo>()
            .await?;

        if !_resp.account_blocked && !_resp.trading_blocked {
            println!("Good to trade. You have {} in balance", _resp.cash);
        }
        Ok(())
    }

    pub async fn open_position(
        &self,
        stock_symbol: &String,
        amount: &f32,
    ) -> Result<Option<OpenPositionResponse>, Box<dyn std::error::Error>> {
        /*
            Failures:
            403 - Forbidden
            Buying power or shares is not sufficient.
            422 - Unprocessable
            Input parameters are not recognized.
        */
        println!(
            "Attempting to purchase shares of {} worth {}",
            stock_symbol, amount
        );
        let request_body = OpenPostionRequest {
            symbol: stock_symbol.to_owned(),
            notional: amount.to_owned(),
            side: String::from("buy"),
            market_type: String::from("market"),
            time_in_force: String::from("day"),
        };

        let res = self
            .client
            .post(format!("{}{}", self.base_url, "/orders"))
            .headers(self.get_headers())
            .json(&request_body)
            .send()
            .await?;
        if !res.status().is_success() {
            println!("Something went wrong when attempting to purchase shares of {}", stock_symbol);
            return Ok(None);
        }
        println!("Successfully purchased {}", stock_symbol);
        Ok(Some(res.json::<OpenPositionResponse>().await?))
    }

    pub async fn close_position(
        &self,
        stock_symbol: &String,
        quantity: &f32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        /*
        Errors:
            404 Not found
            Position is not found.
        */
        let query_url = format!(
            "{}/{}/{}?qty={}",
            self.base_url, "positions", stock_symbol, quantity
        );

        let res = self
            .client
            .delete(query_url)
            .headers(self.get_headers())
            .send()
            .await?;
        if !res.status().is_success() {
            println!("Something went wrong. Status: {:?}", res.status());
            return Ok(false);
        }
        Ok(true)
    }

    pub async fn get_order(&self, order_id: &Uuid) -> Result<OpenPositionResponse, Box<dyn std::error::Error>> {
        let query_url = format!("{}/orders/{}",self.base_url, order_id);
        let res = self.client.get(query_url).headers(self.get_headers()).send().await?;
        if !res.status().is_success() {
            println!("Something went wrong. Status {:?}", res.status());
        }
        let response_data = res.json::<OpenPositionResponse>().await?;
        Ok(response_data)
    }

    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "APCA-API-KEY-ID",
            HeaderValue::from_str(&self.api_key).unwrap(), // TODO remove this and use match
        );
        headers.insert(
            "APCA-API-SECRET-KEY",
            HeaderValue::from_str(&self.api_secret).unwrap(), // TODO remove this and use match
        );
        headers
    }
}
