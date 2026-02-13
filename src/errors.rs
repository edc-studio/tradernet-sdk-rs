use thiserror::Error;

/// Errors returned by the Tradernet SDK.
#[derive(Error, Debug)]
pub enum TradernetError {
    /// Missing API keypair (public or private).
    #[error("missing API keypair")]
    MissingKeypair,
    /// Unsupported API version.
    #[error("unsupported API version: {0}")]
    UnsupportedApiVersion(u8),
    /// Invalid input passed to an SDK method.
    #[error("invalid input: {0}")]
    InvalidInput(String),
    /// HTTP transport error.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    /// I/O error.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// JSON serialization/deserialization error.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    /// WebSocket transport error.
    #[error("websocket error: {0}")]
    Websocket(#[from] tokio_tungstenite::tungstenite::Error),
    /// URL parsing error.
    #[error("url parse error: {0}")]
    Url(#[from] url::ParseError),
    /// Zip archive error.
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
}