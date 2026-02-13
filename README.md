# WebSocket Capability Provider

A wasmCloud capability provider that connects to remote WebSocket servers and forwards received messages to components using the standard `wasmcloud:messaging` interface via wRPC. It implements unidirectional communication (receiving only) with automatic reconnection, configurable message size limits, and TLS support for `wss://` connections (via rustls).

## Building

Prerequisites: [Rust toolchain](https://www.rust-lang.org/tools/install), [wash CLI](https://wasmcloud.com/docs/installation)

```bash
# Build the provider (.par.gz archive)
wash build

# Build the test component
wash build -p ./component
```

## Testing

Run the automated integration test:

```bash
./tests/run_integration_test.sh
```

Or deploy as a WADM application:

```bash
wash up -d
wash app deploy ./wadm.yaml
```

See [TESTING.md](./TESTING.md) for detailed manual testing steps.

## Development

For contributing to this project, see [Agents.md](./Agents.md) for the structured implementation process including:
- Analysis of implementation prompts
- Three-solution approach with confidence ratings
- Comprehensive testing checklist (format, clippy, type checks)
- Documentation templates for future reference

## Configuration

Link configuration values passed via `wash config put`:

| Key | Description | Default |
|-----|-------------|---------|
| `websocket_url` | WebSocket server URL (`ws://` or `wss://`) | *required* |
| `max_reconnect_attempts` | Max reconnection attempts (0 = infinite) | `0` |
| `initial_reconnect_delay_ms` | Initial reconnect delay in ms | `1000` |
| `max_reconnect_delay_ms` | Max reconnect delay in ms (exponential backoff) | `60000` |
| `max_message_size` | Max message size in bytes | `1048576` |

## Messaging Interface

The provider uses the standard `wasmcloud:messaging@0.2.0` interface to forward WebSocket messages to components. Each WebSocket message is wrapped in a `broker-message`:

```wit
// From wasmcloud:messaging@0.2.0
interface types {
    record broker-message {
        subject: string,       // "websocket.<url>" identifying the source connection
        body: list<u8>,        // Raw WebSocket message bytes (text or binary)
        reply-to: option<string>,
    }
}

interface handler {
    use types.{broker-message};
    handle-message: func(msg: broker-message) -> result<_, string>;
}
```

Components export `wasmcloud:messaging/handler` to receive messages. The `subject` field is set to `websocket.<websocket_url>` so the component knows which connection the message came from. The `body` contains the raw bytes of the WebSocket message.

### Linking

```bash
# Create named config
wash config put websocket-config \
  websocket_url=ws://127.0.0.1:8765

# Link component to provider using wasmcloud:messaging
wash link put <component-id> <provider-id> \
  wasmcloud messaging \
  --interface handler \
  --target-config websocket-config
```

Or via WADM:

```yaml
# Link defined on the component (source) to the provider (target)
- type: link
  properties:
    target:
      name: websocket-provider
      config:
        - name: websocket-config
          properties:
            websocket_url: wss://example.com/ws
    namespace: wasmcloud
    package: messaging
    interfaces: [handler]
```

### TLS (wss://)

The provider supports `wss://` URLs out of the box using rustls with Mozilla's WebPKI root certificates. No additional configuration is needed — just use a `wss://` URL in `websocket_url`.

## Architecture

```
WebSocket Server
    │ WebSocket messages
    ▼
WebSocket Provider (Rust + tokio-tungstenite)
    │ wRPC calls via wasmcloud:messaging/handler (over NATS)
    ▼
wasmCloud Component (WebAssembly)
    exports wasmcloud:messaging/handler
```
