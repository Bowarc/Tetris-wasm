use std::time::Duration;

use futures::sink::SinkExt;
use futures::{FutureExt, StreamExt};
use gloo::net::websocket::futures::WebSocket;
use serde::{Deserialize, Serialize};
use yew::platform::time::sleep;
use yew_agent::reactor::{reactor, ReactorScope};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReactorControlSignal {
    Start,
    Stop,
}

#[reactor]
pub async fn WsReactor(mut scope: ReactorScope<ReactorControlSignal, String>) {
    // I want the worker to stop if it does not receive a start command in the first second of it's lifetime
    let mut count = 0;
    while Some(ReactorControlSignal::Start) != scope.next().await {
        count += 1;

        if count > 10 {
            return;
        }

        sleep(Duration::from_millis(100)).fuse().await;
    }

    debug!("Starting ws");

    let mut ws = WebSocket::open("/ws/1").unwrap();

    'inner: loop {
        futures::select! {
            cs = scope.next() => match cs{
                Some(ReactorControlSignal::Start) => {
                    debug!("Ws already started")
                }
                Some(ReactorControlSignal::Stop) => {
                    if let Err(e) = ws.close(Some(1000 /*Normal closure*/), None){
                        error!(format!("Failed to close the websocket due to: {e}"))
                    };
                    break 'inner;
                }
                None => {

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
