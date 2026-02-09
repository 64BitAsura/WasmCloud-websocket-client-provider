# WasmCloud WebSocket Provider

A unidirectional WebSocket client provider for WasmCloud that receives messages from remote WebSocket servers and forwards them to components via the NATS mesh.

## Features

- 🔌 **WebSocket Client**: Connects to remote WebSocket servers (ws:// and wss://)
- 📨 **Unidirectional**: Receives messages only (reply-back deferred)
- 🔄 **Auto-Reconnection**: Automatic reconnection with exponential backoff
- 🚀 **NATS Integration**: Forwards messages to components via NATS
- 🔒 **TLS Support**: Secure WebSocket connections (wss://)
- 🧪 **Well Tested**: 13 unit tests covering all core functionality
- ✅ **wash CLI**: Properly initialized with wasmCloud tooling

## Project Structure

This provider was created using the wash CLI and follows wasmCloud best practices:

```
wasmcloud-websocket-provider/
├── src/
│   ├── main.rs           # Entry point
│   ├── provider.rs       # Provider implementation
│   ├── config.rs         # Configuration module
│   └── websocket.rs      # WebSocket client
├── wit/
│   └── world.wit         # WIT interface definitions
├── component/            # Example test component
├── wasmcloud.toml        # wasmCloud configuration
├── Cargo.toml            # Rust dependencies
└── Makefile              # Development commands
```

## Quick Start

### Prerequisites

- Rust 1.75 or later
- wash CLI (installed automatically during setup)
- NATS server (typically included with WasmCloud)

### Building

```bash
# Using wash (recommended)
wash build

# Using Make
make build          # Development build
make release        # Release build

# Or using Cargo directly
cargo build --release
```

### Testing

```bash
# Using Make
make test           # Run all tests
make test-verbose   # Run with output

# Or using Cargo
cargo test
```

### Linting & Formatting

```bash
# Using Make
make fmt           # Format code
make lint          # Run clippy
make check         # Run both fmt-check and lint
make all           # Format, lint, test, and build

# Or using Cargo
cargo fmt
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

See [examples/configuration.md](./examples/configuration.md) for more examples.

## Architecture

See [Agents.md](./Agents.md) for detailed architecture documentation, implementation guidelines, and development workflow.

## Usage in WasmCloud

1. Build the provider with `wash build`
2. Deploy to your WasmCloud lattice
3. Link it to a component that needs to receive WebSocket messages
4. Messages will be forwarded to the configured NATS subject

## Documentation

- **README.md**: This file - Quick start and overview
- **Agents.md**: Living documentation with architecture and workflows
- **WASH_CLI.md**: wash CLI installation and usage guide
- **examples/configuration.md**: Configuration examples and patterns
- **SUMMARY.md**: Complete project status and metrics

## Development

```bash
# See all available commands
make help

# Run the full CI pipeline locally
make ci

# Watch mode (requires cargo-watch)
make watch
```

## License

Apache-2.0 - See [LICENSE](./LICENSE) for details

## Status

✅ **Phases 1-5 Complete**: Core implementation, testing, CI/CD, documentation, wash CLI integration
📋 **Next**: Integration tests and deployment testing in wasmCloud lattice

For detailed status, see [SUMMARY.md](./SUMMARY.md)
