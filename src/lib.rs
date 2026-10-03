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

pub mod dto;
pub mod auth;
pub mod user;
const BASE_URL: &str = "https://cdsq.dao3.fun";
#[derive(Debug)]
pub struct PickcatAccound {
    pub username: String,
    pub password: String,
    pub client: Client,
}

impl PickcatAccound {
    pub async fn new(username: &str, password: &str) -> Result<Self, Error> {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(30))
            .cookie_store(true)
            .build()?;
        let dto = LoginDTO {
            username: username.to_string(),
            password: password.to_string(),
        };
        client
            .post(format!("{}/api/v1/session", BASE_URL))
            .json(&dto)
            .send()
            .await?;
        Ok(Self {
            username: username.to_string(),
            password: password.to_string(),
            client: client,
        })
    }
}
