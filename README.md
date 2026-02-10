# WebSocket Capability Provider

A wasmCloud capability provider that connects to remote WebSocket servers and forwards received messages to components via wRPC. It implements unidirectional communication (receiving only) with automatic reconnection and configurable message size limits.

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

## Configuration

Link configuration values passed via `wash config put`:

| Key | Description | Default |
|-----|-------------|---------|
| `websocket_url` | WebSocket server URL (`ws://` or `wss://`) | *required* |
| `max_reconnect_attempts` | Max reconnection attempts (0 = infinite) | `0` |
| `initial_reconnect_delay_ms` | Initial reconnect delay in ms | `1000` |
| `max_reconnect_delay_ms` | Max reconnect delay in ms (exponential backoff) | `60000` |
| `max_message_size` | Max message size in bytes | `1048576` |

## WIT Interface

The provider imports `wasmcloud:websocket/message-handler`:

```wit
interface message-handler {
    record websocket-message {
        payload: string,
        message-type: string,
        timestamp: u64,
        size: u32,
    }
    handle-message: func(message: websocket-message) -> result<_, string>;
}
```

Components export this interface to receive WebSocket messages from the provider.

## Architecture

```
WebSocket Server
    │ WebSocket messages
    ▼
WebSocket Provider (Rust + tokio-tungstenite)
    │ wRPC calls (via NATS)
    ▼
wasmCloud Component (WebAssembly)
```
