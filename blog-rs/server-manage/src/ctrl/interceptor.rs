use axum::{
    async_trait,
    body::{Body, Bytes},
    extract::{FromRequest, Request},
    http::{StatusCode, header::HeaderMap, header::HeaderValue},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::post,
    Router,
    extract::ConnectInfo,
};
use axum::http::header;
use std::net::SocketAddr;
use http_body_util::BodyExt;
use log::info;


pub async fn print_request_body(mut request: Request, next: Next) -> Result<impl IntoResponse, Response> {
    let xip = common::net::get_request_ip(&mut request);
    tracing::info!("访问者 ip={:?}", xip);

    let request = buffer_request_body(request).await?;

    Ok(next.run(request).await)
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
