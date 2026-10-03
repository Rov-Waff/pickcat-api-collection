use serde::{Deserialize, Serialize};

pub mod auth;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginDTO {
    pub username: String,
    pub password: String,
}
