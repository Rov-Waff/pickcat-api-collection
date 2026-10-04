//! Offline contract tests for the user-chapter DTOs in `src/dto/user.rs`.
//!
//! Fixtures mirror the payloads in
//! <https://pickcat-docs.xiaole6324.fun/user.html>. No network or credentials
//! are required.

mod common;

use log::{debug, info};
use pickcat_api_collection::dto::{IdValue, user::*};

/// A complete author object as returned by the API (the docs abbreviate it).
const AUTHOR: &str = r#"{
    "id": "01a0fbca-eee1-70ad-a00d-a1ed4c89b195",
    "username": "朗kea9",
    "avatar": { "type": "CUSTOM", "id": 7, "url": "/api/v1/avatars/7" },
    "bio": null,
    "region": null,
    "showFollowingList": true,
    "showFollowersList": true,
    "createdAt": "2026-09-27T05:09:55.600Z",
    "level": { "current": 2 },
    "stats": { "followers": 1, "following": 2, "topics": 3, "replies": 4 },
    "viewerState": { "following": false, "canFollow": true }
}"#;

#[test]
fn parses_user_information_payload() {
    common::init_logging();
    info!("parsing user-information payload fixture");
    let user: GetUserInformationDTO = serde_json::from_str(
        r#"{
            "id": "01a0fbca-eee1-70ad-a00d-a1ed4c89b195",
            "username": "CarbonPremium",
            "avatar": { "type": "PRESET", "id": 1, "url": "/api/v1/avatars/1?v=f558" },
            "bio": null,
            "region": null,
            "showFollowingList": true,
            "showFollowersList": true,
            "createdAt": "2026-10-02T08:46:15.771Z",
            "level": { "current": 1 },
            "stats": { "followers": 0, "following": 0, "topics": 0, "replies": 0 },
            "viewerState": { "following": false, "canFollow": false }
        }"#,
    )
    .expect("user information payload must deserialize");

    debug!("parsed user information: {user:?}");
    info!(
        "user {} Lv.{}: topics={}, replies={}",
        user.username, user.level.current, user.stats.topics, user.stats.replies
    );

    assert_eq!(user.username, "CarbonPremium");
    assert_eq!(user.avatar.avatar_type, "PRESET");
    assert_eq!(user.avatar.id, IdValue::Num(1));
    assert_eq!(user.level.current, 1);
    assert!(user.bio.is_none());
    assert!(!user.viewer_state.can_follow);
}

#[test]
fn parses_user_email_payload() {
    common::init_logging();
    info!("parsing user-email payload fixture");
    let email: GetUserEmailDTO = serde_json::from_str(
        r#"{ "email": "user@example.com", "verifiedAt": "2026-10-02T08:46:15.771Z" }"#,
    )
    .expect("user email payload must deserialize");

    info!("email={}, verifiedAt={}", email.email, email.verified_at);
    assert_eq!(email.email, "user@example.com");
    assert_eq!(email.verified_at, "2026-10-02T08:46:15.771Z");
}

#[test]
fn parses_user_topics_payload() {
    common::init_logging();
    info!("parsing user-topics payload fixture");
    let topics: UserSubjectsDTO = serde_json::from_str(&format!(
        r#"{{
            "items": [
                {{
                    "id": "01a0e145-131b-708a-bfd2-051c68c7cf24",
                    "title": "做了一个联机小测试，欢迎大家体验",
                    "kind": "DISCUSSION",
                    "excerpt": "https://player.codemao.cn/new/327450447",
                    "author": {AUTHOR},
                    "tags": [ {{ "id": "t1", "slug": "creative-works", "name": "创作与作品" }} ],
                    "replyCount": 0, "viewCount": 5, "likeCount": 0, "bookmarkCount": 0,
                    "closedAt": null,
                    "pinned": false, "pinnedGlobally": false, "pinnedTagId": null,
                    "pinnedAt": null, "pinnedUntil": null,
                    "createdAt": "2026-09-27T05:09:55.600Z",
                    "editedAt": null,
                    "lastActivityAt": "2026-09-27T05:10:12.121Z"
                }}
            ],
            "pageInfo": {{ "hasNextPage": false, "nextCursor": null }}
        }}"#
    ))
    .expect("user topics payload must deserialize");

    debug!("parsed topics: {topics:?}");
    info!(
        "topics: {} item(s), hasNextPage={}",
        topics.items.len(),
        topics.page_info.has_next_page
    );

    assert_eq!(topics.items.len(), 1);
    let topic = &topics.items[0];
    assert_eq!(topic.kind, "DISCUSSION");
    assert_eq!(topic.tags[0].slug, "creative-works");
    assert_eq!(topic.author.username, "朗kea9");
    assert_eq!(topic.view_count, 5);
    assert!(!topics.page_info.has_next_page);
    assert!(topics.page_info.next_cursor.is_none());
}

#[test]
fn parses_user_posts_payload_with_null_reply_target() {
    common::init_logging();
    info!("parsing user-posts payload fixture");
    let posts: UserRepliesDTO = serde_json::from_str(&format!(
        r#"{{
            "items": [
                {{
                    "id": "p1", "topicId": "t1", "postNumber": 8,
                    "replyToPostNumber": null,
                    "deleted": false, "children": [], "currentRevision": 1,
                    "likeCount": 2, "pinned": false, "cookedHtml": "<p>...</p>",
                    "author": {AUTHOR},
                    "createdAt": "2026-09-27T05:09:55.600Z", "editedAt": null,
                    "viewerCapabilities": {{}}, "viewerState": {{}}
                }},
                {{
                    "id": "p2", "topicId": "t1", "postNumber": 9,
                    "replyToPostNumber": 8,
                    "deleted": false, "children": [], "currentRevision": 1,
                    "likeCount": 0, "pinned": false, "cookedHtml": "<p>回复</p>",
                    "author": {AUTHOR},
                    "createdAt": "2026-09-27T05:10:12.121Z", "editedAt": null,
                    "viewerCapabilities": {{}}, "viewerState": {{}}
                }}
            ],
            "pageInfo": {{ "hasNextPage": true, "nextCursor": "abc" }}
        }}"#
    ))
    .expect("user posts payload must deserialize");

    debug!("parsed posts: {posts:?}");
    info!(
        "posts: {} item(s), hasNextPage={}, nextCursor={:?}",
        posts.items.len(),
        posts.page_info.has_next_page,
        posts.page_info.next_cursor
    );

    assert_eq!(posts.items.len(), 2);
    assert_eq!(posts.items[0].post_number, 8);
    assert!(posts.items[0].reply_to_post_number.is_none());
    assert_eq!(posts.items[1].reply_to_post_number, Some(8));
    assert_eq!(posts.items[1].author.username, "朗kea9");
    assert!(posts.page_info.has_next_page);
    assert_eq!(posts.page_info.next_cursor.as_deref(), Some("abc"));
}

#[test]
fn update_user_profile_omits_unset_fields() {
    common::init_logging();
    info!("serializing update-user-profile requests");

    let only_bio = serde_json::to_value(UpdateUserProfileDTO {
        bio: Some("Acrb".into()),
    })
    .unwrap();
    info!("with bio: {only_bio}");
    assert_eq!(only_bio, serde_json::json!({ "bio": "Acrb" }));

    let empty = serde_json::to_value(UpdateUserProfileDTO { bio: None }).unwrap();
    info!("without bio: {empty}");
    assert_eq!(empty, serde_json::json!({}));
}

#[test]
fn parses_following_and_followers_payload() {
    common::init_logging();
    info!("parsing following/followers payload fixture");
    let list: CursorListDTO<GetUserInformationDTO> = serde_json::from_str(&format!(
        r#"{{
            "items": [{AUTHOR}],
            "pageInfo": {{ "hasNextPage": true, "nextCursor": "cursor-1" }}
        }}"#
    ))
    .expect("following payload must deserialize");

    assert_eq!(list.items.len(), 1);
    assert_eq!(list.items[0].username, "朗kea9");
    assert!(list.page_info.has_next_page);
    assert_eq!(list.page_info.next_cursor.as_deref(), Some("cursor-1"));
}

#[test]
fn parses_featured_topics_and_collection_payloads() {
    common::init_logging();
    info!("parsing featured-topics and topic-collections fixtures");

    let featured: UserFeaturedTopicsDTO = serde_json::from_str(r#"{ "items": [] }"#).unwrap();
    assert!(featured.items.is_empty());

    let collections: CursorListDTO<serde_json::Value> = serde_json::from_str(
        r#"{ "items": [], "pageInfo": { "hasNextPage": false, "nextCursor": null } }"#,
    )
    .unwrap();
    assert!(collections.items.is_empty());
    assert!(!collections.page_info.has_next_page);
}

#[test]
fn parses_badge_payloads() {
    common::init_logging();
    info!("parsing badges / badge-display / user-badge-displays fixtures");

    let badges: UserBadgesDTO = serde_json::from_str(
        r#"{
            "items": [
                {
                    "userId": "01a0c957-53a6-798f-bf2a-2a83c94280d0",
                    "badgeId": "badge_pioneer",
                    "grantedAt": "2026-09-23T13:48:39.720Z",
                    "revision": "013d6b44-6077-4834-aab2-0bbc80a96195",
                    "badge": {
                        "id": "badge_pioneer",
                        "slug": "badge_pioneer",
                        "name": "开拓者",
                        "description": "参与Pickcat社区第一次内测",
                        "icon": "trophy",
                        "version": 4,
                        "image": {
                            "fileId": "01a0b34f-c685-7777-b60d-fe2a6adb6f52",
                            "url": "/api/v1/files/01a0b34f-c685-7777-b60d-fe2a6adb6f52",
                            "width": 512, "height": 512
                        }
                    }
                }
            ]
        }"#,
    )
    .expect("badges payload must deserialize");
    assert_eq!(badges.items.len(), 1);
    assert_eq!(badges.items[0].badge.name, "开拓者");
    assert_eq!(badges.items[0].badge.image.width, 512);

    let display: BadgeDisplayDTO = serde_json::from_str(
        r#"{ "mode": "AUTO_LATEST", "badgeId": null, "displayedBadge": null }"#,
    )
    .unwrap();
    assert_eq!(display.mode, "AUTO_LATEST");
    assert!(display.badge_id.is_none());
    assert!(display.displayed_badge.is_none());

    let bulk: UserBadgeDisplaysDTO = serde_json::from_str(
        r#"{
            "items": [
                { "userId": "01a0fbca-aaa", "displayedBadge": null },
                {
                    "userId": "03829981-bbb",
                    "displayedBadge": {
                        "id": "badge_pioneer", "name": "开拓者",
                        "description": "参与Pickcat社区第一次内测",
                        "image": {
                            "fileId": "f1", "url": "/api/v1/files/f1",
                            "width": 512, "height": 512
                        }
                    }
                }
            ]
        }"#,
    )
    .unwrap();
    assert_eq!(bulk.items.len(), 2);
    assert!(bulk.items[0].displayed_badge.is_none());
    assert_eq!(
        bulk.items[1].displayed_badge.as_ref().unwrap().id,
        "badge_pioneer"
    );
}

#[test]
fn parses_profile_activities_payload() {
    common::init_logging();
    info!("parsing profile-activities payload fixture");
    let activities: CursorListDTO<ProfileActivityDTO> = serde_json::from_str(
        r#"{
            "items": [
                { "id": "a1", "kind": "POSTED", "occurredAt": "2026-09-27T05:09:55.600Z", "title": "做了一个联机小测试，欢迎大家体验", "level": null },
                { "id": "a2", "kind": "LEVEL_UP", "occurredAt": "2026-09-22T13:39:43.876Z", "title": null, "level": 1 }
            ],
            "pageInfo": { "hasNextPage": false, "nextCursor": null }
        }"#,
    )
    .expect("profile activities payload must deserialize");

    assert_eq!(activities.items.len(), 2);
    assert_eq!(activities.items[0].kind, "POSTED");
    assert!(activities.items[0].level.is_none());
    assert_eq!(activities.items[1].kind, "LEVEL_UP");
    assert_eq!(activities.items[1].level, Some(1));
}

#[test]
fn parses_level_contributions_payload() {
    common::init_logging();
    info!("parsing level-contributions payload fixture");
    let contributions: LevelContributionsDTO = serde_json::from_str(
        r#"{
            "year": 2026,
            "timeZone": "Asia/Shanghai",
            "days": [
                { "date": "2026-09-22", "ability": 1, "responsibility": 0, "care": 0 },
                { "date": "2026-09-23", "ability": 1, "responsibility": 0, "care": 0 }
            ],
            "updating": false,
            "calculatedAt": "2026-10-02T08:49:09.230Z"
        }"#,
    )
    .expect("level contributions payload must deserialize");

    assert_eq!(contributions.year, 2026);
    assert_eq!(contributions.time_zone, "Asia/Shanghai");
    assert_eq!(contributions.days.len(), 2);
    assert_eq!(contributions.days[0].ability, 1);
    assert!(!contributions.updating);
}

#[test]
fn parses_level_progress_payload() {
    common::init_logging();
    info!("parsing level-progress payload fixture");
    let progress: LevelProgressDTO = serde_json::from_str(
        r#"{
            "currentLevel": 1,
            "promotionCeiling": null,
            "scores": { "ability": 0, "responsibility": 0, "care": 0 },
            "familiarity": {
                "tracking": true, "familiarityStartedAt": "2026-10-02T08:47:49.552Z",
                "daysSinceEntranceExam": 0, "validVisitDays": 0,
                "topicsEntered": 1, "postsRead": 16, "effectiveReadingSeconds": 3
            },
            "nextLevel": {
                "level": 2, "admission": "LEVEL_REQUIREMENTS",
                "scores": {
                    "ability": { "current": 0, "required": 100, "met": false },
                    "responsibility": { "current": 0, "required": 60, "met": false },
                    "care": { "current": 0, "required": 100, "met": false }
                },
                "familiarity": {
                    "daysSinceEntranceExam": { "current": 0, "required": 30, "met": false },
                    "validVisitDays": { "current": 0, "required": 20, "met": false },
                    "topicsEntered": { "current": 1, "required": 100, "met": false },
                    "postsRead": { "current": 16, "required": 800, "met": false },
                    "effectiveReadingSeconds": { "current": 3, "required": 43200, "met": false }
                },
                "hardRequirements": [
                    { "key": "NO_ACTIVE_PENALTY", "met": true },
                    { "key": "NO_CONFIRMED_VIOLATION_90_DAYS", "met": true }
                ],
                "blockedByPromotionCeiling": false, "eligible": false
            },
            "lv4Candidate": false,
            "quotas": [
                { "action": "TOPIC_CREATE", "limit": 10, "used": 0, "remaining": 10 },
                { "action": "REPLY_CREATE", "limit": 50, "used": 0, "remaining": 50 }
            ],
            "updating": false,
            "calculatedAt": "2026-10-02T08:49:43.287Z"
        }"#,
    )
    .expect("level progress payload must deserialize");

    assert_eq!(progress.current_level, 1);
    assert!(progress.promotion_ceiling.is_none());
    assert_eq!(progress.familiarity.posts_read, 16);
    let next = progress.next_level.expect("next level present");
    assert_eq!(next.level, 2);
    assert_eq!(next.scores.ability.required, 100);
    assert!(!next.eligible);
    assert_eq!(next.hard_requirements.len(), 2);
    assert_eq!(progress.quotas.len(), 2);
    assert_eq!(progress.quotas[0].action, "TOPIC_CREATE");
}

#[test]
fn parses_quota_and_preset_payloads() {
    common::init_logging();
    info!("parsing content-length / usage / storage / avatar-presets fixtures");

    let limit: ContentLengthLimitDTO =
        serde_json::from_str(r#"{ "level": 1, "topicMaxLength": 20000, "replyMaxLength": 2000 }"#)
            .unwrap();
    assert_eq!(limit.topic_max_length, 20000);

    let usage: TopicCollectionUsageDTO = serde_json::from_str(
        r#"{ "currentLevel": 1, "limitCount": 5, "usedCount": 0, "remainingCount": 5 }"#,
    )
    .unwrap();
    assert_eq!(usage.remaining_count, 5);

    let storage: FileStorageDTO = serde_json::from_str(
        r#"{ "usedBytes": 0, "limitBytes": 20971520, "remainingBytes": 20971520 }"#,
    )
    .unwrap();
    assert_eq!(storage.limit_bytes, 20 * 1024 * 1024);

    let presets: AvatarPresetsDTO = serde_json::from_str(
        r#"{
            "items": [
                { "type": "PRESET", "id": 1, "url": "/api/v1/avatars/1?v=f558" },
                { "type": "PRESET", "id": 2, "url": "/api/v1/avatars/2?v=fed2" }
            ]
        }"#,
    )
    .unwrap();
    assert_eq!(presets.items.len(), 2);
    assert_eq!(presets.items[0].avatar_type, "PRESET");
    assert_eq!(presets.items[1].id, 2);
}

#[test]
fn parses_bookmarks_payload_as_raw_json() {
    common::init_logging();
    info!("parsing bookmarks payload fixture");
    let bookmarks: CursorListDTO<serde_json::Value> = serde_json::from_str(
        r#"{ "items": [], "pageInfo": { "hasNextPage": false, "nextCursor": null } }"#,
    )
    .expect("bookmarks payload must deserialize");
    assert!(bookmarks.items.is_empty());
    assert!(bookmarks.page_info.next_cursor.is_none());
}
