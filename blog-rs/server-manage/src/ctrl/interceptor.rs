use axum::{async_trait, body::{Body, Bytes}, extract::{FromRequest, Request}, http::{StatusCode, header::HeaderMap, header::HeaderValue}, middleware::{self, Next}, response::{IntoResponse, Response}, routing::post, Router, extract::ConnectInfo, Json};
use axum::http::header;
use std::net::SocketAddr;
use http_body_util::BodyExt;
use hyper::body::Buf;
use log::{info, trace};
use plier;
use crate::bean;

pub async fn app(mut request: Request, next: Next) -> Result<impl IntoResponse, Response> {
    let uri = request.uri().clone();

    let now = plier::time::unix_second();
    let uuid = plier::uid::uid_v4();
    request.headers_mut().insert("x-uuid", uuid.parse().unwrap());
    request.headers_mut().insert("x-begin-time", now.into());

    let xip = common::net::get_request_ip(&mut request);
    let user_token = request.headers().get("userToken");

    tracing::info!("访问者 uid {:?} ip {:?} path {:?} user_token {:?}",uuid, xip, uri.path(), user_token);

    let request = buffer_request_body(request).await?;

    let res = next.run(request).await;
    tracing::info!("访问者 uid {} 耗时 {} ", uuid, plier::time::unix_second() - now);
    Ok(res)
}

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

