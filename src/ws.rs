use crate::core::{AsyncCore, Core, WsCredentials};
use crate::errors::TradernetError;
use crate::user_data::Quote;
use crate::ws_types::{
    MarketDepthEvent, MarketDepthUpdate, MarketsEvent, MarketsUpdate, OrderDataRow, OrdersEvent,
    PortfolioEvent, PortfolioUpdate, QuoteEvent, SubscribeRequest, UnsubscribeRequest, WsEvent,
    WsReconnectConfig,
};
use async_stream::{stream, try_stream};
use futures_util::stream::BoxStream;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

/// WebSocket client for streaming Tradernet market data.
pub struct TradernetWebsocket {
    credentials: WsCredentials,
    websocket_url_override: Option<String>,
}

type WsEventsRx = mpsc::UnboundedReceiver<Result<WsEvent, TradernetError>>;

/// Multi-subscription WebSocket session using a single underlying connection.
///
/// Session lifecycle:
/// - call [`subscribe`](Self::subscribe) to activate streams;
/// - consume [`events`](Self::events) for updates and state notifications;
/// - on network loss session reconnects automatically with configured backoff;
/// - all active subscriptions are replayed after reconnect.
///
/// `unsubscribe` is implemented locally (event filtering). If server-side unsubscribe
/// is not supported by protocol, no unsubscribe command is sent.
pub struct TradernetWsSession {
    command_tx: mpsc::UnboundedSender<SessionCommand>,
    events_rx: Arc<Mutex<Option<WsEventsRx>>>,
    subscriptions: Arc<Mutex<SubscriptionState>>,
    closed: Arc<AtomicBool>,
    worker: Arc<Mutex<Option<JoinHandle<()>>>>,
}

enum SessionCommand {
    Subscribe(SubscribeRequest),
    Unsubscribe(UnsubscribeRequest),
    Close,
}

#[derive(Debug, Default, Clone)]
struct SubscriptionState {
    quotes: HashSet<String>,
    order_book: HashSet<String>,
    portfolio: bool,
    orders: bool,
    markets: bool,
}

impl SubscriptionState {
    fn apply_subscribe(&mut self, req: SubscribeRequest) -> Option<SubscribeRequest> {
        match req {
            SubscribeRequest::Quotes { symbols } => {
                let added = insert_symbols(&mut self.quotes, symbols);
                (!added.is_empty()).then_some(SubscribeRequest::Quotes { symbols: added })
            }
            SubscribeRequest::OrderBook { symbols } => {
                let added = insert_symbols(&mut self.order_book, symbols);
                (!added.is_empty()).then_some(SubscribeRequest::OrderBook { symbols: added })
            }
            SubscribeRequest::Portfolio => {
                if self.portfolio {
                    None
                } else {
                    self.portfolio = true;
                    Some(SubscribeRequest::Portfolio)
                }
            }
            SubscribeRequest::Orders => {
                if self.orders {
                    None
                } else {
                    self.orders = true;
                    Some(SubscribeRequest::Orders)
                }
            }
            SubscribeRequest::Markets => {
                if self.markets {
                    None
                } else {
                    self.markets = true;
                    Some(SubscribeRequest::Markets)
                }
            }
        }
    }

    fn apply_unsubscribe(&mut self, req: UnsubscribeRequest) -> Option<UnsubscribeRequest> {
        match req {
            UnsubscribeRequest::Quotes { symbols } => {
                let removed = remove_symbols(&mut self.quotes, symbols);
                (!removed.is_empty()).then_some(UnsubscribeRequest::Quotes { symbols: removed })
            }
            UnsubscribeRequest::OrderBook { symbols } => {
                let removed = remove_symbols(&mut self.order_book, symbols);
                (!removed.is_empty()).then_some(UnsubscribeRequest::OrderBook { symbols: removed })
            }
            UnsubscribeRequest::Portfolio => {
                if self.portfolio {
                    self.portfolio = false;
                    Some(UnsubscribeRequest::Portfolio)
                } else {
                    None
                }
            }
            UnsubscribeRequest::Orders => {
                if self.orders {
                    self.orders = false;
                    Some(UnsubscribeRequest::Orders)
                } else {
                    None
                }
            }
            UnsubscribeRequest::Markets => {
                if self.markets {
                    self.markets = false;
                    Some(UnsubscribeRequest::Markets)
                } else {
                    None
                }
            }
        }
    }

    fn active_requests(&self) -> Vec<SubscribeRequest> {
        let mut requests = Vec::new();
        if !self.quotes.is_empty() {
            requests.push(SubscribeRequest::Quotes {
                symbols: sorted_symbols(&self.quotes),
            });
        }
        if !self.order_book.is_empty() {
            requests.push(SubscribeRequest::OrderBook {
                symbols: sorted_symbols(&self.order_book),
            });
        }
        if self.portfolio {
            requests.push(SubscribeRequest::Portfolio);
        }
        if self.orders {
            requests.push(SubscribeRequest::Orders);
        }
        if self.markets {
            requests.push(SubscribeRequest::Markets);
        }
        requests
    }

    fn allows_event(&self, event: &WsEvent) -> bool {
        match event {
            WsEvent::Quote(quote) => quote
                .c
                .as_ref()
                .is_some_and(|symbol| self.quotes.contains(symbol)),
            WsEvent::MarketDepth(update) => self.order_book.contains(&update.i),
            WsEvent::Portfolio(_) => self.portfolio,
            WsEvent::Orders(_) => self.orders,
            WsEvent::Markets(_) => self.markets,
            WsEvent::Error(_) | WsEvent::Connected | WsEvent::Reconnecting | WsEvent::Closed => {
                true
            }
        }
    }
}

fn normalize_symbols(symbols: Vec<String>) -> Vec<String> {
    let mut unique = HashSet::new();
    let mut normalized = Vec::new();
    for symbol in symbols {
        if symbol.trim().is_empty() {
            continue;
        }
        if unique.insert(symbol.clone()) {
            normalized.push(symbol);
        }
    }
    normalized
}

fn sorted_symbols(set: &HashSet<String>) -> Vec<String> {
    let mut symbols = set.iter().cloned().collect::<Vec<_>>();
    symbols.sort();
    symbols
}

fn insert_symbols(target: &mut HashSet<String>, symbols: Vec<String>) -> Vec<String> {
    let mut added = Vec::new();
    for symbol in normalize_symbols(symbols) {
        if target.insert(symbol.clone()) {
            added.push(symbol);
        }
    }
    added
}

fn remove_symbols(target: &mut HashSet<String>, symbols: Vec<String>) -> Vec<String> {
    let mut removed = Vec::new();
    for symbol in normalize_symbols(symbols) {
        if target.remove(&symbol) {
            removed.push(symbol);
        }
    }
    removed
}

impl TradernetWebsocket {
    /// Creates a new WebSocket client from optional API keys.
    pub fn new(public: Option<String>, private: Option<String>) -> Self {
        Self {
            credentials: WsCredentials { public, private },
            websocket_url_override: None,
        }
    }

    /// Creates a new WebSocket client from [`Core`] credentials.
    pub fn from_core(core: &Core) -> Self {
        Self {
            credentials: core.ws_credentials(),
            websocket_url_override: None,
        }
    }

    /// Creates a new WebSocket client from [`AsyncCore`] credentials.
    pub fn from_async_core(core: &AsyncCore) -> Self {
        Self {
            credentials: core.ws_credentials(),
            websocket_url_override: None,
        }
    }

    /// Overrides WebSocket endpoint URL.
    ///
    /// Useful for tests with local/mock WebSocket servers.
    pub fn with_websocket_url(mut self, websocket_url: impl Into<String>) -> Self {
        self.websocket_url_override = Some(websocket_url.into());
        self
    }

    /// Creates a WebSocket client from [`Core`] with custom endpoint URL.
    pub fn with_websocket_url_from_core(core: &Core, websocket_url: impl Into<String>) -> Self {
        Self {
            credentials: core.ws_credentials(),
            websocket_url_override: Some(websocket_url.into()),
        }
    }

    /// Creates a WebSocket client from [`AsyncCore`] with custom endpoint URL.
    pub fn with_websocket_url_from_async_core(
        core: &AsyncCore,
        websocket_url: impl Into<String>,
    ) -> Self {
        Self {
            credentials: core.ws_credentials(),
            websocket_url_override: Some(websocket_url.into()),
        }
    }

    /// Opens a multi-subscription WebSocket session using default reconnect config.
    pub async fn connect(&self) -> Result<TradernetWsSession, TradernetError> {
        self.connect_with_config(WsReconnectConfig::default()).await
    }

    /// Opens a multi-subscription WebSocket session using custom reconnect config.
    pub async fn connect_with_config(
        &self,
        reconnect: WsReconnectConfig,
    ) -> Result<TradernetWsSession, TradernetError> {
        let url = self.websocket_url_with_auth()?;
        Ok(TradernetWsSession::start(url, reconnect))
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

        let session = self.connect().await?;
        session
            .subscribe(SubscribeRequest::Quotes { symbols })
            .await?;

        let mut events = session.events();
        let stream = try_stream! {
            let _session_guard = session;
            while let Some(message) = events.next().await {
                match message? {
                    WsEvent::Quote(quote) => yield QuoteEvent::Quote(quote),
                    WsEvent::Error(error) => yield QuoteEvent::Error(error),
                    WsEvent::Closed => break,
                    _ => {}
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
        let session = self.connect().await?;
        session
            .subscribe(SubscribeRequest::OrderBook {
                symbols: vec![symbol.to_string()],
            })
            .await?;

        let mut events = session.events();
        let stream = try_stream! {
            let _session_guard = session;
            while let Some(message) = events.next().await {
                match message? {
                    WsEvent::MarketDepth(update) => yield MarketDepthEvent::Update(update),
                    WsEvent::Error(error) => yield MarketDepthEvent::Error(error),
                    WsEvent::Closed => break,
                    _ => {}
                }
            }
        };

        Ok(Box::pin(stream))
    }

    /// Subscribes to portfolio updates.
    pub async fn portfolio(
        &self,
    ) -> Result<BoxStream<'static, Result<PortfolioEvent, TradernetError>>, TradernetError> {
        let session = self.connect().await?;
        session.subscribe(SubscribeRequest::Portfolio).await?;

        let mut events = session.events();
        let stream = try_stream! {
            let _session_guard = session;
            while let Some(message) = events.next().await {
                match message? {
                    WsEvent::Portfolio(update) => yield PortfolioEvent::Portfolio(update),
                    WsEvent::Error(error) => yield PortfolioEvent::Error(error),
                    WsEvent::Closed => break,
                    _ => {}
                }
            }
        };

        Ok(Box::pin(stream))
    }

    /// Subscribes to active orders updates.
    pub async fn orders(
        &self,
    ) -> Result<BoxStream<'static, Result<OrdersEvent, TradernetError>>, TradernetError> {
        let session = self.connect().await?;
        session.subscribe(SubscribeRequest::Orders).await?;

        let mut events = session.events();
        let stream = try_stream! {
            let _session_guard = session;
            while let Some(message) = events.next().await {
                match message? {
                    WsEvent::Orders(update) => yield OrdersEvent::Orders(update),
                    WsEvent::Error(error) => yield OrdersEvent::Error(error),
                    WsEvent::Closed => break,
                    _ => {}
                }
            }
        };

        Ok(Box::pin(stream))
    }

    /// Subscribes to markets status updates.
    pub async fn markets(
        &self,
    ) -> Result<BoxStream<'static, Result<MarketsEvent, TradernetError>>, TradernetError> {
        let session = self.connect().await?;
        session.subscribe(SubscribeRequest::Markets).await?;

        let mut events = session.events();
        let stream = try_stream! {
            let _session_guard = session;
            while let Some(message) = events.next().await {
                match message? {
                    WsEvent::Markets(update) => yield MarketsEvent::Markets(update),
                    WsEvent::Error(error) => yield MarketsEvent::Error(error),
                    WsEvent::Closed => break,
                    _ => {}
                }
            }
        };

        Ok(Box::pin(stream))
    }

    fn websocket_url_with_auth(&self) -> Result<Url, TradernetError> {
        let base_url = self
            .websocket_url_override
            .clone()
            .unwrap_or_else(Core::websocket_url);
        let mut url = Url::parse(&base_url)?;
        let params = self.credentials.websocket_auth();
        for (key, value) in params {
            url.query_pairs_mut().append_pair(&key, &value);
        }
        Ok(url)
    }

    #[cfg(test)]
    fn parse_quote_event(event: &str, data: &Value) -> Result<Option<QuoteEvent>, TradernetError> {
        match Self::parse_ws_event(event, data)? {
            Some(WsEvent::Quote(quote)) => Ok(Some(QuoteEvent::Quote(quote))),
            Some(WsEvent::Error(error)) => Ok(Some(QuoteEvent::Error(error))),
            _ => Ok(None),
        }
    }

    #[cfg(test)]
    fn parse_market_depth_event(
        event: &str,
        data: &Value,
    ) -> Result<Option<MarketDepthEvent>, TradernetError> {
        match Self::parse_ws_event(event, data)? {
            Some(WsEvent::MarketDepth(depth)) => Ok(Some(MarketDepthEvent::Update(depth))),
            Some(WsEvent::Error(error)) => Ok(Some(MarketDepthEvent::Error(error))),
            _ => Ok(None),
        }
    }

    #[cfg(test)]
    fn parse_portfolio_event(
        event: &str,
        data: &Value,
    ) -> Result<Option<PortfolioEvent>, TradernetError> {
        match Self::parse_ws_event(event, data)? {
            Some(WsEvent::Portfolio(portfolio)) => Ok(Some(PortfolioEvent::Portfolio(portfolio))),
            Some(WsEvent::Error(error)) => Ok(Some(PortfolioEvent::Error(error))),
            _ => Ok(None),
        }
    }

    #[cfg(test)]
    fn parse_orders_event(
        event: &str,
        data: &Value,
    ) -> Result<Option<OrdersEvent>, TradernetError> {
        match Self::parse_ws_event(event, data)? {
            Some(WsEvent::Orders(orders)) => Ok(Some(OrdersEvent::Orders(orders))),
            Some(WsEvent::Error(error)) => Ok(Some(OrdersEvent::Error(error))),
            _ => Ok(None),
        }
    }

    #[cfg(test)]
    fn parse_markets_event(
        event: &str,
        data: &Value,
    ) -> Result<Option<MarketsEvent>, TradernetError> {
        match Self::parse_ws_event(event, data)? {
            Some(WsEvent::Markets(markets)) => Ok(Some(MarketsEvent::Markets(markets))),
            Some(WsEvent::Error(error)) => Ok(Some(MarketsEvent::Error(error))),
            _ => Ok(None),
        }
    }

    fn parse_ws_message(text: &str) -> Result<Option<WsEvent>, TradernetError> {
        let parsed: Value = serde_json::from_str(text)?;
        if let Some((event, data)) = parsed.as_array().and_then(|values| {
            let event = values.first()?.as_str()?;
            let data = values.get(1)?;
            Some((event, data))
        }) {
            return Self::parse_ws_event(event, data);
        }
        Ok(None)
    }

    fn parse_ws_event(event: &str, data: &Value) -> Result<Option<WsEvent>, TradernetError> {
        match event {
            "q" => {
                let quote: Quote = serde_json::from_value(data.clone())?;
                Ok(Some(WsEvent::Quote(quote)))
            }
            "b" => {
                let depth: MarketDepthUpdate = serde_json::from_value(data.clone())?;
                Ok(Some(WsEvent::MarketDepth(depth)))
            }
            "portfolio" => {
                let portfolio: PortfolioUpdate = serde_json::from_value(data.clone())?;
                Ok(Some(WsEvent::Portfolio(portfolio)))
            }
            "orders" => {
                let orders: Vec<OrderDataRow> = serde_json::from_value(data.clone())?;
                Ok(Some(WsEvent::Orders(orders)))
            }
            "markets" => {
                let markets: MarketsUpdate = serde_json::from_value(data.clone())?;
                Ok(Some(WsEvent::Markets(markets)))
            }
            "error" => Ok(Some(WsEvent::Error(data.clone()))),
            _ => Ok(None),
        }
    }
}

impl TradernetWsSession {
    fn start(url: Url, reconnect: WsReconnectConfig) -> Self {
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let (events_tx, events_rx) = mpsc::unbounded_channel();
        let subscriptions = Arc::new(Mutex::new(SubscriptionState::default()));
        let closed = Arc::new(AtomicBool::new(false));

        let worker = tokio::spawn(run_session_loop(
            url,
            command_rx,
            events_tx,
            Arc::clone(&subscriptions),
            Arc::clone(&closed),
            reconnect,
        ));

        Self {
            command_tx,
            events_rx: Arc::new(Mutex::new(Some(events_rx))),
            subscriptions,
            closed,
            worker: Arc::new(Mutex::new(Some(worker))),
        }
    }

    /// Adds a subscription request with deduplication.
    pub async fn subscribe(&self, req: SubscribeRequest) -> Result<(), TradernetError> {
        if self.closed.load(Ordering::SeqCst) {
            return Err(TradernetError::InvalidInput(
                "websocket session is closed".to_string(),
            ));
        }

        let delta = {
            let mut state = self.subscriptions.lock().map_err(|_| {
                TradernetError::InvalidInput("subscriptions mutex poisoned".to_string())
            })?;
            state.apply_subscribe(req)
        };

        if let Some(delta) = delta {
            self.command_tx
                .send(SessionCommand::Subscribe(delta))
                .map_err(|_| {
                    TradernetError::InvalidInput(
                        "websocket session command channel closed".to_string(),
                    )
                })?;
        }

        Ok(())
    }

    /// Removes a subscription request.
    ///
    /// If server-side unsubscribe is unavailable for a channel, events are filtered
    /// locally according to active subscription state.
    pub async fn unsubscribe(&self, req: UnsubscribeRequest) -> Result<(), TradernetError> {
        if self.closed.load(Ordering::SeqCst) {
            return Err(TradernetError::InvalidInput(
                "websocket session is closed".to_string(),
            ));
        }

        let delta = {
            let mut state = self.subscriptions.lock().map_err(|_| {
                TradernetError::InvalidInput("subscriptions mutex poisoned".to_string())
            })?;
            state.apply_unsubscribe(req)
        };

        if let Some(delta) = delta {
            self.command_tx
                .send(SessionCommand::Unsubscribe(delta))
                .map_err(|_| {
                    TradernetError::InvalidInput(
                        "websocket session command channel closed".to_string(),
                    )
                })?;
        }

        Ok(())
    }

    /// Returns the session event stream.
    ///
    /// This receiver is single-consumer. Calling `events()` more than once returns
    /// a stream with a single `InvalidInput` error.
    pub fn events(&self) -> BoxStream<'static, Result<WsEvent, TradernetError>> {
        let maybe_rx = self
            .events_rx
            .lock()
            .ok()
            .and_then(|mut guard| guard.take());

        match maybe_rx {
            Some(mut rx) => Box::pin(stream! {
                while let Some(item) = rx.recv().await {
                    yield item;
                }
            }),
            None => Box::pin(stream! {
                yield Err(TradernetError::InvalidInput(
                    "events stream is already taken".to_string(),
                ));
            }),
        }
    }

    /// Closes the session and waits for background worker shutdown.
    pub async fn close(&self) -> Result<(), TradernetError> {
        if self.closed.swap(true, Ordering::SeqCst) {
            return Ok(());
        }

        let _ = self.command_tx.send(SessionCommand::Close);
        let handle = self
            .worker
            .lock()
            .map_err(|_| TradernetError::InvalidInput("worker mutex poisoned".to_string()))?
            .take();

        if let Some(handle) = handle {
            let _ = handle.await;
        }

        Ok(())
    }
}

impl Drop for TradernetWsSession {
    fn drop(&mut self) {
        if !self.closed.swap(true, Ordering::SeqCst) {
            let _ = self.command_tx.send(SessionCommand::Close);
        }
    }
}

async fn run_session_loop(
    url: Url,
    mut command_rx: mpsc::UnboundedReceiver<SessionCommand>,
    events_tx: mpsc::UnboundedSender<Result<WsEvent, TradernetError>>,
    subscriptions: Arc<Mutex<SubscriptionState>>,
    closed: Arc<AtomicBool>,
    reconnect: WsReconnectConfig,
) {
    let mut reconnect_attempt: u32 = 0;
    let mut has_connected_before = false;

    'outer: loop {
        while let Ok(command) = command_rx.try_recv() {
            if matches!(command, SessionCommand::Close) {
                break 'outer;
            }
        }

        if closed.load(Ordering::SeqCst) {
            break;
        }

        let connect_result = connect_async(url.to_string()).await;
        let (ws_stream, _) = match connect_result {
            Ok(connection) => connection,
            Err(error) => {
                log::warn!("websocket connection failed: {error}");
                let _ = events_tx.send(Err(error.into()));
                let _ = events_tx.send(Ok(WsEvent::Reconnecting));

                let delay = backoff_delay(reconnect, reconnect_attempt);
                reconnect_attempt = reconnect_attempt.saturating_add(1);
                sleep(delay).await;
                continue;
            }
        };

        reconnect_attempt = 0;
        if has_connected_before {
            log::info!("websocket reconnected");
        } else {
            log::info!("websocket connected");
        }
        has_connected_before = true;
        let _ = events_tx.send(Ok(WsEvent::Connected));

        let (mut write, mut read) = ws_stream.split();

        let active_requests = subscriptions
            .lock()
            .map(|state| state.active_requests())
            .unwrap_or_default();
        for req in active_requests {
            if let Err(error) = send_subscribe(&mut write, req).await {
                log::warn!("failed to restore subscription after reconnect: {error}");
                let _ = events_tx.send(Err(error));
                let _ = events_tx.send(Ok(WsEvent::Reconnecting));
                break;
            }
        }

        loop {
            tokio::select! {
                command = command_rx.recv() => {
                    let Some(command) = command else {
                        closed.store(true, Ordering::SeqCst);
                        break 'outer;
                    };

                    match command {
                        SessionCommand::Subscribe(req) => {
                            if let Err(error) = send_subscribe(&mut write, req).await {
                                log::warn!("failed to send subscribe request: {error}");
                                let _ = events_tx.send(Err(error));
                                let _ = events_tx.send(Ok(WsEvent::Reconnecting));
                                break;
                            }
                        }
                        SessionCommand::Unsubscribe(req) => {
                            log::info!("server-side unsubscribe is not used, local filtering applied: {req:?}");
                        }
                        SessionCommand::Close => {
                            closed.store(true, Ordering::SeqCst);
                            let _ = write.close().await;
                            break 'outer;
                        }
                    }
                }
                message = read.next() => {
                    match message {
                        Some(Ok(Message::Text(text))) => {
                            match TradernetWebsocket::parse_ws_message(text.as_ref()) {
                                Ok(Some(event)) => {
                                    let should_emit = subscriptions
                                        .lock()
                                        .map(|state| state.allows_event(&event))
                                        .unwrap_or(false);

                                    if should_emit {
                                        let _ = events_tx.send(Ok(event));
                                    }
                                }
                                Ok(None) => {}
                                Err(error) => {
                                    log::warn!("websocket message parse failed: {error}");
                                    let _ = events_tx.send(Err(error));
                                }
                            }
                        }
                        Some(Ok(Message::Close(_))) => {
                            log::warn!("websocket closed by server");
                            let _ = events_tx.send(Ok(WsEvent::Reconnecting));
                            break;
                        }
                        Some(Ok(_)) => {}
                        Some(Err(error)) => {
                            log::warn!("websocket read failed: {error}");
                            let _ = events_tx.send(Err(error.into()));
                            let _ = events_tx.send(Ok(WsEvent::Reconnecting));
                            break;
                        }
                        None => {
                            log::warn!("websocket stream ended");
                            let _ = events_tx.send(Ok(WsEvent::Reconnecting));
                            break;
                        }
                    }
                }
            }
        }

        if closed.load(Ordering::SeqCst) {
            break;
        }

        sleep(backoff_delay(reconnect, reconnect_attempt)).await;
        reconnect_attempt = reconnect_attempt.saturating_add(1);
    }

    let _ = events_tx.send(Ok(WsEvent::Closed));
    log::info!("websocket session closed");
}

async fn send_subscribe<S>(sink: &mut S, req: SubscribeRequest) -> Result<(), TradernetError>
where
    S: futures_util::Sink<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin,
{
    let payload = match req {
        SubscribeRequest::Quotes { symbols } => {
            serde_json::to_string(&serde_json::json!(["quotes", symbols]))?
        }
        SubscribeRequest::OrderBook { symbols } => {
            serde_json::to_string(&serde_json::json!(["orderBook", symbols]))?
        }
        SubscribeRequest::Portfolio => serde_json::to_string(&serde_json::json!(["portfolio"]))?,
        SubscribeRequest::Orders => serde_json::to_string(&serde_json::json!(["orders"]))?,
        SubscribeRequest::Markets => serde_json::to_string(&serde_json::json!(["markets"]))?,
    };

    sink.send(Message::Text(payload)).await?;
    Ok(())
}

fn backoff_delay(config: WsReconnectConfig, attempt: u32) -> std::time::Duration {
    let factor = config.multiplier.max(1.0);
    let base_ms = config.initial_delay.as_millis() as f64;
    let exponent = factor.powi(attempt as i32);
    let raw_ms = base_ms * exponent;
    let max_ms = config.max_delay.as_millis() as f64;
    let delay_ms = raw_ms.min(max_ms).max(0.0);
    std::time::Duration::from_millis(delay_ms as u64)
}

#[cfg(test)]
mod tests {
    use super::TradernetWebsocket;
    use crate::ws_types::{
        MarketDepthEvent, MarketDepthSide, MarketsEvent, OrdersEvent, PortfolioEvent, WsEvent,
    };
    use serde_json::json;

    #[test]
    fn parses_quote_event_q_to_ws_event() {
        let payload = json!({"c": "AAPL.US", "ltp": 123.0});

        let event = TradernetWebsocket::parse_ws_event("q", &payload)
            .expect("must parse")
            .expect("must return event");

        let WsEvent::Quote(quote) = event else {
            panic!("expected quote event")
        };

        assert_eq!(quote.c.as_deref(), Some("AAPL.US"));
    }

    #[test]
    fn parses_quote_event_q_legacy() {
        let payload = json!({"c": "AAPL.US", "ltp": 123.0});

        let event = TradernetWebsocket::parse_quote_event("q", &payload)
            .expect("must parse")
            .expect("must return event");

        let crate::ws_types::QuoteEvent::Quote(quote) = event else {
            panic!("expected quote event")
        };

        assert_eq!(quote.c.as_deref(), Some("AAPL.US"));
    }

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
