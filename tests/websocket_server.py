#!/usr/bin/env python3
"""
Simple WebSocket test server that sends periodic messages.
Used for testing the wasmCloud WebSocket provider.
Supports both ws:// and wss:// (with auto-generated self-signed cert).
"""

import asyncio
import json
import os
import ssl
import subprocess
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


def generate_self_signed_cert(cert_path, key_path):
    """Generate a self-signed certificate for local WSS testing."""
    subprocess.run([
        "openssl", "req", "-x509", "-newkey", "rsa:2048",
        "-keyout", key_path, "-out", cert_path,
        "-days", "1", "-nodes",
        "-subj", "/CN=localhost",
        "-addext", "subjectAltName=DNS:localhost,IP:127.0.0.1",
    ], check=True, capture_output=True)
    print(f"Generated self-signed cert: {cert_path}, key: {key_path}")


async def main():
    """Start the WebSocket server. Use --tls flag or WSS_PORT env var for wss://."""
    host = "127.0.0.1"
    port = int(os.environ.get("WS_PORT", "8765"))
    use_tls = "--tls" in sys.argv or os.environ.get("WSS", "") == "1"

    ssl_context = None
    if use_tls:
        cert_dir = os.path.join(os.path.dirname(__file__), ".certs")
        os.makedirs(cert_dir, exist_ok=True)
        cert_path = os.path.join(cert_dir, "cert.pem")
        key_path = os.path.join(cert_dir, "key.pem")

        if not os.path.exists(cert_path) or not os.path.exists(key_path):
            generate_self_signed_cert(cert_path, key_path)

        ssl_context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
        ssl_context.load_cert_chain(cert_path, key_path)

        scheme = "wss"
    else:
        scheme = "ws"

    print(f"Starting WebSocket test server on {scheme}://{host}:{port}")
    print("Press Ctrl+C to stop")
    print("-" * 50)

    async with websockets.serve(send_messages, host, port, ssl=ssl_context):
        await asyncio.Future()  # Run forever


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\nServer stopped")
