use serde::{Serialize, Deserialize};

///	BlogArticleJSONIn 文章
///	author: AT
///	since: 2024-05-01 16:29:19
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogArticleJSONIn {
	/// search文章编号 【max:20】
	#[serde(rename = "id")]
	pub id: u64,
	/// 作者 【max:20】
	#[serde(rename = "idBlogAuthor")]
	pub id_blog_author: u64,
	/// 文章类型 【max:20】
	#[serde(rename = "idBlogClasses")]
	pub id_blog_classes: u64,
	/// search文章标题 【max:50】
	#[serde(rename = "titleArticle")]
	pub title_article: String,
	/// thing状态:1@正常;2@已删除 【max:3】
	#[serde(rename = "stateArticle")]
	pub state_article: u8,
	/// thing发布状态:1@未发布;2@已发布 【max:3】
	#[serde(rename = "statePublish")]
	pub state_publish: u8,
	/// thing文章可见性:1@私有;2@公开 【max:3】
	#[serde(rename = "statePrivate")]
	pub state_private: u8,
	/// 文章内容 【max:2147483647】
	#[serde(rename = "content")]
	pub content: String,
	/// 点赞 【max:10】
	#[serde(rename = "likeCount")]
	pub like_count: u32,
	/// 观看 【max:10】
	#[serde(rename = "watchCount")]
	pub watch_count: u32,
	/// 评论数量 【max:10】
	#[serde(rename = "viewCount")]
	pub view_count: u32,
	/// 发布时间 【max:20】
	#[serde(rename = "timePublish")]
	pub time_publish: u64,
	/// 顺序 【max:10】
	#[serde(rename = "sequence")]
	pub sequence: u32,
	/// 文章标签 【max:200】
	#[serde(rename = "labelList")]
	pub label_list: String,
}
