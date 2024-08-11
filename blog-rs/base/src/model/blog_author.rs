use serde::{Serialize, Deserialize};
use r2d2_mysql::mysql::{params, Row};
use r2d2_mysql::mysql::params::Params;
use i_dao::model::BaseModel;
use serde::Deserializer;
use serde::de;
use std::fmt;
use serde::de::Unexpected;

pub const TABLE_NAME:&str = "blog_author";
pub const FIELDS:[&str;11] = ["id","created_at","updated_at","pen_name","user_name","user_pwd","google_auth_secret","profile","introduce","mk_footer","contact_mail"];
pub const ALIAS:&str = "blogAuthor";

///	BlogAuthorModel 作者
///	table - blog_author
///	author: AT
///	since: 2024-08-11 18:07:23
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogAuthorModel {
	/// search作者编号 【max:20】 
	#[serde(rename = "id")]
	pub id: u64,
	/// 创建时间 【max:20】 
	#[serde(rename = "createdAt")]
	pub created_at: u64,
	/// 最后更新 【max:20】 
	#[serde(rename = "updatedAt")]
	pub updated_at: u64,
	/// search笔名 【max:20】 
	#[serde(rename = "penName")]
	pub pen_name: String,
	/// search用户名 【max:20】 
	#[serde(rename = "userName")]
	pub user_name: String,
	/// 密码 【max:64】 
	#[serde(rename = "userPwd")]
	pub user_pwd: String,
	/// 谷歌验证器 【max:256】 
	#[serde(rename = "googleAuthSecret")]
	pub google_auth_secret: String,
	/// 头像 【max:255】 
	#[serde(rename = "profile")]
	pub profile: String,
	/// 介绍 【max:255】 
	#[serde(rename = "introduce")]
	pub introduce: String,
	/// mk_页脚 【max:65535】 
	#[serde(rename = "mkFooter")]
	pub mk_footer: String,
	/// 联系邮箱 【max:55】 
	#[serde(rename = "contactMail")]
	pub contact_mail: String,
}

impl BlogAuthorModel {

    pub fn new(pen_name: String, user_name: String, user_pwd: String, google_auth_secret: String, profile: String, introduce: String, mk_footer: String, contact_mail: String) -> BlogAuthorModel {
        BlogAuthorModel{id:0, created_at: 0, updated_at: 0, pen_name, user_name, user_pwd, google_auth_secret, profile, introduce, mk_footer, contact_mail}
    }

    pub fn new_full(id: u64, created_at: u64, updated_at: u64, pen_name: String, user_name: String, user_pwd: String, google_auth_secret: String, profile: String, introduce: String, mk_footer: String, contact_mail: String) -> BlogAuthorModel {
        BlogAuthorModel{id, created_at, updated_at, pen_name, user_name, user_pwd, google_auth_secret, profile, introduce, mk_footer, contact_mail}
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
pub fn pot(row: Row, offset: usize) -> BlogAuthorModel {
	return BlogAuthorModel::new_full(row.get(offset+0).unwrap(),row.get(offset+1).unwrap(),row.get(offset+2).unwrap(),row.get(offset+3).unwrap(),row.get(offset+4).unwrap(),row.get(offset+5).unwrap(),row.get(offset+6).unwrap(),row.get(offset+7).unwrap(),row.get(offset+8).unwrap(),row.get(offset+9).unwrap(),row.get(offset+10).unwrap());
}

///	BlogAuthorJSONOut 作者
///	author: AT
///	since: 2024-08-11 18:07:23
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogAuthorJSONOut {
	/// search作者编号 【max:20】 
	#[serde(rename = "id")]
	pub id: u64,
	/// 创建时间 【max:20】 
	#[serde(rename = "createdAt")]
	pub created_at: u64,
	/// 最后更新 【max:20】 
	#[serde(rename = "updatedAt")]
	pub updated_at: u64,
	/// search笔名 【max:20】 
	#[serde(rename = "penName")]
	pub pen_name: String,
	/// search用户名 【max:20】 
	#[serde(rename = "userName")]
	pub user_name: String,
	/// 密码 【max:64】 
	#[serde(rename = "userPwd")]
	pub user_pwd: String,
	/// 谷歌验证器 【max:256】 
	#[serde(rename = "googleAuthSecret")]
	pub google_auth_secret: String,
	/// 头像 【max:255】 
	#[serde(rename = "profile")]
	pub profile: String,
	/// 介绍 【max:255】 
	#[serde(rename = "introduce")]
	pub introduce: String,
	/// mk_页脚 【max:65535】 
	#[serde(rename = "mkFooter")]
	pub mk_footer: String,
	/// 联系邮箱 【max:55】 
	#[serde(rename = "contactMail")]
	pub contact_mail: String,
}

///	BlogAuthorJSONIn 作者
///	author: AT
///	since: 2024-08-11 18:07:23
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogAuthorJSONIn {
	/// search作者编号 【max:20】
	#[serde(rename = "id")]
	pub id: u64,
	/// search笔名 【max:20】
	#[serde(rename = "penName", deserialize_with = "check_length_pen_name")]
	pub pen_name: String,
	/// search用户名 【max:20】
	#[serde(rename = "userName", deserialize_with = "check_length_user_name")]
	pub user_name: String,
	/// 密码 【max:64】
	#[serde(rename = "userPwd", deserialize_with = "check_length_user_pwd")]
	pub user_pwd: String,
	/// 谷歌验证器 【max:256】
	#[serde(rename = "googleAuthSecret", deserialize_with = "check_length_google_auth_secret")]
	pub google_auth_secret: String,
	/// 头像 【max:255】
	#[serde(rename = "profile", deserialize_with = "check_length_profile")]
	pub profile: String,
	/// 介绍 【max:255】
	#[serde(rename = "introduce", deserialize_with = "check_length_introduce")]
	pub introduce: String,
	/// mk_页脚 【max:65535】
	#[serde(rename = "mkFooter", deserialize_with = "check_length_mk_footer")]
	pub mk_footer: String,
	/// 联系邮箱 【max:55】
	#[serde(rename = "contactMail", deserialize_with = "check_length_contact_mail")]
	pub contact_mail: String,
}

plier::create_serde_string_length_checker!(check_length_pen_name, 0, 20);
plier::create_serde_string_length_checker!(check_length_user_name, 0, 20);
plier::create_serde_string_length_checker!(check_length_user_pwd, 0, 64);
plier::create_serde_string_length_checker!(check_length_google_auth_secret, 0, 256);
plier::create_serde_string_length_checker!(check_length_profile, 0, 255);
plier::create_serde_string_length_checker!(check_length_introduce, 0, 255);
plier::create_serde_string_length_checker!(check_length_mk_footer, 0, 65535);
plier::create_serde_string_length_checker!(check_length_contact_mail, 0, 55);

impl BaseModel for BlogAuthorModel {

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
			"pen_name" => self.pen_name.to_string(),
			"user_name" => self.user_name.to_string(),
			"user_pwd" => self.user_pwd.to_string(),
			"google_auth_secret" => self.google_auth_secret.to_string(),
			"profile" => self.profile.to_string(),
			"introduce" => self.introduce.to_string(),
			"mk_footer" => self.mk_footer.to_string(),
			"contact_mail" => self.contact_mail.to_string(),
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
			"pen_name" => self.pen_name.to_string(),
			"user_name" => self.user_name.to_string(),
			"user_pwd" => self.user_pwd.to_string(),
			"google_auth_secret" => self.google_auth_secret.to_string(),
			"profile" => self.profile.to_string(),
			"introduce" => self.introduce.to_string(),
			"mk_footer" => self.mk_footer.to_string(),
			"contact_mail" => self.contact_mail.to_string(),
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

