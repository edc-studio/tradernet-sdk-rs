use crate::user_data::Quote;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

/// Typed WebSocket event returned by [`crate::ws::TradernetWebsocket::quotes`].
#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum QuoteEvent {
    /// Quote update event (`"q"`).
    Quote(Quote),
    /// Raw error payload from WebSocket (`"error"`).
    Error(Value),
}

/// Typed WebSocket event returned by [`crate::ws::TradernetWebsocket::market_depth`].
#[derive(Debug, Serialize, Deserialize)]
pub enum MarketDepthEvent {
    /// Order book update event (`"b"`).
    Update(MarketDepthUpdate),
    /// Raw error payload from WebSocket (`"error"`).
    Error(Value),
}

/// Typed WebSocket event returned by [`crate::ws::TradernetWebsocket::portfolio`].
#[derive(Debug, Serialize, Deserialize)]
pub enum PortfolioEvent {
    /// Portfolio update event (`"portfolio"`).
    Portfolio(PortfolioUpdate),
    /// Raw error payload from WebSocket (`"error"`).
    Error(Value),
}

/// Typed WebSocket event returned by [`crate::ws::TradernetWebsocket::orders`].
#[derive(Debug, Serialize, Deserialize)]
pub enum OrdersEvent {
    /// Active orders update event (`"orders"`).
    Orders(Vec<OrderDataRow>),
    /// Raw error payload from WebSocket (`"error"`).
    Error(Value),
}

/// Typed WebSocket event returned by [`crate::ws::TradernetWebsocket::markets`].
#[derive(Debug, Serialize, Deserialize)]
pub enum MarketsEvent {
    /// Market statuses update event (`"markets"`).
    Markets(MarketsUpdate),
    /// Raw error payload from WebSocket (`"error"`).
    Error(Value),
}

/// Markets update payload from Tradernet WebSocket event `markets`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketsUpdate {
    /// Request processing timestamp on server.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub t: String,
    /// Market rows.
    #[serde(default, deserialize_with = "deserialize_markets_rows")]
    pub m: Vec<MarketInfoRow>,
}

/// Single market row in markets response (`m`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketInfoRow {
    /// Full market name.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub n: String,
    /// Short market name.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub n2: String,
    /// Current market status.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub s: String,
    /// Market open time (MSK).
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub o: String,
    /// Market close time (MSK).
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub c: String,
    /// Time offset relative to MSK, in minutes.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub dt: String,
}

/// Single order row from Tradernet WebSocket event `orders`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderDataRow {
    /// All-or-nothing marker (0/1).
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub aon: i64,
    /// Order currency.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub cur: String,
    /// Current executed quantity.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub curr_q: f64,
    /// Order date.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub date: String,
    /// Expiration type.
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub exp: i64,
    /// Instrument multiplier.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub fv: f64,
    /// Internal order identifier.
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub order_id: i64,
    /// Tradernet ticker.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub instr: String,
    /// Remaining quantity.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub leaves_qty: f64,
    /// Author login.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub auth_login: String,
    /// Creator login.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub creator_login: String,
    /// Owner login.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub owner_login: String,
    /// Market order identifier.
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub mkt_id: i64,
    /// Company name.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub name: String,
    /// Alternative company name.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub name2: String,
    /// Operation type.
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub oper: i64,
    /// Order price.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub p: f64,
    /// Order quantity.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub q: f64,
    /// Exchange-specific flag.
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub rep: i64,
    /// Current order status.
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub stat: i64,
    /// Status change date.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub stat_d: String,
    /// Original status.
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub stat_orig: i64,
    /// Previous status.
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub stat_prev: i64,
    /// Stop price.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub stop: f64,
    /// Stop activation marker.
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub stop_activated: i64,
    /// Initial stop activation price.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub stop_init_price: f64,
    /// Trailing order percent.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub trailing_price: f64,
    /// Order type.
    #[serde(rename = "type", default, deserialize_with = "deserialize_i64_lossy")]
    pub r#type: i64,
    /// User-specified order identifier.
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub user_order_id: i64,
    /// Trades linked to order.
    #[serde(default, deserialize_with = "deserialize_order_trades")]
    pub trade: Vec<OrderTradeInfo>,
}

/// Single trade row inside order (`trade`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderTradeInfo {
    /// Accrued coupon income.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub acd: f64,
    /// Trade date.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub date: String,
    /// Instrument multiplier.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub fv: f64,
    /// Guarantee amount in trade.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub go_sum: f64,
    /// Trade identifier.
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub id: i64,
    /// Execution price.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub p: f64,
    /// Trade profit.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub profit: f64,
    /// Trade quantity.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub q: f64,
    /// Trade volume/sum.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub v: f64,
}

/// Portfolio update payload from Tradernet WebSocket event `portfolio`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortfolioUpdate {
    /// Account key.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub key: String,
    /// Accounts information.
    #[serde(default, deserialize_with = "deserialize_portfolio_accounts")]
    pub acc: Vec<PortfolioAccountRow>,
    /// Open positions information.
    #[serde(default, deserialize_with = "deserialize_portfolio_positions")]
    pub pos: Vec<PortfolioPositionRow>,
}

/// Account row in portfolio response (`acc`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortfolioAccountRow {
    /// Account currency.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub curr: String,
    /// Account currency exchange rate.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub currval: f64,
    /// Forecast incoming value.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub forecast_in: f64,
    /// Forecast outgoing value.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub forecast_out: f64,
    /// Free funds.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub s: f64,
    /// T+2 incoming value.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub t2_in: f64,
    /// T+2 outgoing value.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub t2_out: f64,
}

/// Position row in portfolio response (`pos`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortfolioPositionRow {
    /// Unique open position identifier.
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub acc_pos_id: i64,
    /// Accrued coupon income.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub accruedint_a: f64,
    /// Position currency.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub curr: String,
    /// Currency exchange rate.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub currval: f64,
    /// Guarantee coefficient.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub fv: f64,
    /// Guarantee amount.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub go: f64,
    /// Position ticker.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub i: String,
    /// Exchange-specific marker.
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub k: i64,
    /// Position quantity.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub q: f64,
    /// Exchange-specific value.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub s: f64,
    /// Position type marker.
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub t: i64,
    /// T+2 incoming date/value.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub t2_in: String,
    /// T+2 outgoing date/value.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub t2_out: String,
    /// Variation margin.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub vm: f64,
    /// Company name.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub name: String,
    /// Alternative company name.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub name2: String,
    /// Current market price.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub mkt_price: f64,
    /// Asset market value.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub market_value: f64,
    /// Balance open price.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub bal_price_a: f64,
    /// Balance value of position.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub open_bal: f64,
    /// Open price.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub price_a: f64,
    /// Profit at previous close.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub profit_close: f64,
    /// Current profit.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub profit_price: f64,
    /// Previous close price.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub close_price: f64,
    /// Related trades list.
    #[serde(default, deserialize_with = "deserialize_portfolio_trades")]
    pub trade: Vec<PortfolioTradeRow>,
    /// Additional accrued data.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub acd: f64,
    /// Base currency for calculations.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub base_currency: String,
    /// Face value.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub face_val_a: f64,
    /// Calculation scheme.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub scheme_calc: String,
    /// Instrument identifier.
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub instr_id: i64,
    /// Instrument yield value.
    #[serde(rename = "Yield", default, deserialize_with = "deserialize_f64_lossy")]
    pub yield_value: f64,
    /// Instrument issue number.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub issue_nb: String,
}

/// Trade row in portfolio position (`trade`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortfolioTradeRow {
    /// Number of trades.
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub trade_count: i64,
}

/// Order book update payload from Tradernet WebSocket event `b`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketDepthUpdate {
    /// Sequence/event number (if provided by server).
    #[serde(default, deserialize_with = "deserialize_option_i64_lossy")]
    pub n: Option<i64>,
    /// Instrument ticker.
    #[serde(default, deserialize_with = "deserialize_string_lossy")]
    pub i: String,
    /// Current order book depth.
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub cnt: i64,
    /// Inserted rows.
    #[serde(default, deserialize_with = "deserialize_market_depth_rows")]
    pub ins: Vec<MarketDepthRow>,
    /// Deleted rows.
    #[serde(default, deserialize_with = "deserialize_market_depth_rows")]
    pub del: Vec<MarketDepthRow>,
    /// Updated rows.
    #[serde(default, deserialize_with = "deserialize_market_depth_rows")]
    pub upd: Vec<MarketDepthRow>,
    /// Extra exchange-specific field (if present).
    #[serde(default, deserialize_with = "deserialize_option_i64_lossy")]
    pub x: Option<i64>,
}

/// Single row within market depth update.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketDepthRow {
    /// Position number in order book.
    #[serde(default, deserialize_with = "deserialize_i64_lossy")]
    pub k: i64,
    /// Price for this row.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub p: f64,
    /// Quantity for this row.
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub q: f64,
    /// Side (`B` = buy, `S` = sell).
    #[serde(default, deserialize_with = "deserialize_market_depth_side")]
    pub s: MarketDepthSide,
}

/// Order book side marker in market depth row.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum MarketDepthSide {
    #[serde(rename = "S")]
    Sell,
    #[serde(rename = "B")]
    Buy,
    #[default]
    Unknown,
}

fn deserialize_market_depth_rows<'de, D>(deserializer: D) -> Result<Vec<MarketDepthRow>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    let mut rows = Vec::new();
    match value {
        None | Some(Value::Null) => {}
        Some(Value::Array(items)) => {
            for item in items {
                match serde_json::from_value::<MarketDepthRow>(item) {
                    Ok(row) => rows.push(row),
                    Err(err) => {
                        log::warn!("market depth row parse failed: {err}");
                    }
                }
            }
        }
        Some(Value::Object(map)) => {
            match serde_json::from_value::<MarketDepthRow>(Value::Object(map)) {
                Ok(row) => rows.push(row),
                Err(err) => {
                    log::warn!("market depth row parse failed: {err}");
                }
            }
        }
        Some(other) => {
            log::warn!("market depth rows expected array/object, got: {other}");
        }
    }
    Ok(rows)
}

fn deserialize_portfolio_accounts<'de, D>(
    deserializer: D,
) -> Result<Vec<PortfolioAccountRow>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    let mut rows = Vec::new();
    match value {
        None | Some(Value::Null) => {}
        Some(Value::Array(items)) => {
            for item in items {
                match serde_json::from_value::<PortfolioAccountRow>(item) {
                    Ok(row) => rows.push(row),
                    Err(err) => {
                        log::warn!("portfolio account row parse failed: {err}");
                    }
                }
            }
        }
        Some(Value::Object(map)) => {
            match serde_json::from_value::<PortfolioAccountRow>(Value::Object(map)) {
                Ok(row) => rows.push(row),
                Err(err) => {
                    log::warn!("portfolio account row parse failed: {err}");
                }
            }
        }
        Some(other) => {
            log::warn!("portfolio account rows expected array/object, got: {other}");
        }
    }
    Ok(rows)
}

fn deserialize_portfolio_positions<'de, D>(
    deserializer: D,
) -> Result<Vec<PortfolioPositionRow>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    let mut rows = Vec::new();
    match value {
        None | Some(Value::Null) => {}
        Some(Value::Array(items)) => {
            for item in items {
                match serde_json::from_value::<PortfolioPositionRow>(item) {
                    Ok(row) => rows.push(row),
                    Err(err) => {
                        log::warn!("portfolio position row parse failed: {err}");
                    }
                }
            }
        }
        Some(Value::Object(map)) => {
            match serde_json::from_value::<PortfolioPositionRow>(Value::Object(map)) {
                Ok(row) => rows.push(row),
                Err(err) => {
                    log::warn!("portfolio position row parse failed: {err}");
                }
            }
        }
        Some(other) => {
            log::warn!("portfolio position rows expected array/object, got: {other}");
        }
    }
    Ok(rows)
}

fn deserialize_portfolio_trades<'de, D>(deserializer: D) -> Result<Vec<PortfolioTradeRow>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    let mut rows = Vec::new();
    match value {
        None | Some(Value::Null) => {}
        Some(Value::Array(items)) => {
            for item in items {
                match serde_json::from_value::<PortfolioTradeRow>(item) {
                    Ok(row) => rows.push(row),
                    Err(err) => {
                        log::warn!("portfolio trade row parse failed: {err}");
                    }
                }
            }
        }
        Some(Value::Object(map)) => {
            match serde_json::from_value::<PortfolioTradeRow>(Value::Object(map)) {
                Ok(row) => rows.push(row),
                Err(err) => {
                    log::warn!("portfolio trade row parse failed: {err}");
                }
            }
        }
        Some(other) => {
            log::warn!("portfolio trade rows expected array/object, got: {other}");
        }
    }
    Ok(rows)
}

fn deserialize_order_trades<'de, D>(deserializer: D) -> Result<Vec<OrderTradeInfo>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    let mut rows = Vec::new();
    match value {
        None | Some(Value::Null) => {}
        Some(Value::Array(items)) => {
            for item in items {
                match serde_json::from_value::<OrderTradeInfo>(item) {
                    Ok(row) => rows.push(row),
                    Err(err) => {
                        log::warn!("order trade row parse failed: {err}");
                    }
                }
            }
        }
        Some(Value::Object(map)) => {
            match serde_json::from_value::<OrderTradeInfo>(Value::Object(map)) {
                Ok(row) => rows.push(row),
                Err(err) => {
                    log::warn!("order trade row parse failed: {err}");
                }
            }
        }
        Some(other) => {
            log::warn!("order trade rows expected array/object, got: {other}");
        }
    }
    Ok(rows)
}

fn deserialize_markets_rows<'de, D>(deserializer: D) -> Result<Vec<MarketInfoRow>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    let mut rows = Vec::new();
    match value {
        None | Some(Value::Null) => {}
        Some(Value::Array(items)) => {
            for item in items {
                match serde_json::from_value::<MarketInfoRow>(item) {
                    Ok(row) => rows.push(row),
                    Err(err) => {
                        log::warn!("market info row parse failed: {err}");
                    }
                }
            }
        }
        Some(Value::Object(map)) => {
            match serde_json::from_value::<MarketInfoRow>(Value::Object(map)) {
                Ok(row) => rows.push(row),
                Err(err) => {
                    log::warn!("market info row parse failed: {err}");
                }
            }
        }
        Some(other) => {
            log::warn!("market info rows expected array/object, got: {other}");
        }
    }
    Ok(rows)
}

fn deserialize_market_depth_side<'de, D>(deserializer: D) -> Result<MarketDepthSide, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(match value {
        None | Some(Value::Null) => MarketDepthSide::Unknown,
        Some(Value::String(side)) => match side.trim().to_ascii_uppercase().as_str() {
            "B" | "BUY" => MarketDepthSide::Buy,
            "S" | "SELL" => MarketDepthSide::Sell,
            unknown => {
                log::warn!("unknown market depth side `{unknown}`");
                MarketDepthSide::Unknown
            }
        },
        Some(Value::Number(number)) => match number.as_i64().unwrap_or_default() {
            value if value > 0 => MarketDepthSide::Buy,
            value if value < 0 => MarketDepthSide::Sell,
            _ => MarketDepthSide::Unknown,
        },
        Some(Value::Bool(true)) => MarketDepthSide::Buy,
        Some(Value::Bool(false)) => MarketDepthSide::Sell,
        Some(other) => {
            log::warn!("market depth side expected string/number/bool, got: {other}");
            MarketDepthSide::Unknown
        }
    })
}

fn deserialize_string_lossy<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(match value {
        None | Some(Value::Null) => String::new(),
        Some(Value::String(value)) => value,
        Some(Value::Number(value)) => value.to_string(),
        Some(Value::Bool(value)) => value.to_string(),
        Some(other) => {
            log::warn!("string field expected string/number/bool, got: {other}");
            String::new()
        }
    })
}

fn deserialize_i64_lossy<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(match value {
        None | Some(Value::Null) => 0,
        Some(Value::Number(value)) => value
            .as_i64()
            .or_else(|| value.as_f64().map(|value| value.trunc() as i64))
            .unwrap_or(0),
        Some(Value::String(value)) => {
            let value = value.trim();
            if value.is_empty() {
                0
            } else if let Ok(parsed) = value.parse::<i64>() {
                parsed
            } else {
                value
                    .parse::<f64>()
                    .map(|value| value.trunc() as i64)
                    .unwrap_or(0)
            }
        }
        Some(Value::Bool(value)) => i64::from(value),
        Some(other) => {
            log::warn!("i64 field expected number/string/bool/null, got: {other}");
            0
        }
    })
}

fn deserialize_option_i64_lossy<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(match value {
        None | Some(Value::Null) => None,
        Some(Value::Number(value)) => value
            .as_i64()
            .or_else(|| value.as_f64().map(|value| value.trunc() as i64)),
        Some(Value::String(value)) => {
            let value = value.trim();
            if value.is_empty() {
                None
            } else if let Ok(parsed) = value.parse::<i64>() {
                Some(parsed)
            } else {
                value.parse::<f64>().ok().map(|value| value.trunc() as i64)
            }
        }
        Some(Value::Bool(value)) => Some(i64::from(value)),
        Some(other) => {
            log::warn!("optional i64 field expected number/string/bool/null, got: {other}");
            None
        }
    })
}

fn deserialize_f64_lossy<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(match value {
        None | Some(Value::Null) => 0.0,
        Some(Value::Number(value)) => value.as_f64().unwrap_or(0.0),
        Some(Value::String(value)) => value.trim().parse::<f64>().unwrap_or(0.0),
        Some(Value::Bool(value)) => {
            if value {
                1.0
            } else {
                0.0
            }
        }
        Some(other) => {
            log::warn!("f64 field expected number/string/bool/null, got: {other}");
            0.0
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{MarketDepthSide, MarketDepthUpdate, MarketsUpdate, OrderDataRow, PortfolioUpdate};
    use serde_json::json;

    #[test]
    fn market_depth_update_is_lossy_and_does_not_fail_on_bad_types() {
        let payload = json!({
            "n": "102.9",
            "i": 42,
            "cnt": true,
            "ins": null,
            "del": {"k": "3.9", "p": "33.925", "q": "196100", "s": "S"},
            "upd": [
                {"p": 33.925, "s": "S", "q": 196100, "k": 3},
                {"p": "bad", "s": "B", "q": false, "k": "6"},
                {"p": 1, "s": "unexpected", "q": 2, "k": 7}
            ],
            "x": {}
        });

        let update: MarketDepthUpdate =
            serde_json::from_value(payload).expect("lossy parser should not fail");

        assert_eq!(update.n, Some(102));
        assert_eq!(update.i, "42");
        assert_eq!(update.cnt, 1);
        assert!(update.ins.is_empty());
        assert_eq!(update.del.len(), 1);
        assert_eq!(update.del[0].k, 3);
        assert_eq!(update.del[0].p, 33.925);
        assert_eq!(update.del[0].q, 196100.0);
        assert!(matches!(update.del[0].s, MarketDepthSide::Sell));
        assert_eq!(update.upd.len(), 3);
        assert_eq!(update.upd[1].p, 0.0);
        assert_eq!(update.upd[1].q, 0.0);
        assert!(matches!(update.upd[1].s, MarketDepthSide::Buy));
        assert!(matches!(update.upd[2].s, MarketDepthSide::Unknown));
        assert_eq!(update.x, None);
    }

    #[test]
    fn portfolio_update_is_lossy_and_does_not_fail_on_bad_types() {
        let payload = json!({
            "key": 123,
            "acc": [
                {
                    "s": ".00000000",
                    "forecast_in": ".00000000",
                    "forecast_out": true,
                    "curr": "USD",
                    "currval": "78.95",
                    "t2_in": null,
                    "t2_out": "1.1"
                },
                "invalid"
            ],
            "pos": [
                {
                    "i": "AAPL.US",
                    "t": 1,
                    "k": "1.9",
                    "s": 22.4,
                    "q": "100",
                    "fv": "100",
                    "curr": "USD",
                    "currval": 1,
                    "name": "Apple Inc.",
                    "name2": "Apple Inc.",
                    "open_bal": 22.4,
                    "mkt_price": "23.81",
                    "vm": ".00000000",
                    "go": ".00000000",
                    "profit_close": -2.4,
                    "acc_pos_id": 85628162,
                    "accruedint_a": ".00000000",
                    "acd": ".00000000",
                    "bal_price_a": 29.924,
                    "price_a": 29.924,
                    "base_currency": "USD",
                    "face_val_a": 3,
                    "scheme_calc": "T2",
                    "instr_id": "10000007229",
                    "Yield": ".00000000",
                    "issue_nb": "US000902000001",
                    "profit_price": 2.83,
                    "market_value": 2020,
                    "close_price": "2.83",
                    "trade": [{"trade_count": "3.8"}, {"trade_count": false}, "bad"]
                },
                {}
            ]
        });

        let update: PortfolioUpdate =
            serde_json::from_value(payload).expect("lossy parser should not fail");

        assert_eq!(update.key, "123");
        assert_eq!(update.acc.len(), 1);
        assert_eq!(update.acc[0].curr, "USD");
        assert_eq!(update.acc[0].currval, 78.95);
        assert_eq!(update.acc[0].forecast_out, 1.0);
        assert_eq!(update.pos.len(), 2);
        assert_eq!(update.pos[0].i, "AAPL.US");
        assert_eq!(update.pos[0].k, 1);
        assert_eq!(update.pos[0].q, 100.0);
        assert_eq!(update.pos[0].instr_id, 10000007229);
        assert_eq!(update.pos[0].trade.len(), 2);
        assert_eq!(update.pos[0].trade[0].trade_count, 3);
        assert_eq!(update.pos[0].trade[1].trade_count, 0);
    }

    #[test]
    fn order_data_row_is_lossy_and_does_not_fail_on_bad_types() {
        let payload: serde_json::Value = serde_json::from_str(
            r#"{
            "aon": "1",
            "cur": 840,
            "curr_q": true,
            "date": null,
            "exp": "2",
            "fv": "100",
            "order_id": "8757875",
            "instr": "FCX.US",
            "leaves_qty": "12.5",
            "auth_login": true,
            "creator_login": "virtual@virtual.com",
            "owner_login": "virtual@virtual.com",
            "mkt_id": "30000000001",
            "name": "Freeport-McMoran Cp & Gld",
            "name2": "Freeport-McMoran Cp & Gld",
            "oper": "2",
            "p": "6.5611",
            "q": "2625",
            "rep": false,
            "stat": "21",
            "stat_d": 123,
            "stat_orig": "21",
            "stat_prev": "10",
            "stop": "0",
            "stop_activated": "1",
            "stop_init_price": "6.36",
            "trailing_price": null,
            "type": "1",
            "user_order_id": "1450879514204",
            "trade": [
                {
                    "acd": ".00000000",
                    "date": "2015-12-23T17:05:03",
                    "fv": "100",
                    "go_sum": false,
                    "id": "13446624",
                    "p": "6.37",
                    "profit": null,
                    "q": "2625",
                    "v": "16721.25"
                },
                "bad",
                {
                    "id": false
                }
            ]
        }"#,
        )
        .expect("json payload must be valid");

        let row: OrderDataRow =
            serde_json::from_value(payload).expect("lossy parser should not fail");

        assert_eq!(row.aon, 1);
        assert_eq!(row.cur, "840");
        assert_eq!(row.curr_q, 1.0);
        assert_eq!(row.exp, 2);
        assert_eq!(row.order_id, 8757875);
        assert_eq!(row.leaves_qty, 12.5);
        assert_eq!(row.mkt_id, 30000000001);
        assert_eq!(row.stat, 21);
        assert_eq!(row.stat_d, "123");
        assert_eq!(row.stop_activated, 1);
        assert_eq!(row.r#type, 1);
        assert_eq!(row.user_order_id, 1450879514204);
        assert_eq!(row.trade.len(), 2);
        assert_eq!(row.trade[0].id, 13446624);
        assert_eq!(row.trade[0].go_sum, 0.0);
        assert_eq!(row.trade[0].v, 16721.25);
        assert_eq!(row.trade[1].id, 0);
    }

    #[test]
    fn markets_update_is_lossy_and_does_not_fail_on_bad_types() {
        let payload = json!({
            "t": 123,
            "m": [
                {
                    "n": "KASE",
                    "n2": "KASE",
                    "s": "CLOSE",
                    "o": "08:20:00",
                    "c": "14:00:00",
                    "dt": -180
                },
                "invalid",
                {
                    "n": null,
                    "n2": true
                }
            ]
        });

        let update: MarketsUpdate =
            serde_json::from_value(payload).expect("lossy parser should not fail");

        assert_eq!(update.t, "123");
        assert_eq!(update.m.len(), 2);
        assert_eq!(update.m[0].n, "KASE");
        assert_eq!(update.m[0].c, "14:00:00");
        assert_eq!(update.m[0].dt, "-180");
        assert_eq!(update.m[1].n, "");
        assert_eq!(update.m[1].n2, "true");
    }
}
