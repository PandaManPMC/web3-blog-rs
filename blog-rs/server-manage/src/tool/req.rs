use std::string::ToString;
use axum::extract::Request;
use axum::{
    routing::{get, post},
    Json, Router,
    extract::Query,
    http::{StatusCode, header::HeaderMap, header::HeaderValue},
};
use crate::tool;


pub fn set_user_to_req(request: &mut Request, user: base::model::blog_author::BlogAuthorModel){
    request.headers_mut().insert(tool::X_USER_NAME, user.user_name.parse().unwrap());
    request.headers_mut().insert(tool::X_USER_ID, user.id.into());
    request.headers_mut().insert(tool::X_USER_PEN_NAME, user.pen_name.parse().unwrap());
}

pub fn get_user_id(headers: &HeaderMap) -> u64 {
    let mut res:u64 = 0;
    match headers.get(tool::X_USER_ID).cloned() {
        Some(value) => {
            match value.to_str() {
                Ok(value_str) => {
                    match value_str.parse::<u64>() {
                        Ok(parsed_value) => {
                            // 成功解析 u64 类型的值
                            res = parsed_value;
                        }
                        Err(_) => {
                            // 头部值不是有效的 u64 类型
                            res = 0;
                        }
                    }
                }
                Err(_) => {
                    // 头部值不是有效的字符串
                    res = 0;
                }
            }
        }
        None => {
            // 请求头缺失
            res = 0;
        }
    }
    return res;
}