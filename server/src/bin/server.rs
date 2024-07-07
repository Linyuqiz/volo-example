use std::{net::SocketAddr, time::Duration};

use server::example_router;
use tracing::info;
use volo_http::{
    context::ServerContext, http::StatusCode, server::layer::TimeoutLayer, Address, Router, Server,
};

fn timeout_handler(_: &ServerContext) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[volo::main]
async fn main() {
    // std::env::set_var("RUST_LOG", "info");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let app = Router::new()
        .merge(example_router())
        .layer(TimeoutLayer::new(Duration::from_secs(1), timeout_handler));

    let addr = "[::]:8080".parse::<SocketAddr>().unwrap();
    let addr = Address::from(addr);

    info!("Listening on {addr}");

    Server::new(app).run(addr).await.unwrap();
}
