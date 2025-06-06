#[macro_use(trace, debug, info, warn, error)]
extern crate log;

pub mod catchers;
pub mod response;
pub mod routes;

// Needed for tests
pub async fn build_rocket() -> rocket::Rocket<rocket::Ignite> {
    let user_map = routes::UserMap::default();

    rocket::build()
        .manage(user_map)
        .register("/", rocket::catchers![catchers::root_404])
        .mount(
            "/",
            rocket::routes![
                routes::root,
                routes::home,
                routes::_404,
                routes::front_js,
                routes::front_bg_wasm,
                routes::worker_js,
                routes::worker_bg_wasm,
                routes::index_html,
                routes::static_css,
                routes::static_resource,
                routes::favicon_ico,
                routes::sitemap_xml,
                routes::robots_txt,
                routes::ws_join,
                routes::ws_broadcast,
                // Theses routes are troll routes, made to fuck with the bots
                routes::bot_env,
                routes::bot_admin,
                routes::bot_wp,
                routes::bot_wordpress,
                routes::bot_wp_admin,
            ],
        )
        .ignite()
        .await
        .unwrap()
}
