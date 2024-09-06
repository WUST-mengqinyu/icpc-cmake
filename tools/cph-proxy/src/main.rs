mod cfg;
mod handler;
mod init;
pub mod model;

use axum::routing::any;
use axum::{body::Bytes, http::StatusCode, Router};
use cfg::*;
use log::*;
use std::net::IpAddr;

static REQWEST_SINGLTON_CLI: std::sync::OnceLock<reqwest::Client> = std::sync::OnceLock::new();

async fn proxy(ip: IpAddr, port: u32, body: Bytes) -> Result<(), (StatusCode, String)> {
    let cli = REQWEST_SINGLTON_CLI.get_or_init(|| {
        let mut clib = reqwest::Client::builder();
        if let Some((to_host_proxy_ip, to_host_proxy_port)) = GLOBAL_CFG.clone().to_host_proxy {
            clib = clib.proxy(
                reqwest::Proxy::http(&format!(
                    "http://{}:{}",
                    to_host_proxy_ip, to_host_proxy_port
                ))
                .map_err(|e| format!("build proxy failed: {}", e))
                .unwrap(),
            );
        }
        clib.build()
            .map_err(|e| format!("build proxy client failed: {}", e))
            .unwrap()
    });

    let ipstr = match ip {
        std::net::IpAddr::V4(ip) => ip.to_string(),
        std::net::IpAddr::V6(ip) => format!("[{}]", ip),
    };

    cli.post(format!("http://{}:{}", ipstr, port))
        .body(body)
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("send request failed: {}", e),
            )
        })?;
    Ok(())
}

async fn competitive_companion(body: Bytes) -> Result<(), (StatusCode, String)> {
    if let Some((ip, port)) = GLOBAL_CFG.clone().to_host {
        return proxy(ip, port, body).await;
    }
    if GLOBAL_CFG.must_self_host || GLOBAL_CFG.clone().to_host.is_none() {
        debug!("get request: {:?}", body);
        let v: model::ProblemMetaWithTestCase = serde_json::from_slice(&body).map_err(|e| {
            error!("parse failed: {}", e);
            (
                StatusCode::BAD_REQUEST,
                format!("json unmarshal failed: {}, body: {:?}", e, body),
            )
        })?;
        handler::store_data(&v).map_err(|e| {
            error!("store data failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("json unmarshal failed: {}, body: {:?}", e, body),
            )
        })?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    init::init();
    let listen_host = format!("{}:{}", GLOBAL_CFG.listen_host.0, GLOBAL_CFG.listen_host.1);
    info!("starting server in: {}", listen_host);
    let app = Router::new().route("/", any(competitive_companion));
    let listener = tokio::net::TcpListener::bind(listen_host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
