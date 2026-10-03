//! Live tests against `https://cdsq.dao3.fun`.
//!
//! These read `USERNAME`/`PASSWORD` from `.env` (or the environment) and are
//! `#[ignore]`d because they need valid credentials and, in the case of the
//! exam flow, mutate server-side state:
//!
//! ```sh
//! cargo test --test live -- --ignored --nocapture
//! ```
//!
//! Registration/email-verification endpoints are intentionally not tested: they
//! require a CAPTCHA (`captchaVerifyParam`) that cannot be produced from CI.

mod common;

use log::{debug, info, warn};
use pickcat_api_collection::auth::UserBehavior;

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_login_constructs_authenticated_account() {
    common::init_logging();
    let _guard = common::LIVE_LOCK.lock().await;

    let account = common::account().await;
    assert!(!account.username.is_empty());
    info!("shared account authenticated for {}", account.username);
}

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_get_current_user_session() {
    common::init_logging();
    let _guard = common::LIVE_LOCK.lock().await;

    info!("GET /api/v1/session");
    let session = common::account()
        .await
        .get_current_user_session()
        .await
        .unwrap();
    info!(
        "session: user id={}, username={}, avatarType={}, level=Lv.{}",
        session.user.id,
        session.user.username,
        session.user.avatar.typs,
        session.user.level.current
    );
    info!(
        "session window: createdAt={}, expiresAt={}",
        session.created_at, session.expires_at
    );

    assert!(!session.user.id.is_empty());
    assert!(!session.user.username.is_empty());
    assert!(!session.user.avatar.url.is_empty());
    assert!(!session.user.created_at.is_empty());
    assert!(!session.created_at.is_empty());
    assert!(!session.expires_at.is_empty());
    info!("live session assertions passed");
}

#[tokio::test]
#[ignore = "hits the live API; run with `cargo test -- --ignored`"]
async fn live_get_exam_status() {
    common::init_logging();
    let _guard = common::LIVE_LOCK.lock().await;

    info!("GET /api/v1/entrance-exam");
    let status = common::account().await.get_exam_status().await.unwrap();
    info!(
        "exam status: state={}, result={:?}",
        status.state, status.result
    );

    assert!(
        matches!(status.state.as_str(), "AVAILABLE" | "PASSED" | "COOLDOWN"),
        "unexpected exam state: {}",
        status.state
    );

    if status.state == "PASSED" {
        let result = status.result.expect("PASSED must carry a result");
        info!(
            "PASSED result: {}/{} correct (required {}), attempt={}",
            result.correct_count,
            result.total_questions,
            result.required_correct_count,
            result.attempt_id
        );
        assert_eq!(result.status, "PASSED");
        assert_eq!(result.total_questions, 10);
        assert!(result.correct_count >= result.required_correct_count);
    } else {
        assert!(status.result.is_none());
    }
    info!("live exam status assertions passed");
}

/// Walks the exam-taking endpoints from the docs:
/// start -> current question -> submit one answer.
///
/// Only runs when the account's exam state is `AVAILABLE`; otherwise it reports
/// why it was skipped. It deliberately stops after one answer, so it leaves an
/// in-progress attempt behind (the account's own deadline will expire it). Do
/// not run it against a real user account you care about.
#[tokio::test]
#[ignore = "starts a real exam attempt; run with `cargo test -- --ignored`"]
async fn live_start_exam_and_submit_one_answer() {
    common::init_logging();
    let _guard = common::LIVE_LOCK.lock().await;
    let account = common::account().await;

    info!("GET /api/v1/entrance-exam (pre-flight)");
    let status = account.get_exam_status().await.unwrap();
    info!("pre-flight exam state: {}", status.state);
    if status.state != "AVAILABLE" {
        warn!("skipping exam flow: exam state is {}", status.state);
        return;
    }

    info!("POST /api/v1/entrance-exam/attempts");
    let started = account.start_exam().await.unwrap();
    info!(
        "attempt started: id={}, totalQuestions={}, completedQuestions={}, currentOrdinal={}, deadlineAt={}",
        started.attempt.attempt_id,
        started.attempt.total_questions,
        started.attempt.completed_questions,
        started.attempt.current_ordinal,
        started.attempt.deadline_at
    );

    let attempt_id = started.attempt.attempt_id.clone();
    info!("GET /api/v1/entrance-exam/attempts/{attempt_id}/current-question");
    let question = account.get_current_question(&attempt_id).await.unwrap();
    info!(
        "question #{} of {}: id={}, type={}, options={}, deadlineAt={}",
        question.ordinal,
        question.total_questions,
        question.question_id,
        question.question_type,
        question.options.len(),
        question.deadline_at
    );
    debug!("stem: {}", question.stem_html);
    debug!(
        "deliveryToken present: {} chars",
        question.delivery_token.len()
    );
    for option in &question.options {
        debug!(
            "option position={} id={} content={}",
            option.position, option.id, option.content_html
        );
    }

    // Same strategy as the docs' demo: submit the first option.
    let selected = vec![question.options[0].id.clone()];
    info!(
        "PATCH /api/v1/entrance-exam/attempts/{attempt_id}/current-question selecting {selected:?}"
    );
    let response = account
        .submit_answer(
            &attempt_id,
            &question.question_id,
            &question.delivery_token,
            selected,
        )
        .await
        .unwrap();
    info!(
        "submit response: state={}, attempt={:?}, result={:?}",
        response.state, response.attempt, response.result
    );

    assert_eq!(started.state, "IN_PROGRESS");
    assert_eq!(started.attempt.total_questions, 10);
    assert!(!started.attempt.attempt_id.is_empty());
    assert_eq!(question.state, "QUESTION");
    assert_eq!(question.attempt_id, attempt_id);
    assert_eq!(question.ordinal, 0);
    assert_eq!(question.total_questions, 10);
    assert!(!question.question_id.is_empty());
    assert!(!question.stem_html.is_empty());
    assert!(!question.delivery_token.is_empty());
    assert!(!question.options.is_empty());
    assert!(matches!(
        question.question_type.as_str(),
        "SINGLE_CHOICE" | "MULTIPLE_CHOICE"
    ));

    match response.state.as_str() {
        "IN_PROGRESS" => {
            let attempt = response.attempt.expect("IN_PROGRESS must carry an attempt");
            info!(
                "intermediate: completed={}, currentOrdinal={}",
                attempt.completed_questions, attempt.current_ordinal
            );
            assert_eq!(attempt.completed_questions, 1);
            assert!(response.result.is_none());
        }
        "FINISHED" => {
            let result = response.result.expect("FINISHED must carry a result");
            info!(
                "finished: status={}, {}/{} correct",
                result.status, result.correct_count, result.total_questions
            );
            assert_eq!(result.total_questions, 10);
        }
        other => panic!("unexpected submit response state: {other}"),
    }
    info!("live exam flow assertions passed");
}
