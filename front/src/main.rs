#[allow(unused_imports)]
#[macro_use(trace, debug, info, warn, error, log)]
extern crate gloo_console;

mod app;
mod component;
mod scene;

#[derive(Debug, Clone, Copy, PartialEq, yew_router::Routable)]
pub enum Route {
    #[at("/")]
    Default,
    #[at("/home")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[yew::function_component]
fn Router() -> yew::Html {
    use {
        scene::Scene,
        yew::html,
        yew_router::{BrowserRouter, Switch},
    };

    html! {
        <BrowserRouter>
            <Switch<Route> render={ |route: Route| {
                let (scenes, default_scene_index) = match route {
                    Route::Default | Route::Home => {
                        (vec![
                            Scene::Home,
                        ],0)
                    }
                    Route::NotFound => {
                        (vec![
                            Scene::NotFound
                        ],0)
                    },
                };
                html! { <app::App {scenes} {default_scene_index} /> }
            }} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Router>::new().render();
}
