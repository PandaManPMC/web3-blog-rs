
use tokio::sync::RwLock;
use tokio::sync::OnceCell;
use plier::rds;
use fred::prelude::*;
use std::sync::Arc;

static RDS: OnceCell<Arc<RwLock<rds::RDS>>> = OnceCell::const_new();

pub async fn initialize_global_object(r: rds::RDS) {
    let _ = RDS.set(Arc::new(RwLock::new(r)));
}

pub async fn set_user_by_token(user_token: String, u : base::model::blog_author::BlogAuthorModel) -> Result<(), RedisError> {
    let rw = RDS.get().unwrap().write().await;
    let j = serde_json::to_string(&u).unwrap();
    rw.set_expire(&format!("UT:{}", user_token), j, 60000).await
}

pub async fn get_user_by_token(user_token: String) -> Result<Option<base::model::blog_author::BlogAuthorModel>, RedisError> {
    let rd = RDS.get().unwrap().read().await;
    let res = rd.get_string(&format!("UT:{}", user_token)).await;
    if res.is_err() {
        return Err(res.err().unwrap());
    }
    let r = res.unwrap();
    if "" == r {
        return Ok(None);
    }
    let deserialized: base::model::blog_author::BlogAuthorModel = serde_json::from_str(&r).unwrap();
    return Ok(Some(deserialized));
}

pub async fn set_user_secret(user_name: String, secret :String) -> Result<(), RedisError> {
    let rw = RDS.get().unwrap().write().await;
    rw.set_expire(&format!("secret:{}", user_name), secret, 120000).await
}
