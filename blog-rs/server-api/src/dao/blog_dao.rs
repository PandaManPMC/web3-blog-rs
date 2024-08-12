use std::collections::HashMap;
use i_dao::sql;
use log::{debug, warn};
use r2d2_mysql::mysql::{Row, Transaction, Value};
use r2d2_mysql::mysql::prelude::Queryable;
use base::model::blog_article;
use base::model::blog_article::BlogArticleModel;
use std::time::{SystemTime};

pub fn query_list(tx: &mut Transaction, condition_params: &HashMap<String, sql::Params>, condition: &[sql:: Condition]) -> Result<Vec<BlogArticleModel>, String> {
    let article = "article";
    let mut query_sql = format!("SELECT {} FROM {} as {}", blog_article::get_field_sql(article), blog_article::TABLE_NAME, article);
    query_sql = format!("{} LEFT JOIN blog_article_label AS bal ON {}.id = bal.id_blog_article", query_sql, article);
    let mut params: Vec<Value> = vec![];

    let (mut where_sql,page_index,page_size,mut order_by_sql_field,order_by_sql_type) = sql::pot_base_condition(&mut params, &condition);

    for (key, val) in condition_params.iter() {
        let (i_key, operator) = sql::get_real_key_operator(key.to_string());
        if "" != where_sql {
            if "id_blog_label" == i_key {
                where_sql = format!(" {} AND bal.{} {} ?", where_sql, i_key, operator)
            } else {
                where_sql = format!(" {} AND article.{} {} ?", where_sql, i_key, operator)
            }
        } else {
            if "id_blog_label" == i_key {
                where_sql = format!(" bal.{} {} ?", i_key, operator)
            } else {
                where_sql = format!(" article.{} {} ?", i_key, operator)
            }
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
    query_sql = format!(" {} GROUP BY article.id ORDER BY {} {}", query_sql, order_by_sql_field, order_by_sql_type);
    query_sql = format!("{} LIMIT {},{}", query_sql, (page_index-1) * page_size, page_size);

    debug!("blog_article_dao::query_list::query_sql={}", query_sql);
    let result = tx.exec_map(
        query_sql,  params ,|row: Row| blog_article::pot(row, 0)
    );

    if result.is_err() {
        warn!("b_d::blog_article_dao::query_list 失败！ res={:?}", result);
        return Err(result.err().unwrap().to_string());
    }

    return Ok(result.unwrap());
}

/// update_watch_count 更新观看数量
pub fn update_watch_count(tx: &mut Transaction, id: u64, watch_count: u32) -> Result<() , String> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    let stmt = "UPDATE blog_article SET updated_at=?,watch_count=? WHERE id = ?";
    debug!("blog_dao::update_watch_count sql={}", stmt);
    let stmt = tx.prep(stmt)
        .unwrap();

    let mut params: Vec<Value> = vec![];
    params.push(now.into());
    params.push(watch_count.into());
    params.push(id.into());
    let result = tx.exec_drop(stmt.clone(), params);
    if result.is_err() {
        warn!("blog_dao::update_watch_count 失败！ res={:?} sql={:?}", result, stmt);
        return Err(result.err().unwrap().to_string());
    }

    return Ok(());
}

/// update_view_count 更新评论数量
pub fn update_view_count(tx: &mut Transaction, id: u64, view_count: u32) -> Result<() , String> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    let stmt = "UPDATE blog_article SET updated_at=?,view_count=? WHERE id = ?";
    debug!("blog_dao::update_view_count sql={}", stmt);
    let stmt = tx.prep(stmt)
        .unwrap();

    let mut params: Vec<Value> = vec![];
    params.push(now.into());
    params.push(view_count.into());
    params.push(id.into());
    let result = tx.exec_drop(stmt.clone(), params);
    if result.is_err() {
        warn!("blog_dao::update_view_count 失败！ res={:?} sql={:?}", result, stmt);
        return Err(result.err().unwrap().to_string());
    }

    return Ok(());
}
