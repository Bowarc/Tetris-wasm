use futures::{stream::SplitSink, SinkExt, StreamExt};
use rocket::{get, State};
use rocket_ws::{stream::DuplexStream, Message};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub type UserMap = Arc<Mutex<HashMap<i32, Arc<Mutex<SplitSink<DuplexStream, Message>>>>>>;


#[get("/ws/<user_id>")]
pub async fn ws_join<'a>(
    user_id: i32,
    socket: rocket_ws::WebSocket,
    user_map: &'a State<UserMap>,
) -> rocket_ws::Channel<'a> {
    socket.channel(move |stream| {
        Box::pin(async move {
            let (sender, mut receiever) = stream.split();
            // This creates a reference counting heap pointer to the sender with a mutex on top to prevent data races
            let arc_sender = Arc::new(Mutex::new(sender));

            // Add the user to the map, and a pointer to its sender, so we can send them messages through this ws
            let Ok(mut user_map_guard) = user_map.try_lock() else {
                return Err(rocket_ws::result::Error::Http({
                    let mut resp = http::Response::new(None);
                    *resp.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    resp
                }));
            };

            user_map_guard.insert(user_id, arc_sender.clone());
            println!("New ws user: {user_id}");
            drop(user_map_guard); // release the guard, no need to keep it around

            // Here are the messages we receive from this user's websocket,
            // we could push it to a queue that a logic loop handles
            while let Some(message) = receiever.next().await {
                match message {
                    Ok(rocket_ws::Message::Close(_)) => {
                        debug!("Closing ws for user {user_id}");
                        let _e = arc_sender.lock().await.close();
                        user_map.lock().await.remove(&user_id).unwrap();
                        // Here the following receiever.next will give a None so the loop will stop
                    }
                    Ok(m) => { // any other message 
                        println!("Unhandled message: {m}");

                    }
                    Err(_) => {
                        print!("Failed to read user {user_id}'s receiver");
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

#[rocket::get("/broadcast/<content>")]
pub async fn ws_broadcast(content: &str, user_map: &State<UserMap>) -> crate::response::Response {
    let map_lock = user_map.lock().await;

    // Simple example on how to send a message to every saved websockets
    for (_user_id, ws)  in map_lock.iter() {
        let mut sink = ws.lock().await;

        sink.send(Message::Text(format!("Broadcast: {}", content)))
            .await
            .unwrap();
    }

    crate::response::Response::builder()
        .with_content("Ok")
        .build()
}
