///	advertiseInfoService
///	标准 service - 广告 - advertise_info
///	author: AT
///	since: 2024-08-10 14:11:36
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com

use mysql::{Result};
use i_dao::sql;
use i_dao::tok::{i_mysql, dao};
use std::collections::HashMap;
use std::any::Any;
use std::result::Result::Ok;
use r2d2_mysql::mysql::Transaction;
use r2d2_mysql::{MySqlConnectionManager, r2d2};
use crate::{model::advertise_info::AdvertiseInfoModel, dao::advertise_info_dao, service};

pub async fn add(m: &mut AdvertiseInfoModel) -> Result<(), String> {
    let mut call = | tx:&mut Transaction | -> Result<(), String>  {
        return dao::add(tx, m);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn add_batch(lst: &mut Vec<&mut AdvertiseInfoModel>) -> Result<(), String> {
    let mut call = | tx:&mut Transaction |  -> Result<(), String>  {
        return dao::add_batch(tx, lst);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn update_by_id(m: &mut AdvertiseInfoModel) -> Result<(), String> {
    let mut call = | tx:&mut Transaction |  -> Result<(), String>  {
        return dao::update_by_pk(tx, m);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn query_list(params: &HashMap<String, sql::Params>, condition: &[sql::Condition]) -> Result<Vec<AdvertiseInfoModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Vec<AdvertiseInfoModel>, String>  {
        return advertise_info_dao::query_list(tx, params, condition);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn find_by_id(id: u64) -> Result<Option<AdvertiseInfoModel>, String> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<AdvertiseInfoModel>, String>  {
        return advertise_info_dao::find_by_id(tx, id);
    };
    return i_mysql::start_tx(&service::get_data_source_key().await, &mut call).await;
}

pub async fn query_count(params: &HashMap<String, sql::Params>, condition: &[sql::Condition]) -> Result<u64, String> {
    let mut call = | conn:&mut r2d2::PooledConnection<MySqlConnectionManager> |  -> Result<u64, String>  {
        return advertise_info_dao::query_count(conn, params, condition);
    };
    return i_mysql::direct(&service::get_data_source_key().await, &mut call).await;
}

