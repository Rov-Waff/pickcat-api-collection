//! # pickcat-api-collection
//!
//! Pickcat 社区 API（`https://cdsq.dao3.fun/api/v1`）的异步 Rust 客户端，基于
//! [`reqwest`]。接口文档见 <https://pickcat-docs.xiaole6324.fun>。
//!
//! 所有能力都挂在 [`PickcatAccound`] 上，按章节拆成 trait：
//!
//! | 模块 | trait | 内容 |
//! | --- | --- | --- |
//! | [`auth`] | [`auth::UserBehavior`] | 会话、注册、邮箱验证、入站考试 |
//! | [`user`] | [`user::UserProfileBehavior`] | 用户资料、邮箱、关注/粉丝、主题/回帖、徽章、等级、配额等 |
//! | [`topic`] | [`topic::TopicBehavior`] | 主题列表/推荐/详情、楼层、发布、投稿审核 |
//! | [`tag`] | [`tag::TagBehavior`] | 分区标签、侧边子标签 |
//! | [`notification`] | [`notification::NotificationBehavior`] | 通知未读汇总 |
//! | [`reading_session`] | [`reading_session::ReadingSessionBehavior`] | 阅读埋点上报 |
//! | [`media`] | [`media::MediaBehavior`] | 文件上传/读取、头像、表情 |
//!
//! ## 快速开始
//!
//! ```no_run
//! use pickcat_api_collection::auth::UserBehavior;
//! use pickcat_api_collection::PickcatAccound;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), pickcat_api_collection::Error> {
//!     // 登录；后续请求自动携带会话 Cookie
//!     let account = PickcatAccound::new("you@example.com", "password").await?;
//!
//!     let session = account.get_current_user_session().await?;
//!     println!(
//!         "{} Lv.{}",
//!         session.user.username, session.user.level.current
//!     );
//!     Ok(())
//! }
//! ```
//!
//! ## 鉴权
//!
//! Pickcat 使用 Session（Cookie）而不是 JWT：登录成功后凭据由内部带 cookie
//! store 的 [`reqwest::Client`] 保存，调用方无需手动传 token。
//!
//! ## 错误
//!
//! 所有方法返回 [`Result`]，错误类型为 [`Error`]（请求失败 [`Error::Reqwest`]
//! 或响应反序列化失败 [`Error::Serde`]）。
//!
//! ## 写接口
//!
//! `POST /posts`、`POST /files` 等写接口需要 `Idempotency-Key`
//! 请求头，可用 [`topic::generate_idempotency_key`] 生成。

use std::time::Duration;

use anyhow::Result;
use reqwest::Client;

use crate::dto::LoginDTO;

/// 调用 API 时可能出现的错误。
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// 网络请求或响应读取失败。
    #[error("RequestError!")]
    Reqwest(#[from] reqwest::Error),
    /// 响应体无法按预期 DTO 反序列化。
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

/// 已登录的 Pickcat 账号，是访问所有接口的入口。
///
/// 构造时即完成登录并保存会话 Cookie；各章节的方法由对应 trait 提供
/// （见 crate 文档的表格）。
#[derive(Debug)]
pub struct PickcatAccound {
    pub username: String,
    pub password: String,
    pub client: Client,
    /// API 根地址；默认 `https://cdsq.dao3.fun`，测试时可注入 mock server。
    pub base_url: String,
}

impl PickcatAccound {
    /// 使用默认 API 根地址登录。
    pub async fn new(username: &str, password: &str) -> Result<Self, Error> {
        Self::with_base_url(username, password, BASE_URL).await
    }

    /// 与 [`PickcatAccound::new`] 相同，但可指定 API 根地址。
    ///
    /// 主要用于测试：把 `base_url` 指向 mock server 即可离线验证请求。
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
