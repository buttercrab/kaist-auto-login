use warp::Filter;

mod conf;
mod database;
mod ws;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    database::init().await;

    let ws_v1 = warp::path("v1")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(ws::v1::websocket));
    let ws = warp::path("ws").and(ws_v1);

    let routes = warp::get().and(ws);

    let address = conf::address();
    warp::serve(routes).run(address).await;
}
