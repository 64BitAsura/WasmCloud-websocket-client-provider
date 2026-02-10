# Testing the WebSocket Provider

This directory contains tests and utilities for testing the WebSocket provider with a wasmCloud NATS mesh.

## Prerequisites

1. **wash CLI** - wasmCloud CLI tool
   ```bash
   # Already installed in the development environment
   wash --version
   ```

2. **Python 3 with websockets** - For running the test WebSocket server
   ```bash
   pip3 install websockets
   ```

3. **NATS Server** (optional - wash can start one)
   ```bash
   # wash will start NATS automatically
   ```

## Test Setup

### 1. Start the Test WebSocket Server

The test WebSocket server sends periodic messages to connected clients:

```bash
# In Terminal 1
python3 tests/websocket_server.py
```

The server will:
- Listen on `ws://127.0.0.1:8765`
- Send JSON messages every 3 seconds
- Send binary messages every 5th message
- Log all activity to the console

### 2. Build the Provider

```bash
# Build the provider
cargo build --release
```

### 3. Build the Component

```bash
# Build the test component
cd component
wash build
cd ..
```

### 4. Start wasmCloud Host

```bash
# In Terminal 2
wash up
```

This will:
- Start a NATS server
- Start a wasmCloud host
- Connect to the lattice

### 5. Deploy the Provider

```bash
# In Terminal 3
wash start provider ./target/release/wasmcloud-provider-websocket websocket-provider
```

### 6. Deploy the Component

```bash
# Deploy the component
wash start component file://./component/build/custom_component.wasm test-component
```

### 7. Create the Link

Link the component to the provider with WebSocket configuration:

```bash
wash link put test-component websocket-provider \
  wasmcloud:websocket \
  websocket_url=ws://127.0.0.1:8765 \
  max_reconnect_attempts=0 \
  initial_reconnect_delay_ms=1000 \
  max_reconnect_delay_ms=60000 \
  max_message_size=1048576
```

## Expected Behavior

Once linked, you should see:

1. **WebSocket Server Console**: Connection from the provider
2. **Provider Logs**: 
   - "WebSocket connection established"
   - "Received text/binary message"
   - "Message successfully sent to component"
3. **Component Logs**:
   - "Received WebSocket message"
   - Message details (type, size, timestamp)
   - Payload preview

## Viewing Logs

```bash
# View all logs
wash logs

# View provider logs only
wash logs websocket-provider

# View component logs only  
wash logs test-component
```

## Testing Different Scenarios

### Test Reconnection

1. Stop the WebSocket server (Ctrl+C)
2. Observe reconnection attempts in provider logs
3. Restart the server
4. Observe successful reconnection

### Test Message Size Limits

Modify the test server to send larger messages and observe size limit enforcement.

### Test Binary Messages

The server sends binary messages every 5th message - these should be base64-encoded in the component.

## Cleanup

```bash
# Stop the component
wash stop component test-component

# Stop the provider
wash stop provider websocket-provider

# Stop the host
wash down
```

## Troubleshooting

### Provider not connecting to WebSocket server

- Check the WebSocket server is running on `ws://127.0.0.1:8765`
- Check the link configuration has the correct `websocket_url`
- View provider logs for connection errors

### Component not receiving messages

- Verify the link was created successfully: `wash link query`
- Check component logs for errors
- Ensure the component is built with the correct WIT interface

### NATS connection issues

- Check `wash up` started successfully
- Verify NATS is running: `wash get hosts`
- Check network connectivity

## Automated Test Script

For convenience, you can use the automated test script:

```bash
./tests/run_integration_test.sh
```

This script will:
1. Start the WebSocket server in the background
2. Start wasmCloud
3. Build and deploy the provider and component
4. Create the link
5. Monitor logs for 30 seconds
6. Clean up all resources
