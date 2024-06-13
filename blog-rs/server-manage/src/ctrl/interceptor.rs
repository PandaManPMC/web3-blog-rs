use axum::{body::{Body, Bytes}, extract::{FromRequest, Request},
           http::{StatusCode, header::HeaderMap, header::HeaderValue},
           middleware::{self, Next}, response::{IntoResponse, Response},
           routing::post, Router, extract::ConnectInfo, Json,BoxError};
use axum::http::header;
use std::net::SocketAddr;
use http_body_util::BodyExt;
use hyper::body::Buf;
use log::{info, trace};
use plier;
use base::model::blog_author::BlogAuthorModel;
use common::net::rsp;
use common::net::rsp::Rsp;
use crate::{bean, tool};

pub const WHITE_LIST_URL:[&str;12] = ["/admin/login", "/common/verifyReCaptchaToken",
    "/article/getArticleLst",
    "/article/getClassesLst", "/article/getLabelLst", "/article/createClasses",
    "/article/createLabel", "/test/test", "/article/changeArticle",
    "/article/changeArticleLabel", "/article/delClasses", "/article/delLabel"];

pub async fn app(mut request: Request, next: Next) -> Result<Response, Json<Rsp<rsp::Default>>> {
    let uri = request.uri().clone();

    let now = plier::time::unix_second();
    let uuid = plier::uid::uid_v4();
    request.headers_mut().insert("x-uuid", uuid.parse().unwrap());
    request.headers_mut().insert("x-begin-time", now.into());

    let xip = common::net::get_request_ip(&mut request);
    let user_token = request.headers().get(tool::X_USER_TOKEN).cloned();

    tracing::info!("request uid {:?} ip {:?} path {:?} user_token {:?}",uuid, xip, uri.path(), user_token);

    let mut req = buffer_request_body(request).await.unwrap();


    for v in WHITE_LIST_URL {
        if v == uri.path() {
            let res = next.run(req).await;
            tracing::info!("request uid {} time {} ", uuid, plier::time::unix_second() - now);
            return Ok(res)
        }
    }

    if let Some(x) = user_token {
        let ut = x.to_str().unwrap().to_string();
        // let user_res = common::cache::member_rds::get_user_by_ut(ut);
        // if let Some(u) = user_res {
        //     tool::req::set_user_to_req(&mut req, u);
        // } else {
        //     return Err(Json(Rsp::<rsp::Default>::not_login()))
        // }

        let user_res = common::cache::member_rds::get_user_by_token(ut.clone()).await;

        if user_res.is_err() {
            tracing::error!("get_user_by_token：{:?}", user_res);
            return Err(Json(Rsp::<rsp::Default>::not_login()))
        }

        let user = user_res.unwrap();
        if let Some(u) = user {
            tool::req::set_user_to_req(&mut req, u);
        } else{
            return Err(Json(Rsp::<rsp::Default>::not_login()))
        }
    }else{
        return Err(Json(Rsp::<rsp::Default>::not_login()))
    }

    let res = next.run(req).await;
    tracing::info!("request uid {} time {} ", uuid, plier::time::unix_second() - now);
    Ok(res)
}

// pub async fn app(mut request: Request, next: Next) -> Result<Response, Json<Rsp<rsp::Default>>> {
//     let uri = request.uri().clone();
//
//     let now = plier::time::unix_second();
//     let uuid = plier::uid::uid_v4();
//     request.headers_mut().insert("x-uuid", uuid.parse().unwrap());
//     request.headers_mut().insert("x-begin-time", now.into());
//
//     let xip = common::net::get_request_ip(&mut request);
//     let user_token = request.headers().get("userToken").cloned();
//
//     tracing::info!("访问者 uid {:?} ip {:?} path {:?} user_token {:?}",uuid, xip, uri.path(), user_token);
//
//     let req = buffer_request_body(request).await.unwrap();
//
//
//     // 不需要鉴权的 URL
//     for v in WHITE_LIST_URL {
//         if v == uri.path() {
//             let res = next.run(req).await;
//             tracing::info!("访问者 uid {} 耗时 {} ", uuid, plier::time::unix_second() - now);
//             return Ok(res)
//         }
//     }
//
//     // 进行鉴权
//     if let Some(x) = user_token {
//         let ut = x.to_str().unwrap().to_string();
//         let res = common::cache::member_rds::get_user_by_token(ut).await;
//         if res.is_err() {
//             // 无访问权限
//             tracing::error!("get_user_by_token：{:?}", res);
//             return Err(Json(Rsp::<rsp::Default>::not_login()))
//         }
//
//         let member = res.unwrap();
//         if let Some(u) = member{
//             // 有登录，将相关参数写入头
//             set_user_to_req(&mut req, u);
//         } else{
//             // 无访问权限
//             return Err(Json(Rsp::<rsp::Default>::not_login()))
//         }
//     }else{
//         // 无访问权限
//         return Err(Json(Rsp::<rsp::Default>::not_login()))
//     }
//
//     let res = next.run(req).await;
//     tracing::info!("访问者 uid {} 耗时 {} ", uuid, plier::time::unix_second() - now);
//     Ok(res)
// }

async fn buffer_request_body(request: Request) -> Result<Request, Response> {
    let (parts, body) = request.into_parts();

    let bytes = body
        .collect()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?
        .to_bytes();

    tracing::debug!(body = ?bytes);

    Ok(Request::from_parts(parts, Body::from(bytes)))
}

