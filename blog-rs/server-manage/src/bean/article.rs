use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GetArticleLstIn {

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
