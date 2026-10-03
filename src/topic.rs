use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::dto::topic::{
    CreatePostResponseDTO, CreateReplyDTO, CreateTopicDTO, PostSubmissionDTO, TopicDetailDTO,
    TopicListItemDTO, TopicPostDTO, TopicRecommendationsDTO,
};
use crate::dto::user::CursorListDTO;
use crate::{Error, PickcatAccound};

/// 主题与内容接口，来自文档的「主题与内容」章节
/// (<https://pickcat-docs.xiaole6324.fun/topic.html>)。
pub trait TopicBehavior {
    /// `GET /api/v1/topics` —— 公开。`tags` 中的每个标签都会作为独立的
    /// `tag` 参数重复传入；`tag_mode` 通常为 `all`。
    fn list_topics(
        &self,
        limit: Option<u32>,
        tags: &[String],
        tag_mode: Option<&str>,
        cursor: Option<&str>,
    ) -> impl std::future::Future<Output = Result<CursorListDTO<TopicListItemDTO>, Error>> + Send;

    /// `GET /api/v1/topic-recommendations` —— 公开。
    fn get_topic_recommendations(
        &self,
        sort: Option<&str>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> impl std::future::Future<Output = Result<TopicRecommendationsDTO, Error>> + Send;

    /// `GET /api/v1/topics/{topicId}` —— 公开。
    fn get_topic(
        &self,
        topic_id: &str,
    ) -> impl std::future::Future<Output = Result<TopicDetailDTO, Error>> + Send;

    /// `GET /api/v1/topics/{topicId}/posts` —— 公开，游标分页。
    fn get_topic_posts(
        &self,
        topic_id: &str,
        limit: Option<u32>,
        sort: Option<&str>,
        cursor: Option<&str>,
    ) -> impl std::future::Future<Output = Result<CursorListDTO<TopicPostDTO>, Error>> + Send;

    /// `POST /api/v1/posts` —— 需要会话，发布主题首楼。
    ///
    /// `idempotency_key` 会作为必填的 `Idempotency-Key` 请求头发送；可用
    /// [`generate_idempotency_key`] 生成。成功返回 `202`（异步审核）。
    fn create_topic(
        &self,
        idempotency_key: &str,
        title: &str,
        markdown: &str,
        kind: &str,
        tag_ids: Vec<String>,
    ) -> impl std::future::Future<Output = Result<CreatePostResponseDTO, Error>> + Send;

    /// `POST /api/v1/posts` —— 需要会话，发布回帖（可楼中楼）。
    fn create_reply(
        &self,
        idempotency_key: &str,
        topic_id: &str,
        markdown: &str,
        reply_to_post_number: Option<u64>,
    ) -> impl std::future::Future<Output = Result<CreatePostResponseDTO, Error>> + Send;

    /// `GET /api/v1/post-submissions` —— 需要会话，投稿审核列表。
    fn list_post_submissions(
        &self,
        content_role: Option<&str>,
        status: Option<&str>,
        topic_id: Option<&str>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> impl std::future::Future<Output = Result<CursorListDTO<PostSubmissionDTO>, Error>> + Send;

    /// `GET /api/v1/post-submissions/{submissionId}` —— 需要会话。
    fn get_post_submission(
        &self,
        submission_id: &str,
    ) -> impl std::future::Future<Output = Result<PostSubmissionDTO, Error>> + Send;
}

/// 生成一个 UUID v4 形式的字符串，用作 `Idempotency-Key`。
///
/// 只用时间戳 + 进程内计数器，不做密码学随机；仅用于保证请求幂等。
pub fn generate_idempotency_key() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let count = u128::from(COUNTER.fetch_add(1, Ordering::Relaxed));
    let value = nanos ^ (count << 64);

    let mut hex: Vec<char> = format!("{value:032x}").chars().collect();
    hex[12] = '4'; // version 4
    hex[16] = '8'; // variant
    let hex: String = hex.into_iter().collect();

    format!(
        "{}-{}-{}-{}-{}",
        &hex[0..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..32]
    )
}

/// 只在有值时追加查询参数。
fn push_query(params: &mut Vec<(&str, String)>, key: &'static str, value: Option<String>) {
    if let Some(value) = value {
        params.push((key, value));
    }
}

impl TopicBehavior for PickcatAccound {
    async fn list_topics(
        &self,
        limit: Option<u32>,
        tags: &[String],
        tag_mode: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<CursorListDTO<TopicListItemDTO>, Error> {
        let mut params: Vec<(&str, String)> = Vec::new();
        push_query(&mut params, "limit", limit.map(|v| v.to_string()));
        for tag in tags {
            params.push(("tag", tag.clone()));
        }
        push_query(&mut params, "tagMode", tag_mode.map(str::to_string));
        push_query(&mut params, "cursor", cursor.map(str::to_string));

        let mut request = self.client.get(format!("{}/api/v1/topics", self.base_url));
        if !params.is_empty() {
            request = request.query(&params);
        }
        Ok(request
            .send()
            .await?
            .json::<CursorListDTO<TopicListItemDTO>>()
            .await?)
    }

    async fn get_topic_recommendations(
        &self,
        sort: Option<&str>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<TopicRecommendationsDTO, Error> {
        let mut params: Vec<(&str, String)> = Vec::new();
        push_query(&mut params, "sort", sort.map(str::to_string));
        push_query(&mut params, "limit", limit.map(|v| v.to_string()));
        push_query(&mut params, "cursor", cursor.map(str::to_string));

        let mut request = self
            .client
            .get(format!("{}/api/v1/topic-recommendations", self.base_url));
        if !params.is_empty() {
            request = request.query(&params);
        }
        Ok(request
            .send()
            .await?
            .json::<TopicRecommendationsDTO>()
            .await?)
    }

    async fn get_topic(&self, topic_id: &str) -> Result<TopicDetailDTO, Error> {
        Ok(self
            .client
            .get(format!("{}/api/v1/topics/{}", self.base_url, topic_id))
            .send()
            .await?
            .json::<TopicDetailDTO>()
            .await?)
    }

    async fn get_topic_posts(
        &self,
        topic_id: &str,
        limit: Option<u32>,
        sort: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<CursorListDTO<TopicPostDTO>, Error> {
        let mut params: Vec<(&str, String)> = Vec::new();
        push_query(&mut params, "limit", limit.map(|v| v.to_string()));
        push_query(&mut params, "sort", sort.map(str::to_string));
        push_query(&mut params, "cursor", cursor.map(str::to_string));

        let mut request = self.client.get(format!(
            "{}/api/v1/topics/{}/posts",
            self.base_url, topic_id
        ));
        if !params.is_empty() {
            request = request.query(&params);
        }
        Ok(request
            .send()
            .await?
            .json::<CursorListDTO<TopicPostDTO>>()
            .await?)
    }

    async fn create_topic(
        &self,
        idempotency_key: &str,
        title: &str,
        markdown: &str,
        kind: &str,
        tag_ids: Vec<String>,
    ) -> Result<CreatePostResponseDTO, Error> {
        let dto = CreateTopicDTO {
            title: title.to_string(),
            markdown: markdown.to_string(),
            kind: kind.to_string(),
            tag_ids,
        };
        Ok(self
            .client
            .post(format!("{}/api/v1/posts", self.base_url))
            .header("Idempotency-Key", idempotency_key)
            .header("origin", self.base_url.clone())
            .json(&dto)
            .send()
            .await?
            .json::<CreatePostResponseDTO>()
            .await?)
    }

    async fn create_reply(
        &self,
        idempotency_key: &str,
        topic_id: &str,
        markdown: &str,
        reply_to_post_number: Option<u64>,
    ) -> Result<CreatePostResponseDTO, Error> {
        let dto = CreateReplyDTO {
            topic_id: topic_id.to_string(),
            markdown: markdown.to_string(),
            reply_to_post_number,
        };
        Ok(self
            .client
            .post(format!("{}/api/v1/posts", self.base_url))
            .header("Idempotency-Key", idempotency_key)
            .header("origin", self.base_url.clone())
            .json(&dto)
            .send()
            .await?
            .json::<CreatePostResponseDTO>()
            .await?)
    }

    async fn list_post_submissions(
        &self,
        content_role: Option<&str>,
        status: Option<&str>,
        topic_id: Option<&str>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<CursorListDTO<PostSubmissionDTO>, Error> {
        let mut params: Vec<(&str, String)> = Vec::new();
        push_query(&mut params, "contentRole", content_role.map(str::to_string));
        push_query(&mut params, "status", status.map(str::to_string));
        push_query(&mut params, "topicId", topic_id.map(str::to_string));
        push_query(&mut params, "limit", limit.map(|v| v.to_string()));
        push_query(&mut params, "cursor", cursor.map(str::to_string));

        let mut request = self
            .client
            .get(format!("{}/api/v1/post-submissions", self.base_url));
        if !params.is_empty() {
            request = request.query(&params);
        }
        Ok(request
            .send()
            .await?
            .json::<CursorListDTO<PostSubmissionDTO>>()
            .await?)
    }

    async fn get_post_submission(&self, submission_id: &str) -> Result<PostSubmissionDTO, Error> {
        Ok(self
            .client
            .get(format!(
                "{}/api/v1/post-submissions/{}",
                self.base_url, submission_id
            ))
            .send()
            .await?
            .json::<PostSubmissionDTO>()
            .await?)
    }
}
