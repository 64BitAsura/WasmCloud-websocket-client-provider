#!/usr/bin/env python3
"""
Simple WebSocket test server that sends periodic messages.
Used for testing the wasmCloud WebSocket provider.
"""

import asyncio
import json
import sys
from datetime import datetime

try:
    import websockets
except ImportError:
    print("Error: websockets library not installed.")
    print("Install with: pip3 install websockets")
    sys.exit(1)


async def send_messages(websocket):
    """Send test messages to connected clients."""
    client_id = id(websocket)
    remote = getattr(websocket, 'remote_address', None)
    print(f"Client {client_id} connected from {remote}")
    
    message_count = 0
    try:
        while True:
            message_count += 1
            
            # Send text message
            text_msg = json.dumps({
                "type": "test",
                "count": message_count,
                "timestamp": datetime.utcnow().isoformat(),
                "message": f"Test message #{message_count}"
            })
            
            await websocket.send(text_msg)
            print(f"Sent to client {client_id}: {text_msg}")
            
            # Wait before sending next message
            await asyncio.sleep(3)
            
            # Every 5th message, send a binary message
            if message_count % 5 == 0:
                binary_msg = bytes([0x48, 0x65, 0x6C, 0x6C, 0x6F])  # "Hello"
                await websocket.send(binary_msg)
                print(f"Sent binary message to client {client_id}: {binary_msg.hex()}")
                
    except websockets.exceptions.ConnectionClosed:
        print(f"Client {client_id} disconnected")
    except Exception as e:
        print(f"Error with client {client_id}: {e}")


async def main():
    """Start the WebSocket server."""
    host = "127.0.0.1"
    port = 8765
    
    print(f"Starting WebSocket test server on ws://{host}:{port}")
    print("Press Ctrl+C to stop")
    print("-" * 50)
    
    async with websockets.serve(send_messages, host, port):
        await asyncio.Future()  # Run forever


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\nServer stopped")
