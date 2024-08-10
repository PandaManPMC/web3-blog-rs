use axum::{Json, Router};
use axum::routing::get;
use crate::bean::author::AuthorOut;
use crate::ctrl::PREIFIX;
use crate::service;

pub fn init_router(mut router: Router) -> Router {
    router = router.route(&format!("{}{}", PREIFIX, "/author/info"), get(get_info));
    return router;
}

/// 获取作者信息
async fn get_info() -> Json<common::net::rsp::Rsp<AuthorOut>> {
    let author = service::BLOG_AUTHOR.get().expect("BLOG_AUTHOR should be initialized").read().await;
    return Json(common::net::rsp::Rsp::ok(AuthorOut{
        pen_name: author.pen_name.clone(),
        profile: author.profile.clone(),
        introduce: author.introduce.clone(),
    }))
}