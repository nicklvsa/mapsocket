use std::{sync::Arc, collections::HashMap, convert::Infallible};
use tokio::sync::RwLock;
use warp::{Filter};

use crate::{
    types::ClientsT, 
    handlers::healthcheck::healthcheck, 
    handlers::register::{register as reg, unregister as unreg},
    handlers::websocket::websocket,
    handlers::publish::publish
};

pub async fn serve() {
    let clients: ClientsT = Arc::new(RwLock::new(HashMap::new()));
    let healthcheck_route = warp::path!("healthcheck").and_then(healthcheck);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "Content-Type", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers"])
        .allow_methods(vec!["POST", "GET", "HEAD"]);

    let register = warp::path("register");
    let register_routes = register
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(reg)
        .or(register
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_clients(clients.clone()))
            .and_then(unreg));

    let publish_route = warp::path!("publish")
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(publish);

    let socket_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(websocket);

    let routes = healthcheck_route
        .or(register_routes)
        .or(publish_route)
        .or(socket_route)
        .with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

fn with_clients(clients: ClientsT) -> impl Filter<Extract = (ClientsT,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}