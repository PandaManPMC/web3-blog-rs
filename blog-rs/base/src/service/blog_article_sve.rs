///	blogArticleService
///	标准 service - 文章 - blog_article
///	author: AT
///	since: 2024-06-07 15:30:56
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com

use mysql::{Result};
use i_dao::sql;
use i_dao::tok::{i_mysql, dao};
use std::collections::HashMap;
use std::any::Any;
use std::result::Result::Ok;
use r2d2_mysql::mysql::Transaction;
use r2d2_mysql::{MySqlConnectionManager, r2d2};
use crate::{model::blog_article::BlogArticleModel, dao::blog_article_dao, service};

pub async fn add(m: &mut BlogArticleModel) -> Result<(), String> {
    let mut call = | tx:&mut Transaction | -> Result<(), String>  {
        return dao::add(tx, m);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn add_batch(lst: &mut Vec<&mut BlogArticleModel>) -> Result<(), String> {
    let mut call = | tx:&mut Transaction |  -> Result<(), String>  {
        return dao::add_batch(tx, lst);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn update_by_id(m: &mut BlogArticleModel) -> Result<(), String> {
    let mut call = | tx:&mut Transaction |  -> Result<(), String>  {
        return dao::update_by_pk(tx, m);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn query_list(params: &HashMap<String, Box<dyn Any>>, condition: &[sql::Condition]) -> Result<Vec<BlogArticleModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Vec<BlogArticleModel>, String>  {
        return blog_article_dao::query_list(tx, params, condition);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn find_by_id(id: u64) -> Result<Option<BlogArticleModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<BlogArticleModel>, String>  {
        return blog_article_dao::find_by_id(tx, id);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn query_count(params: &HashMap<String, Box<dyn Any>>, condition: &[sql::Condition]) -> Result<u64, String> {
    let mut call = | conn:&mut r2d2::PooledConnection<MySqlConnectionManager> |  -> Result<u64, String>  {
        return blog_article_dao::query_count(conn, params, condition);
    };
    return i_mysql::direct(&service::get_data_source_key().await, &mut call).await;
}


pub async fn find_by_id_blog_author(id_blog_author: u64) -> Result<Option<BlogArticleModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<BlogArticleModel>, String>  {
        return blog_article_dao::find_by_id_blog_author(tx, id_blog_author);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}


pub async fn find_by_id_blog_classes(id_blog_classes: u64) -> Result<Option<BlogArticleModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<BlogArticleModel>, String>  {
        return blog_article_dao::find_by_id_blog_classes(tx, id_blog_classes);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

