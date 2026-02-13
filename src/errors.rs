use thiserror::Error;

#[derive(Error, Debug)]
pub enum TradernetError {
    #[error("missing API keypair")]
    MissingKeypair,
    #[error("unsupported API version: {0}")]
    UnsupportedApiVersion(u8),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("websocket error: {0}")]
    Websocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("url parse error: {0}")]
    Url(#[from] url::ParseError),
}