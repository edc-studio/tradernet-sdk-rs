# Tradernet SDK (Rust) Coverage Matrix

Data sources for comparison:
- Rust SDK: `src/client.rs`, `src/core.rs`, `src/ws.rs`, `src/common/*`, `src/symbols/*`
- Python SDK: `3th-party/tradernet_sdk-2.0.0/tradernet/*`
- Official documentation: `https://freedom24.com/tradernet-api/` (the page loads dynamically; example/section identifiers could not be extracted from HTML — marked as “unverified” in the matrix)

Statuses:
- ✅ implemented in Rust
- ⚠️ partial/analogues (functionality exists but interface differs or access is via Python private methods)
- ❌ missing
- ❔ impossible to verify via official documentation (dynamic content)

## Core / authorization / network requests

| Python SDK | Rust SDK | Status | Notes |
| --- | --- | --- | --- |
| `Core.__init__` | `Core::new`, `Tradernet::new` | ✅ | Initialization with keys |
| `Core.from_config` | `Core::from_config`, `Tradernet::from_config` | ✅ | Read configuration |
| `Core.url` | `Core::url` | ✅ | Base URL |
| `Core.websocket_url` | `Core::websocket_url` | ✅ | WS URL |
| `Core.websocket_auth` | `Core::websocket_auth` | ✅ | WS auth parameters |
| `Core.plain_request` | `Core::plain_request` | ✅ | Unauthorized request |
| `Core.authorized_request` | `Core::authorized_request` | ✅ | Authorized request |
| `Core.list_security_sessions` | `Core::list_security_sessions` | ✅ | Security sessions |
| `NetUtils.request` | `NetUtils::request` | ✅ | HTTP requests |
| `StringUtils.stringify` | `string_utils::stringify` | ✅ | JSON stringify |
| `StringUtils.sign` | `string_utils::sign` | ✅ | HMAC signature |
| `StringUtils.http_build_query` | `string_utils::http_build_query` | ✅ | Query from struct |
| `StringUtils.flatten_*` and `str_from_*` | `string_utils` (internal functions) | ✅ | Internal helpers in Rust |
| `WSUtils.__init__/__ssl_context` | (no direct analogue) | ⚠️ | WS in Rust is implemented differently, without Python SSL utilities |

## REST client (Python `client.py` → Rust `Tradernet`)

| Python SDK | Rust SDK | Status | Notes |
| --- | --- | --- | --- |
| `new_user` | `Tradernet::new_user` | ✅ | |
| `check_missing_fields` | `Tradernet::check_missing_fields` | ✅ | |
| `get_profile_fields` | `Tradernet::get_profile_fields` | ✅ | |
| `user_info` | `Tradernet::user_info` | ✅ | |
| `get_user_data` | `Tradernet::get_user_data` | ✅ | |
| `get_market_status` | `Tradernet::get_market_status` | ✅ | |
| `security_info` | `Tradernet::security_info` | ✅ | |
| `get_options` | `Tradernet::get_options` | ✅ | |
| `get_most_traded` | `Tradernet::get_most_traded` | ✅ | |
| `export_securities` | `Tradernet::export_securities` | ✅ | |
| `get_candles` | `Tradernet::get_candles` | ✅ | |
| `get_trades_history` | `Tradernet::get_trades_history` | ✅ | |
| `find_symbol` | `Tradernet::find_symbol` | ✅ | |
| `get_news` | `Tradernet::get_news` | ✅ | |
| `get_all` | `Tradernet::get_all` | ✅ | |
| `account_summary` | `Tradernet::account_summary` | ✅ | |
| `get_price_alerts` | `Tradernet::get_price_alerts` | ✅ | |
| `add_price_alert` | `Tradernet::add_price_alert` | ✅ | |
| `delete_price_alert` | `Tradernet::delete_price_alert` | ✅ | |
| `get_requests_history` | `Tradernet::get_requests_history` | ✅ | |
| `get_order_files` | `Tradernet::get_order_files` | ✅ | |
| `get_broker_report` | `Tradernet::get_broker_report` | ✅ | |
| `symbol` | `Tradernet::symbol` | ✅ | |
| `symbols` | `Tradernet::symbols` | ✅ | |
| `corporate_actions` | `Tradernet::corporate_actions` | ✅ | |
| `get_quotes` | `Tradernet::get_quotes` | ✅ | |
| `buy` | `Tradernet::buy` | ✅ | |
| `sell` | `Tradernet::sell` | ✅ | |
| `stop` | `Tradernet::stop` | ✅ | |
| `trailing_stop` | `Tradernet::trailing_stop` | ✅ | |
| `take_profit` | `Tradernet::take_profit` | ✅ | |
| `cancel` | `Tradernet::cancel` | ✅ | |
| `cancel_all` | `Tradernet::cancel_all` | ✅ | |
| `get_placed` | `Tradernet::get_placed` | ✅ | |
| `get_historical` | `Tradernet::get_historical` | ✅ | |
| `trade` | `Tradernet::trade` | ✅ | |
| `get_tariffs_list` | `Tradernet::get_tariffs_list` | ✅ | |
| `__refbooks` | `Tradernet::refbooks` | ⚠️ | Private method in Python, public in Rust |
| `__get_refbook` | `Tradernet::get_refbook` | ⚠️ | Private method in Python, public in Rust |
| `__latest_refbook` | `Tradernet::latest_refbook` | ⚠️ | Private method in Python, public in Rust |
| `__extract_zip` | (no separate analogue) | ⚠️ | Decompression is hidden inside `get_refbook` in Rust |

## WebSocket

| Python SDK | Rust SDK | Status | Notes |
| --- | --- | --- | --- |
| `TradernetWebsocket.quotes` | `TradernetWebsocket::quotes` | ✅ | |
| `TradernetWebsocket.market_depth` | `TradernetWebsocket::market_depth` | ✅ | |
| `TradernetWebsocket.portfolio` | `TradernetWebsocket::portfolio` | ✅ | |
| `TradernetWebsocket.orders` | `TradernetWebsocket::orders` | ✅ | |
| `TradernetWebsocket.markets` | `TradernetWebsocket::markets` | ✅ | |
| `TraderNetWSAPI` (deprecated wrapper) | missing | ✅ | Not required, Python marked as deprecated |

## Symbols / options

| Python SDK | Rust SDK | Status | Notes |
| --- | --- | --- | --- |
| `TradernetSymbol.get_data` | `TradernetSymbol::get_candles` | ⚠️ | Python returns raw `get_candles`; Rust splits data by timeframes |
| `OptionProperties` (dataclass) | `OptionProperties` (struct) | ✅ | Model matches |
| `TradernetOption` (parser/formatter) | `TradernetOption` | ✅ | Option utilities |

## Official API documentation (freedom24)

| Section/example (from documentation) | Presence in Rust | Status | Notes |
| --- | --- | --- | --- |
| Set of documentation examples/sections | — | ❔ | Documentation loads dynamically; example identifiers could not be extracted without JS |

## Summary

- Compared to Python SDK 2.0.0: the Rust implementation covers all public methods in `client.py`, and the same set of WebSocket subscriptions. Differences are in the reference book interface (public methods in Rust instead of private) and in utility implementations.
- Compared to the official documentation: automatic extraction of the list of examples/sections without executing JS did not work; either manual access to the documentation or requests to specific JSON endpoints are required to confirm full coverage.