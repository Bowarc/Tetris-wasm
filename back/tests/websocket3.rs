use rocket::{self, figment::Figment, Config};
use rocket::tokio::{self, task};
use rocket_ws::{Message, client::connect};
use futures_util::{SinkExt, StreamExt};
use serial_test::serial;
use shared::ClientMessage;
use std::net::SocketAddr;
use url::Url;
use std::time::Duration;

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
    let _handle = task::spawn(rocket);

    // Find the port Rocket is listening on
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);
    addr
}

#[rocket::async_test]
#[serial]
async fn lines_destroyed_not_echoed_to_sender() {
    let addr = spawn_rocket();
    let ws_url = format!("ws://{}/ws", addr);
    let url = Url::parse(&ws_url).unwrap();

    // Wait for server to boot up
    tokio::time::sleep(Duration::from_millis(300)).await;

    let (mut ws_stream, _) = connect(url, None).await.expect("WebSocket connect failed");

    // Send LinesDestroyed message
    let msg = ClientMessage::LinesDestroyed(vec![0b101010]);
    let json = serde_json::to_string(&msg).unwrap();
    ws_stream.send(Message::Text(json)).await.expect("send failed");

    // Wait for a short time to see if we receive any message
    let mut received_lines_destroyed = false;
    let timeout = tokio::time::sleep(Duration::from_millis(500));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            Some(Ok(Message::Text(text))) = ws_stream.next() => {
                if text.contains("LinesDestroyed") {
                    received_lines_destroyed = true;
                    break;
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

#[rocket::async_test]
#[serial]
async fn lines_destroyed_broadcasted_to_other_client() {
    let addr = spawn_rocket();
    let ws_url = format!("ws://{}/ws", addr);
    let url = Url::parse(&ws_url).unwrap();

    // Wait for server to boot up
    tokio::time::sleep(Duration::from_millis(300)).await;

    // Connect two clients
    let (mut ws1, _) = connect(url.clone(), None).await.expect("Failed to connect ws1");
    let (mut ws2, _) = connect(url, None).await.expect("Failed to connect ws2");

    // ws1 sends LinesDestroyed
    let msg = ClientMessage::LinesDestroyed(vec![0b10000]);
    let json = serde_json::to_string(&msg).unwrap();
    ws1.send(Message::Text(json)).await.expect("send failed");

    // ws2 should receive the message
    let mut received = false;
    let timeout = tokio::time::sleep(Duration::from_millis(1000));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            Some(Ok(Message::Text(text))) = ws2.next() => {
                if text.contains("LinesDestroyed") {
                    received = true;
                    break;
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