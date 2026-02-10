use crate::error::ProviderResult;
use serde::{Deserialize, Serialize};

/// Message wrapper for NATS publishing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    /// Message payload (can be text or binary data as base64)
    pub payload: String,

    /// Message type: "text" or "binary"
    pub message_type: String,

    /// Timestamp when message was received
    pub timestamp: u64,

    /// Original message size in bytes
    pub size: usize,
}

impl WebSocketMessage {
    /// Create a new WebSocket message from raw bytes
    pub fn from_bytes(data: Vec<u8>) -> Self {
        let (payload, message_type) = if let Ok(text) = String::from_utf8(data.clone()) {
            (text, "text".to_string())
        } else {
            // For binary data, encode as base64
            (base64::encode(&data), "binary".to_string())
        };

        Self {
            payload,
            message_type,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            size: data.len(),
        }
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> ProviderResult<Vec<u8>> {
        serde_json::to_vec(self).map_err(|e| e.into())
    }
}

// Base64 encoding helper
mod base64 {
    use base64::{engine::general_purpose, Engine as _};

    pub fn encode(data: &[u8]) -> String {
        general_purpose::STANDARD.encode(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_websocket_message_from_text() {
        let data = b"Hello, World!".to_vec();
        let message = WebSocketMessage::from_bytes(data);

        assert_eq!(message.message_type, "text");
        assert_eq!(message.payload, "Hello, World!");
        assert_eq!(message.size, 13);
        assert!(message.timestamp > 0);
    }

    #[test]
    fn test_websocket_message_from_binary() {
        let data = vec![0xFF, 0xFE, 0xFD, 0xFC];
        let message = WebSocketMessage::from_bytes(data);

        assert_eq!(message.message_type, "binary");
        assert_eq!(message.size, 4);
        assert!(message.timestamp > 0);
        // Payload should be base64 encoded
        assert!(!message.payload.is_empty());
    }

    #[test]
    fn test_websocket_message_to_json() {
        let data = b"test".to_vec();
        let message = WebSocketMessage::from_bytes(data);
        let json_result = message.to_json();

        assert!(json_result.is_ok());
        let json = json_result.unwrap();
        assert!(!json.is_empty());
    }
}
