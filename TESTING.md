# WebSocket Provider Testing

## Quick Test (Automated)

```bash
./tests/run_integration_test.sh
```

This will automatically start a test WebSocket server, build and deploy the provider and component, create links, monitor message flow for 30 seconds, and report results.

## Manual Test Steps

### Prerequisites

```bash
pip3 install websockets
```

### Step 1: Start the Test WebSocket Server

```bash
python3 tests/websocket_server.py
```

Server listens on `ws://127.0.0.1:8765`, sends JSON messages every 3 seconds, and binary messages every 5th message.

For WSS testing with a self-signed certificate:

```bash
python3 tests/websocket_server.py --tls
```

This auto-generates a self-signed cert and listens on `wss://127.0.0.1:8765`.

### Step 2: Build Provider and Component

```bash
wash build
wash build -p ./component
```

The provider archive will be in `build/` (`.par.gz`), the component in `component/build/` (`.wasm`).

### Step 3: Start wasmCloud Host

```bash
wash up
```

Wait until `wash get hosts` shows a host ID.

### Step 4: Deploy Provider and Component

```bash
wash start provider file://./build/wasmcloud-provider-websocket.par.gz websocket-provider
wash start component file://./component/build/custom_template_test_component.wasm test-component
```

Verify both are running:

```bash
wash get inventory
```

### Step 5: Create Config and Link

```bash
# Create named config
wash config put websocket-config \
  websocket_url=ws://127.0.0.1:8765 \
  max_reconnect_attempts=0 \
  initial_reconnect_delay_ms=1000

# Link component to provider (using standard wasmcloud:messaging interface)
wash link put test-component websocket-provider \
  wasmcloud messaging \
  --interface handler \
  --target-config websocket-config
```

### Step 6: Verify

Check the wasmCloud host output for:

- `WebSocket connection established: 101`
- `Received text message: ... bytes`
- `Message successfully sent to component ...`
- `Received message - Subject: websocket.ws://..., Size: ... bytes`

The WebSocket server terminal should show client connections and sent messages.

## Using WADM

You can also deploy the full application declaratively:

```bash
wash up -d
wash app deploy ./wadm.yaml
```

## Testing Edge Cases

### Reconnection

1. Stop the WebSocket server (Ctrl+C)
2. Observe reconnection attempts in provider logs
3. Restart the server — provider reconnects automatically

### Message Size Limits

Set `max_message_size` in the config to enforce size limits. Messages exceeding the limit are skipped.

## Cleanup

```bash
wash down
```

## Troubleshooting

| Problem | Check |
|---------|-------|
| Provider not connecting | Is the WebSocket server running? Check `websocket_url` in config |
| Component not receiving messages | Run `wash link query` and `wash get inventory` |
| NATS connection issues | Run `wash get hosts`, try `wash down && wash up` |

## Architecture

```
WebSocket Server (127.0.0.1:8765)
    │ WebSocket messages
    ▼
WebSocket Provider (Rust + tokio)
    │ wRPC calls (via NATS)
    ▼
wasmCloud Component (WebAssembly)
```
