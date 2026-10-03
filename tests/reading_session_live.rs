//! Live tests for the reading-session endpoint in `src/reading_session.rs`.

mod common;

use log::{info, warn};
use pickcat_api_collection::dto::reading_session::VisiblePostDTO;
use pickcat_api_collection::reading_session::ReadingSessionBehavior;
use pickcat_api_collection::topic::{TopicBehavior, generate_idempotency_key};

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_report_reading_batch() {
    common::init_logging();
    let account = common::account().await;

    info!("GET /api/v1/topics?limit=1 (to pick a topic)");
    let topics = account.list_topics(Some(1), &[], None, None).await.unwrap();
    let Some(topic) = topics.items.first() else {
        warn!("no topics returned; skipping reading report");
        return;
    };

    info!("GET /api/v1/topics/{}/posts?limit=1", topic.id);
    let posts = account
        .get_topic_posts(&topic.id, Some(1), None, None)
        .await
        .unwrap();
    let visible_posts: Vec<VisiblePostDTO> = posts
        .items
        .iter()
        .map(|post| VisiblePostDTO {
            post_id: post.id.clone(),
            visible_ms: 1000,
        })
        .collect();
    if visible_posts.is_empty() {
        warn!("no posts returned; skipping reading report");
        return;
    }

    // 前端生成的 session id；这里复用 UUID 生成器。
    let session_id = generate_idempotency_key();
    info!("PUT /api/v1/reading-sessions/{session_id}/batches/1");
    let response = account
        .report_reading_batch(&session_id, 1, &topic.id, 1500, visible_posts)
        .await
        .unwrap();
    info!(
        "accepted={}ms, postsRead={}, effectiveReadingSeconds={}",
        response.accepted_elapsed_ms,
        response.familiarity.posts_read,
        response.familiarity.effective_reading_seconds
    );

    assert!(response.accepted_elapsed_ms <= 1500);
}
