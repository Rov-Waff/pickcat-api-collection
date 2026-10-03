//! Offline contract tests for the topic/content DTOs in `src/dto/topic.rs`.
//!
//! Fixtures mirror the payloads in
//! <https://pickcat-docs.xiaole6324.fun/topic.html>. No network or credentials
//! are required.

mod common;

use log::{debug, info};
use pickcat_api_collection::dto::topic::*;
use pickcat_api_collection::dto::user::CursorListDTO;
use pickcat_api_collection::topic::generate_idempotency_key;

/// 主题列表项（作者为精简的 `TopicAuthorDTO`）。
const TOPIC_ITEM: &str = r#"{
    "id": "01a0e145-131b-708a-bfd2-051c68c7cf24",
    "title": "做了一个联机小测试，欢迎大家体验",
    "kind": "DISCUSSION",
    "excerpt": "https://player.codemao.cn/new/327450447",
    "author": {
        "id": "u1", "username": "朗kea9",
        "avatar": { "type": "CUSTOM", "id": 7, "url": "/api/v1/avatars/7" },
        "displayedBadge": null
    },
    "tags": [ { "id": "t1", "slug": "creative-works", "name": "创作与作品" } ],
    "replyCount": 0, "viewCount": 5, "likeCount": 0, "bookmarkCount": 0,
    "closedAt": null,
    "pinned": false, "pinnedGlobally": false, "pinnedTagId": null,
    "pinnedAt": null, "pinnedUntil": null,
    "createdAt": "2026-09-27T05:09:55.600Z",
    "editedAt": null,
    "lastActivityAt": "2026-09-27T05:10:12.121Z"
}"#;

#[test]
fn parses_topic_list_payload() {
    common::init_logging();
    info!("parsing topic list fixture");
    let list: CursorListDTO<TopicListItemDTO> = serde_json::from_str(&format!(
        r#"{{ "items": [{TOPIC_ITEM}], "pageInfo": {{ "hasNextPage": false, "nextCursor": null }} }}"#
    ))
    .expect("topic list payload must deserialize");

    assert_eq!(list.items.len(), 1);
    assert_eq!(list.items[0].kind, "DISCUSSION");
    assert_eq!(list.items[0].author.username, "朗kea9");
    assert!(list.items[0].author.displayed_badge.is_none());
    assert!(list.items[0].question_state.is_none());
    assert!(!list.page_info.has_next_page);
}

#[test]
fn parses_topic_recommendations_payload() {
    common::init_logging();
    info!("parsing topic recommendations fixture");
    // 推荐项是主题列表项 + reason，用 serde(flatten) 合并。
    let mut item: serde_json::Value = serde_json::from_str(TOPIC_ITEM).unwrap();
    item["reason"] = serde_json::json!("PINNED");

    let payload = serde_json::json!({
        "strategy": "PERSONALIZED",
        "items": [item],
        "pageInfo": { "hasNextPage": true, "nextCursor": "cursor-1" }
    });
    let rec: TopicRecommendationsDTO =
        serde_json::from_value(payload).expect("recommendations payload must deserialize");

    debug!("parsed recommendations: {rec:?}");
    assert_eq!(rec.strategy, "PERSONALIZED");
    assert_eq!(rec.items.len(), 1);
    assert_eq!(rec.items[0].reason, "PINNED");
    assert_eq!(rec.items[0].topic.title, "做了一个联机小测试，欢迎大家体验");
    assert!(rec.page_info.has_next_page);
}

#[test]
fn parses_topic_detail_payload() {
    common::init_logging();
    info!("parsing topic detail fixture");
    let topic: TopicDetailDTO = serde_json::from_str(
        r#"{
            "id": "01a0eb91-5d11-7fb0-b998-179add93620f",
            "title": "【共琢一轮月】……获奖名单公示",
            "kind": "ANNOUNCEMENT",
            "author": { "id": "u1", "username": "hajimes", "avatar": { "type": "PRESET", "id": 1, "url": "/x" }, "displayedBadge": null },
            "tags": [ { "id": "t1", "slug": "events-competitions", "name": "活动与赛事" } ],
            "replyCount": 24, "viewCount": 96, "likeCount": 0, "bookmarkCount": 0,
            "collection": null,
            "closedAt": null, "closedBy": null,
            "pinned": true, "pinnedGlobally": true, "pinnedTagId": null,
            "pinnedAt": "2026-09-29T08:33:47.233Z", "pinnedUntil": null,
            "lastActivityAt": "2026-10-02T07:04:32.485Z",
            "createdAt": "2026-09-29T05:09:27.428Z",
            "editedAt": "2026-09-29T09:59:19.985Z",
            "updatedAt": "2026-10-02T07:04:32.485Z",
            "firstPost": {
                "id": "p1", "topicId": "01a0eb91-5d11-7fb0-b998-179add93620f", "postNumber": 1, "replyToPostNumber": null,
                "deleted": false, "children": [], "currentRevision": 2, "likeCount": 4,
                "pinned": false, "cookedHtml": "<p>...</p>",
                "author": { "id": "u1", "username": "hajimes", "avatar": { "type": "PRESET", "id": 1, "url": "/x" }, "displayedBadge": null },
                "createdAt": "2026-09-29T05:09:27.428Z", "editedAt": "2026-09-29T09:59:19.985Z",
                "viewerCapabilities": { "canEdit": false, "canLike": true, "canBookmark": true, "canReport": true, "canSelectAnswer": false },
                "viewerState": { "liked": false, "bookmarkId": null, "selectedAnswer": false }
            },
            "repliesTruncated": true,
            "events": [],
            "eventsTruncated": false,
            "viewerCapabilities": {
                "canEdit": false, "canReply": true, "canDelete": false, "canBookmark": true,
                "canClose": false, "canReopen": false, "canPinGlobally": false, "pinnableTagIds": []
            },
            "viewerState": { "bookmarkId": null }
        }"#,
    )
    .expect("topic detail payload must deserialize");

    assert_eq!(topic.kind, "ANNOUNCEMENT");
    assert_eq!(topic.author.username, "hajimes");
    assert!(topic.pinned && topic.pinned_globally);
    assert_eq!(topic.first_post.post_number, 1);
    assert!(topic.first_post.viewer_capabilities.can_like);
    assert!(topic.replies_truncated);
    assert!(topic.viewer_capabilities.can_reply);
    assert!(topic.viewer_state.bookmark_id.is_none());
}

#[test]
fn parses_topic_posts_payload() {
    common::init_logging();
    info!("parsing topic posts fixture");
    let posts: CursorListDTO<TopicPostDTO> = serde_json::from_str(
        r#"{
            "items": [
                {
                    "id": "p8", "topicId": "t1", "postNumber": 8, "replyToPostNumber": 1,
                    "deleted": false, "children": [], "currentRevision": 1, "likeCount": 2,
                    "pinned": false, "cookedHtml": "<p>获奖了哈哈哈</p>",
                    "author": { "id": "u2", "username": "member", "avatar": { "type": "PRESET", "id": 2, "url": "/y" }, "displayedBadge": null },
                    "createdAt": "2026-10-01T00:00:00.000Z", "editedAt": null,
                    "viewerCapabilities": { "canEdit": false, "canLike": true, "canBookmark": true, "canReport": true, "canSelectAnswer": false },
                    "viewerState": { "liked": false, "bookmarkId": null, "selectedAnswer": false }
                }
            ],
            "pageInfo": { "hasNextPage": false, "nextCursor": null }
        }"#,
    )
    .expect("topic posts payload must deserialize");

    assert_eq!(posts.items.len(), 1);
    assert_eq!(posts.items[0].post_number, 8);
    assert_eq!(posts.items[0].reply_to_post_number, Some(1));
    assert_eq!(posts.items[0].author.username, "member");
}

#[test]
fn serializes_create_payloads() {
    common::init_logging();
    info!("serializing create-topic / create-reply requests");

    let topic = serde_json::to_value(CreateTopicDTO {
        title: "标题".into(),
        markdown: "正文 ![01a0ff24-...]".into(),
        kind: "DISCUSSION".into(),
        tag_ids: vec!["01a0ab26-84a8-718b-996a-3960a4789d05".into()],
    })
    .unwrap();
    assert_eq!(
        topic,
        serde_json::json!({
            "title": "标题",
            "markdown": "正文 ![01a0ff24-...]",
            "kind": "DISCUSSION",
            "tagIds": ["01a0ab26-84a8-718b-996a-3960a4789d05"]
        })
    );

    let reply_without_parent = serde_json::to_value(CreateReplyDTO {
        topic_id: "01a0c95a-...".into(),
        markdown: "合影".into(),
        reply_to_post_number: None,
    })
    .unwrap();
    assert_eq!(
        reply_without_parent,
        serde_json::json!({ "topicId": "01a0c95a-...", "markdown": "合影" })
    );

    let reply_with_parent = serde_json::to_value(CreateReplyDTO {
        topic_id: "01a0c95a-...".into(),
        markdown: "合影".into(),
        reply_to_post_number: Some(1),
    })
    .unwrap();
    assert_eq!(reply_with_parent["replyToPostNumber"], 1);
}

#[test]
fn parses_create_and_submission_payloads() {
    common::init_logging();
    info!("parsing post create + submission fixtures");

    let created: CreatePostResponseDTO = serde_json::from_str(
        r#"{
            "submissionId": "01a0ff25-1f05-7f09-8d1b-a32e2b0bab00",
            "topicId": "01a0ff25-1f07-77f3-beb8-0dabe6dba5f3",
            "postId": "01a0ff25-1f0a-7ec1-890d-86612754c302",
            "status": "PENDING_PROVIDER"
        }"#,
    )
    .expect("create post response must deserialize");
    assert_eq!(created.status, "PENDING_PROVIDER");
    assert!(!created.submission_id.is_empty());

    let submissions: CursorListDTO<PostSubmissionDTO> = serde_json::from_str(
        r#"{
            "items": [
                {
                    "id": "01a0ff25-1f05-7f09-8d1b-a32e2b0bab00",
                    "status": "PENDING_PROVIDER",
                    "contentRole": "TOPIC_FIRST_POST",
                    "request": { "kind": "DISCUSSION", "title": "...", "tagIds": ["..."], "markdown": "..." },
                    "riskLevel": null,
                    "topicId": "01a0ff25-1f07-77f3-beb8-0dabe6dba5f3",
                    "postId": "01a0ff25-1f0a-7ec1-890d-86612754c302",
                    "postNumber": 1,
                    "baseRevision": null,
                    "createdAt": "2026-10-03T00:39:14.613Z",
                    "updatedAt": "2026-10-03T00:39:14.613Z"
                }
            ],
            "pageInfo": { "hasNextPage": false, "nextCursor": null }
        }"#,
    )
    .expect("submissions payload must deserialize");
    assert_eq!(submissions.items.len(), 1);
    assert_eq!(submissions.items[0].content_role, "TOPIC_FIRST_POST");
    assert!(submissions.items[0].risk_level.is_none());
    assert_eq!(submissions.items[0].post_number, 1);
}

#[test]
fn generates_uuid_v4_style_idempotency_keys() {
    common::init_logging();
    info!("generating idempotency keys");

    let first = generate_idempotency_key();
    let second = generate_idempotency_key();
    debug!("keys: {first} / {second}");

    let parts: Vec<&str> = first.split('-').collect();
    assert_eq!(parts.len(), 5);
    assert_eq!(
        parts.iter().map(|p| p.len()).collect::<Vec<_>>(),
        vec![8, 4, 4, 4, 12]
    );
    assert_eq!(&parts[2][0..1], "4");
    assert!(matches!(&parts[3][0..1], "8" | "9" | "a" | "b"));
    assert!(first.chars().all(|c| c.is_ascii_hexdigit() || c == '-'));
    assert_ne!(first, second);
}
