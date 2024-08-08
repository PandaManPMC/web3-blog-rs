
pub mod blog;
use std::collections::HashMap;


use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};
use base::model::blog_author::BlogAuthorModel;

lazy_static::lazy_static! {
    /// DATA_SOURCE_KEY 数据源 key
    static ref DATA_SOURCE_KEY: Arc<RwLock<String>> = Arc::new(RwLock::new({
        String::from("mysql1")
    }));

    /// 缓存标签, id -> label_name
    static ref LABEL_LIST: RwLock<HashMap<u64, String>> = RwLock::new(HashMap::new());
}

/// 缓存作者
pub static BLOG_AUTHOR: OnceCell<RwLock<BlogAuthorModel>> = OnceCell::const_new();

pub fn initialize_blog_author(author: BlogAuthorModel) {
    BLOG_AUTHOR.set(RwLock::new(author)).expect("initialize_blog_author error");
}

pub async fn get_author_pen_name() -> String {
    let author = BLOG_AUTHOR.get().expect("BLOG_AUTHOR should be initialized").read().await;
    return author.pen_name.clone();
}

pub async fn set_blog_author(author_new: BlogAuthorModel) {
    let mut author = BLOG_AUTHOR.get().expect("CONFIG should be initialized").write().await;
    author.pen_name = author_new.pen_name;
    author.updated_at = author_new.updated_at;
}

pub async fn get_data_source_key() -> String {
    return DATA_SOURCE_KEY.read().await.to_string();
}

pub async fn set_date_source_key(key: String) {
    let mut w = DATA_SOURCE_KEY.write().await;
    *w = key;
}
