//! Mock-server tests for `src/topic.rs`.

mod common;

use serde_json::json;
use wiremock::matchers::{body_json, header, header_exists, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use pickcat_api_collection::topic::TopicBehavior;

const EMPTY_PAGE: &str =
    r#"{ "items": [], "pageInfo": { "hasNextPage": false, "nextCursor": null } }"#;

#[tokio::test]
async fn list_topics_repeats_tag_params() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/topics"))
        .and(query_param("limit", "20"))
        .and(query_param("tag", "interest-plaza"))
        .and(query_param("tag", "creative-works"))
        .and(query_param("tagMode", "all"))
        .respond_with(ResponseTemplate::new(200).set_body_string(EMPTY_PAGE))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let tags = vec!["interest-plaza".to_string(), "creative-works".to_string()];
    let topics = account
        .list_topics(Some(20), &tags, Some("all"), None)
        .await
        .unwrap();
    assert!(topics.items.is_empty());
    server.verify().await;
}

#[tokio::test]
async fn get_topic_recommendations_hits_recommendations() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/topic-recommendations"))
        .and(query_param("sort", "recommended"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "strategy": "PERSONALIZED",
            "items": [],
            "pageInfo": { "hasNextPage": false, "nextCursor": null }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let rec = account
        .get_topic_recommendations(Some("recommended"), Some(20), None)
        .await
        .unwrap();
    assert_eq!(rec.strategy, "PERSONALIZED");
    server.verify().await;
}

#[tokio::test]
async fn get_topic_hits_topic_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/topics/t1"))
        .respond_with(ResponseTemplate::new(200).set_body_string(TOPIC_DETAIL))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let topic = account.get_topic("t1").await.unwrap();
    assert_eq!(topic.id, "t1");
    assert_eq!(topic.first_post.post_number, 1);
    server.verify().await;
}

#[tokio::test]
async fn get_topic_posts_sends_sort_and_limit() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/topics/t1/posts"))
        .and(query_param("limit", "50"))
        .and(query_param("sort", "hot"))
        .respond_with(ResponseTemplate::new(200).set_body_string(EMPTY_PAGE))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let posts = account
        .get_topic_posts("t1", Some(50), Some("hot"), None)
        .await
        .unwrap();
    assert!(posts.items.is_empty());
    server.verify().await;
}

#[tokio::test]
async fn create_topic_sends_idempotency_key_origin_and_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v1/posts"))
        .and(header("idempotency-key", "key-1"))
        .and(header_exists("origin"))
        .and(body_json(json!({
            "title": "标题",
            "markdown": "正文",
            "kind": "DISCUSSION",
            "tagIds": ["t1"]
        })))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "submissionId": "s1", "topicId": "t1", "postId": "p1",
            "status": "PENDING_PROVIDER"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let created = account
        .create_topic(
            "key-1",
            "标题",
            "正文",
            "DISCUSSION",
            vec!["t1".to_string()],
        )
        .await
        .unwrap();
    assert_eq!(created.submission_id, "s1");
    assert_eq!(created.status, "PENDING_PROVIDER");
    server.verify().await;
}

#[tokio::test]
async fn create_reply_omits_reply_to_when_none() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v1/posts"))
        .and(header("idempotency-key", "key-2"))
        .and(body_json(json!({ "topicId": "t1", "markdown": "合影" })))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "submissionId": "s2", "topicId": "t1", "postId": "p2",
            "status": "PENDING_PROVIDER"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let created = account
        .create_reply("key-2", "t1", "合影", None)
        .await
        .unwrap();
    assert_eq!(created.topic_id, "t1");
    server.verify().await;
}

#[tokio::test]
async fn list_post_submissions_sends_filters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/post-submissions"))
        .and(query_param("contentRole", "TOPIC_REPLY"))
        .and(query_param("status", "PENDING"))
        .and(query_param("limit", "100"))
        .respond_with(ResponseTemplate::new(200).set_body_string(EMPTY_PAGE))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let submissions = account
        .list_post_submissions(Some("TOPIC_REPLY"), Some("PENDING"), None, Some(100), None)
        .await
        .unwrap();
    assert!(submissions.items.is_empty());
    server.verify().await;
}

const TOPIC_DETAIL: &str = r#"{
    "id": "t1", "title": "标题", "kind": "DISCUSSION",
    "author": { "id": "u1", "username": "a", "avatar": { "type": "PRESET", "id": 1, "url": "/x" }, "displayedBadge": null },
    "tags": [],
    "replyCount": 0, "viewCount": 0, "likeCount": 0, "bookmarkCount": 0,
    "collection": null, "closedAt": null, "closedBy": null,
    "pinned": false, "pinnedGlobally": false, "pinnedTagId": null,
    "pinnedAt": null, "pinnedUntil": null,
    "lastActivityAt": "2026-10-02T07:04:32.485Z",
    "createdAt": "2026-09-29T05:09:27.428Z",
    "editedAt": null,
    "updatedAt": "2026-10-02T07:04:32.485Z",
    "firstPost": {
        "id": "p1", "topicId": "t1", "postNumber": 1, "replyToPostNumber": null,
        "deleted": false, "children": [], "currentRevision": 1, "likeCount": 0,
        "pinned": false, "cookedHtml": "<p>...</p>",
        "author": { "id": "u1", "username": "a", "avatar": { "type": "PRESET", "id": 1, "url": "/x" }, "displayedBadge": null },
        "createdAt": "2026-09-29T05:09:27.428Z", "editedAt": null,
        "viewerCapabilities": { "canEdit": false, "canLike": true, "canBookmark": true, "canReport": true, "canSelectAnswer": false },
        "viewerState": { "liked": false, "bookmarkId": null, "selectedAnswer": false }
    },
    "repliesTruncated": false,
    "events": [],
    "eventsTruncated": false,
    "viewerCapabilities": {
        "canEdit": false, "canReply": true, "canDelete": false, "canBookmark": true,
        "canClose": false, "canReopen": false, "canPinGlobally": false, "pinnableTagIds": []
    },
    "viewerState": { "bookmarkId": null }
}"#;
