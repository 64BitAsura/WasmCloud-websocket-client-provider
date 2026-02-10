# Agents.md - Living Documentation

## Project Overview
WasmCloud WebSocket Provider - A capability provider that acts as a unidirectional WebSocket client, receiving messages from remote WebSocket servers and forwarding them to wasmcloud components via the NATS mesh.

## Architecture

### High-Level Design
```
Remote WS Server → WebSocket Client Provider → NATS Mesh → WasmCloud Component
```

### Components

#### 1. WebSocket Client Provider
- **Purpose**: Capability provider that connects to remote WebSocket servers
- **Responsibilities**:
  - Establish and maintain WebSocket connections
  - Receive messages from WebSocket servers (unidirectional)
  - Convert WebSocket messages to NATS messages
  - Forward messages to target components via NATS
  - Handle connection lifecycle (connect, disconnect, reconnect)
  - Provide configuration for WebSocket endpoints

#### 2. NATS Message Forwarding
- **Purpose**: Bridge between WebSocket and wasmcloud components
- **Responsibilities**:
  - Publish received WebSocket messages to NATS subjects
  - Use wasmcloud messaging patterns
  - Handle message serialization/deserialization
  - Ensure reliable message delivery

#### 3. Configuration
- **WebSocket URL**: Target WebSocket server endpoint
- **NATS Subject**: Target subject for forwarded messages
- **Reconnection Policy**: Backoff strategy for reconnections
- **Message Format**: JSON or binary encoding

## Workflow

### Provider Startup
1. Provider starts and registers with wasmcloud host
2. Provider reads configuration (WebSocket URL, NATS subject)
3. Provider establishes WebSocket connection to remote server
4. Provider enters message receive loop

### Message Flow
1. WebSocket server sends message
2. Provider receives message in WebSocket handler
3. Provider wraps message in NATS-compatible format
4. Provider publishes message to configured NATS subject
5. Component receives and processes message

### Connection Management
1. **Initial Connection**: Provider attempts to connect to WebSocket server
2. **Connection Loss**: Provider detects disconnection
3. **Reconnection**: Provider implements exponential backoff retry logic
4. **Health Monitoring**: Provider reports connection status

## Implementation Guidelines

### Technology Stack
- **Language**: Rust
- **wasmcloud SDK**: `wasmcloud-provider-sdk`
- **WebSocket Client**: `tokio-tungstenite`
- **NATS**: Provided by wasmcloud SDK
- **Async Runtime**: `tokio`
- **Serialization**: `serde` with JSON support

### Project Structure
```
websocket-provider/
├── Cargo.toml              # Dependencies and metadata
├── src/
│   ├── lib.rs              # Provider trait implementation
│   ├── websocket.rs        # WebSocket client logic
│   ├── nats_forwarder.rs   # NATS message forwarding
│   ├── config.rs           # Configuration handling
│   └── error.rs            # Error types
├── tests/
│   ├── unit/               # Unit tests
│   │   ├── websocket_test.rs
│   │   └── nats_test.rs
│   └── integration/        # Integration tests
│       └── provider_test.rs
├── examples/
│   └── simple_client.rs    # Example usage
└── README.md               # User documentation
```

### Development Phases

#### Phase 1: Project Setup ✓
- [x] Create repository structure
- [x] Initialize Cargo project
- [x] Add wasmcloud dependencies
- [x] Set up linting and formatting

#### Phase 2: Core Provider (In Progress)
- [ ] Implement provider trait
- [ ] Add configuration parsing
- [ ] Create basic WebSocket client
- [ ] Add unit tests

#### Phase 3: Message Forwarding
- [ ] Implement NATS message publishing
- [ ] Add message serialization
- [ ] Create integration tests
- [ ] Test with mock WebSocket server

#### Phase 4: Connection Management
- [ ] Add reconnection logic
- [ ] Implement health checks
- [ ] Add connection state management
- [ ] Test failure scenarios

#### Phase 5: Integration Testing
- [ ] Deploy provider in wasmcloud NATS mesh
- [ ] Create test component
- [ ] End-to-end message flow test
- [ ] Performance testing

#### Phase 6: CI/CD Pipeline
- [ ] GitHub Actions workflow
- [ ] Automated testing
- [ ] Linting and formatting checks
- [ ] Build and release automation

### Testing Strategy

#### Unit Tests
- WebSocket connection establishment
- Message parsing and transformation
- NATS message publishing
- Configuration validation
- Error handling

#### Integration Tests
- Provider registration with wasmcloud
- WebSocket message reception
- NATS message forwarding
- Reconnection behavior
- Multiple concurrent connections

#### Deployment Tests
- Provider deployment in wasmcloud host
- Component communication via NATS
- End-to-end message flow
- Performance under load

### Code Quality Standards
- **Linting**: `cargo clippy` with no warnings
- **Formatting**: `cargo fmt` following Rust conventions
- **Testing**: Minimum 80% code coverage
- **Documentation**: All public APIs documented
- **Error Handling**: Comprehensive error types with context

### Security Considerations
- WebSocket URL validation
- TLS/SSL support for secure WebSocket connections
- Message size limits to prevent DoS
- Authentication (if required by WebSocket server)
- Rate limiting for reconnection attempts

## Future Enhancements (Deferred)
- **Bidirectional Communication**: Reply-back feature from component to WebSocket server
- **Multiple Connections**: Support for multiple WebSocket servers
- **Message Filtering**: Filter messages based on content
- **Message Transformation**: Apply transformations before forwarding
- **Metrics and Monitoring**: Prometheus metrics export
- **Dynamic Configuration**: Runtime configuration updates

## References
- [wasmcloud Documentation](https://wasmcloud.com/)
- [wasmcloud Provider SDK](https://github.com/wasmCloud/wasmCloud)
- [NATS Messaging](https://nats.io/)
- [WebSocket RFC 6455](https://tools.ietf.org/html/rfc6455)

## Change Log
- 2026-02-10: Initial documentation created
- 2026-02-10: Architecture and implementation guidelines defined
