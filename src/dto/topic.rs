use serde::{Deserialize, Serialize};

use super::user::{PageInfoDTO, TopicAuthorDTO, UserSubjectInfoDTO, UserSubjectTagDTO};

/// 主题列表项；与用户主页的主题列表项是同一结构。
pub type TopicListItemDTO = UserSubjectInfoDTO;

/// `GET /api/v1/topic-recommendations`：主题列表项额外带 `reason`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedTopicDTO {
    #[serde(flatten)]
    pub topic: UserSubjectInfoDTO,
    /// 推荐原因，如 `PINNED` / `TRENDING`。
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicRecommendationsDTO {
    pub strategy: String,
    pub items: Vec<RecommendedTopicDTO>,
    pub page_info: PageInfoDTO,
}

// ---------------------------------------------------------------------------
// 楼层
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostViewerCapabilitiesDTO {
    pub can_edit: bool,
    pub can_like: bool,
    pub can_bookmark: bool,
    pub can_report: bool,
    pub can_select_answer: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostViewerStateDTO {
    pub liked: bool,
    pub bookmark_id: Option<String>,
    pub selected_answer: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicPostDTO {
    pub id: String,
    pub topic_id: String,
    pub post_number: u64,
    pub reply_to_post_number: Option<u64>,
    pub deleted: bool,
    pub children: Vec<TopicPostDTO>,
    pub current_revision: u64,
    pub like_count: u64,
    pub pinned: bool,
    pub cooked_html: String,
    pub author: TopicAuthorDTO,
    pub created_at: String,
    pub edited_at: Option<String>,
    pub viewer_capabilities: PostViewerCapabilitiesDTO,
    pub viewer_state: PostViewerStateDTO,
}

// ---------------------------------------------------------------------------
// 主题详情
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicViewerCapabilitiesDTO {
    pub can_edit: bool,
    pub can_reply: bool,
    pub can_delete: bool,
    pub can_bookmark: bool,
    pub can_close: bool,
    pub can_reopen: bool,
    pub can_pin_globally: bool,
    pub pinnable_tag_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicViewerStateDTO {
    pub bookmark_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicDetailDTO {
    pub id: String,
    pub title: String,
    pub kind: String,
    pub author: TopicAuthorDTO,
    pub tags: Vec<UserSubjectTagDTO>,
    pub reply_count: u64,
    pub view_count: u64,
    pub like_count: u64,
    pub bookmark_count: u64,
    /// 所属合集，结构未文档化。
    pub collection: Option<serde_json::Value>,
    pub closed_at: Option<String>,
    /// 关闭操作人；文档标注为 DateTime/String，实际类型未验证。
    pub closed_by: Option<serde_json::Value>,
    pub pinned: bool,
    pub pinned_globally: bool,
    pub pinned_tag_id: Option<String>,
    pub pinned_at: Option<String>,
    pub pinned_until: Option<String>,
    pub last_activity_at: String,
    pub created_at: String,
    pub edited_at: Option<String>,
    pub updated_at: String,
    pub first_post: TopicPostDTO,
    pub replies_truncated: bool,
    /// 主题事件流，结构未文档化。
    pub events: Vec<serde_json::Value>,
    pub events_truncated: bool,
    pub viewer_capabilities: TopicViewerCapabilitiesDTO,
    pub viewer_state: TopicViewerStateDTO,
}

// ---------------------------------------------------------------------------
// 发布主题 / 回帖
// ---------------------------------------------------------------------------

/// 发布主题首楼（`POST /api/v1/posts`）。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTopicDTO {
    pub title: String,
    pub markdown: String,
    pub kind: String,
    pub tag_ids: Vec<String>,
}

/// 发布回帖（`POST /api/v1/posts`）。`reply_to_post_number` 用于楼中楼。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReplyDTO {
    pub topic_id: String,
    pub markdown: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_post_number: Option<u64>,
}

/// `POST /api/v1/posts` 的 202 响应（异步审核受理）。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostResponseDTO {
    pub submission_id: String,
    pub topic_id: String,
    pub post_id: String,
    pub status: String,
}

// ---------------------------------------------------------------------------
// 投稿审核
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostSubmissionDTO {
    pub id: String,
    pub status: String,
    pub content_role: String,
    /// 提交内容原样回显，随 `content_role` 变化，结构未固定。
    pub request: serde_json::Value,
    pub risk_level: Option<String>,
    pub topic_id: String,
    pub post_id: String,
    pub post_number: u64,
    pub base_revision: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
