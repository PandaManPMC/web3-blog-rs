use serde::{Deserialize, Serialize, de};
use serde::de::Unexpected;
use serde::Deserializer;
use serde::de::{Visitor};
use std::fmt;
use serde::de::MapAccess;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct LoginIn {
    /// search用户名 【max:20】
    #[serde(rename = "userName", deserialize_with = "check_length_user_name")]
    pub user_name: String,
    /// 密码 【max:64】
    #[serde(rename = "userPwd", deserialize_with = "check_length_user_pwd")]
    pub user_pwd: String,
    /// 谷歌验证器 【max:64】
    #[serde(default)]
    #[serde(rename = "googleAuthCode")]
    pub google_auth_code: String,

    /// verify_re_captcha_token_v2 验证码 token
    #[serde(default)]
    #[serde(rename = "captchaToken")]
    pub captcha_token: String,
}

plier::create_serde_string_length_checker!(check_length_user_name, 6, 20);
plier::create_serde_string_length_checker!(check_length_user_pwd, 8, 64);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ChangePwdIn {
    /// 密码 【max:64】
    #[serde(rename = "userPwd", deserialize_with = "check_length_user_pwd")]
    pub user_pwd: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct LoginOut {
    /// search作者编号 【max:20】
    #[serde(rename = "id")]
    pub id: u64,
    /// search笔名 【max:20】
    #[serde(rename = "penName")]
    pub pen_name: String,
    /// 通行凭证
    #[serde(rename = "userToken")]
    pub user_token: String,
    /// true 已绑定google验证，false未绑定
    #[serde(rename = "googleAuth")]
    pub google_auth: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GetStartBindGoogleSecretOut {
    #[serde(rename = "secret")]
    pub secret: String,
    #[serde(rename = "qrCodeUrl")]
    pub qr_code_url: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BindGoogleSecretIn {
    #[serde(rename = "googleAuthCode", deserialize_with = "check_length_google_auth_code")]
    pub google_auth_code: String,
}

plier::create_serde_string_length_checker!(check_length_google_auth_code, 6, 6);
