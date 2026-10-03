use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserInformationDTO {
    pub id: String,
    pub username: String,
    pub avatar: UserAvatarDTO,
    pub bio: Option<String>,
    pub region: Option<String>,
    pub show_following_list: bool,
    pub show_followers_list: bool,
    pub created_at: String,
    pub level: UserLevelDTO,
    pub stats: UserStatsDTO,
    pub viewer_state: UserViewerStateDTO,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAvatarDTO {
    #[serde(rename = "type")]
    pub avatar_type: String,
    pub id: u64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLevelDTO {
    pub current: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatsDTO {
    pub followers: u64,
    pub following: u64,
    pub topics: u64,
    pub replies: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserViewerStateDTO {
    pub following: bool,
    pub can_follow: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserEmailDTO {
    pub email: String,
    pub verified_at: String,
}

/// 主题/楼层列表项里的作者：只含 `id`/`username`/`avatar`/`displayedBadge`，
/// 与完整的 `GetUserInformationDTO` 不同。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicAuthorDTO {
    pub id: String,
    pub username: String,
    pub avatar: UserAvatarDTO,
    pub displayed_badge: Option<DisplayedBadgeDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSubjectInfoDTO {
    pub id: String,
    pub title: String,
    pub kind: String,
    pub excerpt: String,
    pub author: TopicAuthorDTO,
    pub tags: Vec<UserSubjectTagDTO>,
    pub reply_count: u64,
    pub view_count: u64,
    pub like_count: u64,
    pub bookmark_count: u64,
    /// 仅 `kind == "QUESTION"` 时存在，结构未文档化。
    pub question_state: Option<serde_json::Value>,
    pub closed_at: Option<String>,
    pub pinned: bool,
    pub pinned_globally: bool,
    pub pinned_tag_id: Option<String>,
    pub pinned_at: Option<String>,
    pub pinned_until: Option<String>,
    pub created_at: String,
    pub edited_at: Option<String>,
    pub last_activity_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubjectTagDTO {
    pub id: String,
    pub slug: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSubjectsDTO {
    pub items: Vec<UserSubjectInfoDTO>,
    pub page_info: UserSubjectsPageInfoDTO,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSubjectsPageInfoDTO {
    pub has_next_page: bool,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRepliesDTO {
    pub items: Vec<UserReplyDTO>,
    pub page_info: UserRepliesPageInfoDTO,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserReplyDTO {
    pub id: String,
    pub topic_id: String,
    pub post_number: u64,
    pub reply_to_post_number: Option<u64>,
    pub deleted: bool,
    pub children: Vec<UserReplyDTO>,
    pub current_revision: u64,
    pub like_count: u64,
    pub pinned: bool,
    pub cooked_html: String,
    pub author: TopicAuthorDTO,
    pub created_at: String,
    pub edited_at: Option<String>,
    pub viewer_capabilities: UserReplyViewerCapabilitiesDTO,
    pub viewer_state: UserReplyViewerStateDTO,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRepliesPageInfoDTO {
    pub has_next_page: bool,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserReplyViewerCapabilitiesDTO {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserReplyViewerStateDTO {}

// ---------------------------------------------------------------------------
// 通用游标分页
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfoDTO {
    pub has_next_page: bool,
    pub next_cursor: Option<String>,
}

/// `{ "items": [...], "pageInfo": {...} }` 形式的通用游标分页响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CursorListDTO<T> {
    pub items: Vec<T>,
    pub page_info: PageInfoDTO,
}

// ---------------------------------------------------------------------------
// PATCH /users/{userId}
// ---------------------------------------------------------------------------

/// 只提交需要变更的字段；`None` 的字段不会出现在请求体里。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserProfileDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
}

// ---------------------------------------------------------------------------
// GET /users/{userId}/featured-topics
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFeaturedTopicsDTO {
    pub items: Vec<UserSubjectInfoDTO>,
}

// ---------------------------------------------------------------------------
// 徽章
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadgeImageDTO {
    pub file_id: String,
    pub url: String,
    pub width: u64,
    pub height: u64,
}

/// `GET /users/{userId}/badges` 中的完整徽章详情。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadgeDTO {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub version: u64,
    pub image: BadgeImageDTO,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBadgeDTO {
    pub user_id: String,
    pub badge_id: String,
    pub granted_at: String,
    pub revision: String,
    pub badge: BadgeDTO,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBadgesDTO {
    pub items: Vec<UserBadgeDTO>,
}

/// 批量/展示接口里返回的精简徽章对象。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayedBadgeDTO {
    pub id: String,
    pub name: String,
    pub description: String,
    pub image: BadgeImageDTO,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadgeDisplayDTO {
    pub mode: String,
    pub badge_id: Option<String>,
    pub displayed_badge: Option<DisplayedBadgeDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBadgeDisplayItemDTO {
    pub user_id: String,
    pub displayed_badge: Option<DisplayedBadgeDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBadgeDisplaysDTO {
    pub items: Vec<UserBadgeDisplayItemDTO>,
}

// ---------------------------------------------------------------------------
// 用户动态
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileActivityDTO {
    pub id: String,
    /// `POSTED` / `LIKED` / `LEVEL_UP`
    pub kind: String,
    pub occurred_at: String,
    pub title: Option<String>,
    pub level: Option<u64>,
}

// ---------------------------------------------------------------------------
// 贡献日历
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelContributionDayDTO {
    pub date: String,
    pub ability: u64,
    pub responsibility: u64,
    pub care: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelContributionsDTO {
    pub year: u64,
    pub time_zone: String,
    pub days: Vec<LevelContributionDayDTO>,
    pub updating: bool,
    pub calculated_at: String,
}

// ---------------------------------------------------------------------------
// 等级进度
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelScoresDTO {
    pub ability: u64,
    pub responsibility: u64,
    pub care: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelFamiliarityDTO {
    pub tracking: bool,
    pub familiarity_started_at: String,
    pub days_since_entrance_exam: u64,
    pub valid_visit_days: u64,
    pub topics_entered: u64,
    pub posts_read: u64,
    pub effective_reading_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelRequirementDTO {
    pub current: u64,
    pub required: u64,
    pub met: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextLevelScoresDTO {
    pub ability: LevelRequirementDTO,
    pub responsibility: LevelRequirementDTO,
    pub care: LevelRequirementDTO,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextLevelFamiliarityDTO {
    pub days_since_entrance_exam: LevelRequirementDTO,
    pub valid_visit_days: LevelRequirementDTO,
    pub topics_entered: LevelRequirementDTO,
    pub posts_read: LevelRequirementDTO,
    pub effective_reading_seconds: LevelRequirementDTO,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardRequirementDTO {
    pub key: String,
    pub met: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextLevelDTO {
    pub level: u64,
    pub admission: String,
    pub scores: NextLevelScoresDTO,
    pub familiarity: NextLevelFamiliarityDTO,
    pub hard_requirements: Vec<HardRequirementDTO>,
    pub blocked_by_promotion_ceiling: bool,
    pub eligible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaDTO {
    pub action: String,
    pub limit: u64,
    pub used: u64,
    pub remaining: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelProgressDTO {
    pub current_level: u64,
    /// 文档中观测为 `null`，类型未验证。
    pub promotion_ceiling: Option<serde_json::Value>,
    pub scores: LevelScoresDTO,
    pub familiarity: LevelFamiliarityDTO,
    pub next_level: Option<NextLevelDTO>,
    pub lv4_candidate: bool,
    pub quotas: Vec<QuotaDTO>,
    pub updating: bool,
    pub calculated_at: String,
}

// ---------------------------------------------------------------------------
// 配额 / 上限
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentLengthLimitDTO {
    pub level: u64,
    pub topic_max_length: u64,
    pub reply_max_length: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicCollectionUsageDTO {
    pub current_level: u64,
    pub limit_count: u64,
    pub used_count: u64,
    pub remaining_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileStorageDTO {
    pub used_bytes: u64,
    pub limit_bytes: u64,
    pub remaining_bytes: u64,
}

// ---------------------------------------------------------------------------
// 预设头像
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarPresetDTO {
    #[serde(rename = "type")]
    pub avatar_type: String,
    pub id: u64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarPresetsDTO {
    pub items: Vec<AvatarPresetDTO>,
}
