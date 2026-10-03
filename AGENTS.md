# AGENTS.md

Pickcat 社区 API 的 Rust 客户端（edition 2024），基于 `reqwest`，没有本地服务或数据库。测试会直接访问线上 `https://cdsq.dao3.fun/api/v1`。
接口文档：<https://pickcat-docs.xiaole6324.fun>（mdBook）。

## 常用命令

- `cargo test` —— 运行不联网的测试：DTO 契约测试（`tests/*_contract.rs`）与 mock-server 行为测试（`tests/http_*.rs`）。所有联网测试都是 `#[ignore]`，默认不联网、不需要凭据。
- `cargo test -- --ignored --nocapture` —— 运行联网测试；需要 `.env` 中有效的 `USERNAME` / `PASSWORD`。
- 单文件 / 单测：
  - `cargo test --test user_contract`
  - `cargo test --test user_contract parses_user_email_payload`
  - `cargo test --test dto_contract` / `cargo test --test live -- --ignored`
- 日志：默认过滤级别为 `info`，配合 `--nocapture` 可见；`RUST_LOG=debug` 打印完整 DTO。
- `cargo fmt`、`cargo clippy --tests`。仓库没有 rustfmt / clippy / CI 配置。

## 模块边界

- `src/lib.rs`：`Error`、`PickcatAccound`（登录并持有带 cookie store 的 `reqwest::Client`）、私有常量 `BASE_URL`（模块内用 `self.base_url`）。`PickcatAccound::with_base_url()` 可指向 mock server，行为方法一律用 `self.base_url` 拼 URL。
- `src/auth.rs`：trait `UserBehavior` —— 会话、注册、邮箱验证、入站考试（状态 / 开始 / 取当前题 / 交答案）。
- `src/user.rs`：trait `UserProfileBehavior` —— 用户章节全部接口（资料/修改资料、邮箱、关注/粉丝、主题/回帖、精选主题/合集、徽章、动态、等级贡献/进度、配额、书签、预设头像）。
- `src/topic.rs`：trait `TopicBehavior` —— 主题与内容章节（主题列表/推荐/详情、楼层、发布主题/回帖、投稿审核）；`generate_idempotency_key()` 生成写接口所需的 `Idempotency-Key`。
- `src/tag.rs` / `src/notification.rs` / `src/reading_session.rs` / `src/media.rs`：对应「分区标签 / 通知 / 阅读会话 / 媒体资源」章节（`TagBehavior`、`NotificationBehavior`、`ReadingSessionBehavior`、`MediaBehavior`）；媒体读取接口返回 `Vec<u8>`。
- `src/dto.rs`：`LoginDTO`，并导出 `dto::auth`、`dto::user`、`dto::topic`、`dto::tag`、`dto::notification`、`dto::reading_session`、`dto::media`。
- `src/dto/*.rs`：请求/响应 DTO；字段名靠 `#[serde(rename = ...)]` / `rename_all = "camelCase"` 对齐 JSON。共享类型（`CursorListDTO`、`PageInfoDTO`、`TopicAuthorDTO`、`LevelFamiliarityDTO` 等）放在 `dto::user`，其他 DTO 模块复用。
- `tests/common/mod.rs`：`init_logging()`、`account()`（`OnceCell` 全局单次登录）、`LIVE_LOCK`。

## 新增接口 / 测试的约定

- 按现有模式：在对应模块定义 trait（方法返回 `impl Future<Output = Result<DTO, Error>> + Send`），再为 `PickcatAccound` 实现。
- 测试分三层：`tests/<area>_contract.rs` 用文档 JSON 做 `serde_json::from_str`，验证 DTO / rename 映射；`tests/http_<area>.rs` 用 `wiremock` + `common::mock_account()`（内部 `PickcatAccound::with_base_url()`）断言方法、路径、query、请求头、请求体；`tests/<area>_live.rs` 打真实 API，全部标 `#[ignore = "hits the live API; ..."]`。
- 需要 URL query 的接口：`reqwest` 0.13 的 `.query()` 由 `query` feature 控制，`Cargo.toml` 已启用。

## 容易踩的坑

- `PickcatAccound::new()` **不检查登录响应**：凭据错误也返回 `Ok`。验证登录必须再调用 `get_current_user_session()`（`tests/common/account()` 就是这么做的）。
- `.env` 已被 `.gitignore`，不要提交。当前凭据可能失效（`POST /api/v1/session` 返回 `401 INVALID_CREDENTIALS`）。
- 联网测试共享同一个登录会话，考试流程会修改服务端状态，因此用 `LIVE_LOCK` 串行执行；不要并行跑这些用例。
- 注册 / 邮箱验证依赖 CAPTCHA（`captchaVerifyParam`），不做测试。
- 已知缺陷：`UserBehavior::verify_email` 的 trait 签名是 `(id, code, password)`，impl 却是 `(code, password, id)`，参数顺序不一致。
- 写接口（如 `POST /api/v1/posts`）必须带 `Idempotency-Key` 请求头；`topic::generate_idempotency_key()` 可生成，`create_topic` / `create_reply` 会自动带上该头与 `origin`。
- 发布主题/回帖是**异步审核**：返回 `202` 与 `submissionId`，内容先落库、送第三方审核，审核期间他人不可见；用投稿接口（`get_post_submission` 等）查询状态。
- 主题/楼层的 `author` 是精简对象（`id`/`username`/`avatar`/`displayedBadge`），用 `TopicAuthorDTO`，不要用完整的 `GetUserInformationDTO`。
- 媒体上传（`POST /api/v1/files`）需要 reqwest 的 `multipart` feature（已在 `Cargo.toml` 启用）；文件/头像/表情读取接口返回原始 `Vec<u8>`，不走 `json()`。
