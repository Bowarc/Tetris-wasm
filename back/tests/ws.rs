use std::{
    any::Any,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Once,
};

use back::build_rocket;
use rocket::{http::Header, local::asynchronous::Client, Rocket};
use tokio::net::TcpListener;
use tokio_tungstenite::connect_async;
use logger::{Config, Output};

type Port = u16;

async fn start_test() -> Port {
    static INIT: Once = Once::new();
    // Void any logs made using loggers
    INIT.call_once(|| {
        logger::init(
            Config::default()
                .level(log::LevelFilter::Off)
                .filter("rocket", log::LevelFilter::Off)
                // .output(Output::CustomStream(Box::new(std::io::sink()))),
        );
    });

    let rocket = back::build_rocket().await;

    // Since they run in parallel, we add a small delay so they all don't check the port, see that it's fine and try to run on it
    //
    // I could have done a simple loop with launch.is err, but if it doesn't fail, it does not return, soo we don't ever get an anwser
    tokio::time::sleep(tokio::time::Duration::from_millis(random::get_inc(0, 500))).await;

    let mut port = rocket.config().port;
    loop {
        if TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port))
            .await
            .is_ok()
        {
            break;
        }

        port += 1;
    }

    // Create a modify request for the rocket
    let f = rocket.figment().clone().merge(("port", port));

    // Create a new rocket based on the first one, with the modification
    let rocket = Rocket::custom(rocket.config())
        .mount("/", rocket.routes().cloned().collect::<Vec<_>>())
        .manage(back::routes::UserMap::default())
        .configure(f)
        .ignite()
        .await
        .unwrap();

    // println!(
    //     "{:?}",
    //     rocket
    //         .routes()
    //         .map(|route| route.uri.to_string())
    //         .collect::<Vec<String>>()
    // );

    tokio::spawn(async { rocket.launch().await.unwrap(); });
        // .await
        // .unwrap();


    println!("Rocket launched on port: {port}");
    port
}

#[rocket::async_test]
async fn ws() {
    start_test().await;

    let client = Client::untracked(build_rocket().await)
        .await
        .expect("valid rocket instance");

    println!(
        "{:#?}",
        client
            .rocket()
            .routes()
            .map(|route| format!("{}, {}", route.uri, route.method))
            .collect::<Vec<String>>()
    );

    let response = client
        .get("/ws/1")
        .header(Header::new("Host", "localhost"))
        .header(Header::new("Connection", "upgrade"))
        .header(Header::new("Upgrade", "websocket"))
        .header(Header::new("Sec-WebSocket-Version", "13"))
        .header(Header::new("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ=="))
        .dispatch()
        .await;
    print!("\n{response:?}\n\n");
}

#[rocket::async_test]
async fn test2() {
    let port = start_test().await;

    let (stream, response) = connect_async(format!("ws://127.0.0.1:{port}/ws/1"))
        .await
        .unwrap();

    println!("Response: {response:?}");
}
