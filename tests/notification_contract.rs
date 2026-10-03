//! Offline contract tests for the notification DTO in `src/dto/notification.rs`.

mod common;

use log::info;
use pickcat_api_collection::dto::notification::NotificationSummaryDTO;

#[test]
fn parses_notification_summary_payload() {
    common::init_logging();
    info!("parsing notification-summary fixture");
    let summary: NotificationSummaryDTO = serde_json::from_str(
        r#"{
            "unreadCount": 3,
            "unreadCountByType": {
                "REPLY_CREATED": 1,
                "MANAGEMENT_ACTION": 0,
                "POST_LIKED": 2,
                "TOPIC_EVENT": 0,
                "USER_FOLLOWED": 0,
                "LEVEL_CERTIFICATION": 0
            }
        }"#,
    )
    .expect("notification summary must deserialize");

    assert_eq!(summary.unread_count, 3);
    assert_eq!(summary.unread_count_by_type.get("POST_LIKED"), Some(&2));
    assert_eq!(summary.unread_count_by_type.get("REPLY_CREATED"), Some(&1));
    assert_eq!(summary.unread_count_by_type.len(), 6);
}
