use crate::error::{ProviderError, ProviderResult};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;

/// Configuration for the WebSocket provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// WebSocket server URL to connect to
    pub websocket_url: String,

    /// NATS subject to publish received messages to
    pub nats_subject: String,

    /// Maximum reconnection attempts (0 for infinite)
    #[serde(default = "default_max_reconnect_attempts")]
    pub max_reconnect_attempts: u32,

    /// Initial reconnection delay in milliseconds
    #[serde(default = "default_initial_reconnect_delay_ms")]
    pub initial_reconnect_delay_ms: u64,

    /// Maximum reconnection delay in milliseconds
    #[serde(default = "default_max_reconnect_delay_ms")]
    pub max_reconnect_delay_ms: u64,

    /// Maximum message size in bytes
    #[serde(default = "default_max_message_size")]
    pub max_message_size: usize,
}

fn default_max_reconnect_attempts() -> u32 {
    0 // Infinite retries
}

fn default_initial_reconnect_delay_ms() -> u64 {
    1000 // 1 second
}

fn default_max_reconnect_delay_ms() -> u64 {
    60000 // 60 seconds
}

fn default_max_message_size() -> usize {
    1024 * 1024 // 1 MB
}

impl ProviderConfig {
    /// Create a new configuration
    pub fn new(websocket_url: String, nats_subject: String) -> Self {
        Self {
            websocket_url,
            nats_subject,
            max_reconnect_attempts: default_max_reconnect_attempts(),
            initial_reconnect_delay_ms: default_initial_reconnect_delay_ms(),
            max_reconnect_delay_ms: default_max_reconnect_delay_ms(),
            max_message_size: default_max_message_size(),
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> ProviderResult<()> {
        // Validate WebSocket URL
        let url = Url::parse(&self.websocket_url)?;
        if url.scheme() != "ws" && url.scheme() != "wss" {
            return Err(ProviderError::ConfigError(
                "WebSocket URL must use ws:// or wss:// scheme".to_string(),
            ));
        }

        // Validate NATS subject
        if self.nats_subject.is_empty() {
            return Err(ProviderError::ConfigError(
                "NATS subject cannot be empty".to_string(),
            ));
        }

        // Validate reconnection delays
        if self.initial_reconnect_delay_ms > self.max_reconnect_delay_ms {
            return Err(ProviderError::ConfigError(
                "Initial reconnect delay cannot be greater than max reconnect delay".to_string(),
            ));
        }

        Ok(())
    }

    /// Get the initial reconnection delay as Duration
    pub fn initial_reconnect_delay(&self) -> Duration {
        Duration::from_millis(self.initial_reconnect_delay_ms)
    }

    /// Get the maximum reconnection delay as Duration
    pub fn max_reconnect_delay(&self) -> Duration {
        Duration::from_millis(self.max_reconnect_delay_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation_valid_ws() {
        let config = ProviderConfig::new(
            "ws://localhost:8080".to_string(),
            "websocket.messages".to_string(),
        );
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_valid_wss() {
        let config = ProviderConfig::new(
            "wss://example.com/socket".to_string(),
            "websocket.messages".to_string(),
        );
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_invalid_scheme() {
        let config = ProviderConfig::new(
            "http://localhost:8080".to_string(),
            "websocket.messages".to_string(),
        );
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_empty_subject() {
        let config = ProviderConfig::new("ws://localhost:8080".to_string(), "".to_string());
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_invalid_delays() {
        let mut config = ProviderConfig::new(
            "ws://localhost:8080".to_string(),
            "websocket.messages".to_string(),
        );
        config.initial_reconnect_delay_ms = 10000;
        config.max_reconnect_delay_ms = 5000;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_default_values() {
        let config = ProviderConfig::new(
            "ws://localhost:8080".to_string(),
            "websocket.messages".to_string(),
        );
        assert_eq!(config.max_reconnect_attempts, 0);
        assert_eq!(config.initial_reconnect_delay_ms, 1000);
        assert_eq!(config.max_reconnect_delay_ms, 60000);
        assert_eq!(config.max_message_size, 1024 * 1024);
    }
}
