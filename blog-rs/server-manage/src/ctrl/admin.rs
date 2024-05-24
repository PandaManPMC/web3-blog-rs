use axum::{
    routing::{get, post},
    http::StatusCode,
    http::header::HeaderMap,
    Json, Router,
};
use axum::extract::Request;
use serde::{Deserialize, Serialize};
use log::{warn,info, debug};
use crate::bean;
use axum::async_trait;
use axum::extract::FromRequest;
use serde::de::DeserializeOwned;
use hyper::body::Body;

pub fn init_router(mut router: Router) -> Router {
    router = router.route("/admin/login", post(login));

    return router;
}


/// login 登录
async fn login(
    headers: HeaderMap,
    Json(payload): Json<bean::admin::LoginIn>
) -> (StatusCode, Json<bean::admin::LoginOut>) {

    let real_ip = common::net::get_client_real_ip(&headers);
    tracing::info!("login 访问者 ip={:?}", real_ip);

    debug!("{:?}", payload);

    let out = bean::admin::LoginOut {
        id: 1,
        pen_name: "老泥鳅".to_string(),
    };

    (StatusCode::CREATED, Json(out))
}

