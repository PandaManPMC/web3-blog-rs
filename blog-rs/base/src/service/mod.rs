pub mod blog_article_sve;
pub mod blog_author_sve;
pub mod blog_classes_sve;
pub mod blog_label_sve;
pub mod blog_view_sve;

// use tokio::sync::RwLock;
use tokio::sync::OnceCell;
use std::sync::Arc;
use std::sync::RwLock;

lazy_static::lazy_static! {
    /// DATA_SOURCE_KEY 数据源 key
    static ref DATA_SOURCE_KEY: RwLock<String> = RwLock::new({
        String::from("mysql1")
    });
}

pub fn get_data_source_key() -> String {
    return DATA_SOURCE_KEY.read().unwrap().to_string();
}

pub fn set_date_source_key(key: String) {
    let mut w = DATA_SOURCE_KEY.write().unwrap();
    *w = key;
}

// lazy_static::lazy_static! {
//     /// DATA_SOURCE_KEY 数据源 key
//     static ref DATA_SOURCE_KEY: Arc<RwLock<String>> = Arc::new(RwLock::new(String::from("mysql1")));
// }
//
// pub async fn get_data_source_key() -> String {
//     return DATA_SOURCE_KEY.read().await.to_string();
// }
//
// pub async fn set_date_source_key(key: String) {
//     let mut w = crate::service::DATA_SOURCE_KEY.write().await;
//     *w = key;
// }