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

/// WebSocket client that receives messages from a remote server
pub struct WebSocketClient {
    config: Arc<ProviderConfig>,
    message_tx: mpsc::UnboundedSender<Vec<u8>>,
}

impl WebSocketClient {
    /// Create a new WebSocket client
    pub fn new(config: Arc<ProviderConfig>, message_tx: mpsc::UnboundedSender<Vec<u8>>) -> Self {
        Self { config, message_tx }
    }

    /// Start the WebSocket client with automatic reconnection
    pub async fn run(&self) {
        let mut attempt = 0u32;

        loop {
            info!(
                "Attempting to connect to WebSocket server: {}",
                self.config.websocket_url
            );

            match self.connect_and_listen().await {
                Ok(()) => {
                    info!("WebSocket connection closed normally");
                    // Reset attempt counter on successful connection
                    attempt = 0;
                }
                Err(e) => {
                    error!("WebSocket connection error: {:#}", e);
                }
            }

            // Check if we should attempt reconnection
            if self.config.max_reconnect_attempts > 0 {
                attempt += 1;
                if attempt >= self.config.max_reconnect_attempts {
                    error!(
                        "Maximum reconnection attempts ({}) reached. Stopping.",
                        self.config.max_reconnect_attempts
                    );
                    break;
                }
            }

            // Wait before reconnecting with exponential backoff
            let backoff_secs = std::cmp::min(
                self.config.reconnect_interval_secs * (1 << std::cmp::min(attempt, 5)),
                300, // Max 5 minutes
            );

            warn!(
                "Reconnecting in {} seconds (attempt {})...",
                backoff_secs,
                attempt + 1
            );
            sleep(Duration::from_secs(backoff_secs)).await;
        }
    }

    /// Connect to WebSocket server and listen for messages
    async fn connect_and_listen(&self) -> Result<()> {
        // Connect to the WebSocket server
        let (ws_stream, _) = connect_async(&self.config.websocket_url)
            .await
            .context("Failed to connect to WebSocket server")?;

        info!("Connected to WebSocket server successfully");

        let (_write, mut read) = ws_stream.split();

        // Listen for incoming messages
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => {
                    if let Err(e) = self.handle_message(msg).await {
                        warn!("Error handling message: {:#}", e);
                    }
                }
                Err(e) => {
                    return Err(anyhow::anyhow!("WebSocket error: {}", e));
                }
            }
        }

        Ok(())
    }

    /// Handle an incoming WebSocket message
    async fn handle_message(&self, message: Message) -> Result<()> {
        match message {
            Message::Text(text) => {
                debug!("Received text message: {} bytes", text.len());
                self.message_tx
                    .send(text.into_bytes())
                    .context("Failed to forward message")?;
            }
            Message::Binary(data) => {
                debug!("Received binary message: {} bytes", data.len());
                self.message_tx
                    .send(data)
                    .context("Failed to forward message")?;
            }
            Message::Ping(_) => {
                debug!("Received ping");
                // Pong is automatically sent by tungstenite
            }
            Message::Pong(_) => {
                debug!("Received pong");
            }
            Message::Close(frame) => {
                info!("Received close frame: {:?}", frame);
                return Err(anyhow::anyhow!("Connection closed by server"));
            }
            Message::Frame(_) => {
                // Raw frames are not expected in normal operation
                debug!("Received raw frame");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_websocket_client_creation() {
        let config = Arc::new(ProviderConfig::default());
        let (tx, _rx) = mpsc::unbounded_channel();
        let client = WebSocketClient::new(config.clone(), tx);

        assert_eq!(client.config.websocket_url, config.websocket_url);
    }

    #[tokio::test]
    async fn test_handle_text_message() {
        let config = Arc::new(ProviderConfig::default());
        let (tx, mut rx) = mpsc::unbounded_channel();
        let client = WebSocketClient::new(config, tx);

        let message = Message::Text("Hello, World!".to_string());
        client.handle_message(message).await.unwrap();

        let received = rx.recv().await.unwrap();
        assert_eq!(received, b"Hello, World!");
    }

    #[tokio::test]
    async fn test_handle_binary_message() {
        let config = Arc::new(ProviderConfig::default());
        let (tx, mut rx) = mpsc::unbounded_channel();
        let client = WebSocketClient::new(config, tx);

        let data = vec![1, 2, 3, 4, 5];
        let message = Message::Binary(data.clone());
        client.handle_message(message).await.unwrap();

        let received = rx.recv().await.unwrap();
        assert_eq!(received, data);
    }

    #[tokio::test]
    async fn test_handle_close_message() {
        let config = Arc::new(ProviderConfig::default());
        let (tx, _rx) = mpsc::unbounded_channel();
        let client = WebSocketClient::new(config, tx);

        let message = Message::Close(None);
        let result = client.handle_message(message).await;

        assert!(result.is_err());
    }
}
