//! Shared helpers for the integration tests.
//!
//! Each integration test binary compiles this module separately, so some items
//! are unused in the offline-only binary; `#![allow(dead_code)]` keeps the
//! compiler quiet about that.

#![allow(dead_code)]

use std::sync::Once;

use pickcat_api_collection::PickcatAccound;
use pickcat_api_collection::user::UserBehavior;

/// Initializes logging once for the test binary and loads `.env`.
///
/// Uses a default filter of `info`, so logs show up with
/// `cargo test -- --nocapture`; override with `RUST_LOG=debug`.
pub fn init_logging() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        dotenvy::dotenv().ok();
        let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .try_init();
        log::info!("test logging initialized");
    });
}

/// Shared, authenticated account. `OnceCell` guarantees a single login even
/// when the ignored tests run in parallel.
static ACCOUNT: tokio::sync::OnceCell<PickcatAccound> = tokio::sync::OnceCell::const_new();

/// Serializes the live tests: the exam flow mutates server-side state and
/// other tests must not observe a half-finished attempt.
pub static LIVE_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

/// Logs in once, reading credentials from `.env` (or the environment).
///
/// `PickcatAccound::new` ignores the login response, so the session is fetched
/// here to surface bad credentials instead of failing later with a confusing
/// parse error.
pub async fn account() -> &'static PickcatAccound {
    ACCOUNT
        .get_or_init(|| async {
            init_logging();

            let username = std::env::var("USERNAME").expect("USERNAME is not set (add it to .env)");
            let password = std::env::var("PASSWORD").expect("PASSWORD is not set (add it to .env)");

            log::info!("logging in as {username}");
            let account = PickcatAccound::new(&username, &password)
                .await
                .expect("failed to construct PickcatAccound");

            let session = account
                .get_current_user_session()
                .await
                .expect("login failed; check USERNAME/PASSWORD in .env");
            log::info!(
                "login ok: user id={}, username={}, level=Lv.{}, expiresAt={}",
                session.user.id,
                session.user.username,
                session.user.level.current,
                session.expires_at
            );

            account
        })
        .await
}
