///	blogAuthorService
///	标准 service - 作者 - blog_author
///	author: AT
///	since: 2024-06-09 15:31:16
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com

use mysql::{Result};
use i_dao::sql;
use i_dao::tok::{i_mysql, dao};
use std::collections::HashMap;
use std::any::Any;
use std::result::Result::Ok;
use r2d2_mysql::mysql::Transaction;
use r2d2_mysql::{MySqlConnectionManager, r2d2};
use crate::{model::blog_author::BlogAuthorModel, dao::blog_author_dao, service};

pub async fn add(m: &mut BlogAuthorModel) -> Result<(), String> {
    let mut call = | tx:&mut Transaction | -> Result<(), String>  {
        return dao::add(tx, m);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn add_batch(lst: &mut Vec<&mut BlogAuthorModel>) -> Result<(), String> {
    let mut call = | tx:&mut Transaction |  -> Result<(), String>  {
        return dao::add_batch(tx, lst);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn update_by_id(m: &mut BlogAuthorModel) -> Result<(), String> {
    let mut call = | tx:&mut Transaction |  -> Result<(), String>  {
        return dao::update_by_pk(tx, m);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn query_list(params: &HashMap<String, sql::Params>, condition: &[sql::Condition]) -> Result<Vec<BlogAuthorModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Vec<BlogAuthorModel>, String>  {
        return blog_author_dao::query_list(tx, params, condition);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn find_by_id(id: u64) -> Result<Option<BlogAuthorModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<BlogAuthorModel>, String>  {
        return blog_author_dao::find_by_id(tx, id);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn query_count(params: &HashMap<String, sql::Params>, condition: &[sql::Condition]) -> Result<u64, String> {
    let mut call = | conn:&mut r2d2::PooledConnection<MySqlConnectionManager> |  -> Result<u64, String>  {
        return blog_author_dao::query_count(conn, params, condition);
    };
    return i_mysql::direct(&service::get_data_source_key().await, &mut call).await;
}


pub async fn find_by_pen_name(pen_name: String) -> Result<Option<BlogAuthorModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<BlogAuthorModel>, String>  {
        return blog_author_dao::find_by_pen_name(tx, pen_name.clone());
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}


pub async fn find_by_user_name(user_name: String) -> Result<Option<BlogAuthorModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<BlogAuthorModel>, String>  {
        return blog_author_dao::find_by_user_name(tx, user_name.clone());
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

