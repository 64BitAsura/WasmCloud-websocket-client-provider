//! WasmCloud WebSocket Provider
//!
//! A unidirectional WebSocket client provider that receives messages from
//! remote WebSocket servers and forwards them to WasmCloud components via NATS.

mod config;
mod websocket;

use anyhow::{Context as AnyhowContext, Result};
use config::ProviderConfig;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use wasmcloud_provider_sdk::{
    run_provider, LinkConfig, LinkDeleteInfo, Provider, ProviderInitConfig,
};
use websocket::WebSocketClient;

/// Main provider struct
#[derive(Default, Clone)]
struct WebSocketProvider {
    /// Configuration per linked component
    linked_from: Arc<RwLock<HashMap<String, ProviderConfig>>>,
    /// NATS client for publishing messages
    nats_client: Arc<RwLock<Option<async_nats::Client>>>,
}

impl WebSocketProvider {
    /// Provider name
    fn name() -> &'static str {
        "websocket-provider"
    }

    /// Execute the provider
    pub async fn run() -> Result<()> {
        // Initialize basic tracing
        tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
            )
            .init();

        let provider = Self::default();

        // Connect to NATS (use localhost by default, or environment variable)
        let nats_url =
            std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
        info!("Connecting to NATS at {}", nats_url);

        let nats_client = async_nats::connect(&nats_url)
            .await
            .context("Failed to connect to NATS")?;

        info!("Connected to NATS successfully");

        // Store NATS client
        {
            let mut nats_lock = provider.nats_client.write().await;
            *nats_lock = Some(nats_client);
        }

        let shutdown = run_provider(provider.clone(), Self::name())
            .await
            .context("Failed to run provider")?;

        // Wait for shutdown signal
        shutdown.await;
        Ok(())
    }

    /// Start the message forwarding task
    async fn start_message_forwarder(
        &self,
        config: Arc<ProviderConfig>,
        mut message_rx: mpsc::UnboundedReceiver<Vec<u8>>,
    ) {
        let nats_client = self.nats_client.clone();

        tokio::spawn(async move {
            while let Some(message) = message_rx.recv().await {
                // Get NATS client
                let client = {
                    let lock = nats_client.read().await;
                    lock.clone()
                };

                if let Some(client) = client {
                    // Publish to NATS subject
                    match client
                        .publish(config.nats_subject.clone(), message.into())
                        .await
                    {
                        Ok(_) => {
                            info!("Published message to NATS subject: {}", config.nats_subject);
                        }
                        Err(e) => {
                            error!("Failed to publish message to NATS: {:#}", e);
                        }
                    }
                } else {
                    error!("NATS client not available");
                }
            }
        });
    }
}

/// Implementation of the WasmCloud Provider trait
impl Provider for WebSocketProvider {
    /// Initialize provider with configuration
    async fn init(&self, config: impl ProviderInitConfig) -> Result<()> {
        let provider_id = config.get_provider_id();
        info!(provider_id, "Initializing WebSocket provider");
        Ok(())
    }

    /// Handle link establishment when a component links to this provider
    async fn receive_link_config_as_target(
        &self,
        LinkConfig {
            source_id, config, ..
        }: LinkConfig<'_>,
    ) -> Result<()> {
        info!(
            "Received link from component {} with config: {:?}",
            source_id, config
        );

        // Parse configuration
        let provider_config: ProviderConfig = serde_json::from_str(
            &serde_json::to_string(&config).context("Failed to serialize config")?,
        )
        .context("Failed to parse provider configuration")?;

        // Validate configuration
        provider_config
            .validate()
            .context("Invalid configuration")?;

        info!("Configuration validated successfully");
        let provider_config = Arc::new(provider_config);

        // Store config
        self.linked_from
            .write()
            .await
            .insert(source_id.to_string(), (*provider_config).clone());

        // Create message channel
        let (message_tx, message_rx) = mpsc::unbounded_channel();

        // Start message forwarder
        self.start_message_forwarder(provider_config.clone(), message_rx)
            .await;

        // Create and start WebSocket client
        let ws_client = WebSocketClient::new(provider_config.clone(), message_tx);

        // Spawn WebSocket client task
        tokio::spawn(async move {
            ws_client.run().await;
        });

        info!(
            "WebSocket client started successfully for component {}",
            source_id
        );

        Ok(())
    }

    /// Handle link deletion
    async fn delete_link_as_target(&self, link: impl LinkDeleteInfo) -> Result<()> {
        let source_id = link.get_source_id();
        info!("Deleting link from component {}", source_id);

        self.linked_from.write().await.remove(source_id);

        // WebSocket client will be stopped when task is dropped
        Ok(())
    }

    /// Shutdown the provider
    async fn shutdown(&self) -> Result<()> {
        info!("Shutting down WebSocket provider");
        self.linked_from.write().await.clear();
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    WebSocketProvider::run().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let provider = WebSocketProvider::default();
        assert!(provider.linked_from.try_read().is_ok());
    }

    #[tokio::test]
    async fn test_config_parsing() {
        let config_str = r#"{
            "websocket_url": "ws://example.com:8080",
            "nats_subject": "test.messages",
            "reconnect_interval_secs": 10,
            "max_reconnect_attempts": 5,
            "tls_verification": false
        }"#;

        let config: ProviderConfig = serde_json::from_str(config_str).unwrap();
        assert_eq!(config.websocket_url, "ws://example.com:8080");
        assert_eq!(config.nats_subject, "test.messages");
        assert_eq!(config.reconnect_interval_secs, 10);
        assert_eq!(config.max_reconnect_attempts, 5);
        assert!(!config.tls_verification);
    }
}
