use serde::{Serialize, Deserialize};

///	BlogViewJSONIn 评论
///	author: AT
///	since: 2024-05-01 16:29:19
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogViewJSONIn {
	/// search评论编号 【max:20】
	#[serde(rename = "id")]
	pub id: u64,
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
