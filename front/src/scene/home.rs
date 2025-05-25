use yew::{function_component, html, Html};

#[derive(yew::Properties, std::cmp::PartialEq)]
pub struct Props {
    pub current_scene: yew::UseStateHandle<crate::scene::Scene>,
}

#[function_component]
pub fn Home(_props: &Props) -> Html {
    if let Some(nav) = yew_router::hooks::use_navigator() {
        nav.replace(&crate::Route::Home)
    } else {
        error!("Failed to retrieve the navigator")
    }

    let reactor_sub = yew_agent::reactor::use_reactor_subscription::<crate::component::WsReactor>();
    reactor_sub.send(crate::component::ReactorControlSignal::Start);

    html! { <>
        // <button onclick={start}>{ "Start ws" }</button>
    </>}
}
