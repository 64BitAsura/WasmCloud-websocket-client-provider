# WasmCloud WebSocket Provider - Living Documentation

## Overview

This provider implements a **unidirectional WebSocket client** for WasmCloud that:
- Connects to a remote WebSocket server
- Receives messages from the server
- Forwards received messages to WasmCloud components via NATS mesh
- Does NOT support reply-back functionality (deferred feature)

## Project Initialization with wash CLI ✅

**This project has been properly initialized using the `wash` CLI tool.** The project structure follows wasmCloud best practices with:

- Proper provider scaffolding from wasmCloud templates
- WIT interface definitions in `wit/` directory
- wasmcloud.toml configuration
- wash build integration
- Component examples in `component/` directory

See [WASH_CLI.md](./WASH_CLI.md) for detailed information about wash CLI usage.

## Architecture

### High-Level Components

```
┌─────────────────────────────────────────────────────────────┐
│                    WasmCloud Lattice                         │
│  ┌────────────────┐              ┌──────────────────┐       │
│  │   Component    │◄─────────────┤  WebSocket       │       │
│  │   (Actor)      │   NATS       │  Provider        │       │
│  └────────────────┘              └──────────┬───────┘       │
│                                              │               │
└──────────────────────────────────────────────┼───────────────┘
                                               │
                                               │ WebSocket
                                               │ Connection
                                               ▼
                                    ┌─────────────────────┐
                                    │  Remote WebSocket   │
                                    │      Server         │
                                    └─────────────────────┘
```

### Component Details

1. **WebSocket Client Module**
   - Manages WebSocket connection lifecycle
   - Handles reconnection with exponential backoff
   - Processes incoming messages
   - Thread-safe message queue

2. **NATS Integration Module**
   - Publishes received messages to NATS
   - Handles message serialization
   - Manages NATS connection state

3. **Provider Core**
   - Implements wasmcloud-provider-sdk traits
   - Manages provider lifecycle
   - Configuration handling
   - Health check endpoints

## Implementation Guidelines

### Technology Stack

- **Language**: Rust (edition 2021)
- **WebSocket**: `tokio-tungstenite` for async WebSocket client
- **NATS**: `async-nats` for NATS messaging
- **Runtime**: Tokio for async runtime
- **Serialization**: `serde` + `serde_json` for message handling
- **Provider SDK**: `wasmcloud-provider-sdk`

### Key Dependencies

```toml
[dependencies]
wasmcloud-provider-sdk = "0.6"
tokio = { version = "1.35", features = ["full"] }
tokio-tungstenite = "0.21"
async-nats = "0.33"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1.0"
url = "2.5"
```

### Configuration Schema

The provider accepts the following configuration parameters:

```rust
struct ProviderConfig {
    /// WebSocket server URL (ws:// or wss://)
    websocket_url: String,
    
    /// NATS subject to publish received messages
    nats_subject: String,
    
    /// Reconnection attempt interval in seconds
    reconnect_interval_secs: u64,
    
    /// Maximum reconnection attempts (0 = infinite)
    max_reconnect_attempts: u32,
    
    /// Enable TLS verification for wss:// connections
    tls_verification: bool,
}
```

### Message Flow

1. **Connection Establishment**
   - Provider starts and reads configuration
   - Establishes WebSocket connection to remote server
   - Sets up NATS connection within lattice

2. **Message Reception**
   - WebSocket client receives message from server
   - Message is validated and logged
   - Message is serialized (if needed)

3. **Message Forwarding**
   - Serialized message published to configured NATS subject
   - Components subscribed to subject receive the message
   - Errors are logged but don't crash provider

4. **Reconnection Logic**
   - On connection loss, trigger reconnection
   - Use exponential backoff between attempts
   - Log all reconnection attempts
   - Respect max_reconnect_attempts if configured

### Testing Strategy

#### Unit Tests
- Configuration parsing and validation
- Message serialization/deserialization
- Reconnection logic (using mock connections)
- Error handling paths

#### Integration Tests
- WebSocket connection to test server
- NATS message publishing
- End-to-end message flow
- Reconnection scenarios

#### Deployment Tests
- Deploy provider in wasmcloud lattice
- Deploy test component that subscribes to messages
- Send messages through WebSocket
- Verify component receives messages via NATS

### Development Workflow

1. **Code Changes**
   - Make focused, incremental changes
   - Follow Rust best practices and idioms
   - Add appropriate error handling

2. **Testing**
   - Run unit tests: `cargo test --lib`
   - Run integration tests: `cargo test --test '*'`
   - Run all tests: `cargo test`

3. **Linting & Formatting**
   - Format code: `cargo fmt`
   - Run clippy: `cargo clippy -- -D warnings`
   - Fix any warnings before commit

4. **Build**
   - Debug build: `cargo build`
   - Release build: `cargo build --release`

5. **Documentation**
   - Update this file with architectural changes
   - Document new configuration options
   - Update README with usage examples

### CI/CD Pipeline

The GitHub Actions pipeline should:
1. Run on push and pull requests
2. Check formatting (`cargo fmt --check`)
3. Run clippy (`cargo clippy -- -D warnings`)
4. Run all tests (`cargo test`)
5. Build release binary
6. Cache cargo dependencies for faster builds

### Error Handling

- Use `anyhow::Result` for error propagation
- Log errors with appropriate levels (error, warn, info, debug)
- Don't panic in production code
- Handle WebSocket disconnections gracefully
- Implement proper cleanup on shutdown

### Security Considerations

- Validate WebSocket URLs before connecting
- Support TLS/WSS connections with certificate verification
- Sanitize log messages to avoid leaking sensitive data
- Implement timeouts for connections and operations
- Rate limiting for reconnection attempts

### Future Enhancements (Out of Scope)

- Bidirectional communication (send messages to WebSocket server)
- Message filtering and transformation
- Multiple WebSocket connections per provider instance
- Message buffering and replay on reconnection
- Metrics and monitoring integration

## Current Status

### Completed
- Initial documentation structure
- Architecture design
- Project initialization (Cargo-based, pending wash CLI migration)
- Core provider implementation
- Configuration module with validation
- WebSocket client with auto-reconnection
- NATS message forwarding
- Comprehensive unit tests (13 tests passing)
- CI/CD pipeline (GitHub Actions)
- Code formatting and linting setup
- **wash CLI initialization and proper project structure** ✅
- WIT interface definitions
- wasmcloud.toml configuration

### Pending
- Integration tests with actual WebSocket server
- Deployment testing in wasmCloud lattice
- Provider archive (PAR) creation with wash
- Example component for integration testing
- Performance benchmarks

## Known Issues

None yet.

## Contributing Guidelines

1. Follow the incremental development approach
2. Each feature must include tests
3. Update this document with architectural changes
4. Run linting and formatting before commits
5. Keep commits focused and atomic
