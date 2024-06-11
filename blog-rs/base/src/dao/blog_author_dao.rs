///	blogAuthorDao
///	标准 DAO - 作者 - blog_author
///	author: AT
///	since: 2024-06-09 15:31:16
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com

use log::{debug, warn};
use std::collections::HashMap;
use std::any::Any;
use r2d2_mysql::mysql::{Transaction, Value, Row, params::Params, prelude::Queryable};
use i_dao::{sql};
use i_dao::tok::dao;
use r2d2_mysql::{MySqlConnectionManager, r2d2};
use mysql::params;
use crate::{model::blog_author,model::blog_author::BlogAuthorModel};

pub fn query_list(tx: &mut Transaction, condition_params: &HashMap<String, sql::Params>, condition: &[sql:: Condition]) -> Result<Vec<BlogAuthorModel>, String> {
    let mut query_sql = format!("SELECT {} FROM {}", blog_author::get_field_sql(""), blog_author::TABLE_NAME);
    let mut params: Vec<Value> = vec![];

    let (mut where_sql,page_index,page_size,mut order_by_sql_field,order_by_sql_type) = sql::pot_base_condition(&mut params, &condition);

    for (key, val) in condition_params.iter() {
        let (i_key, operator) = sql::get_real_key_operator(key.to_string());
        if "" != where_sql {
            where_sql = format!(" {} AND {} {} ?", where_sql, i_key, operator)
        } else {
            where_sql = format!(" {} {} ?", i_key, operator)
        }

        if !sql::pot_params_condition_by_enum(&mut params, val) {
            warn!("test_user_dao::query_list::pot_params_condition - {} 参数装入失败", key)
        }
    }

    if "" != where_sql{
        query_sql = format!("{} WHERE {}", query_sql, where_sql);
    }
    if "" == order_by_sql_field {
        order_by_sql_field = "id".to_string();
    }
    query_sql = format!(" {} ORDER BY {} {}", query_sql, order_by_sql_field, order_by_sql_type);
    query_sql = format!("{} LIMIT {},{}", query_sql, (page_index-1) * page_size, page_size);

    debug!("blog_author_dao::query_list::query_sql={}", query_sql);
    let result = tx.exec_map(
        query_sql,  params ,|row: Row| blog_author::pot(row, 0)
    );

    if result.is_err() {
        warn!("b_d::blog_author_dao::query_list 失败！ res={:?}", result);
        return Err(result.err().unwrap().to_string());
    }

    return Ok(result.unwrap());
}

pub fn query_count(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, condition_params: &HashMap<String, sql::Params>, condition: &[sql:: Condition]) -> Result<u64, String> {
    let mut query_sql = format!("SELECT COUNT(1) AS co FROM {}", blog_author::TABLE_NAME);
    let mut params: Vec<Value> = vec![];

    let mut where_sql = sql::pot_base_condition_by_time(&mut params, &condition);

    for (key, val) in condition_params.iter() {
        let (i_key, operator) = sql::get_real_key_operator(key.to_string());
        if "" != where_sql {
            where_sql = format!(" {} AND {} {} ?", where_sql, i_key, operator)
        } else {
            where_sql = format!(" {} {} ?", i_key, operator)
        }

        if !sql::pot_params_condition_by_enum(&mut params, val) {
            warn!("test_user_dao::query_count::pot_params_condition - {} 参数装入失败", key)
        }
    }

    if "" != where_sql {
        query_sql = format!("{} WHERE {}", query_sql, where_sql);
    }

    debug!("blog_author_dao::query_count::query_sql={}", query_sql);
    let result = conn.exec_first::<u64,_,_> (
        query_sql,  params );

    return match result {
        Err(e) => {
           Err(e.to_string().into())
        },
        Ok(op) => {
            return match op {
                Some(c) => {
                    Ok(c)
                },
                _ => {
                    Ok(0)
                }
            }
        },
    };
}

pub fn find_by_id(tx: &mut Transaction, id: u64) -> Result<Option<BlogAuthorModel>, String> {
    let query_sql = format!("SELECT {} FROM {} WHERE {} = ? LIMIT 0,1", blog_author::get_field_sql("") ,blog_author::TABLE_NAME, blog_author::FIELDS[0]);
    let result = tx.exec_map(
        query_sql, (id,),|row: Row| blog_author::pot(row, 0)
    );
    if result.is_err() {
        warn!("b_d::blog_author_dao::find_by_id 失败！ res={:?}", result);
        return match result {
            Err(e) => {
                Err(e.to_string().into())
            },
            Ok(_) => {
                unimplemented!()
            },
        };
    }

    let mut lst = result.unwrap();
    if 0 == lst.len() {
        return Ok(None);
    }

    let one:Option<BlogAuthorModel> = lst.pop();
    return Ok(one);
}


pub fn find_by_pen_name(tx: &mut Transaction, pen_name: String) -> Result<Option<BlogAuthorModel>, String> {
    let query_sql = format!("SELECT {} FROM {} WHERE pen_name = ? ORDER BY id DESC LIMIT 0,1", blog_author::get_field_sql(""), blog_author::TABLE_NAME);
    let result = tx.exec_map(
        query_sql, (pen_name,),|row: Row| blog_author::pot(row, 0)
    );
    if result.is_err() {
        warn!("b_d::blog_author_dao::pen_name 失败！ res={:?}", result);
        return match result {
            Err(e) => {
                Err(e.to_string().into())
            },
            Ok(_) => {
                unimplemented!()
            },
        };
    }

    let mut lst = result.unwrap();
    if 0 == lst.len() {
        return Ok(None);
    }

    let one:Option<BlogAuthorModel> = lst.pop();
    return Ok(one);
}

pub fn find_by_user_name(tx: &mut Transaction, user_name: String) -> Result<Option<BlogAuthorModel>, String> {
    let query_sql = format!("SELECT {} FROM {} WHERE user_name = ? ORDER BY id DESC LIMIT 0,1", blog_author::get_field_sql(""), blog_author::TABLE_NAME);
    let result = tx.exec_map(
        query_sql, (user_name,),|row: Row| blog_author::pot(row, 0)
    );
    if result.is_err() {
        warn!("b_d::blog_author_dao::user_name 失败！ res={:?}", result);
        return match result {
            Err(e) => {
                Err(e.to_string().into())
            },
            Ok(_) => {
                unimplemented!()
            },
        };
    }

    let mut lst = result.unwrap();
    if 0 == lst.len() {
        return Ok(None);
    }

    let one:Option<BlogAuthorModel> = lst.pop();
    return Ok(one);
}

