use std::collections::HashMap;
use axum::{
    routing::{get, post},
    http::StatusCode,
    http::header::HeaderMap,
    response::IntoResponse,
    Json, Router,
    extract::Request,
    extract::FromRequest,
    extract::Multipart,
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
use tokio::fs;

lazy_static::lazy_static! {
    static ref LOCK: Arc<Mutex<bool>> = Arc::new(Mutex::new({
        true
    }));
}

pub fn init_router(mut router: Router) -> Router {
    router = router.route("/a770x/common/fileUpload", post(file_upload));
    router = router.route("/a770x/common/verifyReCaptchaToken", get(verify_re_captcha_token_v2));

    return router;
}

/// file_upload 文件上传
async fn file_upload(headers: HeaderMap, mut multipart: Multipart) -> Json<common::net::rsp::Rsp<bean::common::FileUploadOut>> {
    let _ = LOCK.lock().await;

    let nf = multipart.next_field().await;
    if nf.is_err() {
        tracing::warn!("{:?}", nf);
        return Json(common::net::rsp::Rsp::<bean::common::FileUploadOut>::err_de())
    }

    let of = nf.unwrap();
    if of.is_none() {
        return Json(common::net::rsp::Rsp::<bean::common::FileUploadOut>::fail("请上传文件".to_string()))
    }

    let field = of.unwrap();

    let name = field.name().unwrap_or("file").to_string();
    let file_name = field.file_name().unwrap_or("file").to_string();
    let content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
    let data = field.bytes().await.unwrap();

    // 使用 UUID 生成唯一文件名
    let file_name = format!("{}_{}", plier::uid::uid_v4(), file_name);

    unsafe {
        // 创建目录并保存文件
        let fp = format!("{}/{}", configs::get_str("basics", "file_path"), file_name);
        info!("文件={}", fp);
        match fs::create_dir_all(&configs::get_str("basics", "file_path")).await {
            Ok(_) => match fs::write(&fp, &data).await {
                Ok(_) => {
                    info!("Uploaded: name = {}, file_name = {}, content_type = {}, size = {} bytes",name, file_name, content_type, data.len());
                }
                Err(e) => {
                    tracing::error!("Failed to write file:{:?}", e);
                    return Json(common::net::rsp::Rsp::<bean::common::FileUploadOut>::err_de())
                }
            },
            Err(e) => {
                tracing::error!("Failed to write file:{:?}", e);
                return Json(common::net::rsp::Rsp::<bean::common::FileUploadOut>::err_de())
            }
        }

        let out = bean::common::FileUploadOut{
            file_url: format!("{}/{}", configs::get_str("basics", "file_http"), file_name),
        };
        let rsp = common::net::rsp::Rsp::ok(out);
        Json(rsp)

    }

}

/// verify_re_captcha_token_v2 验证码校验
async fn verify_re_captcha_token_v2(
    query: Query<bean::common::VerifyReCaptchaTokenIn>,
) -> Json<common::net::rsp::Rsp<String>> {
    debug!("{:?}", query);

    unsafe {
        let res = tool::http::verify_re_captcha_token_v2(query.captcha_token.clone(), configs::get_str("reCAPTCHA", "SERVER")).await;
        tracing::info!("{:?}", res);
        if res.is_err() {
            tracing::error!("{:?}", res);
            return Json(common::net::rsp::Rsp::<String>::err_de())
        }

        let ver = res.unwrap();
        if !ver.success {
            return Json(common::net::rsp::Rsp::<String>::fail("验证码错误".to_string()))
        }
    }

    let ran = plier::uid::uid_v4();

    let c_res = common::cache::member_rds::set_user_captcha_token(ran.clone()).await;
    if c_res.is_err() {
        tracing::error!("{:?}", c_res);
        return Json(common::net::rsp::Rsp::<String>::err_de())
    }

    let rsp = common::net::rsp::Rsp::ok(ran);
    return Json(rsp);
}
