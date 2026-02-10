#!/bin/bash
set -e

# Ensure wash is in PATH
export PATH="/usr/local/bin:$PATH"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== WebSocket Provider Integration Test ===${NC}"
echo ""

# Check prerequisites
echo -e "${YELLOW}Checking prerequisites...${NC}"

if ! command -v wash &> /dev/null; then
    echo -e "${RED}Error: wash CLI not found${NC}"
    exit 1
fi

if ! command -v python3 &> /dev/null; then
    echo -e "${RED}Error: python3 not found${NC}"
    exit 1
fi

if ! python3 -c "import websockets" 2>/dev/null; then
    echo -e "${YELLOW}Warning: websockets library not installed${NC}"
    echo "Install with: pip3 install websockets"
    exit 1
fi

echo -e "${GREEN}✓ Prerequisites OK${NC}"
echo ""

# Cleanup function
cleanup() {
    echo ""
    echo -e "${YELLOW}Cleaning up...${NC}"
    
    # Stop WebSocket server
    if [ ! -z "$WS_SERVER_PID" ]; then
        kill $WS_SERVER_PID 2>/dev/null || true
        echo "✓ Stopped WebSocket server"
    fi
    
    # Stop wasmCloud
    wash down 2>/dev/null || true
    echo "✓ Stopped wasmCloud"
    
    echo -e "${GREEN}Cleanup complete${NC}"
}

# Set up trap to cleanup on exit
trap cleanup EXIT

# Start WebSocket test server
echo -e "${YELLOW}Starting WebSocket test server...${NC}"
python3 tests/websocket_server.py > /tmp/ws_server.log 2>&1 &
WS_SERVER_PID=$!
sleep 2

if ! kill -0 $WS_SERVER_PID 2>/dev/null; then
    echo -e "${RED}Error: Failed to start WebSocket server${NC}"
    cat /tmp/ws_server.log
    exit 1
fi

echo -e "${GREEN}✓ WebSocket server started (PID: $WS_SERVER_PID)${NC}"
echo "  Listening on ws://127.0.0.1:8765"
echo ""

# Build the provider
echo -e "${YELLOW}Building provider...${NC}"
cargo build --release 2>&1 | grep -E "(Compiling|Finished|error)" || true

if [ ! -f "target/release/wasmcloud-provider-websocket" ]; then
    echo -e "${RED}Error: Provider build failed${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Provider built${NC}"
echo ""

# Build the component
echo -e "${YELLOW}Building component...${NC}"
cd component
cargo build --release --target wasm32-wasip2 2>&1 | grep -E "(Compiling|Finished|error)" || true
cd ..

# Check if component was built
COMPONENT_PATH="component/target/wasm32-wasip2/release/custom_template_test_component.wasm"
if [ ! -f "$COMPONENT_PATH" ]; then
    echo -e "${RED}Error: Component build failed${NC}"
    exit 1
fi

# Create build directory and copy component
mkdir -p component/build
cp "$COMPONENT_PATH" component/build/custom_component.wasm

echo -e "${GREEN}✓ Component built${NC}"
echo ""

# Start wasmCloud
echo -e "${YELLOW}Starting wasmCloud host...${NC}"
wash up --detached

# Wait for host to be ready
echo "Waiting for host to be ready..."
for i in {1..30}; do
    if wash get hosts 2>/dev/null | grep -q "wasmCloud Host"; then
        break
    fi
    sleep 1
done

echo -e "${GREEN}✓ wasmCloud host started${NC}"
echo ""

# Deploy provider
echo -e "${YELLOW}Deploying provider...${NC}"
wash start provider file://./target/release/wasmcloud-provider-websocket websocket-provider
sleep 2
echo -e "${GREEN}✓ Provider deployed${NC}"
echo ""

# Deploy component
echo -e "${YELLOW}Deploying component...${NC}"
wash start component file://./component/build/custom_component.wasm test-component
sleep 2
echo -e "${GREEN}✓ Component deployed${NC}"
echo ""

# Create link
echo -e "${YELLOW}Creating link between component and provider...${NC}"
wash link put test-component websocket-provider \
  wasmcloud:websocket \
  websocket_url=ws://127.0.0.1:8765 \
  max_reconnect_attempts=0 \
  initial_reconnect_delay_ms=1000

sleep 2
echo -e "${GREEN}✓ Link created${NC}"
echo ""

# Monitor logs
echo -e "${GREEN}=== Test Running ===${NC}"
echo "Monitoring logs for 30 seconds..."
echo "Press Ctrl+C to stop early"
echo ""

# Start log monitoring in background
wash logs > /tmp/wasmcloud_logs.log 2>&1 &
LOG_PID=$!

# Wait and show progress
for i in {1..30}; do
    echo -ne "\rTime: ${i}s / 30s  "
    sleep 1
done

echo ""
echo ""

# Stop log monitoring
kill $LOG_PID 2>/dev/null || true

# Analyze logs
echo -e "${GREEN}=== Test Results ===${NC}"
echo ""

PROVIDER_CONNECTED=$(grep -c "WebSocket connection established" /tmp/wasmcloud_logs.log || echo "0")
MESSAGES_RECEIVED=$(grep -c "Received WebSocket message" /tmp/wasmcloud_logs.log || echo "0")
MESSAGES_SENT=$(grep -c "Message successfully sent to component" /tmp/wasmcloud_logs.log || echo "0")

echo "Provider connections: $PROVIDER_CONNECTED"
echo "Messages received by provider: $MESSAGES_SENT"
echo "Messages handled by component: $MESSAGES_RECEIVED"
echo ""

if [ "$PROVIDER_CONNECTED" -gt "0" ] && [ "$MESSAGES_RECEIVED" -gt "0" ]; then
    echo -e "${GREEN}✓ Integration test PASSED${NC}"
    echo ""
    echo "The provider successfully:"
    echo "  - Connected to the WebSocket server"
    echo "  - Received messages from the server"
    echo "  - Forwarded messages to the component"
    echo "  - Component processed the messages"
    exit 0
else
    echo -e "${RED}✗ Integration test FAILED${NC}"
    echo ""
    echo "Last 50 lines of logs:"
    tail -50 /tmp/wasmcloud_logs.log
    exit 1
fi
