# WebSocket Provider Test Demonstration

This document provides a step-by-step guide to manually test the WebSocket provider with a wasmCloud NATS mesh.

## Quick Test (Automated)

For a fully automated test, run:

```bash
./tests/run_integration_test.sh
```

This will automatically:
1. Start a test WebSocket server
2. Build and deploy the provider and component
3. Create the necessary links
4. Monitor message flow for 30 seconds
5. Report results and clean up

## Manual Test Steps

If you prefer to test manually or troubleshoot issues, follow these steps:

### Step 1: Install Prerequisites

```bash
# Install Python websockets library
pip3 install websockets
```

### Step 2: Start the Test WebSocket Server

In Terminal 1:
```bash
python3 tests/websocket_server.py
```

You should see:
```
Starting WebSocket test server on ws://127.0.0.1:8765
Press Ctrl+C to stop
--------------------------------------------------
```

### Step 3: Build the Provider

In Terminal 2:
```bash
cargo build --release
```

Wait for the build to complete. The provider binary will be at:
`./target/release/wasmcloud-provider-websocket`

### Step 4: Build the Component

```bash
cd component
wash build
cd ..
```

The component will be built to:
`./component/build/custom_component.wasm`

### Step 5: Start wasmCloud Host

In Terminal 3:
```bash
wash up
```

This starts:
- A NATS server
- A wasmCloud host
- Web UI at http://localhost:4000

### Step 6: Deploy the Provider

In Terminal 4:
```bash
wash start provider \
  file://./target/release/wasmcloud-provider-websocket \
  websocket-provider
```

### Step 7: Deploy the Component

```bash
wash start component \
  file://./component/build/custom_component.wasm \
  test-component
```

### Step 8: Create the Link

This links the component to the provider with WebSocket configuration:

```bash
wash link put test-component websocket-provider \
  wasmcloud:websocket \
  websocket_url=ws://127.0.0.1:8765
```

### Step 9: Monitor the System

#### View All Logs
```bash
wash logs
```

#### View Provider Logs Only
```bash
wash logs websocket-provider
```

#### View Component Logs Only
```bash
wash logs test-component
```

### Expected Output

**Terminal 1 (WebSocket Server):**
```
Client 140123456789 connected from ('127.0.0.1', 54321)
Sent to client 140123456789: {"type": "test", "count": 1, ...}
Sent to client 140123456789: {"type": "test", "count": 2, ...}
```

**Provider Logs:**
```
WebSocket connection established: 101
Received text message: 123 bytes
Message successfully sent to component test-component
```

**Component Logs:**
```
Received WebSocket message - Type: text, Size: 123 bytes, Timestamp: 1707598234
Message payload: {"type":"test","count":1,...}
```

## Verification Checklist

- [ ] WebSocket server shows client connected
- [ ] Provider logs show "WebSocket connection established"
- [ ] Provider logs show "Received text/binary message"
- [ ] Provider logs show "Message successfully sent to component"
- [ ] Component logs show "Received WebSocket message"
- [ ] Component logs show message details and payload

## Testing Edge Cases

### Test Reconnection

1. In the WebSocket server terminal, press Ctrl+C to stop the server
2. Observe in provider logs:
   ```
   WebSocket connection error: ...
   Attempting reconnection #1 after 1s
   ```
3. Restart the server: `python3 tests/websocket_server.py`
4. Observe successful reconnection in provider logs

### Test with Different WebSocket URLs

Update the link with a different URL:

```bash
# Delete old link
wash link del test-component websocket-provider wasmcloud:websocket

# Create new link with different URL
wash link put test-component websocket-provider \
  wasmcloud:websocket \
  websocket_url=wss://echo.websocket.org
```

### Test Message Size Limits

Configure a small message size limit:

```bash
wash link put test-component websocket-provider \
  wasmcloud:websocket \
  websocket_url=ws://127.0.0.1:8765 \
  max_message_size=100
```

Messages larger than 100 bytes will be skipped by the provider.

## Cleanup

```bash
# Stop everything
wash down

# Stop WebSocket server (Ctrl+C in Terminal 1)
```

## Troubleshooting

### "Failed to connect to WebSocket server"

**Check:**
- Is the WebSocket server running?
- Is it listening on the correct address?
- Check firewall settings

**Debug:**
```bash
# Test connection with curl
curl -i -N -H "Connection: Upgrade" \
  -H "Upgrade: websocket" \
  -H "Sec-WebSocket-Key: test" \
  -H "Sec-WebSocket-Version: 13" \
  http://127.0.0.1:8765/
```

### "Component not receiving messages"

**Check:**
- Is the link created? `wash link query`
- Are provider and component running? `wash get inventory`
- Check component logs for errors

**Debug:**
```bash
# Check link status
wash link query

# Check provider status  
wash get providers

# Check component status
wash get components
```

### "NATS connection failed"

**Check:**
- Is wash up running?
- Is NATS server started?

**Debug:**
```bash
# Check hosts
wash get hosts

# Restart wasmCloud
wash down
wash up
```

## Architecture Diagram

```
┌─────────────────────┐
│  WebSocket Server   │
│  (127.0.0.1:8765)   │
└──────────┬──────────┘
           │ WebSocket
           │ messages
           ▼
┌─────────────────────┐
│  WebSocket Provider │
│  (Rust + tokio)     │
└──────────┬──────────┘
           │ wRPC calls
           │ (via NATS)
           ▼
┌─────────────────────┐
│ wasmCloud Component │
│ (WebAssembly)       │
└─────────────────────┘
```

## Next Steps

After successful testing, you can:

1. **Deploy to Production**: Use WADM manifests for deployment
2. **Connect to Real Services**: Update the `websocket_url` configuration
3. **Add Business Logic**: Enhance the component to process messages
4. **Monitor with Observability**: Enable OpenTelemetry for tracing
5. **Scale Horizontally**: Deploy multiple instances across hosts
