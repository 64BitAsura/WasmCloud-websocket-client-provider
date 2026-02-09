//! WebSocket client module for receiving messages

use anyhow::{Context, Result};
use futures_util::StreamExt;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, error, info, warn};

use crate::config::ProviderConfig;

/// WebSocket client that connects to a remote server and receives messages
pub struct WebSocketClient {
    config: Arc<ProviderConfig>,
    message_tx: mpsc::UnboundedSender<Vec<u8>>,
}

impl WebSocketClient {
    /// Create a new WebSocket client
    pub fn new(config: Arc<ProviderConfig>, message_tx: mpsc::UnboundedSender<Vec<u8>>) -> Self {
        Self { config, message_tx }
    }

    /// Run the WebSocket client with auto-reconnection
    pub async fn run(&self) {
        let mut attempt = 0u32;

        loop {
            info!(
                "Connecting to WebSocket server: {}",
                self.config.websocket_url
            );

            match self.connect_and_receive().await {
                Ok(_) => {
                    info!("WebSocket connection closed normally");
                    attempt = 0; // Reset attempts on successful connection
                }
                Err(e) => {
                    error!("WebSocket connection error: {:#}", e);
                }
            }

            // Check if we should retry
            if self.config.max_reconnect_attempts > 0
                && attempt >= self.config.max_reconnect_attempts
            {
                error!(
                    "Maximum reconnection attempts ({}) reached. Giving up.",
                    self.config.max_reconnect_attempts
                );
                break;
            }

            attempt += 1;
            let delay = self.calculate_backoff(attempt);
            warn!(
                "Reconnecting in {} seconds (attempt {})...",
                delay.as_secs(),
                attempt
            );
            sleep(delay).await;
        }
    }

    /// Connect to WebSocket server and receive messages
    async fn connect_and_receive(&self) -> Result<()> {
        let (ws_stream, _) = connect_async(&self.config.websocket_url)
            .await
            .context("Failed to connect to WebSocket server")?;

        info!("WebSocket connection established");

        let (_, mut read) = ws_stream.split();

        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => {
                    if let Err(e) = self.handle_message(msg).await {
                        error!("Error handling message: {:#}", e);
                    }
                }
                Err(e) => {
                    return Err(anyhow::anyhow!("WebSocket error: {}", e));
                }
            }
        }

        Ok(())
    }

    /// Handle a WebSocket message
    async fn handle_message(&self, message: Message) -> Result<()> {
        match message {
            Message::Text(text) => {
                debug!("Received text message: {} bytes", text.len());
                self.message_tx
                    .send(text.into_bytes())
                    .context("Failed to send message to channel")?;
            }
            Message::Binary(data) => {
                debug!("Received binary message: {} bytes", data.len());
                self.message_tx
                    .send(data)
                    .context("Failed to send message to channel")?;
            }
            Message::Close(_) => {
                info!("Received close message from server");
                return Err(anyhow::anyhow!("Connection closed by server"));
            }
            Message::Ping(_) | Message::Pong(_) => {
                debug!("Received ping/pong message");
            }
            Message::Frame(_) => {
                debug!("Received frame message");
            }
        }
        Ok(())
    }

    /// Calculate exponential backoff delay
    fn calculate_backoff(&self, attempt: u32) -> Duration {
        let base_delay = self.config.reconnect_interval_secs;
        let exponential_delay = base_delay * 2u64.pow(attempt.saturating_sub(1));
        let capped_delay = exponential_delay.min(300); // Cap at 5 minutes
        Duration::from_secs(capped_delay)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_websocket_client_creation() {
        let config = Arc::new(ProviderConfig::default());
        let (tx, _rx) = mpsc::unbounded_channel();
        let client = WebSocketClient::new(config, tx);
        assert!(client.config.reconnect_interval_secs > 0);
    }

    #[test]
    fn test_handle_text_message() {
        let config = Arc::new(ProviderConfig::default());
        let (tx, mut rx) = mpsc::unbounded_channel();
        let client = WebSocketClient::new(config, tx);

        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let message = Message::Text("test message".to_string());
            client.handle_message(message).await.unwrap();

            let received = rx.recv().await.unwrap();
            assert_eq!(received, b"test message");
        });
    }

    #[test]
    fn test_handle_binary_message() {
        let config = Arc::new(ProviderConfig::default());
        let (tx, mut rx) = mpsc::unbounded_channel();
        let client = WebSocketClient::new(config, tx);

        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let data = vec![1, 2, 3, 4, 5];
            let message = Message::Binary(data.clone());
            client.handle_message(message).await.unwrap();

            let received = rx.recv().await.unwrap();
            assert_eq!(received, data);
        });
    }

    #[test]
    fn test_handle_close_message() {
        let config = Arc::new(ProviderConfig::default());
        let (tx, _rx) = mpsc::unbounded_channel();
        let client = WebSocketClient::new(config, tx);

        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let message = Message::Close(None);
            let result = client.handle_message(message).await;
            assert!(result.is_err());
        });
    }
}
