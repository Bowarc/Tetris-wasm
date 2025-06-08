use futures::{stream::SplitSink, SinkExt, StreamExt};
use rocket::{get, State};
use rocket_ws::{stream::DuplexStream, Message};
use shared::{ClientMessage, ServerMessage};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use serde_json;

pub type UserMap = Arc<Mutex<HashMap<u128, Arc<Mutex<SplitSink<DuplexStream, Message>>>>>>;

pub const MAX_RETRIES: u32 = 5;

#[get("/ws")]
pub async fn ws_join<'a>(
    socket: rocket_ws::WebSocket,
    user_map: &'a State<UserMap>,
) -> rocket_ws::Channel<'a> {
    debug!("New websocket connection request received.");

    socket.channel(move |stream| {
        Box::pin(async move {
            let (sender, mut receiever) = stream.split();
            // This creates a reference counting heap pointer to the sender with a mutex on top to prevent data races
            let arc_sender = Arc::new(Mutex::new(sender));

            // Add the user to the map, and a pointer to its sender, so we can send them messages through this ws
            // Retry 5 times before failing
            let mut tries: u32 = 0;
            // let mut user_map = user_map.clone();
            let mut user_id = random::get_inc(u128::MIN, u128::MAX);
            while tries < MAX_RETRIES {
                debug!("Retry number: {tries} for user id: {user_id}");
                if let Ok(mut user_map_guard) = user_map.try_lock() {
                    
                    // Generate a random user ID until we find one that does not exist in the map
                    while user_map_guard.contains_key(&user_id) {
                        // If the user ID already exists, generate a new one
                        debug!("User ID {user_id} already exists, generating a new one.");
                        user_id = random::get_inc(u128::MIN, u128::MAX);
                        debug!("New user ID generated: {user_id}");
                    }
                    user_map_guard.insert(user_id, arc_sender.clone());
                    debug!("User map updated with user id: {user_id}");
                    break;
                } else {
                    // Failed to lock, wait a bit and try again
                    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                }
                tries += 1;
            }
            if tries == MAX_RETRIES {
                // Failed to lock the user map after 5 tries, return an error
                debug!("Failed to lock the user map after {MAX_RETRIES} tries. Returning error.");
                return Err(rocket_ws::result::Error::Http({
                    let mut resp = http::Response::new(None);
                    *resp.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    resp
                }));
            }
            debug!("New ws user: {user_id}");

            // Here are the messages we receive from this user's websocket,
            while let Some(message) = receiever.next().await {
                match message {
                    Ok(rocket_ws::Message::Close(_)) => {
                        debug!("Closing ws for user {user_id}");
                        let _e = arc_sender.lock().await.close();
                        user_map.lock().await.remove(&user_id).unwrap();
                        // Here the following receiever.next will give a None so the loop will stop
                    }
                    Ok(Message::Text(text_message)) => {
                        // Parse the text message as a ClientMessage
                        let client_message: ClientMessage = match serde_json::from_str(&text_message) {
                            Ok(msg) => msg,
                            Err(e) => {
                                debug!("Failed to parse message from user {user_id}: {e}");
                                continue; // Skip this message
                            }
                        };
                        debug!("Received message from user {user_id}: {client_message:?}");
                        match &client_message {
                            ClientMessage::BoardUpdate(board_update) => {
                                // Handle the board update message
                                debug!("User {user_id} sent a board update: {board_update:?}");
                                // Broadcast the message to all users except the source user
                                broadcast_message(user_id, &client_message, user_map).await;
                            }
                            ClientMessage::LinesDestroyed(line_shapes) => {
                                // Handle the chat message
                                debug!("User {user_id} sent a lines destroyed message: {line_shapes:?}");
                                // TODO: should we use only user_map (and thus if GameOver -> close socket) or use a separate
                                // list of users still alive?
                                // Select a random user to send the message to
                                let map_lock = user_map.lock().await;
                                let keys: Vec<u128> = map_lock.keys().cloned().collect();
                                // TODO: should we hold the lock for the whole LinesDestroyed case duration?
                                drop(map_lock); // Release the lock before using the keys.
                                if keys.len() < 2 {
                                    debug!("Not enough users to send lines destroyed message to. Skipping.");
                                    continue; // Not enough users to send the message to
                                }
                                let mut victim_id = user_id;
                                while victim_id == user_id {
                                    // Pick a random user ID from the keys
                                    victim_id = *random::pick(&keys);
                                }
                                debug!("User {user_id} sent a lines destroyed message to user {victim_id}: {line_shapes:?}");
                                send_lines_destroyed_message(victim_id, &client_message, user_map).await;
                            }
                            // TODO: broadcast board update to everyone (except source) and broadcast lines destroyed only
                            // to one random person.
                            ClientMessage::GameOver => {
                                // Handle the game over message
                                debug!("User {user_id} sent a game over message.");
                                // Broadcast the game over message to all users except the source user
                                broadcast_message(user_id, &client_message, user_map).await;
                            }
                        }
                    }
                    Ok(m) => {
                        debug!("Unhandled message type: {m:?}");
                    }
                    Err(_) => {
                        print!("Failed to read user {user_id}'s receiver. Terminating connection.");
                        let _e = arc_sender.lock().await.close();
                        user_map.lock().await.remove(&user_id);
                        // Here the following receiever.next will give a None so the loop will stop
                    }
                }
            }
            // If we get to this point, the websocket is closed
            Ok(())
        })
    })
}

pub async fn send_lines_destroyed_message (victim_id: u128, client_message: &ClientMessage, user_map: &State<UserMap>) {
    let server_message = ServerMessage::Broadcast { user_id: victim_id, msg: client_message.clone() };

    let map_lock = user_map.lock().await;

    // Send the lines destroyed message to the victim user
    debug!("Sending lines destroyed message to user {victim_id}: {:?}", server_message);
    if let Some(ws) = map_lock.get(&victim_id) {
        let content = serde_json::to_string(&server_message).unwrap();
        let mut sink = ws.lock().await;

        sink.send(Message::Text(format!("Broadcast: {}", content)))
            .await
            .unwrap();
    } else {
        debug!("User {victim_id} not found in user map. Not sending message.");
    }
}
pub async fn broadcast_message(source_user_id: u128, client_message: &ClientMessage, user_map: &State<UserMap>) {
    let server_message = ServerMessage::Broadcast { user_id: source_user_id, msg: client_message.clone() };

    let map_lock = user_map.lock().await;

    // Broadcast the board update to all users except the source user
    debug!("Broadcasting board update from user {source_user_id}: {:?}", server_message);
    for (_user_id, ws) in map_lock.iter() {
        if source_user_id == *_user_id {
            // Do not send the message to the source user
            continue;
        }
        let content = serde_json::to_string(&server_message).unwrap();
        let mut sink = ws.lock().await;

        sink.send(Message::Text(format!("Broadcast: {}", content)))
            .await
            .unwrap();
    }
}

#[rocket::get("/broadcast/<content>")]
pub async fn ws_broadcast(content: &str, user_map: &State<UserMap>) -> crate::response::Response {
    
    debug!("New broadcast request received.");


    let map_lock = user_map.lock().await;

    // Simple example on how to send a message to every saved websockets
    for (_user_id, ws) in map_lock.iter() {
        let mut sink = ws.lock().await;

        sink.send(Message::Text(format!("Broadcast: {}", content)))
            .await
            .unwrap();
    }

    crate::response::Response::builder()
        .with_content("Ok")
        .build()
}
