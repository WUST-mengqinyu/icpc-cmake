#[cfg(feature = "clipboard_proxy")]
pub mod clipboard_proxy;

use crate::cfg::get_global_cfg;
use crate::{service, HttpResp};
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::{any, get};
use axum::{body::Bytes, http::StatusCode, Router};
use idl_gen::{
    clipboard::Clipboard,
    info::{ConfigInfo, Echo},
};
use log::*;
use std::net::IpAddr;

static ECHO_SERVICE: service::EchoService = service::EchoService {};

pub fn axum_router(r: Router) -> Router {
    r.route("/", any(competitive_companion))
        .route("/echo", get(echo))
        .route("/config/local", get(local_config))
        .route("/proxy/send_clipboard", get(send_clipboard))
}

static REQWEST_SINGLTON_CLI: std::sync::OnceLock<reqwest::Client> = std::sync::OnceLock::new();

async fn proxy(ip: IpAddr, port: u16, body: Bytes) -> Result<(), (StatusCode, String)> {
    let cli = REQWEST_SINGLTON_CLI.get_or_init(|| {
        let mut clib = reqwest::Client::builder();
        if let Some((to_host_proxy_ip, to_host_proxy_port)) = get_global_cfg().to_host_proxy {
            clib = clib.proxy(
                reqwest::Proxy::http(format!(
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
    if let Some((ip, port)) = get_global_cfg().to_host {
        return proxy(ip, port, body).await;
    }
    if get_global_cfg().must_self_host || get_global_cfg().to_host.is_none() {
        debug!("get request: {:?}", body);

        let data = serde_json::from_slice(&body).map_err(|e| {
            error!("parse failed: {:#}", e);
            (
                StatusCode::BAD_REQUEST,
                format!("json unmarshal failed: {}, body: {:?}", e, body),
            )
        })?;
        service::store_data(data).await.map_err(|e| {
            error!("store data failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("json unmarshal failed: {}", e),
            )
        })?;
    }
    Ok(())
}

macro_rules! rpc2http {
    ($i:ident, $req_tp:ident, $http_source:ident) => {
        async fn $i($http_source(req): $http_source<idl_gen::info::$req_tp>) -> impl IntoResponse {
            let resp = ECHO_SERVICE.$i(volo_grpc::Request::new(req)).await;
            match resp {
                Ok(res) => match res.get_ref().base_resp.clone().map(|x| (x.code, x.msg)) {
                    None => HttpResp::Resp(res.into_inner()),
                    Some((code, msg)) => HttpResp::InternalErr(code, msg),
                },
                Err(e) => {
                    error!("failed: {:?}", e);
                    HttpResp::PanicErr(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
    };
}

// TODO: make it with grpc-gateway: https://github.com/cloudwego/volo/issues/80
rpc2http!(echo, EchoRequest, Query);
rpc2http!(local_config, LocalConfigRequest, Query);
rpc2http!(send_clipboard, SendClipboardRequest, Query);
