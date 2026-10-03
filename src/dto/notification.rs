use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// `GET /api/v1/notification-summary` 的未读汇总。
///
/// `unread_count_by_type` 用 map 承载，避免写死类型枚举；已知 key：
/// `REPLY_CREATED` / `MANAGEMENT_ACTION` / `POST_LIKED` / `TOPIC_EVENT` /
/// `USER_FOLLOWED` / `LEVEL_CERTIFICATION`。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSummaryDTO {
    pub unread_count: u64,
    pub unread_count_by_type: BTreeMap<String, u64>,
}
