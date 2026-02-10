mod config;
mod error;
mod nats_forwarder;
mod websocket;

pub use config::ProviderConfig;
pub use error::{ProviderError, ProviderResult};
pub use nats_forwarder::WebSocketMessage;
pub use websocket::WebSocketClient;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use wasmcloud_provider_sdk::{
    run_provider, LinkConfig, LinkDeleteInfo, Provider, ProviderInitConfig,
};

/// State for a single WebSocket connection
struct ConnectionState {
    /// Configuration for this connection
    _config: ProviderConfig,
    /// Handle to the WebSocket task  
    _task_handle: tokio::task::JoinHandle<()>,
}

/// Main WebSocket capability provider
#[derive(Clone)]
pub struct WebSocketProvider {
    /// All components linked to this provider and their connections
    connections: Arc<RwLock<HashMap<String, ConnectionState>>>,
    /// NATS client for publishing messages
    nats_client: Arc<RwLock<Option<async_nats::Client>>>,
}

impl Default for WebSocketProvider {
    fn default() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            nats_client: Arc::new(RwLock::new(None)),
        }
    }
}

impl WebSocketProvider {
    /// Create a new WebSocket provider instance
    pub fn new() -> Self {
        Self::default()
    }

    fn name() -> &'static str {
        "websocket-provider"
    }

    /// Run the provider
    pub async fn run() -> anyhow::Result<()> {
        // Initialize logging
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
            )
            .init();

        info!("Starting WebSocket Provider");

        let provider = Self::new();
        let shutdown = run_provider(provider.clone(), Self::name()).await?;

        // Wait for shutdown signal
        shutdown.await;
        Ok(())
    }

    /// Get or create NATS client connection
    async fn get_nats_client(&self) -> anyhow::Result<async_nats::Client> {
        let mut client_guard = self.nats_client.write().await;

        if let Some(ref client) = *client_guard {
            return Ok(client.clone());
        }

        // Connect to NATS using wasmcloud host's NATS server
        // The NATS URL should be available from environment or host data
        let nats_url = std::env::var("NATS_URL")
            .or_else(|_| std::env::var("NATS_HOST"))
            .unwrap_or_else(|_| "127.0.0.1:4222".to_string());

        info!("Connecting to NATS at: {}", nats_url);

        let client = async_nats::connect(&nats_url).await?;
        *client_guard = Some(client.clone());

        Ok(client)
    }
}

/// Implement the Provider trait for wasmcloud integration
impl Provider for WebSocketProvider {
    /// Initialize the provider
    async fn init(&self, config: impl ProviderInitConfig) -> anyhow::Result<()> {
        let provider_id = config.get_provider_id();
        let initial_config = config.get_config();
        info!(
            provider_id,
            ?initial_config,
            "initializing WebSocket provider"
        );

        // Initialize NATS client
        self.get_nats_client().await?;
        info!("NATS client initialized");

        Ok(())
    }

    /// Handle incoming link from a component
    async fn receive_link_config_as_target(
        &self,
        LinkConfig {
            source_id, config, ..
        }: LinkConfig<'_>,
    ) -> anyhow::Result<()> {
        info!("Received link configuration from component: {}", source_id);

        // Parse provider configuration from link values
        let websocket_url = config
            .get("websocket_url")
            .ok_or_else(|| anyhow::anyhow!("Missing required config: websocket_url"))?
            .to_string();

        let nats_subject = config
            .get("nats_subject")
            .ok_or_else(|| anyhow::anyhow!("Missing required config: nats_subject"))?
            .to_string();

        let ws_config = ProviderConfig::new(websocket_url, nats_subject);
        ws_config
            .validate()
            .map_err(|e| anyhow::anyhow!("Configuration validation failed: {}", e))?;

        info!(
            "Starting WebSocket client for URL: {}",
            ws_config.websocket_url
        );
        info!(
            "Messages will be forwarded to NATS subject: {}",
            ws_config.nats_subject
        );

        // Get NATS client
        let nats_client = self.get_nats_client().await?;

        // Spawn WebSocket client task
        let config_clone = ws_config.clone();
        let task_handle = tokio::spawn(async move {
            let ws_client = WebSocketClient::new(config_clone.clone());
            let nats_subject = config_clone.nats_subject.clone();

            // Create message handler that forwards to NATS
            let result = ws_client
                .run(|data| {
                    let message = WebSocketMessage::from_bytes(data);
                    match message.to_json() {
                        Ok(json_data) => {
                            let subject = nats_subject.clone();
                            let nats = nats_client.clone();
                            tokio::spawn(async move {
                                if let Err(e) = nats.publish(subject, json_data.into()).await {
                                    error!("Failed to publish to NATS: {}", e);
                                }
                            });
                            Ok(())
                        }
                        Err(e) => {
                            error!("Failed to serialize message: {}", e);
                            Err(e)
                        }
                    }
                })
                .await;

            if let Err(e) = result {
                error!("WebSocket client error: {}", e);
            }
        });

        // Store connection state
        self.connections.write().await.insert(
            source_id.to_string(),
            ConnectionState {
                _config: ws_config,
                _task_handle: task_handle,
            },
        );

        info!(
            "WebSocket connection established for component: {}",
            source_id
        );
        Ok(())
    }

    /// Handle link deletion
    async fn delete_link_as_target(&self, link: impl LinkDeleteInfo) -> anyhow::Result<()> {
        let source_id = link.get_source_id();
        info!("Deleting link with component: {}", source_id);

        // Remove connection state (task will be cancelled on drop)
        if let Some(state) = self.connections.write().await.remove(source_id) {
            info!("WebSocket connection closed for component: {}", source_id);
            // Task is aborted when dropped
            state._task_handle.abort();
        } else {
            warn!("No connection found for component: {}", source_id);
        }

        Ok(())
    }

    /// Handle provider shutdown
    async fn shutdown(&self) -> anyhow::Result<()> {
        info!("Shutting down WebSocket provider");

        // Clean up all connections
        let mut connections = self.connections.write().await;
        for (source_id, state) in connections.drain() {
            info!("Closing WebSocket connection for component: {}", source_id);
            state._task_handle.abort();
        }

        info!("WebSocket provider shutdown complete");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let _provider = WebSocketProvider::new();
        // Provider created successfully
    }
}
