use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use log::debug;
use crate::bean;

pub fn init_router(mut router: Router) -> Router {
    router = router.route("/article/publish", post(publish));
    return router;
}

async fn publish(
    Json(payload): Json<bean::admin::LoginIn>,
) -> (StatusCode, Json<bean::admin::LoginOut>) {

    debug!("{:?}", payload);

    let out = bean::admin::LoginOut {
        id: 1,
        pen_name: "老泥鳅".to_string(),
    };

    (StatusCode::CREATED, Json(out))
}
