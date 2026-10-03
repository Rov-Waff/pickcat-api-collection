use serde::{Deserialize, Serialize};

use super::user::CursorListDTO;

/// `POST /api/v1/files` 成功（201）后返回的已上传文件。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadedFileDTO {
    pub id: String,
    pub ownership: String,
    pub uploaded_by_user_id: String,
}

/// 我的文件列表；列表项结构未文档化。
pub type FileListDTO = CursorListDTO<serde_json::Value>;
