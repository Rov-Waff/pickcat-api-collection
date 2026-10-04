//! 按 API 章节拆分的请求/响应 DTO。
//!
//! 共享类型（`CursorListDTO`、`PageInfoDTO`、`TopicAuthorDTO`、
//! `LevelFamiliarityDTO` 等）放在 [`user`] 模块，供其他 DTO 模块复用。

use serde::{Deserialize, Serialize};

pub mod auth;
pub mod media;
pub mod notification;
pub mod reading_session;
pub mod tag;
pub mod topic;
pub mod user;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginDTO {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdValue {
    Num(u64),
    Str(String),
}
