use serde::{Deserialize, Serialize, de};
use serde::de::Unexpected;
use serde::Deserializer;
use serde::de::{Visitor};
use std::fmt;
use serde::de::MapAccess;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ReadIn {
    #[serde(rename = "id")]
    pub id: u64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct ReadOut {
    /// search文章编号 【max:20】
    #[serde(rename = "id")]
    pub id: u64,
    /// 文章类型 【max:20】
    #[serde(rename = "idBlogClasses")]
    pub id_blog_classes: u64,
    /// search文章标题 【max:50】
    #[serde(rename = "titleArticle")]
    pub title_article: String,
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

    /// 作者pem名
    #[serde(rename = "pemName")]
    pub pem_name: String,

    /// 头像 【max:255】
    #[serde(rename = "profile")]
    pub profile: String,

    /// 介绍 【max:255】
    #[serde(rename = "introduce")]
    pub introduce: String,

    /// mk_页脚 【max:65535】
    #[serde(rename = "mkFooter")]
    pub mk_footer: String,

    /// 标签
    #[serde(rename = "labels")]
    pub labels: Vec<String>,

    /// search类型名称 【max:20】
    #[serde(rename = "classesName")]
    pub classes_name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GetArticleList {

    /// 文章类型 【max:20】
    #[serde(rename = "idBlogClasses")]
    #[serde(default)]
    pub id_blog_classes: u64,

    /// 文章标签 【max:20】
    #[serde(rename = "idBlogLabel")]
    #[serde(default)]
    pub id_blog_label: u64,

    #[serde(rename = "pageIndex")]
    pub page_index: i64,

    #[serde(rename = "pageSize")]
    pub page_size: i64,

}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GetArticleComment {

    /// 文章编号 【max:20】
    #[serde(rename = "idBlog")]
    pub id_blog: u64,

    #[serde(rename = "pageIndex")]
    pub page_index: i64,

    #[serde(rename = "pageSize")]
    pub page_size: i64,

}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogArticleOut {
    /// search文章编号 【max:20】
    #[serde(rename = "id")]
    pub id: u64,
    /// 文章类型 【max:20】
    #[serde(rename = "idBlogClasses")]
    pub id_blog_classes: u64,
    /// search文章标题 【max:50】
    #[serde(rename = "titleArticle")]
    pub title_article: String,
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
    /// 作者pem名
    #[serde(rename = "pemName")]
    pub pem_name: String,
    #[serde(rename = "profile")]
    pub profile: String,
    /// 标签
    #[serde(rename = "labels")]
    pub labels: Vec<String>,

}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogLabelOut {
    /// search文章标签编号 【max:20】
    #[serde(rename = "id")]
    pub id: u64,
    /// search标签名称 【max:20】
    #[serde(rename = "labelName")]
    pub label_name: String,
    /// 顺序 【max:10】
    #[serde(rename = "sequence")]
    pub sequence: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogClassesOut {
    /// search文章类型编号 【max:20】
    #[serde(rename = "id")]
    pub id: u64,
    /// search类型名称 【max:20】
    #[serde(rename = "classesName")]
    pub classes_name: String,
    /// 顺序 【max:10】
    #[serde(rename = "sequence")]
    pub sequence: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GetViewListIn {
    /// 文章编号 【max:20】
    #[serde(rename = "idBlog")]
    pub id_blog: u64
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogViewOut {
    /// 创建时间 【max:20】
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    /// 评论内容 【max:200】
    #[serde(rename = "viewContent")]
    pub view_content: String,
    /// 代币符号 【max:20】
    #[serde(rename = "coinSymbol")]
    pub coin_symbol: String,
    /// 小费金额 【max:79】
    #[serde(rename = "tipAmount")]
    pub tip_amount: String,
    /// search地址 【max:155】
    #[serde(rename = "address")]
    pub address: String,
    /// 小费金额USD 【max:79】
    #[serde(rename = "tipAmountUsd")]
    pub tip_amount_usd: String,
    /// 票据 【max:64】
    #[serde(rename = "ticket")]
    pub ticket: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GetViewTicketIn {
    /// search地址 【max:155】
    #[serde(rename = "address" , deserialize_with = "check_length_address")]
    pub address: String,
}

plier::create_serde_string_length_checker!(check_length_address, 1, 155);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GetViewTicketOut {
    /// 票据 【max:64】
    #[serde(rename = "ticket")]
    pub ticket: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct CreateViewIn {
    /// 文章编号 【max:20】
    #[serde(rename = "idBlog")]
    pub id_blog: u64,

    /// 评论内容 【max:200】
    #[serde(rename = "viewContent")]
    pub view_content: String,

    /// 代币符号 【max:20】
    #[serde(rename = "coinSymbol")]
    pub coin_symbol: String,

    /// 票据 【max:64】
    #[serde(rename = "ticket")]
    pub ticket: String,
}
