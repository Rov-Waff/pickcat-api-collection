//! Offline contract tests for the media DTOs in `src/dto/media.rs`.

mod common;

use log::info;
use pickcat_api_collection::dto::media::*;

#[test]
fn parses_uploaded_file_payload() {
    common::init_logging();
    info!("parsing uploaded-file fixture");
    let uploaded: UploadedFileDTO = serde_json::from_str(
        r#"{
            "id": "01a0ff24-bc3f-7140-a78c-73f6b40137b0",
            "ownership": "PERSONAL",
            "uploadedByUserId": "01a0fcb0-5f5b-772d-a8ba-4b39d1ee4445"
        }"#,
    )
    .expect("uploaded file must deserialize");

    assert_eq!(uploaded.ownership, "PERSONAL");
    assert!(!uploaded.uploaded_by_user_id.is_empty());
}

#[test]
fn parses_file_list_payload() {
    common::init_logging();
    info!("parsing file-list fixture");
    let files: FileListDTO = serde_json::from_str(
        r#"{ "pageInfo": { "hasNextPage": false, "nextCursor": null }, "items": [] }"#,
    )
    .expect("file list must deserialize");

    assert!(files.items.is_empty());
    assert!(!files.page_info.has_next_page);
}
