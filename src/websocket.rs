use crate::config::LinkConfig;
use futures_util::StreamExt;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::Message, Connector};
use tracing::{debug, error, info, warn};

/// Build a rustls Connector with webpki root certificates for wss:// connections
fn build_tls_connector() -> Connector {
    let root_store =
        rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let tls_config = rustls::ClientConfig::builder_with_provider(
        rustls::crypto::ring::default_provider().into(),
    )
    .with_safe_default_protocol_versions()
    .expect("failed to set TLS protocol versions")
    .with_root_certificates(root_store)
    .with_no_client_auth();
    Connector::Rustls(std::sync::Arc::new(tls_config))
}

/// WebSocket client handler
pub struct WebSocketClient {
    config: LinkConfig,
}

impl WebSocketClient {
    /// Create a new WebSocket client
    pub fn new(config: LinkConfig) -> Self {
        Self { config }
    }

    /// Connect to the WebSocket server and start receiving messages
    pub async fn run<F>(&self, mut message_handler: F) -> anyhow::Result<()>
    where
        F: FnMut(Vec<u8>) -> anyhow::Result<()> + Send,
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
    async fn connect_and_receive<F>(&self, message_handler: &mut F) -> anyhow::Result<()>
    where
        F: FnMut(Vec<u8>) -> anyhow::Result<()>,
    {
        info!(
            "Connecting to WebSocket server: {}",
            self.config.websocket_url
        );

        // Use TLS connector for wss:// URLs, plain for ws://
        let connector = if self.config.websocket_url.starts_with("wss://") {
            info!("Using TLS (rustls) for wss:// connection");
            Some(build_tls_connector())
        } else {
            None
        };

        let (ws_stream, response) = connect_async_tls_with_config(
            &self.config.websocket_url,
            None,
            false,
            connector,
        )
        .await?;

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
                        return Err(anyhow::anyhow!("Connection closed"));
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
