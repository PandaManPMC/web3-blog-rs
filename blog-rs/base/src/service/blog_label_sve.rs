///	blogLabelService
///	标准 service - 文章标签 - blog_label
///	author: AT
///	since: 2024-05-03 11:58:15
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com

use mysql::{Result};
use i_dao::{dao, i_mysql, sql};
use std::collections::HashMap;
use std::any::Any;
use std::result::Result::Ok;
use r2d2_mysql::mysql::Transaction;
use r2d2_mysql::{MySqlConnectionManager, r2d2};
use crate::{model::blog_label::BlogLabelModel, dao::blog_label_dao, service};

pub fn add(m: &mut BlogLabelModel) -> Result<(), Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction | -> Result<(), Box<dyn std::error::Error>>  {
        return dao::add(tx, m);
    };
    return Ok(i_mysql::start_tx(&service::get_data_source_key(), &mut call)?);
}

pub fn add_batch(lst: &mut Vec<&mut BlogLabelModel>) -> Result<(),Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<(), Box<dyn std::error::Error>>  {
        return dao::add_batch(tx, lst);
    };
    return Ok(i_mysql::start_tx(&service::get_data_source_key(), &mut call)?);
}

pub fn update_by_id(m: &mut BlogLabelModel) -> Result<(),Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<(), Box<dyn std::error::Error>>  {
        return dao::update_by_pk(tx, m);
    };
    return Ok(i_mysql::start_tx(&service::get_data_source_key(), &mut call)?);
}

pub fn query_list(params: &HashMap<String, Box<dyn Any>>, condition: &[sql::Condition]) -> Result<Vec<BlogLabelModel>, Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<Vec<BlogLabelModel>, Box<dyn std::error::Error>>  {
        let result = blog_label_dao::query_list(tx, params, condition);
        return Ok(result?);
    };
    return Ok(i_mysql::start_tx(&service::get_data_source_key(), &mut call)?);
}

pub fn find_by_id(id: u64) -> Result<Option<BlogLabelModel>, Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<BlogLabelModel>, Box<dyn std::error::Error>>  {
        let result = blog_label_dao::find_by_id(tx, id);
        return Ok(result?);
    };
    return Ok(i_mysql::start_tx(&service::get_data_source_key(), &mut call)?);
}

pub fn query_count(params: &HashMap<String, Box<dyn Any>>, condition: &[sql::Condition]) -> Result<u64, Box<dyn std::error::Error>> {
    let mut call = | conn:&mut r2d2::PooledConnection<MySqlConnectionManager> |  -> Result<u64, Box<dyn std::error::Error>>  {
        let result = blog_label_dao::query_count(conn, params, condition);
        return Ok(result?);
    };
    return Ok(i_mysql::direct(&service::get_data_source_key(), &mut call)?);
}


pub fn find_by_label_name(label_name: String) -> Result<Option<BlogLabelModel>, Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<BlogLabelModel>, Box<dyn std::error::Error>>  {
        let result = blog_label_dao::find_by_label_name(tx, label_name.clone());
        return Ok(result?);
    };
    return Ok(i_mysql::start_tx(&service::get_data_source_key(), &mut call)?);
}

