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
