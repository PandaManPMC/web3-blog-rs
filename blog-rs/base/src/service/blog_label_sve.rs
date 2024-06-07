///	blogLabelService
///	标准 service - 文章标签 - blog_label
///	author: AT
///	since: 2024-06-07 17:02:09
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com

use mysql::{Result};
use i_dao::sql;
use i_dao::tok::{i_mysql, dao};
use std::collections::HashMap;
use std::any::Any;
use std::result::Result::Ok;
use r2d2_mysql::mysql::Transaction;
use r2d2_mysql::{MySqlConnectionManager, r2d2};
use crate::{model::blog_label::BlogLabelModel, dao::blog_label_dao, service};

pub async fn add(m: &mut BlogLabelModel) -> Result<(), String> {
    let mut call = | tx:&mut Transaction | -> Result<(), String>  {
        return dao::add(tx, m);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn add_batch(lst: &mut Vec<&mut BlogLabelModel>) -> Result<(), String> {
    let mut call = | tx:&mut Transaction |  -> Result<(), String>  {
        return dao::add_batch(tx, lst);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn update_by_id(m: &mut BlogLabelModel) -> Result<(), String> {
    let mut call = | tx:&mut Transaction |  -> Result<(), String>  {
        return dao::update_by_pk(tx, m);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn query_list(params: &HashMap<String, sql::Params>, condition: &[sql::Condition]) -> Result<Vec<BlogLabelModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Vec<BlogLabelModel>, String>  {
        return blog_label_dao::query_list(tx, params, condition);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn find_by_id(id: u64) -> Result<Option<BlogLabelModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<BlogLabelModel>, String>  {
        return blog_label_dao::find_by_id(tx, id);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn query_count(params: &HashMap<String, sql::Params>, condition: &[sql::Condition]) -> Result<u64, String> {
    let mut call = | conn:&mut r2d2::PooledConnection<MySqlConnectionManager> |  -> Result<u64, String>  {
        return blog_label_dao::query_count(conn, params, condition);
    };
    return i_mysql::direct(&service::get_data_source_key().await, &mut call).await;
}


pub async fn find_by_label_name(label_name: String) -> Result<Option<BlogLabelModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<BlogLabelModel>, String>  {
        return blog_label_dao::find_by_label_name(tx, label_name.clone());
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

