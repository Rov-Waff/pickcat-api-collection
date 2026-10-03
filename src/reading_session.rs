//! 阅读会话章节：阅读埋点上报。
//!
//! 参见 <https://pickcat-docs.xiaole6324.fun/reading-session.html>。

use crate::dto::reading_session::{ReadingBatchDTO, ReadingBatchResponseDTO, VisiblePostDTO};
use crate::{Error, PickcatAccound};

/// 阅读埋点，来自文档的「阅读会话」章节
/// (<https://pickcat-docs.xiaole6324.fun/reading-session.html>)。
pub trait ReadingSessionBehavior {
    /// `PUT /api/v1/reading-sessions/{sessionId}/batches/{batchNo}` —— 需要会话。
    ///
    /// `session_id` 由调用方生成；`batch_no` 为批次序号。成功返回 `201`。
    /// `visible_posts` 至少要有 1 项，否则服务端返回 `400 VALIDATION_FAILED`。
    fn report_reading_batch(
        &self,
        session_id: &str,
        batch_no: u32,
        topic_id: &str,
        elapsed_ms: u64,
        visible_posts: Vec<VisiblePostDTO>,
    ) -> impl std::future::Future<Output = Result<ReadingBatchResponseDTO, Error>> + Send;
}

impl ReadingSessionBehavior for PickcatAccound {
    async fn report_reading_batch(
        &self,
        session_id: &str,
        batch_no: u32,
        topic_id: &str,
        elapsed_ms: u64,
        visible_posts: Vec<VisiblePostDTO>,
    ) -> Result<ReadingBatchResponseDTO, Error> {
        let dto = ReadingBatchDTO {
            topic_id: topic_id.to_string(),
            elapsed_ms,
            visible_posts,
        };
        Ok(self
            .client
            .put(format!(
                "{}/api/v1/reading-sessions/{}/batches/{}",
                self.base_url, session_id, batch_no
            ))
            .json(&dto)
            .send()
            .await?
            .json::<ReadingBatchResponseDTO>()
            .await?)
    }
}
