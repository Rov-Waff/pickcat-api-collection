//! Offline contract tests for the reading-session DTOs in
//! `src/dto/reading_session.rs`.

mod common;

use log::info;
use pickcat_api_collection::dto::reading_session::*;

#[test]
fn serializes_reading_batch_request() {
    common::init_logging();
    info!("serializing reading-batch request");
    let batch = serde_json::to_value(ReadingBatchDTO {
        topic_id: "01a0eb91-5d11-7fb0-b998-179add93620f".into(),
        elapsed_ms: 9589,
        visible_posts: vec![
            VisiblePostDTO {
                post_id: "p1".into(),
                visible_ms: 2184,
            },
            VisiblePostDTO {
                post_id: "p2".into(),
                visible_ms: 972,
            },
        ],
    })
    .unwrap();

    assert_eq!(batch["topicId"], "01a0eb91-5d11-7fb0-b998-179add93620f");
    assert_eq!(batch["elapsedMs"], 9589);
    assert_eq!(batch["visiblePosts"][0]["postId"], "p1");
    assert_eq!(batch["visiblePosts"][1]["visibleMs"], 972);
}

#[test]
fn parses_reading_batch_response() {
    common::init_logging();
    info!("parsing reading-batch response fixture");
    let response: ReadingBatchResponseDTO = serde_json::from_str(
        r#"{
            "acceptedElapsedMs": 3029,
            "familiarity": {
                "tracking": true,
                "familiarityStartedAt": "2026-10-02T08:47:49.552Z",
                "daysSinceEntranceExam": 0,
                "topicsEntered": 1,
                "postsRead": 16,
                "effectiveReadingSeconds": 3,
                "validVisitDays": 0
            }
        }"#,
    )
    .expect("reading batch response must deserialize");

    assert_eq!(response.accepted_elapsed_ms, 3029);
    assert_eq!(response.familiarity.posts_read, 16);
    assert_eq!(response.familiarity.effective_reading_seconds, 3);
}
