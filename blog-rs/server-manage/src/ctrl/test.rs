use axum::http::HeaderMap;
use axum::{Json, Router};
use axum::routing::post;
use log::info;
use crate::bean;
use crate::service;

pub fn init_router(mut router: Router) -> Router {
    router = router.route("/test/test", post(test));
    return router;
}

async fn test(
    headers: HeaderMap,
    Json(payload): Json<bean::admin::LoginIn>
) -> Json<common::net::rsp::Rsp<bean::admin::LoginOut>> {
    let real_ip = common::net::get_client_real_ip(&headers);
    tracing::info!("login 访问者 ip={:?}", real_ip);
    tracing::debug!("{:?}", payload);

    service::get_data_source_key().await;
    let author_res = service::author::find_by_user_name(payload.user_name.clone()).await;
    info!("{:?}", author_res);

    read().await;

    let user_res = common::cache::member_rds::get_user_by_token("111".to_string()).await;

    return Json(common::net::rsp::Rsp::<bean::admin::LoginOut>::err_de())
}
pub async fn read(){
    info!("test");
}
