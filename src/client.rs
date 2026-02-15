use crate::core::Core;
use crate::errors::TradernetError;
use crate::user_data::UserDataResponse;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
use regex::Regex;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::io::Read;
use zip::ZipArchive;

/// High-level REST client for the Tradernet API.
pub struct Tradernet {
    core: Core,
}

impl Tradernet {
    /// Creates a REST client with optional API keys.
    #[allow(clippy::result_large_err)]
    pub fn new(public: Option<String>, private: Option<String>) -> Result<Self, TradernetError> {
        Ok(Self {
            core: Core::new(public, private)?,
        })
    }

    /// Creates a REST client from an INI config file with credentials.
    #[allow(clippy::result_large_err)]
    pub fn from_config(path: impl AsRef<std::path::Path>) -> Result<Self, TradernetError> {
        Ok(Self {
            core: Core::from_config(path)?,
        })
    }

    /// Returns a reference to the underlying [`Core`].
    pub fn core(&self) -> &Core {
        &self.core
    }

    /// Returns authenticated user information.
    pub fn user_info(&self) -> Result<Value, TradernetError> {
        self.core
            .authorized_request("GetAllUserTexInfo", None, Some(2))
    }

    /// Registers a new user.
    #[allow(clippy::too_many_arguments)]
    pub fn new_user(
        &self,
        login: &str,
        reception: impl ToString,
        phone: &str,
        lastname: &str,
        firstname: &str,
        password: Option<&str>,
        utm_campaign: Option<&str>,
        tariff: Option<i64>,
    ) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("login".to_string(), Value::String(login.to_string()));
        params.insert(
            "pwd".to_string(),
            Value::String(password.unwrap_or_default().to_string()),
        );
        params.insert(
            "reception".to_string(),
            Value::String(reception.to_string()),
        );
        params.insert("phone".to_string(), Value::String(phone.to_string()));
        params.insert("lastname".to_string(), Value::String(lastname.to_string()));
        params.insert(
            "firstname".to_string(),
            Value::String(firstname.to_string()),
        );
        if let Some(tariff) = tariff {
            params.insert("tariff_id".to_string(), Value::Number(tariff.into()));
        }
        if let Some(utm_campaign) = utm_campaign {
            params.insert(
                "utm_campaign".to_string(),
                Value::String(utm_campaign.to_string()),
            );
        }

        self.core.plain_request("registerNewUser", Some(params))
    }

    /// Checks missing onboarding fields for the given step and office.
    pub fn check_missing_fields(&self, step: i64, office: &str) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("step".to_string(), Value::Number(step.into()));
        params.insert("office".to_string(), Value::String(office.to_string()));
        self.core
            .authorized_request("checkStep", Some(params), Some(2))
    }

    /// Returns profile fields for a given reception.
    pub fn get_profile_fields(&self, reception: i64) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("reception".to_string(), Value::Number(reception.into()));
        self.core
            .authorized_request("getProfileFields", Some(params), Some(2))
    }

    /// Returns user data (OPQ) for the authenticated account.
    pub fn get_user_data(&self) -> Result<UserDataResponse, TradernetError> {
        let response = self.core.authorized_request("getOPQ", None, Some(2))?;
        let response_text = response.to_string();
        let mut deserializer = serde_json::Deserializer::from_str(&response_text);
        let data = serde_path_to_error::deserialize(&mut deserializer).map_err(|error| {
            TradernetError::JsonPath {
                path: error.path().to_string(),
                source: Box::new(error.into_inner()),
            }
        })?;
        Ok(data)
    }

    /// Returns market status for a given market code.
    pub fn get_market_status(
        &self,
        market: &str,
        mode: Option<&str>,
    ) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("market".to_string(), Value::String(market.to_string()));
        if let Some(mode) = mode {
            params.insert("mode".to_string(), Value::String(mode.to_string()));
        }
        self.core
            .authorized_request("getMarketStatus", Some(params), Some(2))
    }

    /// Returns the most traded securities for a given exchange/type.
    pub fn get_most_traded(
        &self,
        instrument_type: &str,
        exchange: &str,
        gainers: bool,
        limit: i64,
    ) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert(
            "type".to_string(),
            Value::String(instrument_type.to_string()),
        );
        params.insert("exchange".to_string(), Value::String(exchange.to_string()));
        params.insert(
            "gainers".to_string(),
            Value::Number((gainers as i64).into()),
        );
        params.insert("limit".to_string(), Value::Number(limit.into()));
        self.core.plain_request("getTopSecurities", Some(params))
    }

    /// Exports securities data for a list of symbols.
    pub fn export_securities<I, S>(
        &self,
        symbols: I,
        fields: Option<&[&str]>,
    ) -> Result<Value, TradernetError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let symbols = symbols
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<_>>();
        let mut results = Vec::new();

        for chunk in symbols.chunks(Core::MAX_EXPORT_SIZE) {
            let mut params = Vec::new();
            if let Some(fields) = fields {
                params.push(("params".to_string(), fields.join(" ")));
            }
            params.push(("tickers".to_string(), chunk.join(" ")));

            let response = self.core.get_request("/securities/export", Some(&params))?;
            let mut result: Vec<Value> = response.json()?;
            results.append(&mut result);
        }

        Ok(Value::Array(results))
    }

    /// Returns security information for the given ticker.
    pub fn security_info(&self, symbol: &str, sup: bool) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("ticker".to_string(), Value::String(symbol.to_string()));
        params.insert("sup".to_string(), Value::Bool(sup));
        self.core
            .authorized_request("getSecurityInfo", Some(params), Some(2))
    }

    /// Returns options chain data for an underlying on a given exchange.
    pub fn get_options(&self, underlying: &str, exchange: &str) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert(
            "underlying".to_string(),
            Value::String(underlying.to_string()),
        );
        params.insert("mkt".to_string(), Value::String(exchange.to_string()));
        self.core
            .authorized_request("getOptionsByMkt", Some(params), Some(2))
    }

    /// Returns candle data for a symbol and time range.
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
            Value::Number((timeframe_seconds / 60).into()),
        );
        params.insert(
            "date_from".to_string(),
            Value::String(start.format("%d.%m.%Y %H:%M").to_string()),
        );
        params.insert(
            "date_to".to_string(),
            Value::String(end.format("%d.%m.%Y %H:%M").to_string()),
        );
        params.insert(
            "intervalMode".to_string(),
            Value::String("OpenRay".to_string()),
        );

        self.core
            .authorized_request("getHloc", Some(params), Some(2))
    }

    /// Returns quote data for a list of symbols.
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
        self.core
            .authorized_request("getStockQuotesJson", Some(params), Some(2))
    }

    /// Returns trades history for a given date range.
    pub fn get_trades_history(
        &self,
        start: NaiveDate,
        end: NaiveDate,
        trade_id: Option<i64>,
        limit: Option<i64>,
        symbol: Option<&str>,
        currency: Option<&str>,
    ) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("beginDate".to_string(), Value::String(start.to_string()));
        params.insert("endDate".to_string(), Value::String(end.to_string()));
        if let Some(trade_id) = trade_id {
            params.insert("tradeId".to_string(), Value::Number(trade_id.into()));
        }
        if let Some(limit) = limit {
            params.insert("max".to_string(), Value::Number(limit.into()));
        }
        if let Some(symbol) = symbol {
            params.insert("nt_ticker".to_string(), Value::String(symbol.to_string()));
        }
        if let Some(currency) = currency {
            params.insert("curr".to_string(), Value::String(currency.to_string()));
        }
        self.core
            .authorized_request("getTradesHistory", Some(params), Some(2))
    }

    /// Searches for symbols by text (optionally within a specific exchange).
    pub fn find_symbol(
        &self,
        symbol: &str,
        exchange: Option<&str>,
    ) -> Result<Value, TradernetError> {
        let text = match exchange {
            Some(exchange) => format!("{symbol}@{exchange}"),
            None => symbol.to_string(),
        };
        let mut params = Map::new();
        params.insert("text".to_string(), Value::String(text));
        self.core.plain_request("tickerFinder", Some(params))
    }

    /// Retrieves news items for a query and optional filters.
    pub fn get_news(
        &self,
        query: &str,
        symbol: Option<&str>,
        story_id: Option<&str>,
        limit: i64,
    ) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("searchFor".to_string(), Value::String(query.to_string()));
        if let Some(symbol) = symbol {
            params.insert("ticker".to_string(), Value::String(symbol.to_string()));
        }
        if let Some(story_id) = story_id {
            params.insert("storyId".to_string(), Value::String(story_id.to_string()));
        }
        params.insert("limit".to_string(), Value::Number(limit.into()));
        self.core
            .authorized_request("getNews", Some(params), Some(2))
    }

    /// Returns a filtered list of securities from refbooks.
    pub fn get_all(
        &self,
        mut filters: Option<HashMap<String, Value>>,
        show_expired: bool,
    ) -> Result<Value, TradernetError> {
        let mut filters = filters.take().unwrap_or_default();
        if !show_expired {
            filters.insert("istrade".to_string(), Value::Number(1.into()));
        }

        let market_code = filters
            .get("mkt_short_code")
            .and_then(|value| value.as_str())
            .map(|value| value.to_string());

        let refbook = self.get_refbook(market_code.as_deref())?;
        let filtered = refbook
            .into_iter()
            .filter(|symbol: &Map<String, Value>| {
                filters
                    .iter()
                    .all(|(field, expected)| symbol.get(field) == Some(expected))
            })
            .map(Value::Object)
            .collect::<Vec<Value>>();

        Ok(Value::Array(filtered))
    }

    /// Returns a summary of account positions.
    pub fn account_summary(&self) -> Result<Value, TradernetError> {
        self.core
            .authorized_request("getPositionJson", None, Some(2))
    }

    /// Returns price alerts for all symbols or a specific ticker.
    pub fn get_price_alerts(&self, symbol: Option<&str>) -> Result<Value, TradernetError> {
        let params = symbol.map(|symbol| {
            let mut params = Map::new();
            params.insert("ticker".to_string(), Value::String(symbol.to_string()));
            params
        });
        self.core
            .authorized_request("getAlertsList", params, Some(2))
    }

    /// Creates a price alert for a symbol.
    #[allow(clippy::too_many_arguments)]
    pub fn add_price_alert<I, S>(
        &self,
        symbol: &str,
        price: I,
        trigger_type: &str,
        quote_type: &str,
        send_to: &str,
        frequency: i64,
        expire: i64,
    ) -> Result<Value, TradernetError>
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        let prices = price
            .into_iter()
            .map(|value| Value::String(value.to_string()))
            .collect::<Vec<_>>();
        let mut params = Map::new();
        params.insert("ticker".to_string(), Value::String(symbol.to_string()));
        params.insert("price".to_string(), Value::Array(prices));
        params.insert(
            "trigger_type".to_string(),
            Value::String(trigger_type.to_string()),
        );
        params.insert(
            "quote_type".to_string(),
            Value::String(quote_type.to_string()),
        );
        params.insert(
            "notification_type".to_string(),
            Value::String(send_to.to_string()),
        );
        params.insert("alert_period".to_string(), Value::Number(frequency.into()));
        params.insert("expire".to_string(), Value::Number(expire.into()));
        self.core
            .authorized_request("addPriceAlert", Some(params), Some(2))
    }

    /// Deletes a price alert by identifier.
    #[allow(clippy::result_large_err)]
    pub fn delete_price_alert(&self, alert_id: i64) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("id".to_string(), Value::Number(alert_id.into()));
        params.insert("del".to_string(), Value::Bool(true));
        self.core
            .authorized_request("addPriceAlert", Some(params), Some(2))
    }

    /// Returns history for client requests (CPS history).
    #[allow(clippy::too_many_arguments)]
    pub fn get_requests_history(
        &self,
        doc_id: Option<i64>,
        exec_id: Option<i64>,
        start: DateTime<Local>,
        end: DateTime<Local>,
        limit: Option<i64>,
        offset: Option<i64>,
        status: Option<i64>,
    ) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert(
            "date_from".to_string(),
            Value::String(start.format("%Y-%m-%dT%H:%M:%S").to_string()),
        );
        params.insert(
            "date_to".to_string(),
            Value::String(end.format("%Y-%m-%dT%H:%M:%S").to_string()),
        );
        if let Some(doc_id) = doc_id {
            params.insert("cpsDocId".to_string(), Value::Number(doc_id.into()));
        }
        if let Some(exec_id) = exec_id {
            params.insert("id".to_string(), Value::Number(exec_id.into()));
        }
        if let Some(limit) = limit {
            params.insert("limit".to_string(), Value::Number(limit.into()));
        }
        if let Some(offset) = offset {
            params.insert("offset".to_string(), Value::Number(offset.into()));
        }
        if let Some(status) = status {
            params.insert("cps_status".to_string(), Value::Number(status.into()));
        }
        self.core
            .authorized_request("getClientCpsHistory", Some(params), Some(2))
    }

    /// Downloads files for a specific order by ID.
    pub fn get_order_files(
        &self,
        order_id: Option<i64>,
        internal_id: Option<i64>,
    ) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        if let Some(internal_id) = internal_id {
            params.insert("internal_id".to_string(), Value::Number(internal_id.into()));
        } else if let Some(order_id) = order_id {
            params.insert("id".to_string(), Value::Number(order_id.into()));
        } else {
            return Err(TradernetError::InvalidInput(
                "Either order_id or internal_id must be specified".to_string(),
            ));
        }
        self.core
            .authorized_request("getCpsFiles", Some(params), Some(2))
    }

    /// Retrieves broker report for a given date range.
    pub fn get_broker_report(
        &self,
        start: NaiveDate,
        end: NaiveDate,
        period: NaiveTime,
        data_block_type: Option<&str>,
    ) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("date_start".to_string(), Value::String(start.to_string()));
        params.insert("date_end".to_string(), Value::String(end.to_string()));
        params.insert(
            "time_period".to_string(),
            Value::String(period.format("%H:%M:%S").to_string()),
        );
        params.insert("format".to_string(), Value::String("json".to_string()));
        if let Some(data_block_type) = data_block_type {
            params.insert(
                "type".to_string(),
                Value::String(data_block_type.to_string()),
            );
        }
        self.core
            .authorized_request("getBrokerReport", Some(params), Some(2))
    }

    /// Returns detailed information about a symbol.
    pub fn symbol(&self, symbol: &str, lang: &str) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("ticker".to_string(), Value::String(symbol.to_string()));
        params.insert("lang".to_string(), Value::String(lang.to_string()));
        self.core
            .authorized_request("getStockData", Some(params), Some(2))
    }

    /// Returns the list of symbols for an exchange.
    pub fn symbols(&self, exchange: Option<&str>) -> Result<Value, TradernetError> {
        let params = exchange.map(|exchange| {
            let mut params = Map::new();
            params.insert("mkt".to_string(), Value::String(exchange.to_lowercase()));
            params
        });
        self.core
            .authorized_request("getReadyList", params, Some(2))
    }

    /// Returns planned corporate actions for a reception.
    pub fn corporate_actions(&self, reception: i64) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("reception".to_string(), Value::Number(reception.into()));
        self.core
            .authorized_request("getPlannedCorpActions", Some(params), Some(2))
    }

    /// Places a buy order.
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
            return Err(TradernetError::InvalidInput(
                "Quantity must be positive".to_string(),
            ));
        }

        self.trade(
            symbol,
            quantity,
            price,
            duration,
            use_margin,
            custom_order_id,
        )
    }

    /// Places a stop-loss order for a symbol.
    pub fn stop(&self, symbol: &str, price: f64) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("instr_name".to_string(), Value::String(symbol.to_string()));
        let price = serde_json::Number::from_f64(price)
            .ok_or_else(|| TradernetError::InvalidInput("Invalid price".to_string()))?;
        params.insert("stop_loss".to_string(), Value::Number(price));
        self.core
            .authorized_request("putStopLoss", Some(params), Some(2))
    }

    /// Places a trailing stop-loss order.
    pub fn trailing_stop(&self, symbol: &str, percent: i64) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("instr_name".to_string(), Value::String(symbol.to_string()));
        params.insert(
            "stop_loss_percent".to_string(),
            Value::Number(percent.into()),
        );
        params.insert(
            "stoploss_trailing_percent".to_string(),
            Value::Number(percent.into()),
        );
        self.core
            .authorized_request("putStopLoss", Some(params), Some(2))
    }

    /// Places a take-profit order.
    pub fn take_profit(&self, symbol: &str, price: f64) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("instr_name".to_string(), Value::String(symbol.to_string()));
        let price = serde_json::Number::from_f64(price)
            .ok_or_else(|| TradernetError::InvalidInput("Invalid price".to_string()))?;
        params.insert("take_profit".to_string(), Value::Number(price));
        self.core
            .authorized_request("putStopLoss", Some(params), Some(2))
    }

    /// Places a sell order.
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
            return Err(TradernetError::InvalidInput(
                "Quantity must be positive".to_string(),
            ));
        }

        self.trade(
            symbol,
            -quantity,
            price,
            duration,
            use_margin,
            custom_order_id,
        )
    }

    /// Cancels a single order by ID.
    pub fn cancel(&self, order_id: i64) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("order_id".to_string(), Value::Number(order_id.into()));
        self.core
            .authorized_request("delTradeOrder", Some(params), Some(2))
    }

    /// Cancels all active orders.
    pub fn cancel_all(&self) -> Result<Value, TradernetError> {
        let placed = self.get_placed(true)?;
        let orders = placed
            .get("result")
            .and_then(|value| value.get("orders"))
            .and_then(|value| value.get("order"))
            .and_then(|value| value.as_array())
            .cloned()
            .unwrap_or_default();

        let mut results = Vec::new();
        for order in orders {
            if let Some(order_id) = order.get("id").and_then(|value| value.as_i64()) {
                results.push(self.cancel(order_id)?);
            }
        }

        Ok(Value::Array(results))
    }

    /// Returns placed orders (active-only when specified).
    pub fn get_placed(&self, active: bool) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert(
            "active_only".to_string(),
            Value::Number((active as i64).into()),
        );
        self.core
            .authorized_request("getNotifyOrderJson", Some(params), Some(2))
    }

    /// Returns historical orders for the given time range.
    pub fn get_historical(
        &self,
        start: DateTime<Local>,
        end: DateTime<Local>,
    ) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert(
            "from".to_string(),
            Value::String(start.format("%Y-%m-%dT%H:%M:%S").to_string()),
        );
        params.insert(
            "till".to_string(),
            Value::String(end.format("%Y-%m-%dT%H:%M:%S").to_string()),
        );
        self.core
            .authorized_request("getOrdersHistory", Some(params), Some(2))
    }

    /// Returns the list of available tariffs.
    pub fn get_tariffs_list(&self) -> Result<Value, TradernetError> {
        self.core
            .authorized_request("GetListTariffs", None, Some(2))
    }

    /// Places a trade order (buy/sell depending on quantity sign).
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
        params.insert(
            "order_type_id".to_string(),
            Value::Number((if price != 0.0 { 2 } else { 1 }).into()),
        );
        params.insert(
            "qty".to_string(),
            Value::Number(quantity.unsigned_abs().into()),
        );
        let limit_price = serde_json::Number::from_f64(price)
            .ok_or_else(|| TradernetError::InvalidInput("Invalid price".to_string()))?;
        params.insert("limit_price".to_string(), Value::Number(limit_price));
        params.insert(
            "expiration_id".to_string(),
            Value::Number(duration_code.into()),
        );
        if let Some(custom_order_id) = custom_order_id {
            params.insert(
                "user_order_id".to_string(),
                Value::Number(custom_order_id.into()),
            );
        }

        self.core
            .authorized_request("putTradeOrder", Some(params), Some(2))
    }

    fn get_refbook(&self, name: Option<&str>) -> Result<Vec<Map<String, Value>>, TradernetError> {
        let reference_date = self.latest_refbook()?;
        if name.is_none() || name == Some("all") {
            let mut result = Vec::new();
            for refbook_name in self.refbooks(&reference_date)? {
                result.extend(self.get_refbook(Some(&refbook_name))?);
            }
            return Ok(result);
        }

        let name = name.unwrap_or("all");
        let uri = format!("/refbooks/{reference_date}/{name}.json.zip");
        let mut response = self.core.get_request(&uri, None)?;
        let mut content = Vec::new();
        response.read_to_end(&mut content)?;

        let mut archive = ZipArchive::new(std::io::Cursor::new(content))?;
        if archive.len() != 1 {
            return Err(TradernetError::InvalidInput(
                "More than one file in the archive".to_string(),
            ));
        }

        let mut file = archive.by_index(0)?;
        let mut json = String::new();
        file.read_to_string(&mut json)?;
        let parsed: Vec<Map<String, Value>> = serde_json::from_str(&json)?;
        Ok(parsed)
    }

    fn latest_refbook(&self) -> Result<String, TradernetError> {
        let mut response = self.core.get_request("/refbooks", None)?;
        let mut content = String::new();
        response.read_to_string(&mut content)?;
        let regex = Regex::new(r"\d{4}-\d{2}-\d{2}/")
            .map_err(|err| TradernetError::InvalidInput(err.to_string()))?;
        let mut dates = regex
            .find_iter(&content)
            .map(|mat| mat.as_str().trim_end_matches('/').to_string())
            .collect::<Vec<_>>();
        dates.sort();
        dates
            .pop()
            .ok_or_else(|| TradernetError::InvalidInput("No refbook dates found".to_string()))
    }

    fn refbooks(&self, reference_date: &str) -> Result<Vec<String>, TradernetError> {
        let path = format!("/refbooks/{reference_date}");
        let mut response = self.core.get_request(&path, None)?;
        let mut content = String::new();
        response.read_to_string(&mut content)?;
        let regex = Regex::new(r"([A-Za-z0-9_]+)\.json\.zip")
            .map_err(|err| TradernetError::InvalidInput(err.to_string()))?;
        let mut result = regex
            .captures_iter(&content)
            .filter_map(|cap| cap.get(1).map(|value| value.as_str().to_string()))
            .collect::<Vec<_>>();
        result.sort();
        result.dedup();
        Ok(result)
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
