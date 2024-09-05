mod cfg;
mod handler;
pub mod model;

use axum::routing::any;
use axum::{body::Bytes, http::StatusCode, Router};
use cfg::*;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

async fn competitive_companion(body: Bytes) -> Result<(), (StatusCode, String)> {
    match GLOBAL_CFG.clone().to_host {
        Some((ip, port)) => {
            let mut clib = reqwest::Client::builder();
            if let Some((to_host_proxy_ip, to_host_proxy_port)) = GLOBAL_CFG.clone().to_host_proxy {
                clib = clib.proxy(
                    reqwest::Proxy::http(&format!(
                        "http://{}:{}",
                        to_host_proxy_ip, to_host_proxy_port
                    ))
                    .map_err(|e| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("build proxy failed: {}", e),
                        )
                    })?,
                );
            }
            let cli = clib.build().map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("build proxy client failed: {}", e),
                )
            })?;
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
        }
        None => {
            tracing::info!("get request: {:?}", body);
            let v: model::ProblemMetaWithTestCase = serde_json::from_slice(&body).map_err(|e| {
                tracing::error!("parse failed: {}", e);
                (
                    StatusCode::BAD_REQUEST,
                    format!("json unmarshal failed: {}, body: {:?}", e, body),
                )
            })?;
            handler::store_data(&v).map_err(|e| {
                tracing::error!("store data failed: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("json unmarshal failed: {}, body: {:?}", e, body),
                )
            })?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let cfg = GLOBAL_CFG.clone(); // init config
    tracing_subscriber::registry().with(fmt::layer()).init();
    tracing::info!("starting server");
    let app = Router::new().route("/", any(competitive_companion));
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", cfg.listen_host.0, cfg.listen_host.1))
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();
}
