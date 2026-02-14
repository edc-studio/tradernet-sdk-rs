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
    Http(#[from] Box<reqwest::Error>),
    /// I/O error.
    #[error("io error: {0}")]
    Io(#[from] Box<std::io::Error>),
    /// JSON serialization/deserialization error.
    #[error("json error: {0}")]
    Json(#[from] Box<serde_json::Error>),
    /// WebSocket transport error.
    #[error("websocket error: {0}")]
    Websocket(#[from] Box<tokio_tungstenite::tungstenite::Error>),
    /// URL parsing error.
    #[error("url parse error: {0}")]
    Url(#[from] Box<url::ParseError>),
    /// Zip archive error.
    #[error("zip error: {0}")]
    Zip(#[from] Box<zip::result::ZipError>),
}

impl From<reqwest::Error> for TradernetError {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(Box::new(error))
    }
}

impl From<std::io::Error> for TradernetError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(Box::new(error))
    }
}

impl From<serde_json::Error> for TradernetError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(Box::new(error))
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for TradernetError {
    fn from(error: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::Websocket(Box::new(error))
    }
}

impl From<url::ParseError> for TradernetError {
    fn from(error: url::ParseError) -> Self {
        Self::Url(Box::new(error))
    }
}

impl From<zip::result::ZipError> for TradernetError {
    fn from(error: zip::result::ZipError) -> Self {
        Self::Zip(Box::new(error))
    }
}
