use std::{sync::LazyLock, time::Duration};

use futures::{lock::Mutex, sink::SinkExt};
use futures::{FutureExt, StreamExt};
use gloo::net::websocket::{futures::WebSocket, Message};
use serde::{Deserialize, Serialize};
use yew::platform::time::sleep;
use yew_agent::reactor::{reactor, ReactorScope};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReactorControlSignal {
    Start,
    Stop,
    WsMessage(shared::ClientMessage),
}

static LOCK: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

#[reactor]
pub async fn WsReactor(mut scope: ReactorScope<ReactorControlSignal, String>) {
    debug!("Plop");
    // I want the worker to stop if it does not receive a start command in the first second of it's lifetime
    let mut count = 0;
    while Some(ReactorControlSignal::Start) != scope.next().await {
        count += 1;

        if count > 10 {
            return;
        }

        sleep(Duration::from_millis(100)).fuse().await;
    }

    let mut lock = loop {
        match LOCK.try_lock() {
            Some(lock) => break lock,
            None => {
                debug!("Waiting for lock");
                continue;
            }
        }
    };

    if *lock != 0{
        return;
    }

    debug!(format!("Got lock: {lock:?}"));

    debug!("Starting ws");

    let mut ws = match WebSocket::open("ws://127.0.0.1:42071/ws") {
        Ok(ws) => {
            error!("New Ws has been created");
            ws
        }
        Err(e) => {
            error!(format!("Cannot open websocket due to: {e}"));
            return;
        }
    };

    *lock += 1;

    drop(lock);

    'inner: loop {
        futures::select! {
            rcs = scope.next() => match rcs{
                Some(ReactorControlSignal::Start) => {
                    debug!("Ws already started")
                }
                Some(ReactorControlSignal::Stop) => {
                    if let Err(e) = ws.close(Some(1000 /*Normal closure*/), None){
                        error!(format!("Failed to close the websocket due to: {e}"))
                    };
                    break 'inner;
                }
                Some(ReactorControlSignal::WsMessage(msg)) => {
                    warn!("Received message from game, sending to server");
                    let msg_string = match serde_json::to_string(&msg) {
                        Ok(s) => s,
                        Err(e) => {
                            error!(format!("Failed to serialize message: {msg:?} due to: {e}"));
                            if let Err(e) = scope.send(format!("Failed to serialize message: {msg:?} due to: {e}")).await{
                                error!(format!("Failed to send back error to scope due to: {e}"));
                            };
                            continue 'inner;
                        }
                    };
                    if let Err(e) = ws.send(Message::Text(msg_string)).await {
                        scope.send(format!("Failed to send message to websocket due to: {e}")).await.unwrap();
                    }
                }
                None => {
                    debug!("Scope received nothing");
                }
            },
            ws_message_opt = ws.next().fuse() => match ws_message_opt{
                Some(Ok(msg)) => {
                    debug!(format!("Ws received: {msg:?}"));
                    scope.send(format!("New message: {msg:?}")).await.unwrap();
                }
                Some(Err(e)) => {
                    error!(format!("Ws received a message but an error occured: {e}"));
                    break 'inner;
                }
                None => {
                    debug!("Received nothing");
                }
            },
        }
    }
}
