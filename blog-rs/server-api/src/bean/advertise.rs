use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct AdvertiseInfoOut {
    /// 链接 【max:255】
    #[serde(rename = "linkAdvertise")]
    pub link_advertise: String,
    /// 标题 【max:100】
    #[serde(rename = "title")]
    pub title: String,
    /// 介绍 【max:255】
    #[serde(rename = "introduce")]
    pub introduce: String,
    /// 图片 【max:255】
    #[serde(rename = "img1")]
    pub img1: String,
    /// 顺序 【max:10】
    #[serde(rename = "sequence")]
    pub sequence: u32,
}