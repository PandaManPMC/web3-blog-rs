use serde::{Serialize, Deserialize};

///	BlogLabelJSONIn 文章标签
///	author: AT
///	since: 2024-04-30 17:53:58
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
struct BlogLabelJSONIn {
	/// search文章标签编号 【max:20】
	#[serde(rename = "id")]
	pub id: u64,
	/// search标签名称 【max:20】
	#[serde(rename = "labelName")]
	pub label_name: String,
	/// thing状态:1@可见;2@不可见 【max:3】
	#[serde(rename = "state")]
	pub state: u8,
}
