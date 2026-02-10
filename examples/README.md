# Example WebSocket Provider Configuration

This directory contains example configurations for using the WebSocket provider.

## Basic Configuration

```toml
# Link configuration for wasmCloud
[link_config]
websocket_url = "ws://localhost:8080/socket"
nats_subject = "websocket.messages"
```

## Advanced Configuration

```toml
# Link configuration with all options
[link_config]
websocket_url = "wss://example.com/socket"
nats_subject = "app.websocket.events"

# These are handled by the provider internally with defaults
# max_reconnect_attempts = 0  # 0 = infinite
# initial_reconnect_delay_ms = 1000
# max_reconnect_delay_ms = 60000
# max_message_size = 1048576  # 1 MB
```

## Usage Examples

### Echo Server Test

Test the provider with a simple WebSocket echo server:

```bash
# Install websocat (WebSocket client/server)
cargo install websocat

# Start an echo server
websocat -s 8080 &

# Configure your provider to connect to ws://localhost:8080
```

### Subscribe to Messages

Subscribe to the NATS subject to see forwarded messages:

```bash
# Install NATS CLI
# https://github.com/nats-io/natscli

# Subscribe to messages
nats sub "websocket.messages"
```

### Send Test Messages

```bash
# Using websocat to send test messages
echo "Hello, WebSocket!" | websocat ws://localhost:8080
```

## Integration with WasmCloud

### 1. Deploy the Provider

```bash
# Build the provider
cargo build --release

# Start wasmCloud host
wash up

# Deploy the provider
wash start provider file://./target/release/libwebsocket_provider.so websocket-provider
```

### 2. Link to a Component

```bash
# Link the provider to your component
wash link put \
  <component-id> \
  websocket-provider \
  --link-name default \
  --values websocket_url=ws://localhost:8080,nats_subject=websocket.messages
```

### 3. Monitor Messages

```bash
# Watch logs
wash logs websocket-provider
```

## Testing Scenarios

### Scenario 1: Public WebSocket API

Connect to a public WebSocket API:

```toml
[link_config]
websocket_url = "wss://stream.binance.com:9443/ws/btcusdt@trade"
nats_subject = "crypto.trades.btcusdt"
```

### Scenario 2: Local Development

Connect to local development server:

```toml
[link_config]
websocket_url = "ws://localhost:3000/ws"
nats_subject = "dev.websocket.events"
```

### Scenario 3: Multiple Instances

Run multiple provider instances for different WebSocket sources:

```bash
# Instance 1: Crypto prices
wash link put component1 websocket-provider-1 \
  --values websocket_url=wss://api.example.com/prices,nats_subject=prices

# Instance 2: Chat messages
wash link put component2 websocket-provider-2 \
  --values websocket_url=wss://api.example.com/chat,nats_subject=chat
```
