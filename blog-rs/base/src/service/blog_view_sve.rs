///	blogViewService
///	标准 service - 评论 - blog_view
///	author: AT
///	since: 2024-08-12 11:21:49
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com

use mysql::{Result};
use i_dao::sql;
use i_dao::tok::{i_mysql, dao};
use std::collections::HashMap;
use std::any::Any;
use std::result::Result::Ok;
use r2d2_mysql::mysql::Transaction;
use r2d2_mysql::{MySqlConnectionManager, r2d2};
use crate::{model::blog_view::BlogViewModel, dao::blog_view_dao, service};

pub async fn add(m: &mut BlogViewModel) -> Result<(), String> {
    let mut call = | tx:&mut Transaction | -> Result<(), String>  {
        return dao::add(tx, m);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn add_batch(lst: &mut Vec<&mut BlogViewModel>) -> Result<(), String> {
    let mut call = | tx:&mut Transaction |  -> Result<(), String>  {
        return dao::add_batch(tx, lst);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn update_by_id(m: &mut BlogViewModel) -> Result<(), String> {
    let mut call = | tx:&mut Transaction |  -> Result<(), String>  {
        return dao::update_by_pk(tx, m);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn query_list(params: &HashMap<String, sql::Params>, condition: &[sql::Condition]) -> Result<Vec<BlogViewModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Vec<BlogViewModel>, String>  {
        return blog_view_dao::query_list(tx, params, condition);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn find_by_id(id: u64) -> Result<Option<BlogViewModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<BlogViewModel>, String>  {
        return blog_view_dao::find_by_id(tx, id);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn query_count(params: &HashMap<String, sql::Params>, condition: &[sql::Condition]) -> Result<u64, String> {
    let mut call = | conn:&mut r2d2::PooledConnection<MySqlConnectionManager> |  -> Result<u64, String>  {
        return blog_view_dao::query_count(conn, params, condition);
    };
    return i_mysql::direct(&service::get_data_source_key().await, &mut call).await;
}


pub async fn find_by_id_blog_article(id_blog_article: u64) -> Result<Option<BlogViewModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<BlogViewModel>, String>  {
        return blog_view_dao::find_by_id_blog_article(tx, id_blog_article);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}


pub async fn find_by_address(address: String) -> Result<Option<BlogViewModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<BlogViewModel>, String>  {
        return blog_view_dao::find_by_address(tx, address.clone());
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}


pub async fn find_by_ticket(ticket: String) -> Result<Option<BlogViewModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<BlogViewModel>, String>  {
        return blog_view_dao::find_by_ticket(tx, ticket.clone());
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

