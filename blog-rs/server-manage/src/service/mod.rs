pub mod author;

use tokio::sync::OnceCell;
use std::sync::Arc;
use tokio::sync::RwLock;




// static DATA_SOURCE_KEY: OnceCell<Arc<RwLock<String>>> = OnceCell::const_new();
//
// pub async fn initialize_global_object(s : String) {
//     let _ = DATA_SOURCE_KEY.set(Arc::new(RwLock::new(s)));
// }
//
// pub async fn get_data_source_key() -> String {
//      let a = DATA_SOURCE_KEY.get().unwrap().read().await;
//     return a.to_string();
//     // return DATA_SOURCE_KEY.read().await.to_string();
// }
//
// pub async fn set_date_source_key(key: String) {
//     let mut w = DATA_SOURCE_KEY.get().unwrap().write().await;
//     *w = key;
// }



lazy_static::lazy_static! {
    /// DATA_SOURCE_KEY 数据源 key
    static ref DATA_SOURCE_KEY: Arc<RwLock<String>> = Arc::new(RwLock::new({
        String::from("mysql1")
    }));
}

pub async fn get_data_source_key() -> String {
    return DATA_SOURCE_KEY.read().await.to_string();
}

pub async fn set_date_source_key(key: String) {
    let mut w = DATA_SOURCE_KEY.write().await;
    *w = key;
}

