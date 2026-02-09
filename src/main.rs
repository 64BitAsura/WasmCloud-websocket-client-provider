//! WasmCloud WebSocket Provider
//!
//! A unidirectional WebSocket client provider that receives messages from
//! remote WebSocket servers and forwards them to WasmCloud components via NATS.

mod config;
mod provider;
mod websocket;

use provider::WebSocketProvider;

/// Entry point for the WebSocket provider
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    WebSocketProvider::run().await?;
    eprintln!("WebSocket provider exiting");
    Ok(())
}
