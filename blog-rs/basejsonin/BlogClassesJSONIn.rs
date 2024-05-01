use serde::{Serialize, Deserialize};

///	BlogClassesJSONIn 文章类型
///	author: AT
///	since: 2024-05-01 16:29:19
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogClassesJSONIn {
	/// search文章类型编号 【max:20】
	#[serde(rename = "id")]
	pub id: u64,
	/// search类型名称 【max:20】
	#[serde(rename = "classesName")]
	pub classes_name: String,
	/// thing状态:1@可见;2@不可见 【max:3】
	#[serde(rename = "state")]
	pub state: u8,
}
