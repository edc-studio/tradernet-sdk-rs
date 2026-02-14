use crate::core::Core;
use crate::errors::TradernetError;
use async_stream::try_stream;
use futures_util::stream::BoxStream;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use std::collections::HashSet;
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
    pub async fn quotes<I, S>(&self, symbols: I) -> Result<BoxStream<'static, Result<Value, TradernetError>>, TradernetError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let symbols = symbols.into_iter().map(|s| s.as_ref().to_string()).collect::<Vec<_>>();
        let query = serde_json::to_string(&serde_json::json!(["quotes", symbols]))?;
        self.stream_filtered(vec![query], &["q", "error"]).await
    }

    /// Subscribes to order book updates for a symbol.
    pub async fn market_depth(&self, symbol: &str) -> Result<BoxStream<'static, Result<Value, TradernetError>>, TradernetError> {
        let query = serde_json::to_string(&serde_json::json!(["orderBook", [symbol]]))?;
        self.stream_filtered(vec![query], &["b", "error"]).await
    }

    /// Subscribes to portfolio updates.
    pub async fn portfolio(&self) -> Result<BoxStream<'static, Result<Value, TradernetError>>, TradernetError> {
        let query = serde_json::to_string(&serde_json::json!(["portfolio"]))?;
        self.stream_filtered(vec![query], &["portfolio", "error"]).await
    }

    /// Subscribes to active orders updates.
    pub async fn orders(&self) -> Result<BoxStream<'static, Result<Value, TradernetError>>, TradernetError> {
        let query = serde_json::to_string(&serde_json::json!(["orders"]))?;
        self.stream_filtered(vec![query], &["orders", "error"]).await
    }

    /// Subscribes to markets status updates.
    pub async fn markets(&self) -> Result<BoxStream<'static, Result<Value, TradernetError>>, TradernetError> {
        let query = serde_json::to_string(&serde_json::json!(["markets"]))?;
        self.stream_filtered(vec![query], &["markets", "error"]).await
    }

    async fn stream_filtered(
        &self,
        query: Vec<String>,
        allowed_events: &[&str],
    ) -> Result<BoxStream<'static, Result<Value, TradernetError>>, TradernetError> {
        let url = self.websocket_url_with_auth()?;
        let (ws_stream, _) = connect_async(url.to_string()).await?;
        let (mut write, mut read) = ws_stream.split();

        for message in query {
            write.send(Message::Text(message)).await?;
        }

        let allowed = allowed_events.iter().map(|event| event.to_string()).collect::<HashSet<_>>();
        let stream = try_stream! {
            while let Some(message) = read.next().await {
                let message = message?;
                if let Message::Text(text) = message {
                    let parsed: Value = serde_json::from_str(&text)?;
                    if let Some((event, data)) = parsed.as_array().and_then(|values| {
                        let event = values.first()?.as_str()?;
                        let data = values.get(1)?;
                        Some((event, data))
                    })
                        && allowed.contains(event)
                    {
                        yield data.clone();
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
}