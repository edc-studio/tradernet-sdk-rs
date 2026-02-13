use crate::core::Core;
use crate::errors::TradernetError;
use chrono::{DateTime, Local, NaiveDateTime};
use serde_json::{Map, Value};

pub struct Tradernet {
    core: Core,
}

impl Tradernet {
    pub fn new(public: Option<String>, private: Option<String>) -> Result<Self, TradernetError> {
        Ok(Self {
            core: Core::new(public, private)?,
        })
    }

    pub fn from_config(path: impl AsRef<std::path::Path>) -> Result<Self, TradernetError> {
        Ok(Self {
            core: Core::from_config(path)?,
        })
    }

    pub fn core(&self) -> &Core {
        &self.core
    }

    pub fn user_info(&self) -> Result<Value, TradernetError> {
        self.core.authorized_request("GetAllUserTexInfo", None, Some(2))
    }

    pub fn get_user_data(&self) -> Result<Value, TradernetError> {
        self.core.authorized_request("getOPQ", None, Some(2))
    }

    pub fn get_market_status(&self, market: &str, mode: Option<&str>) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("market".to_string(), Value::String(market.to_string()));
        if let Some(mode) = mode {
            params.insert("mode".to_string(), Value::String(mode.to_string()));
        }
        self.core.authorized_request("getMarketStatus", Some(params), Some(2))
    }

    pub fn security_info(&self, symbol: &str, sup: bool) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("ticker".to_string(), Value::String(symbol.to_string()));
        params.insert("sup".to_string(), Value::Bool(sup));
        self.core.authorized_request("getSecurityInfo", Some(params), Some(2))
    }

    pub fn get_options(&self, underlying: &str, exchange: &str) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("underlying".to_string(), Value::String(underlying.to_string()));
        params.insert("mkt".to_string(), Value::String(exchange.to_string()));
        self.core.authorized_request("getOptionsByMkt", Some(params), Some(2))
    }

    pub fn get_candles(
        &self,
        symbol: &str,
        start: NaiveDateTime,
        end: NaiveDateTime,
        timeframe_seconds: i64,
    ) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("id".to_string(), Value::String(symbol.to_string()));
        params.insert("count".to_string(), Value::Number((-1).into()));
        params.insert(
            "timeframe".to_string(),
            Value::Number(((timeframe_seconds / 60) as i64).into()),
        );
        params.insert(
            "date_from".to_string(),
            Value::String(start.format("%d.%m.%Y %H:%M").to_string()),
        );
        params.insert(
            "date_to".to_string(),
            Value::String(end.format("%d.%m.%Y %H:%M").to_string()),
        );
        params.insert("intervalMode".to_string(), Value::String("OpenRay".to_string()));

        self.core.authorized_request("getHloc", Some(params), Some(2))
    }

    pub fn get_quotes<I, S>(&self, symbols: I) -> Result<Value, TradernetError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = symbols
            .into_iter()
            .map(|symbol| symbol.as_ref().to_string())
            .collect::<Vec<String>>()
            .join(",");
        let mut params = Map::new();
        params.insert("tickers".to_string(), Value::String(joined));
        self.core.authorized_request("getStockQuotesJson", Some(params), Some(2))
    }

    pub fn buy(
        &self,
        symbol: &str,
        quantity: i64,
        price: f64,
        duration: &str,
        use_margin: bool,
        custom_order_id: Option<i64>,
    ) -> Result<Value, TradernetError> {
        if quantity <= 0 {
            return Err(TradernetError::InvalidInput("Quantity must be positive".to_string()));
        }

        self.trade(symbol, quantity, price, duration, use_margin, custom_order_id)
    }

    pub fn sell(
        &self,
        symbol: &str,
        quantity: i64,
        price: f64,
        duration: &str,
        use_margin: bool,
        custom_order_id: Option<i64>,
    ) -> Result<Value, TradernetError> {
        if quantity <= 0 {
            return Err(TradernetError::InvalidInput("Quantity must be positive".to_string()));
        }

        self.trade(symbol, -quantity, price, duration, use_margin, custom_order_id)
    }

    pub fn cancel(&self, order_id: i64) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("order_id".to_string(), Value::Number(order_id.into()));
        self.core.authorized_request("delTradeOrder", Some(params), Some(2))
    }

    pub fn get_placed(&self, active: bool) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("active_only".to_string(), Value::Number((active as i64).into()));
        self.core.authorized_request("getNotifyOrderJson", Some(params), Some(2))
    }

    pub fn get_historical(&self, start: DateTime<Local>, end: DateTime<Local>) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert(
            "from".to_string(),
            Value::String(start.format("%Y-%m-%dT%H:%M:%S").to_string()),
        );
        params.insert(
            "till".to_string(),
            Value::String(end.format("%Y-%m-%dT%H:%M:%S").to_string()),
        );
        self.core.authorized_request("getOrdersHistory", Some(params), Some(2))
    }

    pub fn trade(
        &self,
        symbol: &str,
        quantity: i64,
        price: f64,
        duration: &str,
        use_margin: bool,
        custom_order_id: Option<i64>,
    ) -> Result<Value, TradernetError> {
        if duration.eq_ignore_ascii_case("ioc") {
            let order = self.trade(symbol, quantity, price, "day", use_margin, custom_order_id)?;
            if let Some(order_id) = order.get("order_id").and_then(|value| value.as_i64()) {
                let _ = self.cancel(order_id);
            }
            return Ok(order);
        }

        if quantity == 0 {
            return Err(TradernetError::InvalidInput("Zero quantity".to_string()));
        }

        let duration_code = duration_code(duration)
            .ok_or_else(|| TradernetError::InvalidInput(format!("Unknown duration {duration}")))?;

        let action_id = if quantity > 0 {
            if use_margin { 2 } else { 1 }
        } else if use_margin {
            4
        } else {
            3
        };

        let mut params = Map::new();
        params.insert("instr_name".to_string(), Value::String(symbol.to_string()));
        params.insert("action_id".to_string(), Value::Number(action_id.into()));
        params.insert("order_type_id".to_string(), Value::Number((if price != 0.0 { 2 } else { 1 }).into()));
        params.insert("qty".to_string(), Value::Number(quantity.unsigned_abs().into()));
        let limit_price = serde_json::Number::from_f64(price)
            .ok_or_else(|| TradernetError::InvalidInput("Invalid price".to_string()))?;
        params.insert("limit_price".to_string(), Value::Number(limit_price));
        params.insert("expiration_id".to_string(), Value::Number(duration_code.into()));
        if let Some(custom_order_id) = custom_order_id {
            params.insert("user_order_id".to_string(), Value::Number(custom_order_id.into()));
        }

        self.core.authorized_request("putTradeOrder", Some(params), Some(2))
    }
}

fn duration_code(duration: &str) -> Option<i64> {
    match duration.to_ascii_lowercase().as_str() {
        "day" => Some(1),
        "ext" => Some(2),
        "gtc" => Some(3),
        _ => None,
    }
}