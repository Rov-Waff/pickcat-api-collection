//! Live tests for the media endpoints in `src/media.rs`.

mod common;

use log::{info, warn};
use pickcat_api_collection::media::MediaBehavior;

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_get_avatar_and_emoji() {
    common::init_logging();
    let account = common::account().await;

    info!("GET /api/v1/avatars/1");
    let avatar = account.get_avatar("1", None).await.unwrap();
    info!("avatar bytes: {}", avatar.len());
    assert!(!avatar.is_empty());

    info!("GET /api/v1/emojis/gif_expression_bianchenmao_funny/image");
    match account
        .get_emoji_image("gif_expression_bianchenmao_funny", None)
        .await
    {
        Ok(bytes) => info!("emoji bytes: {}", bytes.len()),
        Err(error) => warn!("emoji fetch failed (name may not exist): {error}"),
    }
}

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_get_file_and_list_files() {
    common::init_logging();
    let account = common::account().await;

    // 文档里徽章图片的 fileId。
    info!("GET /api/v1/files/01a0b34f-c685-7777-b60d-fe2a6adb6f52");
    match account
        .get_file("01a0b34f-c685-7777-b60d-fe2a6adb6f52")
        .await
    {
        Ok(bytes) => {
            info!("file bytes: {}", bytes.len());
            assert!(!bytes.is_empty());
        }
        Err(error) => warn!("file fetch failed: {error}"),
    }

    info!("GET /api/v1/files?scope=OWN&state=READY&limit=20");
    let files = account
        .list_files(Some("OWN"), Some("READY"), Some(20), None)
        .await
        .unwrap();
    info!("files: {} item(s)", files.items.len());
    assert!(files.items.len() <= 20);
}
