use futures::{SinkExt, StreamExt};
use rocket::tokio;
use rocket::tokio::task;
use rocket::figment::Figment;
use rocket::Config;
use serial_test::serial;
use shared::ClientMessage;
use std::net::SocketAddr;
use tokio_tungstenite::connect_async;

fn spawn_rocket() -> SocketAddr {
    // Use a random port for parallel test safety
    let config = Config {
        port: 0,
        ..Config::default()
    };
    let figment = Figment::from(config);

    let rocket = server::rocket().configure(figment); // <-- adjust to your Rocket builder
    let rocket = rocket.launch();

    // Spawn the server in a background task
    let handle = task::spawn(rocket);

    // Wait for the server to be ready and get the port
    let rt = tokio::runtime::Handle::current();
    let addr = rt.block_on(async {
        loop {
            if let Ok(listener) = std::net::TcpListener::bind("127.0.0.1:0") {
                let addr = listener.local_addr().unwrap();
                drop(listener);
                break addr;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    });
    addr
}

#[tokio::test]
#[serial]
async fn lines_destroyed_not_echoed_to_sender() {
    let addr = spawn_rocket();
    let ws_url = format!("ws://{}/ws", addr);

    let (mut ws_stream, _) = connect_async(&ws_url)
        .await
        .expect("Failed to connect");

    // Send LinesDestroyed message
    let msg = ClientMessage::LinesDestroyed(vec![0b101010]);
    let json = serde_json::to_string(&msg).unwrap();
    ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(json.into())).await.unwrap();

    // Wait for a short time to see if we receive any message
    let mut received_lines_destroyed = false;
    let timeout = tokio::time::sleep(std::time::Duration::from_millis(500));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            Some(Ok(msg)) = ws_stream.next() => {
                if let tokio_tungstenite::tungstenite::Message::Text(text) = msg {
                    if text.contains("LinesDestroyed") {
                        received_lines_destroyed = true;
                        break;
                    }
                }
            }
            _ = &mut timeout => {
                break;
            }
        }
    }

    assert!(!received_lines_destroyed, "Sender should not receive LinesDestroyed message");
    ws_stream.close(None).await.expect("Failed to close websocket");
}

#[tokio::test]
#[serial]
async fn lines_destroyed_broadcasted_to_other_client() {
    let addr = spawn_rocket();
    let ws_url = format!("ws://{}/ws", addr);

    // Connect two clients
    let (mut ws1, _) = connect_async(&ws_url)
        .await
        .expect("Failed to connect ws1");
    let (mut ws2, _) = connect_async(&ws_url)
        .await
        .expect("Failed to connect ws2");

    // ws1 sends LinesDestroyed
    let msg = ClientMessage::LinesDestroyed(vec![0b10000]);
    let json = serde_json::to_string(&msg).unwrap();
    ws1.send(tokio_tungstenite::tungstenite::Message::Text(json.into())).await.unwrap();

    // ws2 should receive the message
    let mut received = false;
    let timeout = tokio::time::sleep(std::time::Duration::from_millis(1000));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            Some(Ok(msg)) = ws2.next() => {
                if let tokio_tungstenite::tungstenite::Message::Text(text) = msg {
                    if text.contains("LinesDestroyed") {
                        received = true;
                        break;
                    }
                }
            }
            _ = &mut timeout => {
                break;
            }
        }
    }

    assert!(received, "Other client should receive LinesDestroyed message");
    ws1.close(None).await.expect("Failed to close websocket");
    ws2.close(None).await.expect("Failed to close websocket");
}