use serde::{Serialize, Deserialize};
use r2d2_mysql::mysql::{params, Row};
use r2d2_mysql::mysql::params::Params;
use i_dao::model::BaseModel;
use serde::Deserializer;
use serde::de;
use std::fmt;
use serde::de::Unexpected;

pub const TABLE_NAME:&str = "blog_article_label";
pub const FIELDS:[&str;6] = ["id","created_at","updated_at","id_blog_article","id_blog_label","state"];
pub const ALIAS:&str = "blogArticleLabel";

///	BlogArticleLabelModel 文章所有标签
///	table - blog_article_label
///	author: AT
///	since: 2024-06-07 17:02:09
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogArticleLabelModel {
	/// search关联编号 【max:20】 
	#[serde(rename = "id")]
	pub id: u64,
	/// 创建时间 【max:20】 
	#[serde(rename = "createdAt")]
	pub created_at: u64,
	/// 最后更新 【max:20】 
	#[serde(rename = "updatedAt")]
	pub updated_at: u64,
	/// 文章编号 【max:20】 
	#[serde(rename = "idBlogArticle")]
	pub id_blog_article: u64,
	/// 标签编号 【max:20】 
	#[serde(rename = "idBlogLabel")]
	pub id_blog_label: u64,
	/// thing状态:1@正常;2@已删除 【max:3】 
	#[serde(rename = "state")]
	pub state: u8,
}

impl BlogArticleLabelModel {

    pub fn new(id_blog_article: u64, id_blog_label: u64, state: u8) -> BlogArticleLabelModel {
        BlogArticleLabelModel{id:0, created_at: 0, updated_at: 0, id_blog_article, id_blog_label, state}
    }

    pub fn new_full(id: u64, created_at: u64, updated_at: u64, id_blog_article: u64, id_blog_label: u64, state: u8) -> BlogArticleLabelModel {
        BlogArticleLabelModel{id, created_at, updated_at, id_blog_article, id_blog_label, state}
    }

    fn set_pk(&mut self, pk: u64) {
        self.id = pk;
    }

    fn set_created_at(&mut self, at: u64) {
        self.created_at = at;
    }

    fn set_updated_at(&mut self, at: u64) {
        self.updated_at = at;
    }
}

/// get_field_sql 获取字段列 sql
pub fn get_field_sql(alias: &str) -> String {
    let mut columns = String::from("");
    for c in FIELDS {
        if "" != columns {
            columns.push_str(", ");
        }
        if "" != alias {
            columns.push_str(&format!("{}.{}" , alias, c));
        } else {
            columns.push_str(&format!("{}" , c));
        }
    }
    return columns;
}

/// pot 罐子 -> 把 mysql-row 按指定偏移 offset 装入结构体
pub fn pot(row: Row, offset: usize) -> BlogArticleLabelModel {
	return BlogArticleLabelModel::new_full(row.get(offset+0).unwrap(),row.get(offset+1).unwrap(),row.get(offset+2).unwrap(),row.get(offset+3).unwrap(),row.get(offset+4).unwrap(),row.get(offset+5).unwrap());
}

///	BlogArticleLabelJSONOut 文章所有标签
///	author: AT
///	since: 2024-06-07 17:02:09
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogArticleLabelJSONOut {
	/// search关联编号 【max:20】 
	#[serde(rename = "id")]
	pub id: u64,
	/// 创建时间 【max:20】 
	#[serde(rename = "createdAt")]
	pub created_at: u64,
	/// 最后更新 【max:20】 
	#[serde(rename = "updatedAt")]
	pub updated_at: u64,
	/// 文章编号 【max:20】 
	#[serde(rename = "idBlogArticle")]
	pub id_blog_article: u64,
	/// 标签编号 【max:20】 
	#[serde(rename = "idBlogLabel")]
	pub id_blog_label: u64,
	/// thing状态:1@正常;2@已删除 【max:3】 
	#[serde(rename = "state")]
	pub state: u8,
}

///	BlogArticleLabelJSONIn 文章所有标签
///	author: AT
///	since: 2024-06-07 17:02:09
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogArticleLabelJSONIn {
	/// search关联编号 【max:20】
	#[serde(rename = "id")]
	pub id: u64,
	/// 文章编号 【max:20】
	#[serde(rename = "idBlogArticle")]
	pub id_blog_article: u64,
	/// 标签编号 【max:20】
	#[serde(rename = "idBlogLabel")]
	pub id_blog_label: u64,
	/// thing状态:1@正常;2@已删除 【max:3】
	#[serde(rename = "state")]
	pub state: u8,
}


impl BaseModel for BlogArticleLabelModel {

    fn get_table_name(&self) -> &str {
        return TABLE_NAME;
    }

    fn get_alias(&self) -> &str {
        return ALIAS;
    }

    fn get_fields_count(&self) -> u16{
        return FIELDS.len().try_into().unwrap();
    }

    fn get_field_sql(&self, alias: &str) -> String {
        return get_field_sql(alias);
    }

    fn get_field_sql_not_pk(&self, alias: &str) -> String {
        let mut columns = String::from("");
        for c in &FIELDS[1..] {
            if "" != columns {
                columns.push_str(", ");
            }
            if "" != alias {
                columns.push_str(&format!("{}.{}" , alias, c));
            } else {
                columns.push_str(&format!("{}" , c));
            }
        }
        return columns;
    }

    fn get_params_insert(&self) -> (r2d2_mysql::mysql::Params, String, String) {
        let mut columns = String::from("");
        let mut keys = String::from("");

        for c in &FIELDS[1..] {
            if "" != columns {
                columns.push_str(", ");
                keys.push_str(", ");
            }
            columns.push_str(&format!("{}" , c));
            keys.push_str(&format!(":{}" , c));
        }

        return (params! {
			"created_at" => self.created_at,
			"updated_at" => self.updated_at,
			"id_blog_article" => self.id_blog_article,
			"id_blog_label" => self.id_blog_label,
			"state" => self.state,
        }, columns, keys);
    }

    fn get_params_update_by_pk(&self) -> (Params, String, String) {
        let mut columns = String::from("");

        for c in &FIELDS[2..] {
            if "" != columns {
                columns.push_str(", ");
            }
            columns.push_str(&format!("{}=:{}" , c, c));
        }

        return (params! {
			"created_at" => self.created_at,
			"updated_at" => self.updated_at,
			"id_blog_article" => self.id_blog_article,
			"id_blog_label" => self.id_blog_label,
			"state" => self.state,
            "id" => self.id,
        }, columns, String::from(format!("{}=:{}",  FIELDS[0], FIELDS[0])))
    }

    fn set_pk(&mut self, pk: u64) {
        self.id = pk;
    }

    fn set_created_at(&mut self, at: u64) {
        self.created_at = at;
    }

    fn set_updated_at(&mut self, at: u64) {
        self.updated_at = at;
    }

}

