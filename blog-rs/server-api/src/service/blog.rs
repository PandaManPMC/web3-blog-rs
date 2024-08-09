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
use crate::service::{CLASSES_LIST, LABEL_LIST};
use crate::{dao, utils};

pub async fn cache_author() {
    tracing::info!("cache_author");

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

/// 缓存笔记本
pub async fn cache_classes() {
    tracing::info!("cache_classes");

    let mut params:HashMap<String, sql::Params> = HashMap::new();
    params.insert(String::from("state"), sql::Params::UInteger8(1));

    let result = base::service::blog_classes_sve::query_list(&params, &utils::limit_max()).await;
    if result.is_err() {
        tracing::warn!("{:?}", result);
        return
    }
    let lst = result.unwrap();

    let mut cache = CLASSES_LIST.write().await;
    for obj in lst {
        tracing::info!("cache_classes {:?}={:?}", obj.id,obj.classes_name);
        cache.insert(obj.id, obj);
    }
}

/// 缓存标签
pub async fn cache_label() {
    tracing::info!("cache_label");

    let mut params:HashMap<String, sql::Params> = HashMap::new();
    params.insert(String::from("state"), sql::Params::UInteger8(1));

    let result = base::service::blog_label_sve::query_list(&params, &utils::limit_max()).await;
    if result.is_err() {
        tracing::warn!("{:?}", result);
        return
    }
    let lst = result.unwrap();

    let mut cache = LABEL_LIST.write().await;
    for label in lst {
        tracing::info!("cache_label {:?}={:?}", label.id,label.label_name);
        cache.insert(label.id, label);
    }
}

/// 获取标签名称
/// id -> label_name
pub async fn find_label_by_id(id: u64) -> Result<String, String> {
    // 1. 查询缓存, 不存在重新获取缓存
    let cache = LABEL_LIST.read().await;
    if let Some(value) = cache.get(&id) {
        Ok(value.label_name.to_string())
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

