//! 分区标签 DTO。

use serde::{Deserialize, Serialize};

/// 分区标签项。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDTO {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagsDTO {
    pub items: Vec<TagDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagSidebarLinksDTO {
    pub source: TagDTO,
    pub items: Vec<TagDTO>,
}
