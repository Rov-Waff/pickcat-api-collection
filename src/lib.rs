use std::time::Duration;

use anyhow::Result;
use reqwest::Client;

use crate::dto::LoginDTO;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("RequestError!")]
    Reqwest(#[from] reqwest::Error),
    #[error("ParseError")]
    Serde(#[from] serde_json::Error),
}

pub mod auth;
pub mod dto;
pub mod media;
pub mod notification;
pub mod reading_session;
pub mod tag;
pub mod topic;
pub mod user;
const BASE_URL: &str = "https://cdsq.dao3.fun";
#[derive(Debug)]
pub struct PickcatAccound {
    pub username: String,
    pub password: String,
    pub client: Client,
    /// API 根地址；默认 [`BASE_URL`]，测试时可注入 mock server。
    pub base_url: String,
}

impl PickcatAccound {
    pub async fn new(username: &str, password: &str) -> Result<Self, Error> {
        Self::with_base_url(username, password, BASE_URL).await
    }

    /// 与 [`PickcatAccound::new`] 相同，但可指定 API 根地址（用于测试）。
    pub async fn with_base_url(
        username: &str,
        password: &str,
        base_url: &str,
    ) -> Result<Self, Error> {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(30))
            .cookie_store(true)
            .build()?;
        let dto = LoginDTO {
            username: username.to_string(),
            password: password.to_string(),
        };
        client
            .post(format!("{}/api/v1/session", base_url))
            .json(&dto)
            .send()
            .await?;
        Ok(Self {
            username: username.to_string(),
            password: password.to_string(),
            client: client,
            base_url: base_url.to_string(),
        })
    }
}
