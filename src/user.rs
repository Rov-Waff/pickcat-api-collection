use crate::{BASE_URL, Error, PickcatAccound};

use crate::dto::user::{
    CurrentQuestionDTO, ExamStatusDTO, RegistrationDTO, SendRegistrationDTO, SendVerifyEmailDTO,
    SessionDTO, StartExamDTO, SubmitExamAnswerDTO, SubmitExamAnswerResponseDTO, VerifyEmailDTO,
};

pub trait UserBehavior {
    fn get_current_user_session(
        &self,
    ) -> impl std::future::Future<Output = Result<SessionDTO, Error>> + Send;
    fn post_registration(
        &self,
        username: &str,
        email: &str,
        captcha: &str,
    ) -> impl std::future::Future<Output = Result<RegistrationDTO, Error>> + Send;
    fn verify_email(
        &self,
        id: &str,
        code: &str,
        password: &str,
    ) -> impl Future<Output = Result<VerifyEmailDTO, Error>> + Send;
    fn get_exam_status(&self) -> impl Future<Output = Result<ExamStatusDTO, Error>> + Send;
    fn start_exam(&self) -> impl Future<Output = Result<StartExamDTO, Error>> + Send;
    fn get_current_question(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<CurrentQuestionDTO, Error>> + Send;
    fn submit_answer(
        &self,
        exam_id: &str,
        question_id: &str,
        token: &str,
        selected_option_ids: Vec<String>,
    ) -> impl Future<Output = Result<SubmitExamAnswerResponseDTO, Error>> + Send;
}

impl UserBehavior for PickcatAccound {
    async fn get_current_user_session(&self) -> Result<SessionDTO, Error> {
        Ok(self
            .client
            .get(format!("{}/api/v1/session", BASE_URL))
            .send()
            .await?
            .json::<SessionDTO>()
            .await?)
    }

    async fn post_registration(
        &self,
        username: &str,
        email: &str,
        captcha: &str,
    ) -> Result<RegistrationDTO, Error> {
        let dto = SendRegistrationDTO {
            username: username.to_string(),
            email: email.to_string(),
            captcha_verify_param: captcha.to_string(),
        };
        Ok(self
            .client
            .post(format!("{}/api/v1/registrations", BASE_URL))
            .json(&dto)
            .send()
            .await?
            .json::<RegistrationDTO>()
            .await?)
    }

    async fn verify_email(
        &self,
        code: &str,
        password: &str,
        id: &str,
    ) -> Result<VerifyEmailDTO, Error> {
        let dto = SendVerifyEmailDTO {
            code: code.to_string(),
            password: password.to_string(),
        };
        Ok(self
            .client
            .patch(format!("{}/api/v1/registrations/{}", BASE_URL, id))
            .json(&dto)
            .send()
            .await?
            .json::<VerifyEmailDTO>()
            .await?)
    }

    async fn get_exam_status(&self) -> Result<ExamStatusDTO, Error> {
        Ok(self
            .client
            .get(format!("{}/api/v1/entrance-exam", BASE_URL))
            .send()
            .await?
            .json::<ExamStatusDTO>()
            .await?)
    }

    async fn start_exam(&self) -> Result<StartExamDTO, Error> {
        Ok(self
            .client
            .post(format!("{}/api/v1/entrance-exam/attempts", BASE_URL))
            .send()
            .await?
            .json::<StartExamDTO>()
            .await?)
    }

    async fn get_current_question(&self, id: &str) -> Result<CurrentQuestionDTO, Error> {
        Ok(self
            .client
            .get(format!(
                "{}/api/v1/entrance-exam/attempts/{}/current-question",
                BASE_URL, id
            ))
            .send()
            .await?
            .json::<CurrentQuestionDTO>()
            .await?)
    }

    async fn submit_answer(
        &self,
        exam_id: &str,
        question_id: &str,
        token: &str,
        selected_option_ids: Vec<String>,
    ) -> Result<SubmitExamAnswerResponseDTO, Error> {
        let dto = SubmitExamAnswerDTO {
            question_id: question_id.to_string(),
            delivery_token: token.to_string(),
            selected_option_ids: selected_option_ids,
        };
        Ok(self
            .client
            .patch(format!(
                "{}/api/v1/entrance-exam/attempts/{}/current-question",
                BASE_URL, exam_id
            ))
            .json(&dto)
            .send()
            .await?
            .json::<SubmitExamAnswerResponseDTO>()
            .await?)
    }
}
