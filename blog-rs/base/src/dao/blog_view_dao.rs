///	blogViewDao
///	标准 DAO - 评论 - blog_view
///	author: AT
///	since: 2024-08-12 11:21:49
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com

use log::{debug, warn};
use std::collections::HashMap;
use std::any::Any;
use r2d2_mysql::mysql::{Transaction, Value, Row, params::Params, prelude::Queryable};
use i_dao::{sql};
use i_dao::tok::dao;
use r2d2_mysql::{MySqlConnectionManager, r2d2};
use mysql::params;
use crate::{model::blog_view,model::blog_view::BlogViewModel};

pub fn query_list(tx: &mut Transaction, condition_params: &HashMap<String, sql::Params>, condition: &[sql:: Condition]) -> Result<Vec<BlogViewModel>, String> {
    let mut query_sql = format!("SELECT {} FROM {}", blog_view::get_field_sql(""), blog_view::TABLE_NAME);
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
            warn!("b_d::blog_view_dao::query_list::pot_params_condition - {} 参数装入失败", key)
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

    debug!("blog_view_dao::query_list::query_sql={}", query_sql);
    let result = tx.exec_map(
        query_sql,  params ,|row: Row| blog_view::pot(row, 0)
    );

    if result.is_err() {
        warn!("b_d::blog_view_dao::query_list 失败！ res={:?}", result);
        return Err(result.err().unwrap().to_string());
    }

    return Ok(result.unwrap());
}

pub fn query_count(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, condition_params: &HashMap<String, sql::Params>, condition: &[sql:: Condition]) -> Result<u64, String> {
    let mut query_sql = format!("SELECT COUNT(1) AS co FROM {}", blog_view::TABLE_NAME);
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
            warn!("b_d::blog_view_dao::query_count::pot_params_condition - {} 参数装入失败", key)
        }
    }

    if "" != where_sql {
        query_sql = format!("{} WHERE {}", query_sql, where_sql);
    }

    debug!("blog_view_dao::query_count::query_sql={}", query_sql);
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

pub fn find_by_id(tx: &mut Transaction, id: u64) -> Result<Option<BlogViewModel>, String> {
    let query_sql = format!("SELECT {} FROM {} WHERE {} = ? LIMIT 0,1", blog_view::get_field_sql("") ,blog_view::TABLE_NAME, blog_view::FIELDS[0]);
    let result = tx.exec_map(
        query_sql, (id,),|row: Row| blog_view::pot(row, 0)
    );
    if result.is_err() {
        warn!("b_d::blog_view_dao::find_by_id 失败！ res={:?}", result);
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

    let one:Option<BlogViewModel> = lst.pop();
    return Ok(one);
}


pub fn find_by_id_blog_article(tx: &mut Transaction, id_blog_article: u64) -> Result<Option<BlogViewModel>, String> {
    let query_sql = format!("SELECT {} FROM {} WHERE id_blog_article = ? ORDER BY id DESC LIMIT 0,1", blog_view::get_field_sql(""), blog_view::TABLE_NAME);
    let result = tx.exec_map(
        query_sql, (id_blog_article,),|row: Row| blog_view::pot(row, 0)
    );
    if result.is_err() {
        warn!("b_d::blog_view_dao::id_blog_article 失败！ res={:?}", result);
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

    let one:Option<BlogViewModel> = lst.pop();
    return Ok(one);
}

pub fn find_by_address(tx: &mut Transaction, address: String) -> Result<Option<BlogViewModel>, String> {
    let query_sql = format!("SELECT {} FROM {} WHERE address = ? ORDER BY id DESC LIMIT 0,1", blog_view::get_field_sql(""), blog_view::TABLE_NAME);
    let result = tx.exec_map(
        query_sql, (address,),|row: Row| blog_view::pot(row, 0)
    );
    if result.is_err() {
        warn!("b_d::blog_view_dao::address 失败！ res={:?}", result);
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

    let one:Option<BlogViewModel> = lst.pop();
    return Ok(one);
}

pub fn find_by_ticket(tx: &mut Transaction, ticket: String) -> Result<Option<BlogViewModel>, String> {
    let query_sql = format!("SELECT {} FROM {} WHERE ticket = ? ORDER BY id DESC LIMIT 0,1", blog_view::get_field_sql(""), blog_view::TABLE_NAME);
    let result = tx.exec_map(
        query_sql, (ticket,),|row: Row| blog_view::pot(row, 0)
    );
    if result.is_err() {
        warn!("b_d::blog_view_dao::ticket 失败！ res={:?}", result);
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

    let one:Option<BlogViewModel> = lst.pop();
    return Ok(one);
}

