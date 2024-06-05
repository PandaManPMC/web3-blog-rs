use serde::{Deserialize, Serialize, de};
use serde::de::Unexpected;
use serde::Deserializer;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct LoginIn {
    /// search用户名 【max:20】
    #[serde(rename = "userName", deserialize_with = "check_length_user_name")]
    pub user_name: String,
    /// 密码 【max:64】
    #[serde(rename = "userPwd")]
    pub user_pwd: String,
    /// 谷歌验证器 【max:64】
    #[serde(default)]
    #[serde(rename = "googleAuthCode")]
    pub google_auth_code: String,
}

fn check_length_user_name<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.len() <= 20 {
        Ok(s)
    } else {
        Err(de::Error::invalid_value(Unexpected::Str(&s), &"a string with at most 20 characters"))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct LoginOut {
    /// search作者编号 【max:20】
    #[serde(rename = "id")]
    pub id: u64,
    /// search笔名 【max:20】
    #[serde(rename = "penName")]
    pub pen_name: String,
    /// 同行凭证
    #[serde(rename = "userToken")]
    pub user_token: String,
}