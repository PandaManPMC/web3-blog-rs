///	blogClassesService
///	标准 service - 文章类型 - blog_classes
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
use crate::{model::blog_classes::BlogClassesModel, dao::blog_classes_dao, service};

pub fn add(m: &mut BlogClassesModel) -> Result<(), Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction | -> Result<(), Box<dyn std::error::Error>>  {
        return dao::add(tx, m);
    };
    return Ok(i_mysql::start_tx(&service::get_data_source_key(), &mut call)?);
}

pub fn add_batch(lst: &mut Vec<&mut BlogClassesModel>) -> Result<(),Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<(), Box<dyn std::error::Error>>  {
        return dao::add_batch(tx, lst);
    };
    return Ok(i_mysql::start_tx(&service::get_data_source_key(), &mut call)?);
}

pub fn update_by_id(m: &mut BlogClassesModel) -> Result<(),Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<(), Box<dyn std::error::Error>>  {
        return dao::update_by_pk(tx, m);
    };
    return Ok(i_mysql::start_tx(&service::get_data_source_key(), &mut call)?);
}

pub fn query_list(params: &HashMap<String, Box<dyn Any>>, condition: &[sql::Condition]) -> Result<Vec<BlogClassesModel>, Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<Vec<BlogClassesModel>, Box<dyn std::error::Error>>  {
        let result = blog_classes_dao::query_list(tx, params, condition);
        return Ok(result?);
    };
    return Ok(i_mysql::start_tx(&service::get_data_source_key(), &mut call)?);
}

pub fn find_by_id(id: u64) -> Result<Option<BlogClassesModel>, Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<BlogClassesModel>, Box<dyn std::error::Error>>  {
        let result = blog_classes_dao::find_by_id(tx, id);
        return Ok(result?);
    };
    return Ok(i_mysql::start_tx(&service::get_data_source_key(), &mut call)?);
}

pub fn query_count(params: &HashMap<String, Box<dyn Any>>, condition: &[sql::Condition]) -> Result<u64, Box<dyn std::error::Error>> {
    let mut call = | conn:&mut r2d2::PooledConnection<MySqlConnectionManager> |  -> Result<u64, Box<dyn std::error::Error>>  {
        let result = blog_classes_dao::query_count(conn, params, condition);
        return Ok(result?);
    };
    return Ok(i_mysql::direct(&service::get_data_source_key(), &mut call)?);
}


pub fn find_by_classes_name(classes_name: String) -> Result<Option<BlogClassesModel>, Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<BlogClassesModel>, Box<dyn std::error::Error>>  {
        let result = blog_classes_dao::find_by_classes_name(tx, classes_name.clone());
        return Ok(result?);
    };
    return Ok(i_mysql::start_tx(&service::get_data_source_key(), &mut call)?);
}

