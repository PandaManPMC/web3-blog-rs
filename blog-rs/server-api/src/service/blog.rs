use std::collections::HashMap;
use axum::Json;
use i_dao::sql;
use base::model::blog_article::BlogArticleModel;
use crate::service::AUTHOR_LIST;
use crate::service::LABEL_LIST;
use crate::utils;

/// 缓存作者信息
pub async fn cache_author() {
    let params:HashMap<String, sql::Params> = HashMap::new();

    let result = base::service::blog_author_sve::query_list(&params, &utils::limit_min()).await;
    if result.is_err() {
        tracing::warn!("{:?}", result);
    }
    let lst = result.unwrap();

    let mut cache = AUTHOR_LIST.lock().unwrap();
    for author in lst {
        cache.insert(author.id, author.pen_name);
    }
}

/// 获取作者信息
/// id -> pen_name
async fn find_author_by_id(id: u64) -> String {
    // 1. 查询缓存, 不存在重新获取缓存
    let cache = AUTHOR_LIST.lock().unwrap();
    if let Some(value) = cache.get(&id) {
        value.to_string()
    } else {
        cache_author().await;
        // 2. 二次查找缓存
        let cache = AUTHOR_LIST.lock().unwrap();
        if let Some(value) = cache.get(&id) {
            value.to_string()
        } else {
            return "".to_string();
        }
    }
}

/// 缓存标签
pub async fn cache_label() {
    let params:HashMap<String, sql::Params> = HashMap::new();
    let result = base::service::blog_label_sve::query_list(&params, &utils::limit_min()).await;
    if result.is_err() {
        tracing::warn!("{:?}", result);
    }
    let lst = result.unwrap();

    let mut cache = LABEL_LIST.lock().unwrap();
    for label in lst {
        cache.insert(label.id, label.label_name);
    }
}

/// 获取标签名称
/// id -> label_name
async fn find_label_by_id(id: u64) -> String {
    // 1. 查询缓存, 不存在重新获取缓存
    let cache = AUTHOR_LIST.lock().unwrap();
    if let Some(value) = cache.get(&id) {
        value.to_string()
    } else {
        cache_author().await;
        // 2. 二次查找缓存
        let cache = AUTHOR_LIST.lock().unwrap();
        if let Some(value) = cache.get(&id) {
            value.to_string()
        } else {
            return "".to_string();
        }
    }
}