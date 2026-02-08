# WasmCloud WebSocket Provider

A unidirectional WebSocket client provider for WasmCloud that receives messages from remote WebSocket servers and forwards them to components via the NATS mesh.

## Features

- 🔌 **WebSocket Client**: Connects to remote WebSocket servers (ws:// and wss://)
- 📨 **Unidirectional**: Receives messages only (reply-back deferred)
- 🔄 **Auto-Reconnection**: Automatic reconnection with exponential backoff
- 🚀 **NATS Integration**: Forwards messages to components via NATS
- 🔒 **TLS Support**: Secure WebSocket connections (wss://)
- 🧪 **Well Tested**: Unit and integration tests with deployment testing

## Quick Start

### Prerequisites

- Rust 1.75 or later
- WasmCloud runtime
- NATS server (typically included with WasmCloud)

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run with output
cargo test -- --nocapture
```

### Linting & Formatting

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings
```

## Configuration

The provider accepts configuration via link definitions in WasmCloud:

```json
{
  "websocket_url": "wss://example.com/ws",
  "nats_subject": "websocket.messages",
  "reconnect_interval_secs": 5,
  "max_reconnect_attempts": 0,
  "tls_verification": true
}
```

### Configuration Parameters

- `websocket_url` (required): WebSocket server URL
- `nats_subject` (required): NATS subject for publishing messages
- `reconnect_interval_secs` (default: 5): Seconds between reconnection attempts
- `max_reconnect_attempts` (default: 0): Maximum reconnections (0 = infinite)
- `tls_verification` (default: true): Enable TLS certificate verification

## Architecture

See [Agents.md](./Agents.md) for detailed architecture documentation, implementation guidelines, and development workflow.

## Usage in WasmCloud

1. Build the provider
2. Deploy to your WasmCloud lattice
3. Link it to a component that needs to receive WebSocket messages
4. Messages will be forwarded to the configured NATS subject

## Development

See [Agents.md](./Agents.md) for:
- Architecture overview
- Implementation guidelines
- Testing strategy
- Development workflow
- Contributing guidelines

## License

Apache-2.0 - See [LICENSE](./LICENSE) for details