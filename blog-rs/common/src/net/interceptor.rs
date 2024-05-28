use axum::{extract::Request, http::StatusCode, Json, middleware::Next, response::Response};
use axum::response::IntoResponse;
use log::info;
use hyper::body::Buf;
use crate::net::rsp;
use crate::net::rsp::Rsp;
use http_body_util::BodyExt;

/// error_handling 全局错误处理
pub async fn error_handling(req: Request, next: Next) -> Result<Response, Json<Rsp<rsp::Default>>> {
    let mut res = next.run(req).await;

    if res.status().is_client_error() || res.status().is_server_error() {

        // 将 Response 转换为 Response<Body> 以提取内容
        let (parts, body) = res.into_parts();
        let bytes = body
            .collect()
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()).unwrap()
            .to_bytes();
        let body_string = String::from_utf8_lossy(bytes.chunk());

        // 记录错误信息或根据需要进行处理
        info!("Error response: status = {}, body = {}", parts.status, body_string);
        if 422 == parts.status {
            // 字段错误
            return Err(Json(Rsp::<rsp::Default>::fail(body_string.to_string())))
        }
        return Err(Json(Rsp::<rsp::Default>::fail(body_string.to_string())))

        // 根据状态码返回不同的错误
        // let error = match parts.status {
        //     StatusCode::BAD_REQUEST => AppError::BadRequest,
        //     StatusCode::UNAUTHORIZED => AppError::Unauthorized,
        //     StatusCode::NOT_FOUND => AppError::NotFound,
        //     _ => AppError::InternalServerError,
        // };
        // Err(error)
    } else {
        Ok(res)
    }
}
