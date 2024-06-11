use std::collections::HashMap;
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
use tokio::sync::Mutex;
use std::sync::Arc;
use axum::extract::Query;
use i_dao::sql;
use iconf::configs;
use base::model::blog_article::BlogArticleModel;
use common::cache;

lazy_static::lazy_static! {
    static ref LOCK: Arc<Mutex<bool>> = Arc::new(Mutex::new({
        true
    }));
}

pub fn init_router(mut router: Router) -> Router {
    router = router.route("/admin/login", post(login));
    router = router.route("/admin/changePwd", post(change_pwd));

    router = router.route("/admin/getStartBindGoogleSecret", get(get_start_bind_google_secret));
    router = router.route("/admin/bindGoogleSecret", post(bind_google_secret));

    return router;
}

/// login 登录
#[debug_handler]
async fn login(
    headers: HeaderMap,
    Json(payload): Json<bean::admin::LoginIn>
) -> Json<common::net::rsp::Rsp<bean::admin::LoginOut>> {
    let _ = LOCK.lock().await;

    let real_ip = common::net::get_client_real_ip(&headers);
    tracing::info!("login 访问者 ip={:?}", real_ip);
    tracing::debug!("{:?}", payload);

    let author_res = base::service::blog_author_sve::find_by_user_name(payload.user_name).await;

    if author_res.is_err() {
        tracing::warn!("{:?}", author_res);
        return Json(common::net::rsp::Rsp::<bean::admin::LoginOut>::err_de())
    }

    let author = author_res.unwrap();
    let mut google_auth = false;

    if let Some(au) = author {
        let ac = au.clone();

        let pwd = plier::md::sha256(payload.user_pwd);
        if pwd != au.user_pwd {
            return Json(common::net::rsp::Rsp::<bean::admin::LoginOut>::fail("username or password incorrect".to_string()))
        }

        // google 验证器校验
        if "" != au.google_auth_secret {
            google_auth = true;

            if plier::str::is_blank(payload.google_auth_code.clone()) {
                return Json(common::net::rsp::Rsp::<bean::admin::LoginOut>::fail("google 验证码错误".to_string()))
            }
            // ---- 进行校验
            unsafe {
                let aes_key = configs::get_str("aes", "key");
                let aes_iv = configs::get_str("aes", "iv");

                let dec = plier::aes::aes256_decrypt_string(au.google_auth_secret.clone(), aes_key, aes_iv);
                if dec.is_err() {
                    tracing::warn!("{:?}", dec);
                    return Json(common::net::rsp::Rsp::<bean::admin::LoginOut>::err_de())
                }

                let secret = dec.unwrap();

                if !plier::authenticator::google_verify_code(secret, payload.google_auth_code.clone(), 0) {
                    return Json(common::net::rsp::Rsp::<bean::admin::LoginOut>::fail("验证码错误".to_string()));
                }
            }
        }

        let ut = common::token::generate_user_token(au.user_name.clone());
        let re = common::cache::member_rds::set_user_by_token(ut.clone(), ac).await;
        if re.is_err() {
            tracing::warn!("{:?}", re);
            return Json(common::net::rsp::Rsp::<bean::admin::LoginOut>::err_de())
        }

        let out = bean::admin::LoginOut {
            id: au.id,
            pen_name: au.pen_name,
            user_token: ut,
            google_auth,
        };


        let rsp = common::net::rsp::Rsp::ok(out);
        Json(rsp)

    }else{
        return Json(common::net::rsp::Rsp::<bean::admin::LoginOut>::fail("username or password incorrect".to_string()))
    }

}

/// change_pwd 修改密码
async fn change_pwd(
    headers: HeaderMap,
    Json(payload): Json<bean::admin::ChangePwdIn>
) -> Json<common::net::rsp::Rsp<u8>> {
    let _ = LOCK.lock().await;

    let author_res = base::service::blog_author_sve::find_by_id(tool::req::get_user_id(&headers)).await;

    if author_res.is_err() {
        tracing::warn!("{:?}", author_res);
        return Json(common::net::rsp::Rsp::<u8>::err_de())
    }

    let mut author = author_res.unwrap().unwrap();
    let pwd = plier::md::sha256(payload.user_pwd);
    author.user_pwd = pwd;

    let res = base::service::blog_author_sve::update_by_id(&mut author).await;
    if res.is_err() {
        tracing::warn!("{:?}", res);
        return Json(common::net::rsp::Rsp::<u8>::err_de())
    }
    Json(common::net::rsp::Rsp::ok(1))
}



/// get_start_bind_google_secret 开始绑定google验证码
async fn get_start_bind_google_secret(
    headers: HeaderMap,
) -> Json<common::net::rsp::Rsp<bean::admin::GetStartBindGoogleSecretOut>> {
    let _ = LOCK.lock().await;

    let user_name = tool::req::get_user_name(&headers);

    let au_res = base::service::blog_author_sve::find_by_user_name(user_name.clone()).await;
    if au_res.is_err() {
        tracing::warn!("{:?}", au_res);
        return Json(common::net::rsp::Rsp::<bean::admin::GetStartBindGoogleSecretOut>::err_de())
    }

    if let Some(au) = au_res.unwrap() {
        if "" != au.google_auth_secret {
            return Json(common::net::rsp::Rsp::<bean::admin::GetStartBindGoogleSecretOut>::fail("不可重复绑定".to_string()));
        }
    }

    let secret = plier::authenticator::google_secret(32);
    let qr_code_url = plier::authenticator::google_qr_url(secret.clone(), user_name.clone(), format!("web3-blog：{}",user_name ).to_string());

    unsafe {
        let aes_key = configs::get_str("aes", "key");
        let aes_iv = configs::get_str("aes", "iv");

        let secret_aes = plier::aes::aes256_encrypt_string(secret.clone(), aes_key, aes_iv);
        let cache_res = cache::member_rds::set_user_secret(user_name.clone(), secret_aes).await;
        if cache_res.is_err() {
            tracing::warn!("{:?}", cache_res);
            return Json(common::net::rsp::Rsp::<bean::admin::GetStartBindGoogleSecretOut>::err_de())
        }
    }

    let lemon = bean::admin::GetStartBindGoogleSecretOut{
        secret,
        qr_code_url,
    };
    Json(common::net::rsp::Rsp::ok(lemon))
}

/// bind_google_secret 绑定google验证器
async fn bind_google_secret (
    headers: HeaderMap,
    Json(payload): Json<bean::admin::BindGoogleSecretIn>,
) -> Json<common::net::rsp::Rsp<u8>> {
    let _ = LOCK.lock().await;

    let user_name = tool::req::get_user_name(&headers);

    let au_res = base::service::blog_author_sve::find_by_user_name(user_name.clone()).await;
    if au_res.is_err() {
        tracing::warn!("{:?}", au_res);
        return Json(common::net::rsp::Rsp::<u8>::err_de())
    }

    let mut au = au_res.unwrap().unwrap();
    if "" != au.google_auth_secret {
        return Json(common::net::rsp::Rsp::<u8>::fail("不可重复绑定".to_string()));
    }

    let cache_res = cache::member_rds::get_user_secret(user_name.clone()).await;
    if cache_res.is_err() {
        tracing::warn!("{:?}", cache_res);
        return Json(common::net::rsp::Rsp::<u8>::err_de())
    }

    let cac = cache_res.unwrap();
    if "" == cac {
        return Json(common::net::rsp::Rsp::<u8>::fail("操作时间太长，secret 已经失效".to_string()));
    }

    unsafe {
        let aes_key = configs::get_str("aes", "key");
        let aes_iv = configs::get_str("aes", "iv");

        let dec = plier::aes::aes256_decrypt_string(cac.clone(), aes_key, aes_iv);
        if dec.is_err() {
            tracing::warn!("{:?}", dec);
            return Json(common::net::rsp::Rsp::<u8>::err_de())
        }

        let secret = dec.unwrap();

        if !plier::authenticator::google_verify_code(secret, payload.google_auth_code, 0) {
            return Json(common::net::rsp::Rsp::<u8>::fail("验证码错误".to_string()));
        }

        au.google_auth_secret = cac;

        let orange = base::service::blog_author_sve::update_by_id(&mut au).await;
        if orange.is_err() {
            tracing::warn!("{:?}", orange);
            return Json(common::net::rsp::Rsp::<u8>::err_de())
        }

    }

    return Json(common::net::rsp::Rsp::<u8>::ok_de())
}