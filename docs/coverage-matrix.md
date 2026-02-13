# Матрица покрытия Tradernet SDK (Rust)

Источник данных для сопоставления:
- Rust SDK: `src/client.rs`, `src/core.rs`, `src/ws.rs`, `src/common/*`, `src/symbols/*`
- Python SDK: `3th-party/tradernet_sdk-2.0.0/tradernet/*`
- Официальная документация: `https://freedom24.com/tradernet-api/` (страница загружается динамически; идентификаторы примеров/разделов из HTML извлечь не удалось — в матрице отмечено как «не подтверждено»)

Статусы:
- ✅ реализовано в Rust
- ⚠️ частично/аналоги (функциональность есть, но отличается интерфейс или доступ через приватные методы Python)
- ❌ отсутствует
- ❔ невозможно проверить по официальной документации (динамический контент)

## Core / авторизация / сетевые запросы

| Python SDK | Rust SDK | Статус | Примечание |
| --- | --- | --- | --- |
| `Core.__init__` | `Core::new`, `Tradernet::new` | ✅ | Инициализация по ключам |
| `Core.from_config` | `Core::from_config`, `Tradernet::from_config` | ✅ | Чтение конфигурации |
| `Core.url` | `Core::url` | ✅ | Базовый URL |
| `Core.websocket_url` | `Core::websocket_url` | ✅ | WS URL |
| `Core.websocket_auth` | `Core::websocket_auth` | ✅ | WS параметры авторизации |
| `Core.plain_request` | `Core::plain_request` | ✅ | Неавторизованный запрос |
| `Core.authorized_request` | `Core::authorized_request` | ✅ | Авторизованный запрос |
| `Core.list_security_sessions` | `Core::list_security_sessions` | ✅ | Сессии бумаг |
| `NetUtils.request` | `NetUtils::request` | ✅ | HTTP‑запросы |
| `StringUtils.stringify` | `string_utils::stringify` | ✅ | JSON stringify |
| `StringUtils.sign` | `string_utils::sign` | ✅ | HMAC‑подпись |
| `StringUtils.http_build_query` | `string_utils::http_build_query` | ✅ | Query из структуры |
| `StringUtils.flatten_*` и `str_from_*` | `string_utils` (внутренние функции) | ✅ | В Rust внутренние helpers |
| `WSUtils.__init__/__ssl_context` | (нет прямого аналога) | ⚠️ | В Rust WS реализован иначе, без SSL‑утилит из Python |

## REST‑клиент (Python `client.py` → Rust `Tradernet`)

| Python SDK | Rust SDK | Статус | Примечание |
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
| `__refbooks` | `Tradernet::refbooks` | ⚠️ | В Python приватный метод, в Rust публичный |
| `__get_refbook` | `Tradernet::get_refbook` | ⚠️ | В Python приватный метод, в Rust публичный |
| `__latest_refbook` | `Tradernet::latest_refbook` | ⚠️ | В Python приватный метод, в Rust публичный |
| `__extract_zip` | (нет отдельного аналога) | ⚠️ | В Rust разархивация скрыта внутри `get_refbook` |

## WebSocket

| Python SDK | Rust SDK | Статус | Примечание |
| --- | --- | --- | --- |
| `TradernetWebsocket.quotes` | `TradernetWebsocket::quotes` | ✅ | |
| `TradernetWebsocket.market_depth` | `TradernetWebsocket::market_depth` | ✅ | |
| `TradernetWebsocket.portfolio` | `TradernetWebsocket::portfolio` | ✅ | |
| `TradernetWebsocket.orders` | `TradernetWebsocket::orders` | ✅ | |
| `TradernetWebsocket.markets` | `TradernetWebsocket::markets` | ✅ | |
| `TraderNetWSAPI` (deprecated wrapper) | отсутствует | ✅ | Не требуется, Python помечен как deprecated |

## Symbols / опционы

| Python SDK | Rust SDK | Статус | Примечание |
| --- | --- | --- | --- |
| `TradernetSymbol.get_data` | `TradernetSymbol::get_candles` | ⚠️ | Python возвращает сырой `get_candles`; Rust выделяет данные по тайм‑фреймам |
| `OptionProperties` (dataclass) | `OptionProperties` (struct) | ✅ | Модель совпадает |
| `TradernetOption` (parser/formatter) | `TradernetOption` | ✅ | Утилиты работы с опционами |

## Официальная документация API (freedom24)

| Раздел/пример (из документации) | Наличие в Rust | Статус | Примечание |
| --- | --- | --- | --- |
| Набор примеров/разделов документации | — | ❔ | Документация грузится динамически, идентификаторы примеров извлечь не удалось без JS |

## Резюме

- По сравнению с Python SDK 2.0.0: Rust‑реализация покрывает все публичные методы `client.py`, а также совпадающий набор WebSocket‑подписок. Отличия — в интерфейсе справочников (в Rust публичные методы вместо приватных) и в реализации утилит.
- По сравнению с официальной документацией: автоматическая выборка списка примеров/разделов без выполнения JS не получилась; требуется либо ручной доступ к документации, либо запрос отдельных JSON‑эндпоинтов, чтобы подтвердить полный охват.