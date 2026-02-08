# Example Configuration for WebSocket Provider

This directory contains example configurations for the wasmCloud WebSocket Provider.

## Basic Configuration

```json
{
  "websocket_url": "ws://localhost:8080/ws",
  "nats_subject": "websocket.messages",
  "reconnect_interval_secs": 5,
  "max_reconnect_attempts": 0,
  "tls_verification": true
}
```

## Secure WebSocket (wss://)

```json
{
  "websocket_url": "wss://api.example.com/stream",
  "nats_subject": "production.websocket.stream",
  "reconnect_interval_secs": 10,
  "max_reconnect_attempts": 10,
  "tls_verification": true
}
```

## Configuration Parameters

### websocket_url (required)
- Type: String
- Description: WebSocket server URL to connect to
- Format: `ws://` or `wss://`
- Example: `ws://localhost:8080/ws` or `wss://api.example.com/stream`

### nats_subject (required)
- Type: String
- Description: NATS subject where received messages will be published
- Format: NATS subject format (no spaces, can use dots for hierarchy)
- Example: `websocket.messages` or `app.websocket.stream`

### reconnect_interval_secs (optional)
- Type: Integer
- Default: 5
- Description: Base interval in seconds between reconnection attempts
- Note: Uses exponential backoff (interval doubles with each attempt, max 300s)

### max_reconnect_attempts (optional)
- Type: Integer
- Default: 0 (unlimited)
- Description: Maximum number of reconnection attempts before giving up
- Set to 0 for unlimited reconnection attempts

### tls_verification (optional)
- Type: Boolean
- Default: true
- Description: Enable TLS certificate verification for wss:// connections
- Only applies to wss:// URLs; ignored for ws://

## Usage in wasmCloud

When linking a component to the provider, pass the configuration as link values:

```bash
wash link put \
  <component-id> \
  <provider-id> \
  --link-name default \
  --values websocket_url=wss://api.example.com/stream,nats_subject=app.messages
```

Or using a wasmCloud manifest (wadm):

```yaml
apiVersion: core.oam.dev/v1beta1
kind: Application
metadata:
  name: websocket-app
spec:
  components:
    - name: websocket-provider
      type: capability
      properties:
        image: yourregistry/wasmcloud-websocket-provider:0.1.0
      traits:
        - type: link
          properties:
            target: your-component
            namespace: default
            values:
              websocket_url: wss://api.example.com/stream
              nats_subject: app.websocket.messages
              reconnect_interval_secs: "10"
              max_reconnect_attempts: "5"
              tls_verification: "true"
```

## Testing Locally

For local testing without wasmCloud, set the NATS_URL environment variable:

```bash
export NATS_URL="nats://localhost:4222"
./target/release/wasmcloud-websocket-provider
```

## Common Use Cases

### Public WebSocket API
```json
{
  "websocket_url": "wss://stream.binance.com:9443/ws/btcusdt@trade",
  "nats_subject": "crypto.trades.btcusdt",
  "reconnect_interval_secs": 5,
  "max_reconnect_attempts": 0,
  "tls_verification": true
}
```

### Internal Development Server
```json
{
  "websocket_url": "ws://localhost:3000/dev/stream",
  "nats_subject": "dev.websocket.messages",
  "reconnect_interval_secs": 2,
  "max_reconnect_attempts": 10,
  "tls_verification": false
}
```

### IoT Data Stream
```json
{
  "websocket_url": "wss://iot.example.com/sensor/temperature",
  "nats_subject": "iot.sensors.temperature",
  "reconnect_interval_secs": 15,
  "max_reconnect_attempts": 0,
  "tls_verification": true
}
```
