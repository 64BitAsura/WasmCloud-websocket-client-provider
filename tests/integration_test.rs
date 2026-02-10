use futures_util::SinkExt;
use tokio::net::TcpListener;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use websocket_provider::{ProviderConfig, WebSocketClient};

/// Helper function to start a mock WebSocket server
async fn start_mock_ws_server(port: u16) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

        // Accept one connection
        if let Ok((stream, _)) = listener.accept().await {
            if let Ok(mut ws_stream) = accept_async(stream).await {
                // Send a test message
                let _ = ws_stream
                    .send(Message::Text("Hello from mock server".to_string()))
                    .await;
                sleep(Duration::from_millis(100)).await;

                // Send a binary message
                let _ = ws_stream.send(Message::Binary(vec![1, 2, 3, 4])).await;
                sleep(Duration::from_millis(100)).await;

                // Close the connection
                let _ = ws_stream.close(None).await;
            }
        }
    })
}

#[tokio::test]
async fn test_websocket_client_integration() {
    // Start mock server
    let port = 9001;
    let server_handle = start_mock_ws_server(port).await;

    // Give server time to start
    sleep(Duration::from_millis(100)).await;

    // Create config
    let config = ProviderConfig::new(
        format!("ws://127.0.0.1:{}", port),
        "test.subject".to_string(),
    );

    // Track received messages
    let received_messages = std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let received_clone = received_messages.clone();

    // Create WebSocket client
    let client = WebSocketClient::new(config);

    // Run client with message handler
    let client_handle = tokio::spawn(async move {
        let _ = client
            .run(move |data| {
                let received = received_clone.clone();
                tokio::spawn(async move {
                    received.lock().await.push(data);
                });
                Ok(())
            })
            .await;
    });

    // Wait for messages to be received
    sleep(Duration::from_millis(500)).await;

    // Check received messages
    let messages = received_messages.lock().await;
    assert!(
        !messages.is_empty(),
        "Should have received at least one message"
    );

    // Verify first message content
    if let Some(first_msg) = messages.first() {
        let text = String::from_utf8(first_msg.clone()).unwrap();
        assert!(text.contains("Hello from mock server"));
    }

    // Clean up
    client_handle.abort();
    server_handle.abort();
}

#[tokio::test]
async fn test_websocket_client_reconnection() {
    // Create config
    let config = ProviderConfig {
        websocket_url: "ws://127.0.0.1:9999".to_string(), // Non-existent server
        nats_subject: "test.subject".to_string(),
        max_reconnect_attempts: 2, // Limit attempts for test
        initial_reconnect_delay_ms: 100,
        max_reconnect_delay_ms: 500,
        max_message_size: 1024 * 1024,
    };

    let client = WebSocketClient::new(config);

    // This should fail after 2 attempts
    let result = client.run(|_| Ok(())).await;

    assert!(
        result.is_err(),
        "Should fail to connect to non-existent server"
    );
}

#[tokio::test]
async fn test_websocket_message_size_limit() {
    // Start mock server that sends a large message
    let port = 9002;
    let server = tokio::spawn(async move {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

        if let Ok((stream, _)) = listener.accept().await {
            if let Ok(mut ws_stream) = accept_async(stream).await {
                // Send a large message
                let large_message = "x".repeat(2000);
                let _ = ws_stream.send(Message::Text(large_message)).await;
                sleep(Duration::from_millis(200)).await;
            }
        }
    });

    // Give server time to start
    sleep(Duration::from_millis(100)).await;

    // Create config with small message size limit
    let config = ProviderConfig {
        websocket_url: format!("ws://127.0.0.1:{}", port),
        nats_subject: "test.subject".to_string(),
        max_reconnect_attempts: 1,
        initial_reconnect_delay_ms: 100,
        max_reconnect_delay_ms: 500,
        max_message_size: 1000, // Small limit
    };

    let received_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let count_clone = received_count.clone();

    let client = WebSocketClient::new(config);

    let client_handle = tokio::spawn(async move {
        let _ = client
            .run(move |_| {
                count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Ok(())
            })
            .await;
    });

    // Wait for potential message
    sleep(Duration::from_millis(300)).await;

    // Message should be skipped due to size limit
    let count = received_count.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(count, 0, "Large message should be skipped");

    // Clean up
    client_handle.abort();
    server.abort();
}
