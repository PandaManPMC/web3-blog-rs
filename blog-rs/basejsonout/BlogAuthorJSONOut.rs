use serde::{Serialize, Deserialize};

///	BlogAuthorJSONOut 作者
///	author: AT
///	since: 2024-05-01 16:29:19
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
	/// 谷歌验证器 【max:64】 
	#[serde(rename = "googleAuthSecret")]
	pub google_auth_secret: String,
}
