use serde::{Deserialize, Serialize, de};
use serde::de::Unexpected;
use serde::Deserializer;
use serde::de::{Visitor};
use std::fmt;
use serde::de::MapAccess;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PublishIn {

    /// 文章类型 【max:20】
    #[serde(rename = "idBlogClasses")]
    pub id_blog_classes: u64,

    /// search文章标题 【max:50】
    #[serde(rename = "titleArticle", deserialize_with = "check_length_title_article")]
    pub title_article: String,

    /// thing发布状态:1@未发布;2@已发布 【max:3】
    #[serde(rename = "statePublish")]
    pub state_publish: u8,

    /// thing文章可见性:1@私有;2@公开 【max:3】
    #[serde(rename = "statePrivate")]
    pub state_private: u8,

    /// 文章内容 【max:2147483647】
    #[serde(rename = "content", deserialize_with = "check_length_content")]
    pub content: String,

    /// 顺序 【max:10】
    #[serde(default)]
    #[serde(rename = "sequence")]
    pub sequence: u32,
}

plier::create_serde_string_length_checker!(check_length_title_article, 0, 50);
plier::create_serde_string_length_checker!(check_length_content, 0, 2147483647);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct CreateClassesIN {
    /// search类型名称 【max:20】
    #[serde(rename = "classesName", deserialize_with="check_length_classes_name")]
    pub classes_name: String,
    /// 顺序 【max:10】
    #[serde(rename = "sequence")]
    pub sequence: u32,
}

plier::create_serde_string_length_checker!(check_length_classes_name, 0, 20);


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GetArticleLstIn {
    /// 文章类型 【max:20】
    #[serde(default)]
    #[serde(rename = "idBlogClasses")]
    pub id_blog_classes: u64,

    /// thing状态:1@正常;2@已删除 【max:3】
    #[serde(default)]
    #[serde(rename = "stateArticle")]
    pub state_article: u8,

    /// thing发布状态:1@未发布;2@已发布 【max:3】
    #[serde(default)]
    #[serde(rename = "statePublish")]
    pub state_publish: u8,

    /// thing文章可见性:1@私有;2@公开 【max:3】
    #[serde(default)]
    #[serde(rename = "statePrivate")]
    pub state_private: u8,

    #[serde(rename = "pageIndex")]
    pub page_index: i64,

    #[serde(rename = "pageSize")]
    pub page_size: i64,

}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GetClassesLstIn {
    /// thing状态:1@可见;2@不可见 【max:3】
    #[serde(default)]
    #[serde(rename = "state")]
    pub state: u8,

    #[serde(rename = "pageIndex")]
    pub page_index: i64,

    #[serde(rename = "pageSize")]
    pub page_size: i64,

}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GetLabelLstIn {

    /// thing状态:1@可见;2@不可见 【max:3】
    #[serde(rename = "state")]
    #[serde(default)]
    pub state: u8,

    #[serde(rename = "pageIndex")]
    pub page_index: i64,

    #[serde(rename = "pageSize")]
    pub page_size: i64,

}