

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct FileUploadOut {
    /// 通行凭证
    #[serde(rename = "fileUrl")]
    pub file_url: String,

}