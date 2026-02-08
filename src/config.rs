//! Configuration module for the WebSocket provider

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use url::Url;

/// Configuration for the WebSocket provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// WebSocket server URL (ws:// or wss://)
    pub websocket_url: String,

    /// NATS subject to publish received messages
    pub nats_subject: String,

    /// Reconnection attempt interval in seconds
    #[serde(default = "default_reconnect_interval")]
    pub reconnect_interval_secs: u64,

    /// Maximum reconnection attempts (0 = infinite)
    #[serde(default)]
    pub max_reconnect_attempts: u32,

    /// Enable TLS verification for wss:// connections
    #[serde(default = "default_true")]
    pub tls_verification: bool,
}

fn default_reconnect_interval() -> u64 {
    5
}

fn default_true() -> bool {
    true
}

impl ProviderConfig {
    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        // Validate WebSocket URL
        let url = Url::parse(&self.websocket_url).context("Invalid WebSocket URL format")?;

        match url.scheme() {
            "ws" | "wss" => {}
            scheme => {
                anyhow::bail!(
                    "Invalid WebSocket scheme: {}. Must be 'ws' or 'wss'",
                    scheme
                )
            }
        }

        // Validate NATS subject
        if self.nats_subject.is_empty() {
            anyhow::bail!("NATS subject cannot be empty");
        }

        if self.nats_subject.contains(' ') {
            anyhow::bail!("NATS subject cannot contain spaces");
        }

        // Validate reconnect interval
        if self.reconnect_interval_secs == 0 {
            anyhow::bail!("Reconnection interval must be greater than 0");
        }

        Ok(())
    }
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            websocket_url: "ws://localhost:8080".to_string(),
            nats_subject: "websocket.messages".to_string(),
            reconnect_interval_secs: 5,
            max_reconnect_attempts: 0,
            tls_verification: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ws_config() {
        let config = ProviderConfig {
            websocket_url: "ws://example.com:8080/ws".to_string(),
            nats_subject: "test.messages".to_string(),
            reconnect_interval_secs: 10,
            max_reconnect_attempts: 5,
            tls_verification: false,
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_valid_wss_config() {
        let config = ProviderConfig {
            websocket_url: "wss://example.com/ws".to_string(),
            nats_subject: "secure.messages".to_string(),
            reconnect_interval_secs: 5,
            max_reconnect_attempts: 0,
            tls_verification: true,
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_scheme() {
        let config = ProviderConfig {
            websocket_url: "http://example.com".to_string(),
            nats_subject: "test.messages".to_string(),
            ..Default::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_url() {
        let config = ProviderConfig {
            websocket_url: "not a url".to_string(),
            nats_subject: "test.messages".to_string(),
            ..Default::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_empty_nats_subject() {
        let config = ProviderConfig {
            websocket_url: "ws://example.com".to_string(),
            nats_subject: "".to_string(),
            ..Default::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_nats_subject_with_spaces() {
        let config = ProviderConfig {
            websocket_url: "ws://example.com".to_string(),
            nats_subject: "test messages".to_string(),
            ..Default::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_zero_reconnect_interval() {
        let config = ProviderConfig {
            websocket_url: "ws://example.com".to_string(),
            nats_subject: "test.messages".to_string(),
            reconnect_interval_secs: 0,
            ..Default::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_default_config() {
        let config = ProviderConfig::default();
        assert!(config.validate().is_ok());
        assert_eq!(config.reconnect_interval_secs, 5);
        assert_eq!(config.max_reconnect_attempts, 0);
        assert!(config.tls_verification);
    }

    #[test]
    fn test_serde_serialization() {
        let config = ProviderConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: ProviderConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.websocket_url, deserialized.websocket_url);
    }
}
