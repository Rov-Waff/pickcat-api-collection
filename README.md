# pickcat-api-collection

Pickcat 社区 API 的异步 Rust 客户端（edition 2024，基于 [`reqwest`]），覆盖文档中已记录的全部接口。

- 接口文档：<https://pickcat-docs.xiaole6324.fun>
- 目标 API：`https://cdsq.dao3.fun/api/v1`

## 特性

- 按章节拆分为 trait：鉴权/考试、用户、主题与内容、分区标签、通知、阅读会话、媒体资源。
- Session（Cookie）鉴权：登录后凭据由内部带 cookie store 的 `reqwest::Client` 自动携带。
- 写接口自动处理 `Idempotency-Key`、`origin` 请求头。
- 三层测试：DTO 契约测试、mock-server 行为测试、可选的联网测试。

## 环境要求

- Rust 1.85+（edition 2024）

## 安装

```toml
[dependencies]
pickcat-api-collection = { git = "https://github.com/<you>/pickcat-api-collection" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## 快速开始

```rust
use pickcat_api_collection::auth::UserBehavior;
use pickcat_api_collection::PickcatAccound;

#[tokio::main]
async fn main() -> Result<(), pickcat_api_collection::Error> {
    // 登录（用户名即账号邮箱）；后续请求自动带会话 Cookie
    let account = PickcatAccound::new("you@example.com", "password").await?;

    let session = account.get_current_user_session().await?;
    println!("{} Lv.{}", session.user.username, session.user.level.current);

    Ok(())
}
```

组合多个章节：

```rust
use pickcat_api_collection::topic::TopicBehavior;
use pickcat_api_collection::user::UserProfileBehavior;

async fn demo(
    account: pickcat_api_collection::PickcatAccound,
) -> Result<(), pickcat_api_collection::Error> {
    // 主题列表（同一个 tag 可重复传以实现多标签过滤）
    let topics = account
        .list_topics(Some(20), &["interest-plaza".to_string()], Some("all"), None)
        .await?;
    println!("{} 个主题", topics.items.len());

    // 当前用户等级进度
    let progress = account.get_level_progress().await?;
    println!("Lv.{}", progress.current_level);
    Ok(())
}
```

## 模块一览

| 模块 | trait | 接口 |
| --- | --- | --- |
| `auth` | `UserBehavior` | 会话、注册、邮箱验证、入站考试（状态/开始/取题/交卷） |
| `user` | `UserProfileBehavior` | 资料、改资料、邮箱、关注/粉丝、主题/回帖、精选主题/合集、徽章、动态、等级、配额、书签、预设头像 |
| `topic` | `TopicBehavior` | 主题列表/推荐/详情、楼层、发布主题/回帖、投稿审核 |
| `tag` | `TagBehavior` | 分区标签、分区侧边子标签 |
| `notification` | `NotificationBehavior` | 通知未读汇总 |
| `reading_session` | `ReadingSessionBehavior` | 阅读埋点上报 |
| `media` | `MediaBehavior` | 文件上传/列表/读取、头像、表情 |

所有方法挂在 `PickcatAccound` 上；导入对应 trait 即可调用。

## 写接口与幂等

`POST /api/v1/posts`（发主题/回帖）与 `POST /api/v1/files`（上传）必须带 `Idempotency-Key` 请求头：

```rust
use pickcat_api_collection::topic::{generate_idempotency_key, TopicBehavior};

async fn demo(
    account: pickcat_api_collection::PickcatAccound,
) -> Result<(), pickcat_api_collection::Error> {
    let key = generate_idempotency_key();
    let created = account
        .create_reply(&key, "<topicId>", "回帖正文", None)
        .await?;
    println!("投稿已受理：{} ({})", created.submission_id, created.status);
    Ok(())
}
```

- 发布内容是**异步审核**：返回 `202` 与 `submissionId`，审核期间主题对他人不可见，可用 `get_post_submission` 查询状态。
- `create_topic` / `create_reply` / `upload_file` 会自动带上 `Idempotency-Key` 与 `origin`。

## 测试

所有测试都在 `tests/` 下，分三层：

```sh
cargo test                  # ① DTO 契约测试 + ② mock-server 行为测试（默认，不联网）
cargo test -- --ignored --nocapture   # ③ 联网测试（需 .env 中有效凭据）
```

- ① `tests/*_contract.rs`：用文档 JSON 验证 DTO / `serde` 映射。
- ② `tests/http_*.rs`：用 [`wiremock`] + `PickcatAccound::with_base_url()` 断言方法、路径、query、请求头、请求体，不联网。
- ③ `tests/*_live.rs`：打真实 API，全部 `#[ignore]`。

联网测试需要仓库根目录的 `.env`（已被 `.gitignore`）：

```dotenv
USERNAME=you@example.com
PASSWORD=your-password
```

单个文件 / 单个用例：

```sh
cargo test --test http_user
cargo test --test user_contract parses_user_email_payload
```

日志默认级别 `info`，配合 `--nocapture` 可见；`RUST_LOG=debug` 打印完整 DTO。

## 项目结构

```
src/
├── lib.rs              # Error、PickcatAccound、BASE_URL
├── auth.rs             # UserBehavior：会话/注册/邮箱验证/考试
├── user.rs             # UserProfileBehavior：用户章节
├── topic.rs            # TopicBehavior：主题与内容 + generate_idempotency_key()
├── tag.rs              # TagBehavior
├── notification.rs     # NotificationBehavior
├── reading_session.rs  # ReadingSessionBehavior
├── media.rs            # MediaBehavior
└── dto/                # 按章节拆分的请求/响应 DTO（共享类型在 dto::user）
tests/
├── common/mod.rs       # 日志、登录、mock_account()
├── *_contract.rs       # 离线 DTO 契约测试
├── http_*.rs           # mock-server 行为测试
└── *_live.rs           # 联网测试（#[ignore]）
```

## 注意事项

- **注册 / 邮箱验证需要 CAPTCHA**（`captchaVerifyParam`），本仓库不覆盖其测试。
- 文档未定型的字段（如合集项、书签项、投稿 `request`）暂以 `serde_json::Value` 承载。
- 媒体读取接口（文件/头像/表情）返回原始 `Vec<u8>`，不走 JSON。
- `PickcatAccound::new()` 不校验登录响应；如需确认登录成功，请在登录后调用 `get_current_user_session()`。

[`reqwest`]: https://docs.rs/reqwest
[`wiremock`]: https://docs.rs/wiremock
