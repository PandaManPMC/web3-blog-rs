use serde::{Deserialize, Serialize};

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
    /// 作者pem名
    #[serde(rename = "pemName")]
    pub pem_name: String,
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
