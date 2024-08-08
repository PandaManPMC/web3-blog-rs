use std::collections::HashMap;
use axum::Json;
use i_dao::sql;
use i_dao::tok::i_mysql;
use log::error;
use r2d2_mysql::mysql::Transaction;
use base::dao::blog_article_dao;
use base::model::blog_article::BlogArticleModel;
use base::model::blog_author::BlogAuthorModel;
use base::service;
use crate::service::LABEL_LIST;
use crate::{dao, utils};

pub async fn cache_author() {
    let d_au = base::service::blog_author_sve::find_by_id(1).await;
    if d_au.is_err() {
        error!("find author error by id = {:?}", 1);
        return;
    }

    let u = d_au.unwrap();

    if u.is_none() {
        error!("author 不存在");
        return;
    }

    crate::service::set_blog_author(u.unwrap()).await;
}

/// 缓存标签
pub async fn cache_label() {
    let params:HashMap<String, sql::Params> = HashMap::new();
    let result = base::service::blog_label_sve::query_list(&params, &utils::limit_max()).await;
    if result.is_err() {
        tracing::warn!("{:?}", result);
        return
    }
    let lst = result.unwrap();

    let mut cache = LABEL_LIST.write().await;
    for label in lst {
        cache.insert(label.id, label.label_name);
    }
}

/// 获取标签名称
/// id -> label_name
pub async fn find_label_by_id(id: u64) -> Result<String, String> {
    // 1. 查询缓存, 不存在重新获取缓存
    let cache = LABEL_LIST.read().await;
    if let Some(value) = cache.get(&id) {
        Ok(value.to_string())
    } else {
        return Err("not find".to_string());
    }
}


pub async fn query_list(params: &HashMap<String, sql::Params>, condition: &[sql::Condition]) -> mysql::Result<Vec<BlogArticleModel>, String> {
    let mut call = | tx:&mut Transaction |  -> mysql::Result<Vec<BlogArticleModel>, String>  {
        return dao::blog_dao::query_list(tx, params, condition);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

