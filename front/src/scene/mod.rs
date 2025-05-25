mod not_found;
pub use not_found::NotFound;
// mod wasm;
// pub use wasm::WASM;
mod home;
pub use home::Home;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Scene {
    Home,
    NotFound,
}

impl Scene {
    pub fn html(&self, current_scene: yew::UseStateHandle<Scene>) -> yew::virtual_dom::VNode {
        use yew::html;

        match self {
            Scene::Home => html! {<Home {current_scene}/>},
            Scene::NotFound => html! {<NotFound />},
        }
    }
}

impl std::fmt::Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scene::Home => write!(f, "Home"),
            Scene::NotFound => write!(f, "Not found"),
        }
    }
}
