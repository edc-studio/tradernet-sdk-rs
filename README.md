# Tradernet SDK for Rust

Rust SDK for the Tradernet REST and WebSocket APIs. It is inspired by the official
Python SDK and provides a synchronous REST client, an asynchronous REST wrapper
for async runtimes, an asynchronous WebSocket streaming client, plus helpers for
symbols and options.

## Features

- REST client with typed helpers for Tradernet endpoints
- Async REST client for use in async runtimes
- WebSocket streaming for quotes, order books, portfolio, and markets
- Helpers for option notation and candle data parsing
- Simple configuration via INI files

## Installation

```bash
cargo add tradernet-sdk-rs
```

## Configuration

Create a `tradernet.ini` file:

```ini
[auth]
public = your_public_key
private = your_private_key
```

## REST usage

```rust
use tradernet_sdk_rs::Tradernet;

fn main() -> Result<(), tradernet_sdk_rs::TradernetError> {
    let client = Tradernet::from_config("tradernet.ini")?;
    let info = client.user_info()?;
    println!("{info:?}");

    let quotes = client.get_quotes(["AAPL.US", "TSLA.US"])?;
    println!("{quotes:?}");
    Ok(())
}
```

## Async REST usage

```rust
use tradernet_sdk_rs::AsyncTradernet;

#[tokio::main]
async fn main() -> Result<(), tradernet_sdk_rs::TradernetError> {
    let client = AsyncTradernet::from_config("tradernet.ini")?;
    let info = client.user_info().await?;
    println!("{info:?}");

    let quotes = client.get_quotes(["AAPL.US", "TSLA.US"]).await?;
    println!("{quotes:?}");
    Ok(())
}
```

## Trading example

```rust
use tradernet_sdk_rs::Tradernet;

fn main() -> Result<(), tradernet_sdk_rs::TradernetError> {
    let client = Tradernet::from_config("tradernet.ini")?;
    let order = client.buy("FRHC.US", 1, 0.0, "day", false, None)?;
    println!("{order:?}");
    Ok(())
}
```

## WebSocket streaming

```rust
use futures_util::StreamExt;
use tradernet_sdk_rs::{Core, TradernetWebsocket};

#[tokio::main]
async fn main() -> Result<(), tradernet_sdk_rs::TradernetError> {
    let core = Core::from_config("tradernet.ini")?;
    let ws = TradernetWebsocket::new(core);
    let mut stream = ws.quotes(["AAPL.US", "TSLA.US"]).await?;

    while let Some(message) = stream.next().await {
        println!("{message:?}");
    }
    Ok(())
}
```

## Options helper

```rust
use tradernet_sdk_rs::TradernetOption;

fn main() -> Result<(), tradernet_sdk_rs::TradernetError> {
    let option = TradernetOption::new("+FRHC.16SEP2022.C55")?;
    println!("{option}");
    Ok(())
}
```

## Symbol candle helper

```rust
use chrono::{NaiveDate, NaiveDateTime};
use tradernet_sdk_rs::{Tradernet, TradernetSymbol};

fn main() -> Result<(), tradernet_sdk_rs::TradernetError> {
    let api = Tradernet::from_config("tradernet.ini")?;
    let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap().and_hms_opt(0, 0, 0).unwrap();

    let mut symbol = TradernetSymbol::new("AAPL.US", Some(api), start, end);
    symbol.get_data()?;
    println!("Loaded {} candles", symbol.candles.len());
    Ok(())
}
```

## Documentation

- Tradernet API docs: `https://freedom24.com/tradernet-api/`
- Rust API docs: build with `cargo doc --open`

## Disclaimer

This project is community-driven and is not affiliated with Freedom24/Tradernet.
Use it at your own risk and make sure you comply with your broker's terms.

## License

MIT. See `LICENSE` in the repository root.