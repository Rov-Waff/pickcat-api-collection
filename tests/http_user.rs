//! Mock-server tests for `src/user.rs`.

mod common;

use serde_json::json;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use pickcat_api_collection::user::UserProfileBehavior;

const USER_BODY: &str = r#"{
    "id": "u1", "username": "mock-user",
    "avatar": { "type": "PRESET", "id": 1, "url": "/api/v1/avatars/1" },
    "bio": null, "region": null,
    "showFollowingList": true, "showFollowersList": true,
    "createdAt": "2026-10-02T08:46:15.771Z",
    "level": { "current": 1 },
    "stats": { "followers": 0, "following": 0, "topics": 0, "replies": 0 },
    "viewerState": { "following": false, "canFollow": false }
}"#;

const EMPTY_PAGE: &str =
    r#"{ "items": [], "pageInfo": { "hasNextPage": false, "nextCursor": null } }"#;

#[tokio::test]
async fn get_user_information_hits_user_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/users/u1"))
        .respond_with(ResponseTemplate::new(200).set_body_string(USER_BODY))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let user = account.get_user_information("u1").await.unwrap();
    assert_eq!(user.username, "mock-user");
    server.verify().await;
}

#[tokio::test]
async fn update_user_profile_patches_only_provided_fields() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/api/v1/users/u1"))
        .and(body_json(json!({ "bio": "Acrb" })))
        .respond_with(ResponseTemplate::new(200).set_body_string(USER_BODY))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let user = account
        .update_user_profile("u1", Some("Acrb"))
        .await
        .unwrap();
    assert_eq!(user.id, "u1");
    server.verify().await;
}

#[tokio::test]
async fn get_user_topics_sends_limit_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/users/u1/topics"))
        .and(query_param("limit", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_string(EMPTY_PAGE))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let topics = account.get_user_topics("u1", Some(20)).await.unwrap();
    assert!(topics.items.is_empty());
    server.verify().await;
}

#[tokio::test]
async fn get_user_following_sends_limit_and_cursor() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/users/u1/following"))
        .and(query_param("limit", "50"))
        .and(query_param("cursor", "c1"))
        .respond_with(ResponseTemplate::new(200).set_body_string(EMPTY_PAGE))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let following = account
        .get_user_following("u1", Some(50), Some("c1"))
        .await
        .unwrap();
    assert!(!following.page_info.has_next_page);
    server.verify().await;
}

#[tokio::test]
async fn get_user_posts_sends_role_and_limit() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/users/u1/posts"))
        .and(query_param("role", "reply"))
        .and(query_param("limit", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_string(EMPTY_PAGE))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let posts = account
        .get_user_posts("u1", Some("reply"), Some(20))
        .await
        .unwrap();
    assert!(posts.items.is_empty());
    server.verify().await;
}

#[tokio::test]
async fn get_user_badge_displays_joins_user_ids() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/user-badge-displays"))
        .and(query_param("userIds", "u1,u2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [
                { "userId": "u1", "displayedBadge": null },
                { "userId": "u2", "displayedBadge": null }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let bulk = account
        .get_user_badge_displays(&["u1".to_string(), "u2".to_string()])
        .await
        .unwrap();
    assert_eq!(bulk.items.len(), 2);
    server.verify().await;
}

#[tokio::test]
async fn get_level_progress_hits_level_progress() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/level-progress"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "currentLevel": 1,
            "promotionCeiling": null,
            "scores": { "ability": 0, "responsibility": 0, "care": 0 },
            "familiarity": {
                "tracking": true, "familiarityStartedAt": "2026-10-02T08:47:49.552Z",
                "daysSinceEntranceExam": 0, "validVisitDays": 0,
                "topicsEntered": 1, "postsRead": 16, "effectiveReadingSeconds": 3
            },
            "nextLevel": null,
            "lv4Candidate": false,
            "quotas": [],
            "updating": false,
            "calculatedAt": "2026-10-02T08:49:43.287Z"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let progress = account.get_level_progress().await.unwrap();
    assert_eq!(progress.current_level, 1);
    assert!(progress.next_level.is_none());
    server.verify().await;
}
