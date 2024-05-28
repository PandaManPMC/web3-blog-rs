use axum::{extract::Request, http::StatusCode, Json, middleware::Next, response::Response};
use axum::response::IntoResponse;
use log::{info, warn};
use hyper::body::Buf;
use crate::net::{get_head_str, rsp};
use crate::net::rsp::Rsp;
use http_body_util::BodyExt;

/// error_handling 全局错误处理
pub async fn error_handling(req: Request, next: Next) -> Result<Response, Json<Rsp<rsp::Default>>> {
    let uuid = get_head_str(req.headers(), "x-uuid");
    let uri = req.uri().clone();

    let res = next.run(req).await;

    if res.status().is_client_error() || res.status().is_server_error() {
        // 将 Response 转换为 Response<Body> 以提取内容
        let (parts, body) = res.into_parts();

        tracing::warn!("error_handling uuid {:?} path {:?} 出现错误 status {:?}", uuid, uri.path(), parts.status);

        let bytes = body
            .collect()
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()).unwrap()
            .to_bytes();
        let body_string = String::from_utf8_lossy(bytes.chunk());

        tracing::info!("error_handling uuid {:?} Error response body {:?}", uuid, body_string);

        if 422 == parts.status {
            // 字段错误
            return Err(Json(Rsp::<rsp::Default>::fail(body_string.to_string())))
        }
        return Err(Json(Rsp::<rsp::Default>::fail("Error: Unexpected by the program".to_string())))
    } else {
        Ok(res)
    }
}
