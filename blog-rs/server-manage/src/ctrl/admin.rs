use axum::{
    routing::{get, post},
    http::StatusCode,
    http::header::HeaderMap,
    response::IntoResponse,
    Json, Router,
    extract::Request,
    extract::FromRequest,
};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use log::{warn,info, debug};
use crate::{bean, tool};
use axum::debug_handler;

pub fn init_router(mut router: Router) -> Router {
    router = router.route("/admin/login", post(login));

    return router;
}

/// login 登录
#[debug_handler]
async fn login(
    headers: HeaderMap,
    Json(payload): Json<bean::admin::LoginIn>
) -> Json<common::net::rsp::Rsp<bean::admin::LoginOut>> {

    let real_ip = common::net::get_client_real_ip(&headers);
    tracing::info!("login 访问者 ip={:?}", real_ip);
    tracing::debug!("{:?}", payload);

    let author_res = base::service::blog_author_sve::find_by_user_name(payload.user_name);

    if author_res.is_err() {
        return Json(common::net::rsp::Rsp::<bean::admin::LoginOut>::err_de())
    }

    let author = author_res.unwrap();

    if let Some(au) = author {
        let ac = au.clone();

        let pwd = plier::md::sha256(payload.user_pwd);
        if pwd != au.user_pwd {
            return Json(common::net::rsp::Rsp::<bean::admin::LoginOut>::fail("username or password incorrect".to_string()))
        }

        // google 验证器校验
        if plier::str::is_not_blank(au.google_auth_secret) {
            if plier::str::is_blank(payload.google_auth_code) {
                return Json(common::net::rsp::Rsp::<bean::admin::LoginOut>::fail("google 验证码错误".to_string()))
            }
            // ---- 进行校验

        }


        let ut = common::token::generate_user_token(au.user_name.clone());
        // let re = common::cache::member_rds::set_user_by_token(ut.clone(), ac).await;
        // if re.is_err() {
        //     tracing::warn!("{:?}", re);
        //     return Json(common::net::rsp::Rsp::<bean::admin::LoginOut>::err_de())
        // }

        let out = bean::admin::LoginOut {
            id: au.id,
            pen_name: au.pen_name,
            user_token: ut,
        };


        let rsp = common::net::rsp::Rsp::ok(out);
        Json(rsp)

    }else{
        return Json(common::net::rsp::Rsp::<bean::admin::LoginOut>::fail("username or password incorrect".to_string()))
    }

}

