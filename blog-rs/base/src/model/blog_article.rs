use serde::{Serialize, Deserialize};
use r2d2_mysql::mysql::{params, Row};
use r2d2_mysql::mysql::params::Params;
use i_dao::model::BaseModel;
use i_dao_proc_macro::BaseModel;

pub const TABLE_NAME:&str = "blog_article";
pub const FIELDS:[&str;16] = ["id","created_at","updated_at","id_blog_author","id_blog_classes","title_article","state_article","state_publish","state_private","content","like_count","watch_count","view_count","time_publish","sequence","label_list"];
pub const ALIAS:&str = "blogArticle";

///	BlogArticleModel 文章
///	table - blog_article
///	author: AT
///	since: 2024-05-02 11:48:20
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogArticleModel {
	/// search文章编号 【max:20】 
	#[serde(rename = "id")]
	pub id: u64,
	/// 创建时间 【max:20】 
	#[serde(rename = "createdAt")]
	pub created_at: u64,
	/// 最后更新 【max:20】 
	#[serde(rename = "updatedAt")]
	pub updated_at: u64,
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

impl BlogArticleModel {

    pub fn new(id_blog_author: u64, id_blog_classes: u64, title_article: String, state_article: u8, state_publish: u8, state_private: u8, content: String, like_count: u32, watch_count: u32, view_count: u32, time_publish: u64, sequence: u32, label_list: String) -> BlogArticleModel {
        BlogArticleModel{id:0, created_at: 0, updated_at: 0, id_blog_author, id_blog_classes, title_article, state_article, state_publish, state_private, content, like_count, watch_count, view_count, time_publish, sequence, label_list}
    }

    pub fn new_full(id: u64, created_at: u64, updated_at: u64, id_blog_author: u64, id_blog_classes: u64, title_article: String, state_article: u8, state_publish: u8, state_private: u8, content: String, like_count: u32, watch_count: u32, view_count: u32, time_publish: u64, sequence: u32, label_list: String) -> BlogArticleModel {
        BlogArticleModel{id, created_at, updated_at, id_blog_author, id_blog_classes, title_article, state_article, state_publish, state_private, content, like_count, watch_count, view_count, time_publish, sequence, label_list}
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
pub fn pot(row: Row, offset: usize) -> BlogArticleModel {
	return BlogArticleModel::new_full(row.get(offset+0).unwrap(),row.get(offset+1).unwrap(),row.get(offset+2).unwrap(),row.get(offset+3).unwrap(),row.get(offset+4).unwrap(),row.get(offset+5).unwrap(),row.get(offset+6).unwrap(),row.get(offset+7).unwrap(),row.get(offset+8).unwrap(),row.get(offset+9).unwrap(),row.get(offset+10).unwrap(),row.get(offset+11).unwrap(),row.get(offset+12).unwrap(),row.get(offset+13).unwrap(),row.get(offset+14).unwrap(),row.get(offset+15).unwrap());
}

///	BlogArticleJSONOut 文章
///	author: AT
///	since: 2024-05-02 11:48:20
///	desc: base AT 2.1,incompatible < 2.1  https://at.pandamancoin.com
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogArticleJSONOut {
	/// search文章编号 【max:20】 
	#[serde(rename = "id")]
	pub id: u64,
	/// 创建时间 【max:20】 
	#[serde(rename = "createdAt")]
	pub created_at: u64,
	/// 最后更新 【max:20】 
	#[serde(rename = "updatedAt")]
	pub updated_at: u64,
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

///	BlogArticleJSONIn 文章
///	author: AT
///	since: 2024-05-02 11:48:20
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

impl BaseModel for BlogArticleModel {

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
			"id_blog_author" => self.id_blog_author,
			"id_blog_classes" => self.id_blog_classes,
			"title_article" => self.title_article.to_string(),
			"state_article" => self.state_article,
			"state_publish" => self.state_publish,
			"state_private" => self.state_private,
			"content" => self.content.to_string(),
			"like_count" => self.like_count,
			"watch_count" => self.watch_count,
			"view_count" => self.view_count,
			"time_publish" => self.time_publish,
			"sequence" => self.sequence,
			"label_list" => self.label_list.to_string(),
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
			"id_blog_author" => self.id_blog_author,
			"id_blog_classes" => self.id_blog_classes,
			"title_article" => self.title_article.to_string(),
			"state_article" => self.state_article,
			"state_publish" => self.state_publish,
			"state_private" => self.state_private,
			"content" => self.content.to_string(),
			"like_count" => self.like_count,
			"watch_count" => self.watch_count,
			"view_count" => self.view_count,
			"time_publish" => self.time_publish,
			"sequence" => self.sequence,
			"label_list" => self.label_list.to_string(),
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
