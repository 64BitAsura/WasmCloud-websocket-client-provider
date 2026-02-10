use thiserror::Error;

/// Custom error types for the WebSocket provider
#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("WebSocket connection error: {0}")]
    WebSocketError(#[from] Box<tungstenite::Error>),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("NATS error: {0}")]
    NatsError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("URL parsing error: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("Provider error: {0}")]
    ProviderSdkError(String),

    #[error("Connection closed")]
    ConnectionClosed,

    #[error("Invalid message format: {0}")]
    InvalidMessage(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl From<tungstenite::Error> for ProviderError {
    fn from(err: tungstenite::Error) -> Self {
        ProviderError::WebSocketError(Box::new(err))
    }
}

/// Result type for provider operations
pub type ProviderResult<T> = Result<T, ProviderError>;
