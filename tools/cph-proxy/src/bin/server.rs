use axum::Router;
use cph_proxy::{cfg::*, handler, init};
use log::*;

#[volo::main]
async fn main() {
    let h = init::init();
    let listen_host = format!(
        "{}:{}",
        get_global_cfg().listen_host.0,
        get_global_cfg().listen_host.1
    );
    info!("starting server in: {}", listen_host);

    let app = handler::axum_router(Router::new());

    let listener = tokio::net::TcpListener::bind(&listen_host).await.unwrap();

    axum::serve(listener, app).await.unwrap();
    drop(h);
}
