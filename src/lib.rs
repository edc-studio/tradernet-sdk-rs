//! Rust SDK for the Tradernet API.
//!
//! This crate provides a synchronous REST client, an asynchronous WebSocket client,
//! and helper types for working with symbols and options.
//!
//! # Quick start
//! ```no_run
//! use tradernet_sdk_rs::Tradernet;
//!
//! # fn main() -> Result<(), tradernet_sdk_rs::TradernetError> {
//! let client = Tradernet::new(Some("public_key".into()), Some("private_key".into()))?;
//! let info = client.user_info()?;
//! println!("{info:?}");
//! # Ok(())
//! # }
//! ```
//!
//! See [`TradernetWebsocket`] for streaming market data.

/// REST API client built on top of [`Core`].
pub mod client;
/// Common networking and string helpers.
pub mod common;
/// Core authentication and request utilities.
pub mod core;
/// Error types returned by the SDK.
pub mod errors;
/// Symbols and options helpers.
pub mod symbols;
/// Typed responses for get_user_data.
pub mod user_data;
/// WebSocket streaming client.
pub mod ws;

pub use crate::client::Tradernet;
pub use crate::core::Core;
pub use crate::errors::TradernetError;
pub use crate::symbols::tradernet_option::TradernetOption;
pub use crate::symbols::tradernet_symbol::TradernetSymbol;
pub use crate::user_data::UserDataResponse;
pub use crate::ws::TradernetWebsocket;
