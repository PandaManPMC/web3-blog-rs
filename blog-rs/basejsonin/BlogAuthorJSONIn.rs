use serde::{Serialize, Deserialize};

///	BlogAuthorJSONIn 作者
///	author: AT
///	since: 2024-04-30 17:53:58
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
struct BlogAuthorJSONIn {
	/// search作者编号 【max:20】
	#[serde(rename = "id")]
	pub id: u64,
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
