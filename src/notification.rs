//! 通知章节（文档目前只记录了未读汇总接口）。
//!
//! 参见 <https://pickcat-docs.xiaole6324.fun/notification.html>。

use crate::dto::notification::NotificationSummaryDTO;
use crate::{Error, PickcatAccound};

/// 通知接口，来自文档的「通知」章节
/// (<https://pickcat-docs.xiaole6324.fun/notification.html>)。
///
/// 文档只记录了未读汇总接口；通知列表 / 已读接口尚未公开。
pub trait NotificationBehavior {
    /// `GET /api/v1/notification-summary` —— 需要会话，首页轮询用。
    fn get_notification_summary(
        &self,
    ) -> impl std::future::Future<Output = Result<NotificationSummaryDTO, Error>> + Send;
}

impl NotificationBehavior for PickcatAccound {
    async fn get_notification_summary(&self) -> Result<NotificationSummaryDTO, Error> {
        Ok(self
            .client
            .get(format!("{}/api/v1/notification-summary", self.base_url))
            .send()
            .await?
            .json::<NotificationSummaryDTO>()
            .await?)
    }
}
