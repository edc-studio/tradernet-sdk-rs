# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-03-26

### Breaking Changes

- Changed `TradernetWebsocket::new` signature:
  - from: `TradernetWebsocket::new(core: Core)`
  - to: `TradernetWebsocket::new(public: Option<String>, private: Option<String>)`
- WebSocket streaming methods now return typed event streams instead of `serde_json::Value`:
  - `quotes` -> `QuoteEvent`
  - `market_depth` -> `MarketDepthEvent`
  - `portfolio` -> `PortfolioEvent`
  - `orders` -> `OrdersEvent`
  - `markets` -> `MarketsEvent`

### Added

- New public module `ws_types` with typed WebSocket events and payloads:
  `QuoteEvent`, `MarketDepthEvent`, `PortfolioEvent`, `OrdersEvent`, `MarketsEvent`,
  `WsEvent`, `SubscribeRequest`, `UnsubscribeRequest`, `WsReconnectConfig`,
  and related DTOs.
- New `TradernetWsSession` API for one-connection multi-subscription workflows:
  - `connect` / `connect_with_config`
  - `subscribe` / `unsubscribe`
  - `events`
  - `close`
- New `WsCredentials` type and credential helpers in `Core`/`AsyncCore`:
  - `ws_credentials()`
  - `websocket_auth()`
- Additional WebSocket constructors:
  - `TradernetWebsocket::from_core`
  - `TradernetWebsocket::from_async_core`
  - `TradernetWebsocket::with_websocket_url`
  - `TradernetWebsocket::with_websocket_url_from_core`
  - `TradernetWebsocket::with_websocket_url_from_async_core`

### Changed

- WebSocket internals were refactored to support:
  - session lifecycle events (`Connected`, `Reconnecting`, `Closed`)
  - automatic reconnect with backoff and subscription replay
  - local unsubscribe filtering semantics
- Improved deserialization robustness for `markets` payload (`Markets.m`) and WS payload parsing.

### Dependencies

- Added direct dependency: `rustls`.
- Enabled `tokio-tungstenite` feature: `rustls-tls-webpki-roots`.
- Extended `tokio` features (`net`, `sync`, `time`) for session/reconnect infrastructure.

### Migration Notes (0.1.1 -> 0.2.0)

- If you used `TradernetWebsocket::new(core)`, migrate to:
  - `TradernetWebsocket::from_core(&core)`, or
  - `TradernetWebsocket::new(public, private)`.
- Update WebSocket stream handling to match typed enums instead of raw JSON.
- For multiple subscriptions over one socket, switch to `connect()` + `TradernetWsSession`.

