
use std::sync::RwLock;
use axum::BoxError;
use once_cell::sync::OnceCell;
use plier::rds;
use serde::{Deserialize, Serialize};
use fred::prelude::*;

static RDS: OnceCell<RwLock<rds::RDS>> = OnceCell::new();

pub async fn initialize_global_object(r: rds::RDS) {
    let _ = RDS.set(RwLock::new(r));
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlogAuthor {
    /// search作者编号 【max:20】
    #[serde(rename = "id")]
    pub id: u64,
    /// 创建时间 【max:20】
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    /// search笔名 【max:20】
    #[serde(rename = "penName")]
    pub pen_name: String,
    /// search用户名 【max:20】
    #[serde(rename = "userName")]
    pub user_name: String,
}

// pub async fn get_user_by_token(user_token: String) -> Result<Option<BlogAuthor>, BoxError> {
//     let rd = RDS.get().expect("RDS is not initialized").read().unwrap();
//     let res = rd.get_string(&format!("UT:{}", user_token)).await?;
//
//     if "" == res {
//         return Ok(None);
//     }
//
//     let deserialized: BlogAuthor = serde_json::from_str(&res).unwrap();
//     return Ok(Some(deserialized));
// }

pub async fn get_user_by_token(user_token: String) -> Result<Option<BlogAuthor>, RedisError> {
    let rd = RDS.get().expect("RDS is not initialized").read().unwrap();
    let res = rd.get_string(&format!("UT:{}", user_token)).await;

    if res.is_err() {
        return Err(res.err().unwrap());
    }

    let r = res.unwrap();
    if "" == r {
        return Ok(None);
    }

    let deserialized: BlogAuthor = serde_json::from_str(&r).unwrap();
    return Ok(Some(deserialized));
}