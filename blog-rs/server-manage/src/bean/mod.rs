use serde::{Deserialize, Serialize};

pub mod admin;
pub mod article;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PkIn {
    /// id 编号
    #[serde(rename = "id")]
    pub id: u64,
}
