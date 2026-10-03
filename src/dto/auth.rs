//! 鉴权与入站考试 DTO。

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarFieldInUserFieldInSessionDTO {
    #[serde(rename = "type")]
    pub typs: String,
    pub id: u32,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelFieldInUserFieldInSessionDTO {
    pub current: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserFieldInSessionDTO {
    pub id: String,
    pub username: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub avatar: AvatarFieldInUserFieldInSessionDTO,
    pub level: LevelFieldInUserFieldInSessionDTO,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionDTO {
    pub user: UserFieldInSessionDTO,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationDTO {
    id: String,
    status: String,
    expires_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendRegistrationDTO {
    pub username: String,
    pub email: String,
    #[serde(rename = "captchaVerifyParam")]
    pub captcha_verify_param: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendVerifyEmailDTO {
    pub code: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarFieldInUserDTO {
    #[serde(rename = "type")]
    pub typs: String,
    pub id: u32,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelFieldInUserDTO {
    pub current: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsFieldInUserDTO {
    pub followers: u32,
    pub following: u32,
    pub topics: u32,
    pub replies: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewerStateFieldInUserDTO {
    pub following: bool,
    #[serde(rename = "canFollow")]
    pub can_follow: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyEmailDTO {
    pub id: String,
    pub username: String,
    pub avatar: AvatarFieldInUserDTO,
    pub bio: Option<String>,
    pub region: Option<String>,
    #[serde(rename = "showFollowingList")]
    pub show_following_list: bool,
    #[serde(rename = "showFollowersList")]
    pub show_followers_list: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub level: LevelFieldInUserDTO,
    pub stats: StatsFieldInUserDTO,
    #[serde(rename = "viewerState")]
    pub viewer_state: ViewerStateFieldInUserDTO,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamStatusResultDTO {
    #[serde(rename = "attemptId")]
    pub attempt_id: String,
    pub status: String,
    #[serde(rename = "totalQuestions")]
    pub total_questions: u32,
    #[serde(rename = "correctCount")]
    pub correct_count: u32,
    #[serde(rename = "requiredCorrectCount")]
    pub required_correct_count: u32,
    #[serde(rename = "startedAt")]
    pub started_at: String,
    #[serde(rename = "finishedAt")]
    pub finished_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamStatusDTO {
    pub state: String,
    pub result: Option<ExamStatusResultDTO>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartExamAttemptDTO {
    #[serde(rename = "attemptId")]
    pub attempt_id: String,
    #[serde(rename = "totalQuestions")]
    pub total_questions: u32,
    #[serde(rename = "completedQuestions")]
    pub completed_questions: u32,
    #[serde(rename = "currentOrdinal")]
    pub current_ordinal: u32,
    #[serde(rename = "deadlineAt")]
    pub deadline_at: String,
    #[serde(rename = "startedAt")]
    pub started_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartExamDTO {
    pub state: String,
    pub attempt: StartExamAttemptDTO,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentQuestionOptionDTO {
    pub id: String,
    pub position: u32,
    #[serde(rename = "contentHtml")]
    pub content_html: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentQuestionDTO {
    pub state: String,
    #[serde(rename = "attemptId")]
    pub attempt_id: String,
    #[serde(rename = "questionId")]
    pub question_id: String,
    pub ordinal: u32,
    #[serde(rename = "totalQuestions")]
    pub total_questions: u32,
    #[serde(rename = "questionType")]
    pub question_type: String,
    #[serde(rename = "stemHtml")]
    pub stem_html: String,
    pub options: Vec<CurrentQuestionOptionDTO>,
    #[serde(rename = "deliveryToken")]
    pub delivery_token: String,
    #[serde(rename = "deadlineAt")]
    pub deadline_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitExamAnswerDTO {
    #[serde(rename = "questionId")]
    pub question_id: String,
    #[serde(rename = "deliveryToken")]
    pub delivery_token: String,
    #[serde(rename = "selectedOptionIds")]
    pub selected_option_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendSubmitAnswerDTO {
    #[serde(rename = "questionId")]
    pub question_id: String,
    #[serde(rename = "deliveryToken")]
    pub delivery_token: String,
    #[serde(rename = "selectedOptionIds")]
    pub selected_option_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitExamAnswerResponseDTO {
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<StartExamAttemptDTO>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ExamStatusResultDTO>,
}
