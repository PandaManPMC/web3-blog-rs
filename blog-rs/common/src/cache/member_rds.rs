
use std::sync::RwLock;
use axum::BoxError;
use once_cell::sync::OnceCell;
use plier::rds;
use serde::{Deserialize, Serialize};
use fred::prelude::*;
use log::error;
use::std::collections::HashMap;
use std::sync::Arc;

static RDS: OnceCell<RwLock<rds::RDS>> = OnceCell::new();

pub async fn initialize_global_object(r: rds::RDS) {
    let _ = RDS.set(RwLock::new(r));
}

lazy_static::lazy_static! {
    static ref USERCACHE: RwLock<HashMap<String, base::model::blog_author::BlogAuthorModel>> = RwLock::new({
        let map = HashMap::new();
        map
    });
}

fn set_user_cache(user_token :String, user : base::model::blog_author::BlogAuthorModel) {
    let mut mw = USERCACHE.write().unwrap();
    mw.insert(user_token, user);
}

fn get_user_cache(user_token :String) -> Option<base::model::blog_author::BlogAuthorModel> {
    let mut mr = USERCACHE.read().unwrap();
    let ds = mr.get(&user_token);

    if ds.is_none(){
        return None;
    }

    let d = ds.unwrap();
    return Some(d.clone());
}

pub fn get_user_by_ut(user_token: String) -> Option<base::model::blog_author::BlogAuthorModel> {
    return get_user_cache(user_token);
}

pub fn set_user_by_ut(user_token: String, u : base::model::blog_author::BlogAuthorModel) {
    set_user_cache(user_token, u);
}

pub async fn get_user_by_token(user_token: String) -> Result<Option<base::model::blog_author::BlogAuthorModel>, RedisError> {
    let rd = RDS.get().expect("RDS is not initialized").read().unwrap();
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