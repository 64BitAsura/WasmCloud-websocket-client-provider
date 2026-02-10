use websocket_provider::WebSocketProvider;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    WebSocketProvider::run().await
}
