//! Mock-server tests for `src/tag.rs`, `src/notification.rs`,
//! `src/reading_session.rs` and `src/media.rs`.

mod common;

use serde_json::json;
use wiremock::matchers::{body_json, body_string_contains, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use pickcat_api_collection::dto::reading_session::VisiblePostDTO;
use pickcat_api_collection::media::MediaBehavior;
use pickcat_api_collection::notification::NotificationBehavior;
use pickcat_api_collection::reading_session::ReadingSessionBehavior;
use pickcat_api_collection::tag::TagBehavior;

#[tokio::test]
async fn list_tags_sends_assignable_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/tags"))
        .and(query_param("assignable", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [
                { "id": "t1", "slug": "creative-works", "name": "创作与作品", "description": "..." }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let tags = account.list_tags(Some(true)).await.unwrap();
    assert_eq!(tags.items[0].slug, "creative-works");
    server.verify().await;
}

#[tokio::test]
async fn get_tag_sidebar_links_hits_slug_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/tags/interest-plaza/sidebar-links"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "source": { "id": "s1", "slug": "interest-plaza", "name": "兴趣广场", "description": null },
            "items": []
        })))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let side = account
        .get_tag_sidebar_links("interest-plaza")
        .await
        .unwrap();
    assert_eq!(side.source.slug, "interest-plaza");
    server.verify().await;
}

#[tokio::test]
async fn get_notification_summary_hits_summary_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/notification-summary"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "unreadCount": 1,
            "unreadCountByType": { "POST_LIKED": 1 }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let summary = account.get_notification_summary().await.unwrap();
    assert_eq!(summary.unread_count, 1);
    server.verify().await;
}

#[tokio::test]
async fn report_reading_batch_puts_expected_body() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/api/v1/reading-sessions/sess-1/batches/3"))
        .and(body_json(json!({
            "topicId": "t1",
            "elapsedMs": 1500,
            "visiblePosts": [{ "postId": "p1", "visibleMs": 1000 }]
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "acceptedElapsedMs": 1000,
            "familiarity": {
                "tracking": true, "familiarityStartedAt": "2026-10-02T08:47:49.552Z",
                "daysSinceEntranceExam": 0, "topicsEntered": 1, "postsRead": 1,
                "effectiveReadingSeconds": 1, "validVisitDays": 0
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let response = account
        .report_reading_batch(
            "sess-1",
            3,
            "t1",
            1500,
            vec![VisiblePostDTO {
                post_id: "p1".to_string(),
                visible_ms: 1000,
            }],
        )
        .await
        .unwrap();
    assert_eq!(response.accepted_elapsed_ms, 1000);
    server.verify().await;
}

#[tokio::test]
async fn upload_file_sends_multipart_with_idempotency_key() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v1/files"))
        .and(header("idempotency-key", "key-3"))
        .and(body_string_contains("name=\"ownership\""))
        .and(body_string_contains("PERSONAL"))
        .and(body_string_contains("filename=\"cover.png\""))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": "f1", "ownership": "PERSONAL", "uploadedByUserId": "u1"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let uploaded = account
        .upload_file("key-3", "PERSONAL", "cover.png", vec![1, 2, 3])
        .await
        .unwrap();
    assert_eq!(uploaded.id, "f1");
    server.verify().await;
}

#[tokio::test]
async fn get_file_returns_raw_bytes() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/files/f1"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(vec![1, 2, 3, 4], "image/png"))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let bytes = account.get_file("f1").await.unwrap();
    assert_eq!(bytes, vec![1, 2, 3, 4]);
    server.verify().await;
}

#[tokio::test]
async fn get_avatar_sends_version_query_and_returns_bytes() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/avatars/1"))
        .and(query_param("v", "hash1"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(vec![9, 9], "image/png"))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let bytes = account.get_avatar("1", Some("hash1")).await.unwrap();
    assert_eq!(bytes.len(), 2);
    server.verify().await;
}

#[tokio::test]
async fn get_emoji_image_sends_emoji_read_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/emojis/gif_expression_funny/image"))
        .and(query_param("emoji-read", "r1"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(vec![7], "image/gif"))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let bytes = account
        .get_emoji_image("gif_expression_funny", Some("r1"))
        .await
        .unwrap();
    assert_eq!(bytes, vec![7]);
    server.verify().await;
}

#[tokio::test]
async fn list_files_sends_scope_and_state() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/files"))
        .and(query_param("scope", "OWN"))
        .and(query_param("state", "READY"))
        .and(query_param("limit", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "pageInfo": { "hasNextPage": false, "nextCursor": null },
            "items": []
        })))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let files = account
        .list_files(Some("OWN"), Some("READY"), Some(20), None)
        .await
        .unwrap();
    assert!(files.items.is_empty());
    server.verify().await;
}
