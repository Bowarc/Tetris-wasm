use yew_agent::Registrable;

use front::component::WsReactor;

fn main() {
    WsReactor::registrar().register();
}
