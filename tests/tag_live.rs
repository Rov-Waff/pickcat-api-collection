//! Live tests for the tag endpoints in `src/tag.rs`.

mod common;

use log::{debug, info};
use pickcat_api_collection::tag::TagBehavior;

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_list_tags() {
    common::init_logging();
    let account = common::account().await;

    info!("GET /api/v1/tags?assignable=true");
    let tags = account.list_tags(Some(true)).await.unwrap();
    info!("assignable tags: {}", tags.items.len());
    for tag in &tags.items {
        debug!("{} ({})", tag.name, tag.slug);
    }

    assert!(!tags.items.is_empty());
    assert!(tags.items.len() <= 9);
}

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_get_tag_sidebar_links() {
    common::init_logging();
    let account = common::account().await;

    info!("GET /api/v1/tags/interest-plaza/sidebar-links");
    let side = account
        .get_tag_sidebar_links("interest-plaza")
        .await
        .unwrap();
    info!(
        "{} has {} sidebar link(s)",
        side.source.name,
        side.items.len()
    );

    assert_eq!(side.source.slug, "interest-plaza");
}
