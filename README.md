# WasmCloud WebSocket Provider

A wasmCloud capability provider that acts as a unidirectional WebSocket client, receiving messages from remote WebSocket servers and forwarding them to wasmCloud components via the NATS mesh.

## Features

- **Unidirectional WebSocket Client**: Connects to remote WebSocket servers and receives messages
- **NATS Message Forwarding**: Automatically forwards received messages to wasmCloud components via NATS
- **Automatic Reconnection**: Implements exponential backoff retry logic for robust connection handling
- **Message Size Limits**: Configurable maximum message size to prevent DoS attacks
- **Support for Text and Binary Messages**: Handles both WebSocket message types
- **Comprehensive Error Handling**: Detailed error types with context

## Architecture

```
Remote WS Server → WebSocket Provider → NATS Mesh → WasmCloud Component
```

The provider:
1. Receives link configuration from a wasmCloud component
2. Connects to the specified WebSocket server
3. Receives messages from the WebSocket server
4. Wraps messages in JSON format with metadata
5. Publishes to the configured NATS subject
6. Component receives and processes the messages

## Configuration

The provider accepts the following link configuration parameters:

- `websocket_url` (required): The WebSocket server URL (ws:// or wss://)
- `nats_subject` (required): The NATS subject to publish messages to

## Message Format

Messages forwarded to NATS are wrapped in JSON format:

```json
{
  "payload": "message content or base64 for binary",
  "message_type": "text",
  "timestamp": 1707598234,
  "size": 1024
}
```

## Building

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Run linting
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt
```

## Running Locally

```bash
# Set up environment
export NATS_URL=127.0.0.1:4222

# Run the provider
cargo run --release
```

## Usage with wasmCloud

1. Start a wasmCloud host
2. Deploy the WebSocket provider
3. Link a component to the provider with configuration:
   - `websocket_url`: Your WebSocket server URL
   - `nats_subject`: NATS subject for message forwarding

### Example Link Configuration

```yaml
apiVersion: core.oam.dev/v1beta1
kind: Application
metadata:
  name: websocket-app
spec:
  components:
    - name: websocket-receiver
      type: component
      properties:
        image: file://./component.wasm
      traits:
        - type: link
          properties:
            target: websocket-provider
            namespace: wasmcloud
            package: websocket
            interfaces:
              - name: websocket-client
            source_config:
              - name: websocket-config
                properties:
                  websocket_url: "ws://example.com/socket"
                  nats_subject: "websocket.messages"
```

## Development

### Project Structure

```
websocket-provider/
├── Cargo.toml              # Dependencies and metadata
├── src/
│   ├── lib.rs              # Provider trait implementation
│   ├── main.rs             # Binary entry point
│   ├── websocket.rs        # WebSocket client logic
│   ├── nats_forwarder.rs   # NATS message forwarding
│   ├── config.rs           # Configuration handling
│   └── error.rs            # Error types
├── tests/
│   ├── unit/               # Unit tests
│   └── integration/        # Integration tests
├── .github/
│   └── workflows/
│       └── ci.yml          # CI/CD pipeline
├── Agents.md               # Living documentation
└── README.md               # This file
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_config_validation

# Run with output
cargo test -- --nocapture
```

### Linting and Formatting

```bash
# Check formatting
cargo fmt --check

# Format code
cargo fmt

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings
```

## CI/CD

The project uses GitHub Actions for continuous integration:

- **Lint**: Runs `cargo fmt` and `cargo clippy`
- **Test**: Runs the full test suite
- **Build**: Builds release artifacts
- **Security Audit**: Runs `cargo audit` to check for vulnerabilities

## Documentation

See [Agents.md](Agents.md) for detailed architecture, workflow, and implementation guidelines.

## Future Enhancements

- [ ] Bidirectional communication (reply-back feature)
- [ ] Support for multiple concurrent WebSocket connections
- [ ] Message filtering based on content
- [ ] Message transformation before forwarding
- [ ] Metrics and monitoring (Prometheus)
- [ ] Dynamic configuration updates
- [ ] TLS/SSL certificate validation options
- [ ] Custom authentication mechanisms

## License

Apache-2.0

## Contributing

Contributions are welcome! Please ensure:
- All tests pass
- Code is formatted with `cargo fmt`
- Clippy passes with no warnings
- New features include tests
- Documentation is updated