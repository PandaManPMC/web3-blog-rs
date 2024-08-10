use axum::{Json, Router};
use axum::routing::get;
use crate::bean::advertise::AdvertiseInfoOut;
use crate::ctrl::PREIFIX;
use crate::service;

pub fn init_router(mut router: Router) -> Router {
    router = router.route(&format!("{}{}", PREIFIX, "/advertise/list"), get(get_list));
    return router;
}

/// 广告列表
async fn get_list() -> Json<common::net::rsp::Rsp<Vec<AdvertiseInfoOut>>> {
    let mut lst: Vec<AdvertiseInfoOut> = Vec::new();
    let cache = service::ADVERTISE_LIST.read().await;
    for (key, value) in cache.iter() {
        lst.push(AdvertiseInfoOut{
            link_advertise: value.link_advertise.clone(),
            title: value.title.clone(),
            introduce: value.introduce.clone(),
            img1: value.img1.clone(),
            sequence: value.sequence,
        })
    }
    Json(common::net::rsp::Rsp::ok(lst))
}