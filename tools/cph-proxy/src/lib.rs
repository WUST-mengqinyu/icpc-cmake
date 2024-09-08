pub mod cfg;
pub mod handler;
pub mod init;
pub mod model;
pub mod service;

use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use volo::FastStr;
#[allow(dead_code)]
pub enum HttpResp<T>
where
    T: Serialize,
{
    PanicErr(StatusCode),
    InternalErr(i32, FastStr),
    Resp(T),
}

impl<T: Serialize> IntoResponse for HttpResp<T> {
    fn into_response(self) -> Response {
        match self {
            HttpResp::PanicErr(status_code) => status_code.into_response(),
            HttpResp::InternalErr(code, msg) => Response::builder()
                .body(Body::from(
                    serde_json::json!({"code": code, "msg": msg}).to_string(),
                ))
                .unwrap(),
            HttpResp::Resp(resp) => Response::builder()
                .body(Body::from(serde_json::json!(resp).to_string()))
                .unwrap(),
        }
    }
}
