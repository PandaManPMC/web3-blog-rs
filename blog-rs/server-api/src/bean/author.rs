use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct AuthorOut {
    /// search笔名 【max:20】
    #[serde(rename = "penName")]
    pub pen_name: String,
    /// 头像 【max:255】
    #[serde(rename = "profile")]
    pub profile: String,
    /// 介绍 【max:255】
    #[serde(rename = "introduce")]
    pub introduce: String,
}