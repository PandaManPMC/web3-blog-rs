use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use ctrl::app_err::AppError;
use crate::ctrl;

pub async fn handle_errors(req: Request, next: Next) -> Result<Response, AppError> {
    let res = next.run(req).await;

    if res.status().is_client_error() || res.status().is_server_error() {
        // 根据状态码返回不同的错误
        let error = match res.status() {
            StatusCode::BAD_REQUEST => AppError::BadRequest,
            StatusCode::UNAUTHORIZED => AppError::Unauthorized,
            StatusCode::NOT_FOUND => AppError::NotFound,
            _ => AppError::InternalServerError,
        };

        Err(error)
    } else {
        Ok(res)
    }
}
