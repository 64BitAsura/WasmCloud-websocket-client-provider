use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Context as _;
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use wasmcloud_provider_sdk::initialize_observability;
use wasmcloud_provider_sdk::{
    run_provider, LinkConfig as SdkLinkConfig, LinkDeleteInfo, Provider, ProviderInitConfig,
};

use crate::config::{LinkConfig, ProviderConfig};
use crate::websocket::WebSocketClient;

pub(crate) mod bindings {
    wit_bindgen_wrpc::generate!({
        with: {
            "wasmcloud:messaging/types@0.2.0": generate,
            "wasmcloud:messaging/handler@0.2.0": generate,
        }
    });
}

// Import the standard messaging interfaces from WIT
use bindings::wasmcloud::messaging::handler;
use bindings::wasmcloud::messaging::types;

/// State for a single WebSocket connection
struct ConnectionState {
    /// Configuration for this connection
    _config: LinkConfig,
    /// Handle to the WebSocket task
    _task_handle: tokio::task::JoinHandle<()>,
}

/// WebSocket provider implementation
#[derive(Default, Clone)]
pub struct WebSocketProvider {
    config: Arc<RwLock<ProviderConfig>>,
    /// All components linked to this provider (target) and their connections
    connections: Arc<RwLock<HashMap<String, ConnectionState>>>,
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
        let shutdown = run_provider(provider.clone(), Self::name())
            .await
            .context("failed to run provider")?;

        // For this unidirectional provider, we don't export any functions
        // Just await shutdown
        shutdown.await;
        Ok(())
    }
}

/// Implement the Provider trait for wasmCloud integration
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

        // Save configuration to provider state
        *self.config.write().await = ProviderConfig::from(initial_config);

        Ok(())
    }

    /// Handle incoming link from a component (component links TO this provider)
    /// This is where we start the WebSocket client
    async fn receive_link_config_as_target(
        &self,
        SdkLinkConfig {
            source_id, config, ..
        }: SdkLinkConfig<'_>,
    ) -> anyhow::Result<()> {
        info!("Received link configuration from component: {}", source_id);

        // Parse link configuration
        let link_config = LinkConfig::from_values(config)?;

        info!(
            "Starting WebSocket client for URL: {}",
            link_config.websocket_url
        );

        // Clone what we need for the task
        let config_clone = link_config.clone();
        let source_id_clone = source_id.to_string();

        // Spawn WebSocket client task
        let task_handle = tokio::spawn(async move {
            let ws_client = WebSocketClient::new(config_clone.clone());

            // Create message handler that forwards to the component via wRPC
            // using the standard wasmcloud:messaging interface
            let ws_url = config_clone.websocket_url.clone();
            let result = ws_client
                .run(move |data| {
                    // Convert WebSocket message to a standard broker-message
                    let message = create_broker_message(data, &ws_url);

                    // Spawn a task to send message to component
                    let source = source_id_clone.clone();
                    tokio::spawn(async move {
                        if let Err(e) = send_message_to_component(&source, message).await {
                            error!("Failed to send message to component {}: {}", source, e);
                        }
                    });

                    Ok(())
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
                _config: link_config,
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

        // Remove connection state (task will be cancelled)
        if let Some(state) = self.connections.write().await.remove(source_id) {
            info!("WebSocket connection closed for component: {}", source_id);
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

/// Create a broker-message from raw WebSocket data
///
/// The subject is set to "websocket.<url>" so the component knows
/// which WebSocket connection the message originated from.
/// The body contains the raw bytes of the WebSocket message.
fn create_broker_message(data: Vec<u8>, websocket_url: &str) -> types::BrokerMessage {
    types::BrokerMessage {
        subject: format!("websocket.{}", websocket_url),
        body: data.into(),
        reply_to: None,
    }
}

/// Send message to component via wRPC using the standard messaging handler
async fn send_message_to_component(
    component_id: &str,
    message: types::BrokerMessage,
) -> anyhow::Result<()> {
    let client = wasmcloud_provider_sdk::get_connection()
        .get_wrpc_client(component_id)
        .await
        .context("failed to get wrpc client")?;

    match handler::handle_message(&client, None, &message).await {
        Ok(Ok(_)) => {
            info!("Message successfully sent to component {}", component_id);
            Ok(())
        }
        Ok(Err(e)) => {
            error!("Component {} returned error: {}", component_id, e);
            anyhow::bail!("Component error: {}", e)
        }
        Err(e) => {
            error!("Failed to call component {}: {}", component_id, e);
            Err(e)
        }
    }
}

/// Base64 encode helper
#[allow(dead_code)]
fn base64_encode(data: &[u8]) -> String {
    use base64::{engine::general_purpose, Engine as _};
    general_purpose::STANDARD.encode(data)
}
