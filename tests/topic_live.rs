//! Live tests for the topic/content endpoints in `src/topic.rs`.
//!
//! These read `USERNAME`/`PASSWORD` from `.env` (or the environment) and are
//! `#[ignore]`d because they hit the real API:
//!
//! ```sh
//! cargo test --test topic_live -- --ignored --nocapture
//! ```

mod common;

use log::{debug, info, warn};
use pickcat_api_collection::topic::TopicBehavior;

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_list_topics() {
    common::init_logging();
    let account = common::account().await;

    info!("GET /api/v1/topics?limit=20");
    let topics = account
        .list_topics(Some(20), &[], None, None)
        .await
        .unwrap();
    info!(
        "topics: {} item(s), hasNextPage={}",
        topics.items.len(),
        topics.page_info.has_next_page
    );
    for topic in &topics.items {
        debug!(
            "[{}] {} by {}",
            topic.kind, topic.title, topic.author.username
        );
    }

    assert!(topics.items.len() <= 20);
}

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_get_topic_recommendations() {
    common::init_logging();
    let account = common::account().await;

    info!("GET /api/v1/topic-recommendations?sort=recommended&limit=20");
    let rec = account
        .get_topic_recommendations(Some("recommended"), Some(20), None)
        .await
        .unwrap();
    info!("strategy={}, {} item(s)", rec.strategy, rec.items.len());
    for item in &rec.items {
        debug!("({}) {}", item.reason, item.topic.title);
    }

    assert!(rec.items.len() <= 20);
}

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_get_topic_detail_and_posts() {
    common::init_logging();
    let account = common::account().await;

    info!("GET /api/v1/topics?limit=1 (to pick a topic)");
    let topics = account.list_topics(Some(1), &[], None, None).await.unwrap();
    let Some(first) = topics.items.first() else {
        warn!("no topics returned; skipping detail/posts");
        return;
    };
    let topic_id = first.id.clone();

    info!("GET /api/v1/topics/{topic_id}");
    let topic = account.get_topic(&topic_id).await.unwrap();
    info!(
        "topic {:?}: replies={}, views={}, firstPost#{}, repliesTruncated={}",
        topic.title,
        topic.reply_count,
        topic.view_count,
        topic.first_post.post_number,
        topic.replies_truncated
    );

    info!("GET /api/v1/topics/{topic_id}/posts?limit=50&sort=hot");
    let posts = account
        .get_topic_posts(&topic_id, Some(50), Some("hot"), None)
        .await
        .unwrap();
    info!(
        "posts: {} item(s), hasNextPage={}",
        posts.items.len(),
        posts.page_info.has_next_page
    );

    assert_eq!(topic.id, topic_id);
    assert_eq!(topic.first_post.topic_id, topic_id);
    assert!(posts.items.len() <= 50);
}

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_list_post_submissions() {
    common::init_logging();
    let account = common::account().await;

    info!("GET /api/v1/post-submissions?limit=20");
    let submissions = account
        .list_post_submissions(None, None, None, Some(20), None)
        .await
        .unwrap();
    info!(
        "submissions: {} item(s), hasNextPage={}",
        submissions.items.len(),
        submissions.page_info.has_next_page
    );
    for submission in &submissions.items {
        debug!(
            "{} {} role={}",
            submission.id, submission.status, submission.content_role
        );
    }

    assert!(submissions.items.len() <= 20);
}
