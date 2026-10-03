//! 阅读会话 DTO。

use serde::{Deserialize, Serialize};

use super::user::LevelFamiliarityDTO;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisiblePostDTO {
    pub post_id: String,
    pub visible_ms: u64,
}

/// `PUT /api/v1/reading-sessions/{sessionId}/batches/{batchNo}` 的请求体。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadingBatchDTO {
    pub topic_id: String,
    pub elapsed_ms: u64,
    /// 至少 1 项，否则服务端返回 `400 VALIDATION_FAILED`。
    pub visible_posts: Vec<VisiblePostDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadingBatchResponseDTO {
    pub accepted_elapsed_ms: u64,
    pub familiarity: LevelFamiliarityDTO,
}
