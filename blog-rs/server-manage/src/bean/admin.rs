use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct LoginIn {
    /// search用户名 【max:20】
    #[serde(rename = "userName")]
    pub user_name: String,
    /// 密码 【max:64】
    #[serde(rename = "userPwd")]
    pub user_pwd: String,
    /// 谷歌验证器 【max:64】
    #[serde(rename = "googleAuthCode")]
    pub google_auth_code: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct LoginOut {
    /// search作者编号 【max:20】
    #[serde(rename = "id")]
    pub id: u64,
    /// search笔名 【max:20】
    #[serde(rename = "penName")]
    pub pen_name: String,
}