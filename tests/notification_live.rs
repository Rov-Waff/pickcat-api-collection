//! Live tests for the notification endpoint in `src/notification.rs`.

mod common;

use log::info;
use pickcat_api_collection::notification::NotificationBehavior;

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_get_notification_summary() {
    common::init_logging();
    let account = common::account().await;

    info!("GET /api/v1/notification-summary");
    let summary = account.get_notification_summary().await.unwrap();
    info!(
        "unread={}, byType={:?}",
        summary.unread_count, summary.unread_count_by_type
    );

    for (kind, count) in &summary.unread_count_by_type {
        info!("  {kind}: {count}");
    }
}
