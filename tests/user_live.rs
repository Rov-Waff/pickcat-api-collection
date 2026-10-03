//! Live tests for the user-chapter endpoints in `src/user.rs`.
//!
//! These read `USERNAME`/`PASSWORD` from `.env` (or the environment) and are
//! `#[ignore]`d because they hit the real API:
//!
//! ```sh
//! cargo test --test user_live -- --ignored --nocapture
//! ```

mod common;

use log::{debug, info};
use pickcat_api_collection::auth::UserBehavior;
use pickcat_api_collection::user::UserProfileBehavior;

async fn current_user_id() -> String {
    common::account()
        .await
        .get_current_user_session()
        .await
        .unwrap()
        .user
        .id
}

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_get_user_information() {
    common::init_logging();
    let _guard = common::LIVE_LOCK.lock().await;
    let account = common::account().await;
    let user_id = current_user_id().await;

    info!("GET /api/v1/users/{user_id}");
    let user = account.get_user_information(&user_id).await.unwrap();
    debug!("user information: {user:?}");
    info!(
        "user {} Lv.{}: topics={}, replies={}",
        user.username, user.level.current, user.stats.topics, user.stats.replies
    );

    assert_eq!(user.id, user_id);
    assert!(!user.username.is_empty());
    assert!(!user.avatar.url.is_empty());
}

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_get_user_email() {
    common::init_logging();
    let _guard = common::LIVE_LOCK.lock().await;
    let account = common::account().await;
    let user_id = current_user_id().await;

    info!("GET /api/v1/users/{user_id}/email");
    let email = account.get_user_email(&user_id).await.unwrap();
    info!("email={}, verifiedAt={}", email.email, email.verified_at);

    assert!(!email.email.is_empty());
    assert!(!email.verified_at.is_empty());
}

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_get_user_topics() {
    common::init_logging();
    let _guard = common::LIVE_LOCK.lock().await;
    let account = common::account().await;
    let user_id = current_user_id().await;

    info!("GET /api/v1/users/{user_id}/topics?limit=20");
    let topics = account.get_user_topics(&user_id, Some(20)).await.unwrap();
    info!(
        "topics: {} item(s), hasNextPage={}",
        topics.items.len(),
        topics.page_info.has_next_page
    );
    for topic in &topics.items {
        debug!("[{}] {}", topic.kind, topic.title);
    }

    assert!(topics.items.len() <= 20);
}

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_get_user_posts() {
    common::init_logging();
    let _guard = common::LIVE_LOCK.lock().await;
    let account = common::account().await;
    let user_id = current_user_id().await;

    info!("GET /api/v1/users/{user_id}/posts?role=reply&limit=20");
    let posts = account
        .get_user_posts(&user_id, Some("reply"), Some(20))
        .await
        .unwrap();
    info!(
        "posts: {} item(s), hasNextPage={}",
        posts.items.len(),
        posts.page_info.has_next_page
    );
    for post in &posts.items {
        debug!(
            "#{} replyTo={:?} {}",
            post.post_number, post.reply_to_post_number, post.cooked_html
        );
    }

    assert!(posts.items.len() <= 20);
}

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_get_user_social_lists() {
    common::init_logging();
    let _guard = common::LIVE_LOCK.lock().await;
    let account = common::account().await;
    let user_id = current_user_id().await;

    info!("GET /api/v1/users/{user_id}/following?limit=50");
    let following = account
        .get_user_following(&user_id, Some(50), None)
        .await
        .unwrap();
    info!(
        "following: {} item(s), hasNextPage={}",
        following.items.len(),
        following.page_info.has_next_page
    );

    info!("GET /api/v1/users/{user_id}/followers?limit=50");
    let followers = account
        .get_user_followers(&user_id, Some(50), None)
        .await
        .unwrap();
    info!(
        "followers: {} item(s), hasNextPage={}",
        followers.items.len(),
        followers.page_info.has_next_page
    );

    assert!(following.items.len() <= 50);
    assert!(followers.items.len() <= 50);
}

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_get_user_topics_and_collections() {
    common::init_logging();
    let _guard = common::LIVE_LOCK.lock().await;
    let account = common::account().await;
    let user_id = current_user_id().await;

    info!("GET /api/v1/users/{user_id}/featured-topics");
    let featured = account.get_user_featured_topics(&user_id).await.unwrap();
    info!("featured: {} item(s)", featured.items.len());

    info!("GET /api/v1/users/{user_id}/topic-collections?limit=20");
    let collections = account
        .get_user_topic_collections(&user_id, Some(20))
        .await
        .unwrap();
    info!(
        "collections: {} item(s), hasNextPage={}",
        collections.items.len(),
        collections.page_info.has_next_page
    );

    assert!(collections.items.len() <= 20);
}

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_get_user_badges() {
    common::init_logging();
    let _guard = common::LIVE_LOCK.lock().await;
    let account = common::account().await;
    let user_id = current_user_id().await;

    info!("GET /api/v1/users/{user_id}/badges");
    let badges = account.get_user_badges(&user_id).await.unwrap();
    info!("badges: {} item(s)", badges.items.len());
    for badge in &badges.items {
        debug!("{} ({})", badge.badge.name, badge.granted_at);
    }

    info!("GET /api/v1/users/{user_id}/badge-display");
    let display = account.get_user_badge_display(&user_id).await.unwrap();
    info!("badge display mode: {}", display.mode);

    info!("GET /api/v1/user-badge-displays?userIds={user_id}");
    let bulk = account
        .get_user_badge_displays(std::slice::from_ref(&user_id))
        .await
        .unwrap();
    info!("bulk badge displays: {} item(s)", bulk.items.len());
    assert_eq!(bulk.items.len(), 1);
}

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_get_user_activities_and_contributions() {
    common::init_logging();
    let _guard = common::LIVE_LOCK.lock().await;
    let account = common::account().await;
    let user_id = current_user_id().await;

    info!("GET /api/v1/users/{user_id}/profile-activities");
    let activities = account
        .get_user_profile_activities(&user_id, None, Some(50))
        .await
        .unwrap();
    info!("user activities: {} item(s)", activities.items.len());

    info!("GET /api/v1/profile-activities");
    let mine = account
        .get_my_profile_activities(None, Some(50))
        .await
        .unwrap();
    info!("my activities: {} item(s)", mine.items.len());

    info!("GET /api/v1/users/{user_id}/level-contributions/2026");
    let user_contrib = account
        .get_user_level_contributions(&user_id, 2026)
        .await
        .unwrap();
    info!("user contribution days: {}", user_contrib.days.len());

    info!("GET /api/v1/level-contributions/2026");
    let my_contrib = account.get_my_level_contributions(2026).await.unwrap();
    info!("my contribution days: {}", my_contrib.days.len());

    assert!(activities.items.len() <= 50);
    assert!(mine.items.len() <= 50);
}

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_get_level_progress_and_quotas() {
    common::init_logging();
    let _guard = common::LIVE_LOCK.lock().await;
    let account = common::account().await;

    info!("GET /api/v1/level-progress");
    let progress = account.get_level_progress().await.unwrap();
    info!(
        "level progress: current=Lv.{}, quotas={}, nextLevel={:?}",
        progress.current_level,
        progress.quotas.len(),
        progress.next_level.as_ref().map(|n| n.level)
    );

    info!("GET /api/v1/content-length-limit");
    let limit = account.get_content_length_limit().await.unwrap();
    info!(
        "content length: topic={}, reply={}",
        limit.topic_max_length, limit.reply_max_length
    );

    info!("GET /api/v1/topic-collection-usage");
    let usage = account.get_topic_collection_usage().await.unwrap();
    info!(
        "collection usage: {}/{}",
        usage.used_count, usage.limit_count
    );

    info!("GET /api/v1/file-storage");
    let storage = account.get_file_storage().await.unwrap();
    info!(
        "file storage: {}/{} bytes",
        storage.used_bytes, storage.limit_bytes
    );

    assert_eq!(limit.level, progress.current_level);
    assert!(usage.remaining_count <= usage.limit_count);
    assert!(storage.remaining_bytes <= storage.limit_bytes);
}

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_get_bookmarks_and_avatar_presets() {
    common::init_logging();
    let _guard = common::LIVE_LOCK.lock().await;
    let account = common::account().await;

    info!("GET /api/v1/bookmarks?limit=20");
    let bookmarks = account.get_bookmarks(Some(20)).await.unwrap();
    info!(
        "bookmarks: {} item(s), hasNextPage={}",
        bookmarks.items.len(),
        bookmarks.page_info.has_next_page
    );

    info!("GET /api/v1/avatar-presets");
    let presets = account.get_avatar_presets().await.unwrap();
    info!("avatar presets: {} item(s)", presets.items.len());

    assert!(bookmarks.items.len() <= 20);
    assert!(!presets.items.is_empty());
}
