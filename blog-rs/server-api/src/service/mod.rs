
pub mod blog;
use std::sync::Mutex;
use std::collections::HashMap;


use std::sync::Arc;
use tokio::sync::RwLock;

lazy_static::lazy_static! {
    /// DATA_SOURCE_KEY 数据源 key
    static ref DATA_SOURCE_KEY: Arc<RwLock<String>> = Arc::new(RwLock::new({
        String::from("mysql1")
    }));
     /// 缓存作者 id -> pen_name
    static ref AUTHOR_LIST: Mutex<HashMap<u64, String>> = Mutex::new(HashMap::new());
    /// 缓存标签, id -> label_name
    static ref LABEL_LIST: Mutex<HashMap<u64, String>> = Mutex::new(HashMap::new());
}

pub async fn get_data_source_key() -> String {
    return DATA_SOURCE_KEY.read().await.to_string();
}

pub async fn set_date_source_key(key: String) {
    let mut w = DATA_SOURCE_KEY.write().await;
    *w = key;
}
