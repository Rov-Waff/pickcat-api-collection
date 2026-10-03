//! Offline contract tests: these deserialize the exact JSON payloads from the
//! docs and guard the `#[serde(rename = ...)]` mappings. They need no network
//! or credentials and run with a plain `cargo test`.
//!
//! Sources:
//! * <https://pickcat-docs.xiaole6324.fun/auth/exam-content.html>
//! * <https://pickcat-docs.xiaole6324.fun/auth/entrance-exam.html>

mod common;

use log::{debug, info};
use pickcat_api_collection::dto::auth::{
    CurrentQuestionDTO, ExamStatusDTO, SessionDTO, StartExamDTO, SubmitExamAnswerResponseDTO,
};

#[test]
fn parses_session_payload() {
    common::init_logging();
    info!("parsing session payload fixture");
    let session: SessionDTO = serde_json::from_str(
        r#"{
                "user": {
                    "id": "01a0fbca-eee1-70ad-a00d-a1ed4c89b195",
                    "username": "CarbonPremium",
                    "createdAt": "2026-10-02T08:46:15.771Z",
                    "avatar": { "type": "PRESET", "id": 1, "url": "/api/v1/avatars/1?v=..." },
                    "level": { "current": 1 },
                    "effectivePermissions": []
                },
                "createdAt": "2026-10-02T10:19:14.833Z",
                "expiresAt": "2026-10-09T10:19:14.833Z",
                "silence": null
            }"#,
    )
    .expect("session payload must deserialize");

    debug!("parsed session: {session:?}");
    info!(
        "session user id={}, username={}, avatarType={}, level=Lv.{}",
        session.user.id,
        session.user.username,
        session.user.avatar.typs,
        session.user.level.current
    );

    assert_eq!(session.user.id, "01a0fbca-eee1-70ad-a00d-a1ed4c89b195");
    assert_eq!(session.user.username, "CarbonPremium");
    assert_eq!(session.user.created_at, "2026-10-02T08:46:15.771Z");
    assert_eq!(session.user.avatar.typs, "PRESET");
    assert_eq!(session.user.avatar.id, 1);
    assert_eq!(session.user.level.current, 1);
    assert_eq!(session.created_at, "2026-10-02T10:19:14.833Z");
    assert_eq!(session.expires_at, "2026-10-09T10:19:14.833Z");
    info!("session payload assertions passed");
}

#[test]
fn parses_exam_status_payloads() {
    common::init_logging();
    info!("parsing AVAILABLE exam status fixture");
    let available: ExamStatusDTO =
        serde_json::from_str(r#"{"state":"AVAILABLE","nextAttemptAt":"2026-10-02T12:56:52.310Z"}"#)
            .unwrap();
    assert_eq!(available.state, "AVAILABLE");
    assert!(available.result.is_none());
    info!(
        "AVAILABLE ok: state={}, result={:?}",
        available.state, available.result
    );

    info!("parsing COOLDOWN exam status fixture");
    let cooldown: ExamStatusDTO =
        serde_json::from_str(r#"{"state":"COOLDOWN","nextAttemptAt":"2026-10-03T08:37:35.348Z"}"#)
            .unwrap();
    assert_eq!(cooldown.state, "COOLDOWN");
    assert!(cooldown.result.is_none());
    info!(
        "COOLDOWN ok: state={}, result={:?}",
        cooldown.state, cooldown.result
    );

    info!("parsing PASSED exam status fixture");
    let passed: ExamStatusDTO = serde_json::from_str(
        r#"{
                "state": "PASSED",
                "result": {
                    "attemptId": "01a0fcb5-97ab-70a8-a38f-e184ca006064",
                    "status": "PASSED",
                    "totalQuestions": 10,
                    "correctCount": 10,
                    "requiredCorrectCount": 9,
                    "startedAt": "2026-10-02T13:02:34.401Z",
                    "finishedAt": "2026-10-02T13:07:24.494Z"
                }
            }"#,
    )
    .unwrap();
    assert_eq!(passed.state, "PASSED");
    let result = passed.result.expect("PASSED must carry a result");
    assert_eq!(result.attempt_id, "01a0fcb5-97ab-70a8-a38f-e184ca006064");
    assert_eq!(result.status, "PASSED");
    assert_eq!(result.total_questions, 10);
    assert_eq!(result.correct_count, 10);
    assert_eq!(result.required_correct_count, 9);
    info!(
        "PASSED ok: {}/{} correct (required {}), finishedAt={}",
        result.correct_count,
        result.total_questions,
        result.required_correct_count,
        result.finished_at
    );
}

#[test]
fn parses_start_exam_payload() {
    common::init_logging();
    info!("parsing start-exam payload fixture");
    let started: StartExamDTO = serde_json::from_str(
        r#"{
                "state": "IN_PROGRESS",
                "attempt": {
                    "attemptId": "01a0fcb5-97ab-70a8-a38f-e184ca006064",
                    "totalQuestions": 10,
                    "completedQuestions": 0,
                    "currentOrdinal": 0,
                    "deadlineAt": "2026-10-02T13:12:34.401Z",
                    "startedAt": "2026-10-02T13:02:34.401Z"
                }
            }"#,
    )
    .unwrap();

    debug!("parsed start exam: {started:?}");
    info!(
        "attempt {}: totalQuestions={}, completed={}, currentOrdinal={}, deadlineAt={}",
        started.attempt.attempt_id,
        started.attempt.total_questions,
        started.attempt.completed_questions,
        started.attempt.current_ordinal,
        started.attempt.deadline_at
    );

    assert_eq!(started.state, "IN_PROGRESS");
    assert_eq!(started.attempt.total_questions, 10);
    assert_eq!(started.attempt.completed_questions, 0);
    assert_eq!(started.attempt.current_ordinal, 0);
    assert_eq!(
        started.attempt.attempt_id,
        "01a0fcb5-97ab-70a8-a38f-e184ca006064"
    );
    info!("start-exam payload assertions passed");
}

#[test]
fn parses_current_question_payload() {
    common::init_logging();
    info!("parsing current-question payload fixture");
    let question: CurrentQuestionDTO = serde_json::from_str(
        r#"{
                "state": "QUESTION",
                "attemptId": "01a0fcb5-97ab-70a8-a38f-e184ca006064",
                "questionId": "01a0fcb5-0000-7000-8000-000000000001",
                "ordinal": 0,
                "totalQuestions": 10,
                "questionType": "MULTIPLE_CHOICE",
                "stemHtml": "<p>题干</p>",
                "options": [
                    { "id": "opt-1", "position": 0, "contentHtml": "<p>选项一</p>" },
                    { "id": "opt-2", "position": 1, "contentHtml": "<p>选项二</p>" }
                ],
                "deliveryToken": "one-shot-token",
                "deadlineAt": "2026-10-02T13:12:34.401Z"
            }"#,
    )
    .unwrap();

    debug!("parsed question: {question:?}");
    info!(
        "question #{} of {}: id={}, type={}, options={}",
        question.ordinal,
        question.total_questions,
        question.question_id,
        question.question_type,
        question.options.len()
    );
    for option in &question.options {
        debug!(
            "option position={} id={} content={}",
            option.position, option.id, option.content_html
        );
    }

    assert_eq!(question.state, "QUESTION");
    assert_eq!(question.question_type, "MULTIPLE_CHOICE");
    assert_eq!(question.options.len(), 2);
    assert_eq!(question.options[0].id, "opt-1");
    assert_eq!(question.options[0].position, 0);
    assert_eq!(question.delivery_token, "one-shot-token");
    info!("current-question payload assertions passed");
}

#[test]
fn parses_submit_answer_intermediate_and_final_payloads() {
    common::init_logging();
    info!("parsing IN_PROGRESS submit-answer payload fixture");
    let intermediate: SubmitExamAnswerResponseDTO = serde_json::from_str(
        r#"{
                "state": "IN_PROGRESS",
                "attempt": {
                    "attemptId": "01a0fcb5-97ab-70a8-a38f-e184ca006064",
                    "totalQuestions": 10,
                    "completedQuestions": 1,
                    "currentOrdinal": 1,
                    "deadlineAt": "2026-10-02T13:13:34.401Z",
                    "startedAt": "2026-10-02T13:02:34.401Z"
                }
            }"#,
    )
    .unwrap();
    assert_eq!(intermediate.state, "IN_PROGRESS");
    let attempt = intermediate.attempt.as_ref().expect("attempt present");
    info!(
        "IN_PROGRESS ok: completed={}, currentOrdinal={}",
        attempt.completed_questions, attempt.current_ordinal
    );
    assert_eq!(attempt.completed_questions, 1);
    assert!(intermediate.result.is_none());

    info!("parsing FINISHED submit-answer payload fixture");
    let finished: SubmitExamAnswerResponseDTO = serde_json::from_str(
        r#"{
                "state": "FINISHED",
                "result": {
                    "attemptId": "01a0fcb5-97ab-70a8-a38f-e184ca006064",
                    "status": "PASSED",
                    "totalQuestions": 10,
                    "correctCount": 10,
                    "requiredCorrectCount": 9,
                    "startedAt": "2026-10-02T13:02:34.401Z",
                    "finishedAt": "2026-10-02T13:07:24.494Z"
                }
            }"#,
    )
    .unwrap();
    assert_eq!(finished.state, "FINISHED");
    assert!(finished.attempt.is_none());
    let result = finished.result.expect("FINISHED must carry a result");
    assert_eq!(result.status, "PASSED");
    info!(
        "FINISHED ok: status={}, {}/{} correct",
        result.status, result.correct_count, result.total_questions
    );
}
