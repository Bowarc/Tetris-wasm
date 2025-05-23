use yew::{function_component, html, Html};

#[derive(yew::Properties, std::cmp::PartialEq)]
pub struct Props {
    pub current_scene: yew::UseStateHandle<crate::scene::Scene>,
}

#[function_component]
pub fn Home(_props: &Props) -> Html {
    use wasm_bindgen_futures::spawn_local;
    use gloo::net::websocket::{Message, futures::WebSocket};
    use futures::{SinkExt, StreamExt};

    if let Some(nav) = yew_router::hooks::use_navigator() {
        nav.replace(&crate::Route::Home)
    } else {
        error!("Failed to retrieve the navigator")
    }

    let ws = WebSocket::open("/ws/1").unwrap();
    let (mut write, mut read) = ws.split();

    spawn_local(async move {
        write
            .send(Message::Text(String::from("test")))
            .await
            .unwrap();
        write
            .send(Message::Text(String::from("test 2")))
            .await
            .unwrap();
    });

    spawn_local(async move {
        while let Some(msg) = read.next().await {
            debug!(format!("{:?}", msg))
        }
        debug!("WebSocket Closed")
    });

    // let (i18n, _) = i18nrs::yew::use_translation();

    html! { <>
    </>}
}
