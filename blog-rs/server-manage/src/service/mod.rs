
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
