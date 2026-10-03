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
pub mod user;
const BASE_URL: &str = "https://cdsq.dao3.fun/api/v1";
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

#[cfg(test)]
mod test {
    use std::env;

    use crate::PickcatAccound;

    #[tokio::test]
    async fn test_login() {
        dotenvy::dotenv().ok();
        env_logger::init();
        let username = env::var("USERNAME").unwrap();
        let password = env::var("PASSWORD").unwrap();

        let _ = PickcatAccound::new(&username, &password).await.unwrap();
    }
}
