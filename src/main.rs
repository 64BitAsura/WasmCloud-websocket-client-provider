//! WebSocket capability provider for wasmCloud
//!
//! This provider connects to remote WebSocket servers and forwards received messages
//! to wasmCloud components via wRPC. It implements unidirectional communication
//! (receiving only) with automatic reconnection and message size limits.

mod config;
mod provider;
mod websocket;

use provider::WebSocketProvider;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    WebSocketProvider::run().await?;
    eprintln!("WebSocket provider exiting");
    Ok(())
}
