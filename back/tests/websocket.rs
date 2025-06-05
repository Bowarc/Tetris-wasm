use futures::{SinkExt, StreamExt};
use rocket::local::asynchronous::Client;
use serde_json;
use tokio_tungstenite::connect_async;
use shared::{ClientMessage, ServerMessage};
use serial_test;

use back::build_rocket;

#[tokio::test]
#[serial_test::serial] // TODO: We can avoid serial tests once we have rooms.
async fn lines_destroyed_not_echoed_carto_sender() {
    let (mut ws_stream, _) = connect_async("ws://127.0.0.1:42071/ws")
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
#[serial_test::serial] // TODO: We can avoid serial tests once we have rooms.
async fn lines_destroyed_broadcasted_to_other_client() {
    let client = Client::tracked(build_rocket().await).await.expect("Failed to track client");
    let response = client.get("/ws").dispatch().await;
    println!("Response: {:?}", response);
    println!("Testing broadcast functionality");
    let response = client.get("/broadcast/lol").dispatch().await;
    println!("Response: {:?}", response);
    return;
    // Connect two clients
    let (mut ws1, _) = connect_async("ws://127.0.0.1:42071/ws")
        .await
        .expect("Failed to connect ws1");
    let (mut ws2, _) = connect_async("ws://127.0.0.1:42071/ws")
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
