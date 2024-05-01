use serde::{Serialize, Deserialize};
use r2d2_mysql::mysql::{params, Row};
use r2d2_mysql::mysql::params::Params;
use i_dao::model::BaseModel;
use i_dao_proc_macro::BaseModel;

pub const TABLE_NAME:&str = "blog_view";
pub const FIELDS:[&str;10] = ["id","created_at","updated_at","id_blog_article","view_content","coin_symbol","tip_amount","visible","address","tip_amount_usd"];
pub const ALIAS:&str = "blogView";

///	BlogViewModel 评论
///	table - blog_view
///	author: AT
///	since: 2024-05-01 16:42:01
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[derive(BaseModel)]
pub struct BlogViewModel {
	/// search评论编号 【max:20】 
	#[serde(rename = "id")]
	pub id: u64,
	/// 创建时间 【max:20】 
	#[serde(rename = "createdAt")]
	pub created_at: u64,
	/// 最后更新 【max:20】 
	#[serde(rename = "updatedAt")]
	pub updated_at: u64,
	/// search文章编号 【max:20】 
	#[serde(rename = "idBlogArticle")]
	pub id_blog_article: u64,
	/// 评论内容 【max:200】 
	#[serde(rename = "viewContent")]
	pub view_content: String,
	/// 代币符号 【max:20】 
	#[serde(rename = "coinSymbol")]
	pub coin_symbol: String,
	/// 小费金额 【max:79】 
	#[serde(rename = "tipAmount")]
	pub tip_amount: String,
	/// thing评论可见性:1@可见;2@不可见 【max:3】 
	#[serde(rename = "visible")]
	pub visible: u8,
	/// search地址 【max:155】 
	#[serde(rename = "address")]
	pub address: String,
	/// 小费金额USD 【max:79】 
	#[serde(rename = "tipAmountUsd")]
	pub tip_amount_usd: String,
}

impl BlogViewModel {
    pub fn new(user_name: String, state: u8) -> BlogViewModel {
        TestUser{id:0, created_at: 0, updated_at: 0, user_name, state}
    }

    pub fn new_full(id: u64, created_at: u64,updated_at: u64,  user_name: String, state: u8) -> BlogViewModel {
        TestUser{id, created_at, updated_at, user_name, state}
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
pub fn pot(row: Row, offset: usize) -> BlogViewModel {
	return BlogViewModel::new_full(row.get(offset+0).unwrap(),row.get(offset+1).unwrap(),row.get(offset+2).unwrap(),row.get(offset+3).unwrap(),row.get(offset+4).unwrap(),row.get(offset+5).unwrap(),row.get(offset+6).unwrap(),row.get(offset+7).unwrap(),row.get(offset+8).unwrap(),row.get(offset+9).unwrap());
}

/** 过程宏 #[derive(BaseModel)] -> 
impl BaseModel for BlogViewModel {

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
            "user_name" => self.user_name.to_string(),
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
            "updated_at" => self.updated_at,
            "user_name" => self.user_name.to_string(),
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
**/
