//! Offline contract tests for the tag DTOs in `src/dto/tag.rs`.

mod common;

use log::{debug, info};
use pickcat_api_collection::dto::tag::*;

#[test]
fn parses_tags_payload() {
    common::init_logging();
    info!("parsing tags fixture");
    let tags: TagsDTO = serde_json::from_str(
        r#"{
            "items": [
                { "id": "01a0ab26-84a8-718b-996a-36be3dda4fa4", "slug": "creative-works", "name": "创作与作品", "description": "作品发布、试玩反馈、开发日志与作品复盘" },
                { "id": "01a0ab26-0000-0000-0000-3960a4789d05", "slug": "learning-technology", "name": "学习与技术", "description": "教程、知识分享与技术讨论" }
            ]
        }"#,
    )
    .expect("tags payload must deserialize");

    debug!("parsed tags: {tags:?}");
    assert_eq!(tags.items.len(), 2);
    assert_eq!(tags.items[0].slug, "creative-works");
    assert_eq!(
        tags.items[1].description.as_deref(),
        Some("教程、知识分享与技术讨论")
    );
}

#[test]
fn parses_tag_sidebar_links_payload() {
    common::init_logging();
    info!("parsing tag sidebar-links fixture");
    let side: TagSidebarLinksDTO = serde_json::from_str(
        r#"{
            "source": { "id": "s1", "slug": "interest-plaza", "name": "兴趣广场", "description": "绘画、音乐、动画……" },
            "items": [
                { "id": "c1", "slug": "interest-painting", "name": "绘画", "description": null }
            ]
        }"#,
    )
    .expect("sidebar links payload must deserialize");

    assert_eq!(side.source.slug, "interest-plaza");
    assert_eq!(side.items.len(), 1);
    assert_eq!(side.items[0].name, "绘画");
    assert!(side.items[0].description.is_none());
}
