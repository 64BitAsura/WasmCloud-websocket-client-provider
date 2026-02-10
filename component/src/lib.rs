wit_bindgen::generate!({ generate_all });

use crate::exports::wasmcloud::websocket::message_handler::{Guest, WebsocketMessage};
use crate::wasi::logging::logging::*;

struct WebSocketComponent;

impl Guest for WebSocketComponent {
    fn handle_message(message: WebsocketMessage) -> Result<(), String> {
        // Log the received message
        log(
            Level::Info,
            "",
            &format!(
                "Received WebSocket message - Type: {}, Size: {} bytes, Timestamp: {}",
                message.message_type, message.size, message.timestamp
            ),
        );

        // Log the payload (truncated if too long)
        let payload_preview = if message.payload.len() > 100 {
            format!("{}...", &message.payload[..100])
        } else {
            message.payload.clone()
        };

        log(
            Level::Info,
            "",
            &format!("Message payload: {}", payload_preview),
        );

        // Successfully handled the message
        Ok(())
    }
}

export!(WebSocketComponent);
