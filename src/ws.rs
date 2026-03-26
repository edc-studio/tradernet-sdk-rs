use crate::core::Core;
use crate::errors::TradernetError;
use crate::user_data::Quote;
use crate::ws_types::{
    MarketDepthEvent, MarketDepthUpdate, MarketsEvent, MarketsUpdate, OrderDataRow, OrdersEvent,
    PortfolioEvent, PortfolioUpdate, QuoteEvent,
};
use async_stream::try_stream;
use futures_util::stream::BoxStream;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

/// WebSocket client for streaming Tradernet market data.
pub struct TradernetWebsocket {
    core: Core,
}

impl TradernetWebsocket {
    /// Creates a new WebSocket client using an authenticated [`Core`].
    pub fn new(core: Core) -> Self {
        Self { core }
    }

    /// Subscribes to quote updates for a list of symbols.
    pub async fn quotes<I, S>(
        &self,
        symbols: I,
    ) -> Result<BoxStream<'static, Result<QuoteEvent, TradernetError>>, TradernetError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let symbols = symbols
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<_>>();
        let query = serde_json::to_string(&serde_json::json!(["quotes", symbols]))?;
        let url = self.websocket_url_with_auth()?;
        let (ws_stream, _) = connect_async(url.to_string()).await?;
        let (mut write, mut read) = ws_stream.split();
        write.send(Message::Text(query)).await?;

        let stream = try_stream! {
            while let Some(message) = read.next().await {
                let message = message?;
                if let Message::Text(text) = message {
                    let parsed: Value = serde_json::from_str(&text)?;
                    if let Some((event, data)) = parsed.as_array().and_then(|values| {
                        let event = values.first()?.as_str()?;
                        let data = values.get(1)?;
                        Some((event, data))
                    }) {
                        match Self::parse_quote_event(event, data)? {
                            Some(event) => yield event,
                            None => {}
                        }
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }

    /// Subscribes to order book updates for a symbol.
    pub async fn market_depth(
        &self,
        symbol: &str,
    ) -> Result<BoxStream<'static, Result<MarketDepthEvent, TradernetError>>, TradernetError> {
        let query = serde_json::to_string(&serde_json::json!(["orderBook", [symbol]]))?;
        let url = self.websocket_url_with_auth()?;
        let (ws_stream, _) = connect_async(url.to_string()).await?;
        let (mut write, mut read) = ws_stream.split();
        write.send(Message::Text(query)).await?;

        let stream = try_stream! {
            while let Some(message) = read.next().await {
                let message = message?;
                if let Message::Text(text) = message {
                    let parsed: Value = serde_json::from_str(&text)?;
                    if let Some((event, data)) = parsed.as_array().and_then(|values| {
                        let event = values.first()?.as_str()?;
                        let data = values.get(1)?;
                        Some((event, data))
                    }) {
                        match Self::parse_market_depth_event(event, data)? {
                            Some(event) => yield event,
                            None => {}
                        }
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }

    /// Subscribes to portfolio updates.
    pub async fn portfolio(
        &self,
    ) -> Result<BoxStream<'static, Result<PortfolioEvent, TradernetError>>, TradernetError> {
        let query = serde_json::to_string(&serde_json::json!(["portfolio"]))?;
        let url = self.websocket_url_with_auth()?;
        let (ws_stream, _) = connect_async(url.to_string()).await?;
        let (mut write, mut read) = ws_stream.split();
        write.send(Message::Text(query)).await?;

        let stream = try_stream! {
            while let Some(message) = read.next().await {
                let message = message?;
                if let Message::Text(text) = message {
                    let parsed: Value = serde_json::from_str(&text)?;
                    if let Some((event, data)) = parsed.as_array().and_then(|values| {
                        let event = values.first()?.as_str()?;
                        let data = values.get(1)?;
                        Some((event, data))
                    }) {
                        match Self::parse_portfolio_event(event, data)? {
                            Some(event) => yield event,
                            None => {}
                        }
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }

    /// Subscribes to active orders updates.
    pub async fn orders(
        &self,
    ) -> Result<BoxStream<'static, Result<OrdersEvent, TradernetError>>, TradernetError> {
        let query = serde_json::to_string(&serde_json::json!(["orders"]))?;
        let url = self.websocket_url_with_auth()?;
        let (ws_stream, _) = connect_async(url.to_string()).await?;
        let (mut write, mut read) = ws_stream.split();
        write.send(Message::Text(query)).await?;

        let stream = try_stream! {
            while let Some(message) = read.next().await {
                let message = message?;
                if let Message::Text(text) = message {
                    let parsed: Value = serde_json::from_str(&text)?;
                    if let Some((event, data)) = parsed.as_array().and_then(|values| {
                        let event = values.first()?.as_str()?;
                        let data = values.get(1)?;
                        Some((event, data))
                    }) {
                        match Self::parse_orders_event(event, data)? {
                            Some(event) => yield event,
                            None => {}
                        }
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }

    /// Subscribes to markets status updates.
    pub async fn markets(
        &self,
    ) -> Result<BoxStream<'static, Result<MarketsEvent, TradernetError>>, TradernetError> {
        let query = serde_json::to_string(&serde_json::json!(["markets"]))?;
        let url = self.websocket_url_with_auth()?;
        let (ws_stream, _) = connect_async(url.to_string()).await?;
        let (mut write, mut read) = ws_stream.split();
        write.send(Message::Text(query)).await?;
        let stream = try_stream! {
            while let Some(message) = read.next().await {
                let message = message?;
                if let Message::Text(text) = message {
                    let parsed: Value = serde_json::from_str(&text)?;
                    if let Some((event, data)) = parsed.as_array().and_then(|values| {
                        let event = values.first()?.as_str()?;
                        let data = values.get(1)?;
                        Some((event, data))
                    }) {
                        match Self::parse_markets_event(event, data)? {
                            Some(event) => yield event,
                            None => {}
                        }
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }

    fn websocket_url_with_auth(&self) -> Result<Url, TradernetError> {
        let mut url = Url::parse(&Core::websocket_url())?;
        let params = self.core.websocket_auth();
        for (key, value) in params {
            url.query_pairs_mut().append_pair(&key, &value);
        }
        Ok(url)
    }

    fn parse_quote_event(event: &str, data: &Value) -> Result<Option<QuoteEvent>, TradernetError> {
        match event {
            "q" => {
                let quote: Quote = serde_json::from_value(data.clone())?;
                Ok(Some(QuoteEvent::Quote(quote)))
            }
            "error" => Ok(Some(QuoteEvent::Error(data.clone()))),
            _ => Ok(None),
        }
    }

    fn parse_market_depth_event(
        event: &str,
        data: &Value,
    ) -> Result<Option<MarketDepthEvent>, TradernetError> {
        match event {
            "b" => {
                let depth: MarketDepthUpdate = serde_json::from_value(data.clone())?;
                Ok(Some(MarketDepthEvent::Update(depth)))
            }
            "error" => Ok(Some(MarketDepthEvent::Error(data.clone()))),
            _ => Ok(None),
        }
    }

    fn parse_portfolio_event(
        event: &str,
        data: &Value,
    ) -> Result<Option<PortfolioEvent>, TradernetError> {
        match event {
            "portfolio" => {
                let portfolio: PortfolioUpdate = serde_json::from_value(data.clone())?;
                Ok(Some(PortfolioEvent::Portfolio(portfolio)))
            }
            "error" => Ok(Some(PortfolioEvent::Error(data.clone()))),
            _ => Ok(None),
        }
    }

    fn parse_orders_event(
        event: &str,
        data: &Value,
    ) -> Result<Option<OrdersEvent>, TradernetError> {
        match event {
            "orders" => {
                let orders: Vec<OrderDataRow> = serde_json::from_value(data.clone())?;
                Ok(Some(OrdersEvent::Orders(orders)))
            }
            "error" => Ok(Some(OrdersEvent::Error(data.clone()))),
            _ => Ok(None),
        }
    }

    fn parse_markets_event(
        event: &str,
        data: &Value,
    ) -> Result<Option<MarketsEvent>, TradernetError> {
        match event {
            "markets" => {
                let markets: MarketsUpdate = serde_json::from_value(data.clone())?;
                Ok(Some(MarketsEvent::Markets(markets)))
            }
            "error" => Ok(Some(MarketsEvent::Error(data.clone()))),
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TradernetWebsocket;
    use crate::ws_types::{
        MarketDepthEvent, MarketDepthSide, MarketsEvent, OrdersEvent, PortfolioEvent,
    };
    use serde_json::json;

    #[test]
    fn parses_market_depth_event_b() {
        let payload = json!({
            "n": 102,
            "i": "AAPL.US",
            "del": [],
            "ins": [],
            "upd": [
                {"p": 33.925, "s": "S", "q": 196100, "k": 3},
                {"p": 33.89, "s": "S", "q": 373700, "k": 6},
                {"p": 33.885, "s": "S", "q": 1198800, "k": 7},
                {"p": 33.88, "s": "S", "q": 251600, "k": 8}
            ],
            "cnt": 21,
            "x": 11
        });

        let event = TradernetWebsocket::parse_market_depth_event("b", &payload)
            .expect("must parse")
            .expect("must return event");

        let MarketDepthEvent::Update(update) = event else {
            panic!("expected market depth update event")
        };

        assert_eq!(update.i, "AAPL.US");
        assert_eq!(update.cnt, 21);
        assert_eq!(update.n, Some(102));
        assert_eq!(update.x, Some(11));
        assert_eq!(update.upd.len(), 4);
        assert!(matches!(update.upd[0].s, MarketDepthSide::Sell));
        assert_eq!(update.upd[0].k, 3);
        assert_eq!(update.upd[0].p, 33.925);
        assert_eq!(update.upd[0].q, 196100.0);
    }

    #[test]
    fn parses_portfolio_event_portfolio() {
        let payload = json!({
            "key": "%test@test.com",
            "acc": [
                {
                    "s": ".00000000",
                    "forecast_in": ".00000000",
                    "forecast_out": ".00000000",
                    "curr": "USD",
                    "currval": 78.95,
                    "t2_in": ".00000000",
                    "t2_out": ".00000000"
                }
            ],
            "pos": [
                {
                    "i": "AAPL.US",
                    "t": 1,
                    "k": 1,
                    "s": 22.4,
                    "q": 100,
                    "fv": 100,
                    "curr": "USD",
                    "currval": 1,
                    "name": "Apple Inc.",
                    "name2": "Apple Inc.",
                    "open_bal": 22.4,
                    "mkt_price": 23.81,
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
                    "instr_id": 10000007229u64,
                    "Yield": ".00000000",
                    "issue_nb": "US000902000001",
                    "profit_price": 2.83,
                    "market_value": 2020,
                    "close_price": 2.83,
                    "trade": [{"trade_count": 2}]
                }
            ]
        });

        let event = TradernetWebsocket::parse_portfolio_event("portfolio", &payload)
            .expect("must parse")
            .expect("must return event");

        let PortfolioEvent::Portfolio(update) = event else {
            panic!("expected portfolio update event")
        };

        assert_eq!(update.key, "%test@test.com");
        assert_eq!(update.acc.len(), 1);
        assert_eq!(update.acc[0].curr, "USD");
        assert_eq!(update.acc[0].s, 0.0);
        assert_eq!(update.pos.len(), 1);
        assert_eq!(update.pos[0].i, "AAPL.US");
        assert_eq!(update.pos[0].instr_id, 10000007229);
        assert_eq!(update.pos[0].trade.len(), 1);
        assert_eq!(update.pos[0].trade[0].trade_count, 2);
    }

    #[test]
    fn parses_orders_event_orders() {
        let payload: serde_json::Value = serde_json::from_str(
            r#"[
            {
                "aon": 0,
                "cur": "USD",
                "curr_q": 0,
                "date": "2015-12-23T17:05:02.133",
                "exp": 1,
                "fv": 0,
                "order_id": 8757875,
                "instr": "FCX.US",
                "leaves_qty": 0,
                "auth_login": "virtual@virtual.com",
                "creator_login": "virtual@virtual.com",
                "owner_login": "virtual@virtual.com",
                "mkt_id": 30000000001,
                "name": "Freeport-McMoran Cp & Gld",
                "name2": "Freeport-McMoran Cp & Gld",
                "oper": 2,
                "p": 6.5611,
                "q": 2625,
                "rep": 0,
                "stat": 21,
                "stat_d": "2015-12-23T17:05:03.283",
                "stat_orig": 21,
                "stat_prev": 10,
                "stop": 0,
                "stop_activated": 1,
                "stop_init_price": 6.36,
                "trailing_price": 0,
                "type": 1,
                "user_order_id": 1450879514204,
                "trade": [
                    {
                        "acd": 0,
                        "date": "2015-12-23T17:05:03",
                        "fv": 100,
                        "go_sum": 0,
                        "id": 13446624,
                        "p": 6.37,
                        "profit": 0,
                        "q": 2625,
                        "v": 16721.25
                    }
                ]
            }
        ]"#,
        )
        .expect("json payload must be valid");

        let event = TradernetWebsocket::parse_orders_event("orders", &payload)
            .expect("must parse")
            .expect("must return event");

        let OrdersEvent::Orders(orders) = event else {
            panic!("expected orders update event")
        };

        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].order_id, 8757875);
        assert_eq!(orders[0].instr, "FCX.US");
        assert_eq!(orders[0].mkt_id, 30000000001);
        assert_eq!(orders[0].user_order_id, 1450879514204);
        assert_eq!(orders[0].trade.len(), 1);
        assert_eq!(orders[0].trade[0].id, 13446624);
    }

    #[test]
    fn parses_orders_event_error() {
        let payload = json!({"code": 400, "message": "boom"});

        let event = TradernetWebsocket::parse_orders_event("error", &payload)
            .expect("must parse")
            .expect("must return event");

        let OrdersEvent::Error(error) = event else {
            panic!("expected error event")
        };

        assert_eq!(error["code"], 400);
        assert_eq!(error["message"], "boom");
    }

    #[test]
    fn parses_markets_event_markets() {
        let payload = json!({
            "t": "2020-11-18 19:29:27",
            "m": [
                {
                    "n": "KASE",
                    "n2": "KASE",
                    "s": "CLOSE",
                    "o": "08:20:00",
                    "c": "14:00:00",
                    "dt": "-180"
                }
            ]
        });

        let event = TradernetWebsocket::parse_markets_event("markets", &payload)
            .expect("must parse")
            .expect("must return event");

        let MarketsEvent::Markets(update) = event else {
            panic!("expected markets update event")
        };

        assert_eq!(update.t, "2020-11-18 19:29:27");
        assert_eq!(update.m.len(), 1);
        assert_eq!(update.m[0].n, "KASE");
        assert_eq!(update.m[0].s, "CLOSE");
        assert_eq!(update.m[0].o, "08:20:00");
        assert_eq!(update.m[0].c, "14:00:00");
        assert_eq!(update.m[0].dt, "-180");
    }
}
