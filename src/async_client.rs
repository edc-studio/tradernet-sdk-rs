use crate::common::client_helpers;
use crate::core::{AsyncCore, Core};
use crate::errors::TradernetError;
use crate::user_data::UserDataResponse;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
use serde_json::{Map, Value};
use std::collections::HashMap;

/// Asynchronous REST client for the Tradernet API.
pub struct AsyncTradernet {
    core: AsyncCore,
}

impl AsyncTradernet {
    /// Creates a REST client with optional API keys.
    #[allow(clippy::result_large_err)]
    pub fn new(public: Option<String>, private: Option<String>) -> Result<Self, TradernetError> {
        Ok(Self {
            core: AsyncCore::new(public, private)?,
        })
    }

    /// Creates a REST client from an INI config file with credentials.
    #[allow(clippy::result_large_err)]
    pub fn from_config(path: impl AsRef<std::path::Path>) -> Result<Self, TradernetError> {
        Ok(Self {
            core: AsyncCore::from_config(path)?,
        })
    }

    /// Returns a reference to the underlying [`AsyncCore`].
    pub fn core(&self) -> &AsyncCore {
        &self.core
    }

    /// Returns authenticated user information.
    pub async fn user_info(&self) -> Result<Value, TradernetError> {
        self.core
            .authorized_request("GetAllUserTexInfo", None, Some(2))
            .await
    }

    /// Registers a new user.
    #[allow(clippy::too_many_arguments)]
    pub async fn new_user(
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

        self.core
            .plain_request("registerNewUser", Some(params))
            .await
    }

    /// Checks missing onboarding fields for the given step and office.
    pub async fn check_missing_fields(
        &self,
        step: i64,
        office: &str,
    ) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("step".to_string(), Value::Number(step.into()));
        params.insert("office".to_string(), Value::String(office.to_string()));
        self.core
            .authorized_request("checkStep", Some(params), Some(2))
            .await
    }

    /// Returns profile fields for a given reception.
    pub async fn get_profile_fields(&self, reception: i64) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("reception".to_string(), Value::Number(reception.into()));
        self.core
            .authorized_request("getProfileFields", Some(params), Some(2))
            .await
    }

    /// Returns user data (OPQ) for the authenticated account.
    pub async fn get_user_data(&self) -> Result<UserDataResponse, TradernetError> {
        let response = self
            .core
            .authorized_request("getOPQ", None, Some(2))
            .await?;
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

    /// Returns the trading status for a market.
    pub async fn get_market_status(
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
            .await
    }

    /// Returns the most traded securities for a market.
    pub async fn get_most_traded(
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
        self.core
            .plain_request("getTopSecurities", Some(params))
            .await
    }

    /// Exports securities list with optional fields.
    pub async fn export_securities<I, S>(
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

            let response = self
                .core
                .get_request("/securities/export", Some(&params))
                .await?;
            let mut result: Vec<Value> = response.json().await?;
            results.append(&mut result);
        }

        Ok(Value::Array(results))
    }

    /// Retrieves security info for a symbol.
    pub async fn security_info(&self, symbol: &str, sup: bool) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("ticker".to_string(), Value::String(symbol.to_string()));
        params.insert("sup".to_string(), Value::Bool(sup));
        self.core
            .authorized_request("getSecurityInfo", Some(params), Some(2))
            .await
    }

    /// Returns options chain for an underlying symbol.
    pub async fn get_options(
        &self,
        underlying: &str,
        exchange: &str,
    ) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert(
            "underlying".to_string(),
            Value::String(underlying.to_string()),
        );
        params.insert("mkt".to_string(), Value::String(exchange.to_string()));
        self.core
            .authorized_request("getOptionsByMkt", Some(params), Some(2))
            .await
    }

    /// Returns candles for a symbol.
    pub async fn get_candles(
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
            .await
    }

    /// Returns quote data for a list of symbols.
    pub async fn get_quotes<I, S>(&self, symbols: I) -> Result<Value, TradernetError>
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
            .await
    }

    /// Returns trade history.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_trades_history(
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
            .await
    }

    /// Finds a symbol by query and optional exchange.
    pub async fn find_symbol(
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
        self.core.plain_request("tickerFinder", Some(params)).await
    }

    /// Retrieves news items for a query and optional filters.
    pub async fn get_news(
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
            .await
    }

    /// Returns a filtered list of securities from refbooks.
    pub async fn get_all(
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

        let refbook = self.get_refbook(market_code.as_deref()).await?;
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
    pub async fn account_summary(&self) -> Result<Value, TradernetError> {
        self.core
            .authorized_request("getPositionJson", None, Some(2))
            .await
    }

    /// Returns price alerts for all symbols or a specific ticker.
    pub async fn get_price_alerts(&self, symbol: Option<&str>) -> Result<Value, TradernetError> {
        let params = symbol.map(|symbol| {
            let mut params = Map::new();
            params.insert("ticker".to_string(), Value::String(symbol.to_string()));
            params
        });
        self.core
            .authorized_request("getAlertsList", params, Some(2))
            .await
    }

    /// Creates a price alert for a symbol.
    #[allow(clippy::too_many_arguments)]
    pub async fn add_price_alert<I, S>(
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
            .await
    }

    /// Deletes a price alert by identifier.
    #[allow(clippy::result_large_err)]
    pub async fn delete_price_alert(&self, alert_id: i64) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("id".to_string(), Value::Number(alert_id.into()));
        params.insert("del".to_string(), Value::Bool(true));
        self.core
            .authorized_request("addPriceAlert", Some(params), Some(2))
            .await
    }

    /// Returns account requests history.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_requests_history(
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
        if let Some(doc_id) = doc_id {
            params.insert("cpsDocId".to_string(), Value::Number(doc_id.into()));
        }
        if let Some(exec_id) = exec_id {
            params.insert("id".to_string(), Value::Number(exec_id.into()));
        }
        params.insert(
            "date_from".to_string(),
            Value::String(start.format("%Y-%m-%dT%H:%M:%S").to_string()),
        );
        params.insert(
            "date_to".to_string(),
            Value::String(end.format("%Y-%m-%dT%H:%M:%S").to_string()),
        );
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
            .await
    }

    /// Returns order-related files.
    pub async fn get_order_files(
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
            .await
    }

    /// Returns broker report data.
    pub async fn get_broker_report(
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
            .await
    }

    /// Returns symbol metadata in a specific language.
    pub async fn symbol(&self, symbol: &str, lang: &str) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("ticker".to_string(), Value::String(symbol.to_string()));
        params.insert("lang".to_string(), Value::String(lang.to_string()));
        self.core
            .authorized_request("getStockData", Some(params), Some(2))
            .await
    }

    /// Returns symbols list for a given exchange.
    pub async fn symbols(&self, exchange: Option<&str>) -> Result<Value, TradernetError> {
        let params = exchange.map(|exchange| {
            let mut params = Map::new();
            params.insert("mkt".to_string(), Value::String(exchange.to_lowercase()));
            params
        });
        self.core
            .authorized_request("getReadyList", params, Some(2))
            .await
    }

    /// Returns corporate actions for a reception ID.
    pub async fn corporate_actions(&self, reception: i64) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("reception".to_string(), Value::Number(reception.into()));
        self.core
            .authorized_request("getPlannedCorpActions", Some(params), Some(2))
            .await
    }

    /// Places a buy order.
    pub async fn buy(
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
        .await
    }

    /// Places a stop-loss order.
    pub async fn stop(&self, symbol: &str, price: f64) -> Result<Value, TradernetError> {
        let params = client_helpers::build_stop_params(symbol, price)?;
        self.core
            .authorized_request("putStopLoss", Some(params), Some(2))
            .await
    }

    /// Places a trailing stop order.
    pub async fn trailing_stop(&self, symbol: &str, percent: i64) -> Result<Value, TradernetError> {
        let params = client_helpers::build_trailing_stop_params(symbol, percent);
        self.core
            .authorized_request("putStopLoss", Some(params), Some(2))
            .await
    }

    /// Places a take-profit order.
    pub async fn take_profit(&self, symbol: &str, price: f64) -> Result<Value, TradernetError> {
        let params = client_helpers::build_take_profit_params(symbol, price)?;
        self.core
            .authorized_request("putStopLoss", Some(params), Some(2))
            .await
    }

    /// Places a sell order.
    pub async fn sell(
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
        .await
    }

    /// Cancels a single order by id.
    pub async fn cancel(&self, order_id: i64) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert("order_id".to_string(), Value::Number(order_id.into()));
        self.core
            .authorized_request("delTradeOrder", Some(params), Some(2))
            .await
    }

    /// Cancels all orders.
    pub async fn cancel_all(&self) -> Result<Value, TradernetError> {
        let placed = self.get_placed(true).await?;
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
                results.push(self.cancel(order_id).await?);
            }
        }

        Ok(Value::Array(results))
    }

    /// Returns placed orders filtered by active flag.
    pub async fn get_placed(&self, active: bool) -> Result<Value, TradernetError> {
        let mut params = Map::new();
        params.insert(
            "active_only".to_string(),
            Value::Number((active as i64).into()),
        );
        self.core
            .authorized_request("getNotifyOrderJson", Some(params), Some(2))
            .await
    }

    /// Returns historical orders.
    pub async fn get_historical(
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
            .await
    }

    /// Returns available tariff list.
    pub async fn get_tariffs_list(&self) -> Result<Value, TradernetError> {
        self.core
            .authorized_request("GetListTariffs", None, Some(2))
            .await
    }

    /// Places a trade order with validation.
    pub async fn trade(
        &self,
        symbol: &str,
        quantity: i64,
        price: f64,
        duration: &str,
        use_margin: bool,
        custom_order_id: Option<i64>,
    ) -> Result<Value, TradernetError> {
        let params = client_helpers::build_trade_params(
            symbol,
            quantity,
            price,
            duration,
            use_margin,
            custom_order_id,
        )?;

        self.core
            .authorized_request("putTradeOrder", Some(params), Some(2))
            .await
    }

    async fn get_refbook(
        &self,
        name: Option<&str>,
    ) -> Result<Vec<Map<String, Value>>, TradernetError> {
        let reference_date = self.latest_refbook().await?;
        if name.is_none() || name == Some("all") {
            let mut result = Vec::new();
            for refbook_name in self.refbooks(&reference_date).await? {
                result.extend(
                    self.get_refbook_named(&reference_date, &refbook_name)
                        .await?,
                );
            }
            return Ok(result);
        }

        let name = name.unwrap_or("all");
        self.get_refbook_named(&reference_date, name).await
    }

    async fn get_refbook_named(
        &self,
        reference_date: &str,
        name: &str,
    ) -> Result<Vec<Map<String, Value>>, TradernetError> {
        let uri = format!("/refbooks/{reference_date}/{name}.json.zip");
        let response = self.core.get_request(&uri, None).await?;
        let content = response.bytes().await?;

        client_helpers::parse_refbook_archive(&content)
    }

    async fn latest_refbook(&self) -> Result<String, TradernetError> {
        let response = self.core.get_request("/refbooks", None).await?;
        let content = response.text().await?;
        client_helpers::parse_latest_refbook_dates(&content)
    }

    async fn refbooks(&self, reference_date: &str) -> Result<Vec<String>, TradernetError> {
        let path = format!("/refbooks/{reference_date}");
        let response = self.core.get_request(&path, None).await?;
        let content = response.text().await?;
        client_helpers::parse_refbooks(&content)
    }
}
