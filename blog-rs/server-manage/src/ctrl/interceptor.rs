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


pub async fn print_request_body(request: Request, next: Next) -> Result<impl IntoResponse, Response> {

    let xip = common::net::get_request_ip(&request);
    info!("请求者 ip={:?}", xip);

    let request = buffer_request_body(request).await?;

    Ok(next.run(request).await)
}




async fn buffer_request_body(request: Request) -> Result<Request, Response> {


    let (parts, body) = request.into_parts();


    info!("有新的请求");

    let bytes = body
        .collect()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?
        .to_bytes();

    do_thing_with_request_body(bytes.clone());

    Ok(Request::from_parts(parts, Body::from(bytes)))
}

fn do_thing_with_request_body(bytes: Bytes) {
    tracing::debug!(body = ?bytes);
}

async fn handler(BufferRequestBody(body): BufferRequestBody) {
    tracing::debug!(?body, "handler received body");
}

struct BufferRequestBody(Bytes);

#[async_trait]
impl<S> FromRequest<S> for BufferRequestBody
    where
        S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(|err| err.into_response())?;

        do_thing_with_request_body(body.clone());

        Ok(Self(body))
    }
}