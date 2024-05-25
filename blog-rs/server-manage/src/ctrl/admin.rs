use axum::{
    routing::{get, post},
    http::StatusCode,
    http::header::HeaderMap,
    response::IntoResponse,
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
    router = router.route("/admin/login2", post(login2));

    return router;
}


/// login 登录
async fn login(
    headers: HeaderMap,
    Json(payload): Json<bean::admin::LoginIn>
) -> (StatusCode, Json<common::net::rsp::Rsp<bean::admin::LoginOut>>) {

    let real_ip = common::net::get_client_real_ip(&headers);
    tracing::info!("login 访问者 ip={:?}", real_ip);
    debug!("{:?}", payload);

    let author_res = base::service::blog_author_sve::find_by_user_name(payload.user_name);

    if author_res.is_err() {

    }

    let author = author_res.unwrap();

    if let Some(au) = author {

    }else{
        return (StatusCode::OK, Json(common::net::rsp::Rsp::<bean::admin::LoginOut>::fail_de()));
    }

    let out = bean::admin::LoginOut {
        id: 1,
        pen_name: "老泥鳅".to_string(),
    };

    let rsp = common::net::rsp::Rsp::ok(out);
    (StatusCode::OK, Json(rsp))
}

async fn login2(
    headers: HeaderMap,
    Json(payload): Json<bean::admin::LoginIn>
) -> Json<common::net::rsp::Rsp<bean::admin::LoginOut>> {

    let real_ip = common::net::get_client_real_ip(&headers);
    tracing::info!("login 访问者 ip={:?}", real_ip);
    debug!("{:?}", payload);

    let author_res = base::service::blog_author_sve::find_by_user_name(payload.user_name);

    if author_res.is_err() {

    }

    let author = author_res.unwrap();

    if let Some(au) = author {

    }else{
        return Json(common::net::rsp::Rsp::<bean::admin::LoginOut>::fail_de())
    }

    let out = bean::admin::LoginOut {
        id: 1,
        pen_name: "老泥鳅".to_string(),
    };

    let rsp = common::net::rsp::Rsp::ok(out);
    Json(rsp)
}

