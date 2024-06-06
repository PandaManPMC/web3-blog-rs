use serde::{Serialize, Deserialize};
use r2d2_mysql::mysql::{params, Row};
use r2d2_mysql::mysql::params::Params;
use i_dao::model::BaseModel;
use serde::Deserializer;
use serde::de;
use std::fmt;
use serde::de::Unexpected;

pub const TABLE_NAME:&str = "blog_label";
pub const FIELDS:[&str;6] = ["id","created_at","updated_at","label_name","state","sequence"];
pub const ALIAS:&str = "blogLabel";

///	BlogLabelModel 文章标签
///	table - blog_label
///	author: AT
///	since: 2024-06-06 15:57:24
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogLabelModel {
	/// search文章标签编号 【max:20】 
	#[serde(rename = "id")]
	pub id: u64,
	/// 创建时间 【max:20】 
	#[serde(rename = "createdAt")]
	pub created_at: u64,
	/// 最后更新 【max:20】 
	#[serde(rename = "updatedAt")]
	pub updated_at: u64,
	/// search标签名称 【max:20】 
	#[serde(rename = "labelName")]
	pub label_name: String,
	/// thing状态:1@可见;2@不可见 【max:3】 
	#[serde(rename = "state")]
	pub state: u8,
	/// 顺序 【max:10】 
	#[serde(rename = "sequence")]
	pub sequence: u32,
}

impl BlogLabelModel {

    pub fn new(label_name: String, state: u8, sequence: u32) -> BlogLabelModel {
        BlogLabelModel{id:0, created_at: 0, updated_at: 0, label_name, state, sequence}
    }

    pub fn new_full(id: u64, created_at: u64, updated_at: u64, label_name: String, state: u8, sequence: u32) -> BlogLabelModel {
        BlogLabelModel{id, created_at, updated_at, label_name, state, sequence}
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
pub fn pot(row: Row, offset: usize) -> BlogLabelModel {
	return BlogLabelModel::new_full(row.get(offset+0).unwrap(),row.get(offset+1).unwrap(),row.get(offset+2).unwrap(),row.get(offset+3).unwrap(),row.get(offset+4).unwrap(),row.get(offset+5).unwrap());
}

///	BlogLabelJSONOut 文章标签
///	author: AT
///	since: 2024-06-06 15:57:24
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogLabelJSONOut {
	/// search文章标签编号 【max:20】 
	#[serde(rename = "id")]
	pub id: u64,
	/// 创建时间 【max:20】 
	#[serde(rename = "createdAt")]
	pub created_at: u64,
	/// 最后更新 【max:20】 
	#[serde(rename = "updatedAt")]
	pub updated_at: u64,
	/// search标签名称 【max:20】 
	#[serde(rename = "labelName")]
	pub label_name: String,
	/// thing状态:1@可见;2@不可见 【max:3】 
	#[serde(rename = "state")]
	pub state: u8,
	/// 顺序 【max:10】 
	#[serde(rename = "sequence")]
	pub sequence: u32,
}

///	BlogLabelJSONIn 文章标签
///	author: AT
///	since: 2024-06-06 15:57:24
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogLabelJSONIn {
	/// search文章标签编号 【max:20】
	#[serde(rename = "id")]
	pub id: u64,
	/// search标签名称 【max:20】
	#[serde(rename = "labelName", deserialize_with = "check_length_label_name")]
	pub label_name: String,
	/// thing状态:1@可见;2@不可见 【max:3】
	#[serde(rename = "state")]
	pub state: u8,
	/// 顺序 【max:10】
	#[serde(rename = "sequence")]
	pub sequence: u32,
}

plier::create_serde_string_length_checker!(check_length_label_name, 0, 20);

impl BaseModel for BlogLabelModel {

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
			"label_name" => self.label_name.to_string(),
			"state" => self.state,
			"sequence" => self.sequence,
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
			"label_name" => self.label_name.to_string(),
			"state" => self.state,
			"sequence" => self.sequence,
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

