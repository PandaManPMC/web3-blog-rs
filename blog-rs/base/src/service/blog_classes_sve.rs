///	blogClassesService
///	标准 service - 文章类型 - blog_classes
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
use crate::{model::blog_classes::BlogClassesModel, dao::blog_classes_dao, service};

pub async fn add(m: &mut BlogClassesModel) -> Result<(), String> {
    let mut call = | tx:&mut Transaction | -> Result<(), String>  {
        return dao::add(tx, m);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn add_batch(lst: &mut Vec<&mut BlogClassesModel>) -> Result<(), String> {
    let mut call = | tx:&mut Transaction |  -> Result<(), String>  {
        return dao::add_batch(tx, lst);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn update_by_id(m: &mut BlogClassesModel) -> Result<(), String> {
    let mut call = | tx:&mut Transaction |  -> Result<(), String>  {
        return dao::update_by_pk(tx, m);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn query_list(params: &HashMap<String, sql::Params>, condition: &[sql::Condition]) -> Result<Vec<BlogClassesModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Vec<BlogClassesModel>, String>  {
        return blog_classes_dao::query_list(tx, params, condition);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn find_by_id(id: u64) -> Result<Option<BlogClassesModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<BlogClassesModel>, String>  {
        return blog_classes_dao::find_by_id(tx, id);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn query_count(params: &HashMap<String, sql::Params>, condition: &[sql::Condition]) -> Result<u64, String> {
    let mut call = | conn:&mut r2d2::PooledConnection<MySqlConnectionManager> |  -> Result<u64, String>  {
        return blog_classes_dao::query_count(conn, params, condition);
    };
    return i_mysql::direct(&service::get_data_source_key().await, &mut call).await;
}


pub async fn find_by_classes_name(classes_name: String) -> Result<Option<BlogClassesModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<BlogClassesModel>, String>  {
        return blog_classes_dao::find_by_classes_name(tx, classes_name.clone());
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

