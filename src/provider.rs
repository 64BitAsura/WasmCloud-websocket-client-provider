use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Context as _;
use tokio::sync::{mpsc, RwLock};
use tracing::{error, info};
use wasmcloud_provider_sdk::initialize_observability;
use wasmcloud_provider_sdk::{
    run_provider, LinkConfig, LinkDeleteInfo, Provider, ProviderInitConfig,
};

use crate::config::ProviderConfig;
use crate::websocket::WebSocketClient;

#[derive(Default, Clone)]
/// WebSocket provider that receives messages from remote servers and forwards to NATS
pub struct WebSocketProvider {
    /// Configuration per linked component
    linked_from: Arc<RwLock<HashMap<String, ProviderConfig>>>,
    /// NATS client for publishing messages
    nats_client: Arc<RwLock<Option<async_nats::Client>>>,
}

impl WebSocketProvider {
    fn name() -> &'static str {
        "websocket-provider"
    }

    /// Execute the provider
    pub async fn run() -> anyhow::Result<()> {
        initialize_observability!(
            Self::name(),
            std::env::var_os("PROVIDER_WEBSOCKET_FLAMEGRAPH_PATH")
        );

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
    async fn init(&self, config: impl ProviderInitConfig) -> anyhow::Result<()> {
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
    ) -> anyhow::Result<()> {
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
    async fn delete_link_as_target(&self, link: impl LinkDeleteInfo) -> anyhow::Result<()> {
        let source_id = link.get_source_id();
        info!("Deleting link from component {}", source_id);

        self.linked_from.write().await.remove(source_id);

        // WebSocket client will be stopped when task is dropped
        Ok(())
    }

    /// Shutdown the provider
    async fn shutdown(&self) -> anyhow::Result<()> {
        info!("Shutting down WebSocket provider");
        self.linked_from.write().await.clear();
        Ok(())
    }
}
