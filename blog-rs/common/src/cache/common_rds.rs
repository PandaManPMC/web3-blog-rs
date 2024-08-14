
use tokio::sync::RwLock;
use tokio::sync::OnceCell;
use plier::rds;
use fred::prelude::*;
use std::sync::Arc;

static RDS: OnceCell<Arc<RwLock<rds::RDS>>> = OnceCell::const_new();

pub async fn initialize_global_object(r: rds::RDS) {
    let _ = RDS.set(Arc::new(RwLock::new(r)));
}

pub async fn set_string_expire(key: String, val :String, expire_second: i64) -> Result<(), RedisError> {
    let rw = RDS.get().unwrap().write().await;
    rw.set_expire(&format!("str:{}", key), val, expire_second).await
}

pub async fn set_string(key: String, val :String) -> Result<(), RedisError> {
    let rw = RDS.get().unwrap().write().await;
    rw.set(&format!("str:{}", key), val).await
}

pub async fn get_string(key: String) -> Result<String, RedisError> {
    let rd = RDS.get().unwrap().read().await;
    let res = rd.get_string(&format!("str:{}", key)).await;
    return res;
}

pub async fn del_string(key: String) -> Result<(), RedisError> {
    let rd = RDS.get().unwrap().read().await;
    let res = rd.del(&format!("str:{}", key)).await;
    return res;
}