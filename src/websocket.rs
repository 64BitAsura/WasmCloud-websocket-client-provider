use crate::config::ProviderConfig;
use crate::error::{ProviderError, ProviderResult};
use futures_util::StreamExt;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, error, info, warn};

/// WebSocket client handler
pub struct WebSocketClient {
    config: ProviderConfig,
}

impl WebSocketClient {
    /// Create a new WebSocket client
    pub fn new(config: ProviderConfig) -> Self {
        Self { config }
    }

    /// Connect to the WebSocket server and start receiving messages
    pub async fn run<F>(&self, mut message_handler: F) -> ProviderResult<()>
    where
        F: FnMut(Vec<u8>) -> ProviderResult<()> + Send,
    {
        let mut reconnect_attempts = 0u32;
        let mut current_delay = self.config.initial_reconnect_delay();

        loop {
            match self.connect_and_receive(&mut message_handler).await {
                Ok(_) => {
                    info!("WebSocket connection closed normally");
                    break Ok(());
                }
                Err(e) => {
                    error!("WebSocket connection error: {}", e);

                    // Check if we should retry
                    if self.config.max_reconnect_attempts > 0
                        && reconnect_attempts >= self.config.max_reconnect_attempts
                    {
                        error!(
                            "Maximum reconnection attempts ({}) reached",
                            self.config.max_reconnect_attempts
                        );
                        return Err(e);
                    }

                    reconnect_attempts += 1;
                    warn!(
                        "Attempting reconnection #{} after {:?}",
                        reconnect_attempts, current_delay
                    );

                    sleep(current_delay).await;

                    // Exponential backoff with max limit
                    current_delay =
                        std::cmp::min(current_delay * 2, self.config.max_reconnect_delay());
                }
            }
        }
    }

    /// Connect to WebSocket server and receive messages
    async fn connect_and_receive<F>(&self, message_handler: &mut F) -> ProviderResult<()>
    where
        F: FnMut(Vec<u8>) -> ProviderResult<()>,
    {
        info!(
            "Connecting to WebSocket server: {}",
            self.config.websocket_url
        );

        let (ws_stream, response) = connect_async(&self.config.websocket_url).await?;

        info!("WebSocket connection established: {:?}", response.status());
        debug!("Response headers: {:?}", response.headers());

        let (_, mut read) = ws_stream.split();

        // Receive messages
        while let Some(message_result) = read.next().await {
            match message_result {
                Ok(message) => match message {
                    Message::Text(text) => {
                        debug!("Received text message: {} bytes", text.len());
                        if text.len() > self.config.max_message_size {
                            warn!(
                                "Message size {} exceeds limit {}, skipping",
                                text.len(),
                                self.config.max_message_size
                            );
                            continue;
                        }
                        message_handler(text.into_bytes())?;
                    }
                    Message::Binary(data) => {
                        debug!("Received binary message: {} bytes", data.len());
                        if data.len() > self.config.max_message_size {
                            warn!(
                                "Message size {} exceeds limit {}, skipping",
                                data.len(),
                                self.config.max_message_size
                            );
                            continue;
                        }
                        message_handler(data)?;
                    }
                    Message::Ping(_) => {
                        debug!("Received ping");
                    }
                    Message::Pong(_) => {
                        debug!("Received pong");
                    }
                    Message::Close(frame) => {
                        info!("Received close frame: {:?}", frame);
                        return Err(ProviderError::ConnectionClosed);
                    }
                    Message::Frame(_) => {
                        debug!("Received raw frame");
                    }
                },
                Err(e) => {
                    error!("Error receiving message: {}", e);
                    return Err(e.into());
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_websocket_client_creation() {
        let config = ProviderConfig::new(
            "ws://localhost:8080".to_string(),
            "test.subject".to_string(),
        );
        let client = WebSocketClient::new(config);
        assert_eq!(client.config.websocket_url, "ws://localhost:8080");
    }
}
