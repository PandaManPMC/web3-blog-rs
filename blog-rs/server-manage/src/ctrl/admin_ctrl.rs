use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use log::{warn,info, debug};
use crate::bean;


pub fn init_router(mut router: Router) -> Router {
    router = router.route("/login", post(login));
    return router;
}

/// login 登录
async fn login(
    Json(payload): Json<bean::admin::LoginIn>,
) -> (StatusCode, Json<bean::admin::LoginOut>) {

    debug!("{:?}", payload);

    let out = bean::admin::LoginOut {
        id: 1,
        pen_name: "老泥鳅".to_string(),
    };

    (StatusCode::CREATED, Json(out))
}
