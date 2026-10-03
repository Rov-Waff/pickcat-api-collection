//! Mock-server tests for `src/auth.rs`: they assert the exact method, path and
//! request body the client sends, without touching the real API.

mod common;

use serde_json::json;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use pickcat_api_collection::auth::UserBehavior;

const SESSION_BODY: &str = r#"{
    "user": {
        "id": "u1", "username": "mock-user",
        "createdAt": "2026-10-02T08:46:15.771Z",
        "avatar": { "type": "PRESET", "id": 1, "url": "/api/v1/avatars/1" },
        "level": { "current": 1 }
    },
    "createdAt": "2026-10-02T10:19:14.833Z",
    "expiresAt": "2026-10-09T10:19:14.833Z"
}"#;

#[tokio::test]
async fn get_current_user_session_hits_session_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/session"))
        .respond_with(ResponseTemplate::new(200).set_body_string(SESSION_BODY))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let session = account.get_current_user_session().await.unwrap();
    assert_eq!(session.user.id, "u1");
    assert_eq!(session.user.username, "mock-user");
    server.verify().await;
}

#[tokio::test]
async fn get_exam_status_hits_entrance_exam() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/entrance-exam"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "state": "AVAILABLE" })))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let status = account.get_exam_status().await.unwrap();
    assert_eq!(status.state, "AVAILABLE");
    assert!(status.result.is_none());
    server.verify().await;
}

#[tokio::test]
async fn start_exam_posts_attempts() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v1/entrance-exam/attempts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "state": "IN_PROGRESS",
            "attempt": {
                "attemptId": "a1", "totalQuestions": 10, "completedQuestions": 0,
                "currentOrdinal": 0, "deadlineAt": "2026-10-02T13:12:34.401Z",
                "startedAt": "2026-10-02T13:02:34.401Z"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let started = account.start_exam().await.unwrap();
    assert_eq!(started.attempt.attempt_id, "a1");
    server.verify().await;
}

#[tokio::test]
async fn get_current_question_uses_attempt_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/entrance-exam/attempts/a1/current-question"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "state": "QUESTION", "attemptId": "a1", "questionId": "q1",
            "ordinal": 0, "totalQuestions": 10, "questionType": "SINGLE_CHOICE",
            "stemHtml": "<p>q</p>",
            "options": [{ "id": "o1", "position": 0, "contentHtml": "<p>a</p>" }],
            "deliveryToken": "t1", "deadlineAt": "2026-10-02T13:12:34.401Z"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let question = account.get_current_question("a1").await.unwrap();
    assert_eq!(question.question_id, "q1");
    assert_eq!(question.delivery_token, "t1");
    server.verify().await;
}

#[tokio::test]
async fn submit_answer_patches_with_expected_body() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/api/v1/entrance-exam/attempts/a1/current-question"))
        .and(body_json(json!({
            "questionId": "q1",
            "deliveryToken": "t1",
            "selectedOptionIds": ["o1"]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "state": "FINISHED",
            "result": {
                "attemptId": "a1", "status": "PASSED", "totalQuestions": 10,
                "correctCount": 9, "requiredCorrectCount": 9,
                "startedAt": "2026-10-02T13:02:34.401Z",
                "finishedAt": "2026-10-02T13:07:24.494Z"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let account = common::mock_account(&server).await;
    let response = account
        .submit_answer("a1", "q1", "t1", vec!["o1".to_string()])
        .await
        .unwrap();
    assert_eq!(response.state, "FINISHED");
    assert!(response.result.is_some());
    server.verify().await;
}
