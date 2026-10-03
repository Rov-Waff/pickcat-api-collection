//! 用户章节：资料、邮箱、关注/粉丝、主题/回帖、徽章、动态、等级、配额等。
//!
//! 参见 <https://pickcat-docs.xiaole6324.fun/user.html>。

use crate::dto::user::*;
use crate::{Error, PickcatAccound};

/// User-chapter endpoints from the docs
/// (<https://pickcat-docs.xiaole6324.fun/user.html>): profile, content,
/// badges, activities, level and account quotas.
///
/// Registration / email verification and the exam flow live in [`crate::auth`].
pub trait UserProfileBehavior {
    /// `GET /api/v1/users/{userId}` — public, works with or without a session.
    fn get_user_information(
        &self,
        user_id: &str,
    ) -> impl std::future::Future<Output = Result<GetUserInformationDTO, Error>> + Send;

    /// `PATCH /api/v1/users/{userId}` — requires a session; only the current
    /// user may edit their own profile. `None` fields are left unchanged.
    fn update_user_profile(
        &self,
        user_id: &str,
        bio: Option<&str>,
    ) -> impl std::future::Future<Output = Result<GetUserInformationDTO, Error>> + Send;

    /// `GET /api/v1/users/{userId}/email` — requires a session and only works
    /// for the current user.
    fn get_user_email(
        &self,
        user_id: &str,
    ) -> impl std::future::Future<Output = Result<GetUserEmailDTO, Error>> + Send;

    /// `GET /api/v1/users/{userId}/following` — public, cursor-paginated.
    fn get_user_following(
        &self,
        user_id: &str,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> impl std::future::Future<Output = Result<CursorListDTO<GetUserInformationDTO>, Error>> + Send;

    /// `GET /api/v1/users/{userId}/followers` — public, cursor-paginated.
    fn get_user_followers(
        &self,
        user_id: &str,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> impl std::future::Future<Output = Result<CursorListDTO<GetUserInformationDTO>, Error>> + Send;

    /// `GET /api/v1/users/{userId}/topics` — public, cursor-paginated.
    fn get_user_topics(
        &self,
        user_id: &str,
        limit: Option<u32>,
    ) -> impl std::future::Future<Output = Result<UserSubjectsDTO, Error>> + Send;

    /// `GET /api/v1/users/{userId}/posts?role=reply` — public, cursor-paginated.
    fn get_user_posts(
        &self,
        user_id: &str,
        role: Option<&str>,
        limit: Option<u32>,
    ) -> impl std::future::Future<Output = Result<UserRepliesDTO, Error>> + Send;

    /// `GET /api/v1/users/{userId}/featured-topics` — public, no pagination.
    fn get_user_featured_topics(
        &self,
        user_id: &str,
    ) -> impl std::future::Future<Output = Result<UserFeaturedTopicsDTO, Error>> + Send;

    /// `GET /api/v1/users/{userId}/topic-collections` — public, paginated.
    ///
    /// Item shape is not documented yet, so items are kept as raw JSON.
    fn get_user_topic_collections(
        &self,
        user_id: &str,
        limit: Option<u32>,
    ) -> impl std::future::Future<Output = Result<CursorListDTO<serde_json::Value>, Error>> + Send;

    /// `GET /api/v1/users/{userId}/badges` — public.
    fn get_user_badges(
        &self,
        user_id: &str,
    ) -> impl std::future::Future<Output = Result<UserBadgesDTO, Error>> + Send;

    /// `GET /api/v1/users/{userId}/badge-display` — public.
    fn get_user_badge_display(
        &self,
        user_id: &str,
    ) -> impl std::future::Future<Output = Result<BadgeDisplayDTO, Error>> + Send;

    /// `GET /api/v1/user-badge-displays?userIds=a,b` — public bulk lookup.
    fn get_user_badge_displays(
        &self,
        user_ids: &[String],
    ) -> impl std::future::Future<Output = Result<UserBadgeDisplaysDTO, Error>> + Send;

    /// `GET /api/v1/users/{userId}/profile-activities` — public, paginated.
    fn get_user_profile_activities(
        &self,
        user_id: &str,
        year: Option<u32>,
        limit: Option<u32>,
    ) -> impl std::future::Future<Output = Result<CursorListDTO<ProfileActivityDTO>, Error>> + Send;

    /// `GET /api/v1/profile-activities` — requires a session.
    fn get_my_profile_activities(
        &self,
        year: Option<u32>,
        limit: Option<u32>,
    ) -> impl std::future::Future<Output = Result<CursorListDTO<ProfileActivityDTO>, Error>> + Send;

    /// `GET /api/v1/users/{userId}/level-contributions/{year}` — public.
    fn get_user_level_contributions(
        &self,
        user_id: &str,
        year: u32,
    ) -> impl std::future::Future<Output = Result<LevelContributionsDTO, Error>> + Send;

    /// `GET /api/v1/level-contributions/{year}` — requires a session.
    fn get_my_level_contributions(
        &self,
        year: u32,
    ) -> impl std::future::Future<Output = Result<LevelContributionsDTO, Error>> + Send;

    /// `GET /api/v1/level-progress` — requires a session.
    fn get_level_progress(
        &self,
    ) -> impl std::future::Future<Output = Result<LevelProgressDTO, Error>> + Send;

    /// `GET /api/v1/content-length-limit` — requires a session.
    fn get_content_length_limit(
        &self,
    ) -> impl std::future::Future<Output = Result<ContentLengthLimitDTO, Error>> + Send;

    /// `GET /api/v1/topic-collection-usage` — requires a session.
    fn get_topic_collection_usage(
        &self,
    ) -> impl std::future::Future<Output = Result<TopicCollectionUsageDTO, Error>> + Send;

    /// `GET /api/v1/file-storage` — requires a session.
    fn get_file_storage(
        &self,
    ) -> impl std::future::Future<Output = Result<FileStorageDTO, Error>> + Send;

    /// `GET /api/v1/bookmarks` — requires a session, paginated.
    ///
    /// Item shape is not documented yet, so items are kept as raw JSON.
    fn get_bookmarks(
        &self,
        limit: Option<u32>,
    ) -> impl std::future::Future<Output = Result<CursorListDTO<serde_json::Value>, Error>> + Send;

    /// `GET /api/v1/avatar-presets` — public.
    fn get_avatar_presets(
        &self,
    ) -> impl std::future::Future<Output = Result<AvatarPresetsDTO, Error>> + Send;
}

/// Adds `limit` / `cursor`-style query parameters only when present.
fn push_query(params: &mut Vec<(&str, String)>, key: &'static str, value: Option<String>) {
    if let Some(value) = value {
        params.push((key, value));
    }
}

impl UserProfileBehavior for PickcatAccound {
    async fn get_user_information(&self, user_id: &str) -> Result<GetUserInformationDTO, Error> {
        Ok(self
            .client
            .get(format!("{}/api/v1/users/{}", self.base_url, user_id))
            .send()
            .await?
            .json::<GetUserInformationDTO>()
            .await?)
    }

    async fn update_user_profile(
        &self,
        user_id: &str,
        bio: Option<&str>,
    ) -> Result<GetUserInformationDTO, Error> {
        let dto = UpdateUserProfileDTO {
            bio: bio.map(str::to_string),
        };
        Ok(self
            .client
            .patch(format!("{}/api/v1/users/{}", self.base_url, user_id))
            .json(&dto)
            .send()
            .await?
            .json::<GetUserInformationDTO>()
            .await?)
    }

    async fn get_user_email(&self, user_id: &str) -> Result<GetUserEmailDTO, Error> {
        Ok(self
            .client
            .get(format!("{}/api/v1/users/{}/email", self.base_url, user_id))
            .send()
            .await?
            .json::<GetUserEmailDTO>()
            .await?)
    }

    async fn get_user_following(
        &self,
        user_id: &str,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<CursorListDTO<GetUserInformationDTO>, Error> {
        let mut params: Vec<(&str, String)> = Vec::new();
        push_query(&mut params, "limit", limit.map(|v| v.to_string()));
        push_query(&mut params, "cursor", cursor.map(str::to_string));

        let mut request = self.client.get(format!(
            "{}/api/v1/users/{}/following",
            self.base_url, user_id
        ));
        if !params.is_empty() {
            request = request.query(&params);
        }
        Ok(request
            .send()
            .await?
            .json::<CursorListDTO<GetUserInformationDTO>>()
            .await?)
    }

    async fn get_user_followers(
        &self,
        user_id: &str,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<CursorListDTO<GetUserInformationDTO>, Error> {
        let mut params: Vec<(&str, String)> = Vec::new();
        push_query(&mut params, "limit", limit.map(|v| v.to_string()));
        push_query(&mut params, "cursor", cursor.map(str::to_string));

        let mut request = self.client.get(format!(
            "{}/api/v1/users/{}/followers",
            self.base_url, user_id
        ));
        if !params.is_empty() {
            request = request.query(&params);
        }
        Ok(request
            .send()
            .await?
            .json::<CursorListDTO<GetUserInformationDTO>>()
            .await?)
    }

    async fn get_user_topics(
        &self,
        user_id: &str,
        limit: Option<u32>,
    ) -> Result<UserSubjectsDTO, Error> {
        let mut request = self
            .client
            .get(format!("{}/api/v1/users/{}/topics", self.base_url, user_id));
        if let Some(limit) = limit {
            request = request.query(&[("limit", limit)]);
        }
        Ok(request.send().await?.json::<UserSubjectsDTO>().await?)
    }

    async fn get_user_posts(
        &self,
        user_id: &str,
        role: Option<&str>,
        limit: Option<u32>,
    ) -> Result<UserRepliesDTO, Error> {
        let mut params: Vec<(&str, String)> = Vec::new();
        push_query(&mut params, "role", role.map(str::to_string));
        push_query(&mut params, "limit", limit.map(|v| v.to_string()));

        let mut request = self
            .client
            .get(format!("{}/api/v1/users/{}/posts", self.base_url, user_id));
        if !params.is_empty() {
            request = request.query(&params);
        }
        Ok(request.send().await?.json::<UserRepliesDTO>().await?)
    }

    async fn get_user_featured_topics(
        &self,
        user_id: &str,
    ) -> Result<UserFeaturedTopicsDTO, Error> {
        Ok(self
            .client
            .get(format!(
                "{}/api/v1/users/{}/featured-topics",
                self.base_url, user_id
            ))
            .send()
            .await?
            .json::<UserFeaturedTopicsDTO>()
            .await?)
    }

    async fn get_user_topic_collections(
        &self,
        user_id: &str,
        limit: Option<u32>,
    ) -> Result<CursorListDTO<serde_json::Value>, Error> {
        let mut request = self.client.get(format!(
            "{}/api/v1/users/{}/topic-collections",
            self.base_url, user_id
        ));
        if let Some(limit) = limit {
            request = request.query(&[("limit", limit)]);
        }
        Ok(request
            .send()
            .await?
            .json::<CursorListDTO<serde_json::Value>>()
            .await?)
    }

    async fn get_user_badges(&self, user_id: &str) -> Result<UserBadgesDTO, Error> {
        Ok(self
            .client
            .get(format!("{}/api/v1/users/{}/badges", self.base_url, user_id))
            .send()
            .await?
            .json::<UserBadgesDTO>()
            .await?)
    }

    async fn get_user_badge_display(&self, user_id: &str) -> Result<BadgeDisplayDTO, Error> {
        Ok(self
            .client
            .get(format!(
                "{}/api/v1/users/{}/badge-display",
                self.base_url, user_id
            ))
            .send()
            .await?
            .json::<BadgeDisplayDTO>()
            .await?)
    }

    async fn get_user_badge_displays(
        &self,
        user_ids: &[String],
    ) -> Result<UserBadgeDisplaysDTO, Error> {
        Ok(self
            .client
            .get(format!("{}/api/v1/user-badge-displays", self.base_url))
            .query(&[("userIds", user_ids.join(","))])
            .send()
            .await?
            .json::<UserBadgeDisplaysDTO>()
            .await?)
    }

    async fn get_user_profile_activities(
        &self,
        user_id: &str,
        year: Option<u32>,
        limit: Option<u32>,
    ) -> Result<CursorListDTO<ProfileActivityDTO>, Error> {
        let mut params: Vec<(&str, String)> = Vec::new();
        push_query(&mut params, "year", year.map(|v| v.to_string()));
        push_query(&mut params, "limit", limit.map(|v| v.to_string()));

        let mut request = self.client.get(format!(
            "{}/api/v1/users/{}/profile-activities",
            self.base_url, user_id
        ));
        if !params.is_empty() {
            request = request.query(&params);
        }
        Ok(request
            .send()
            .await?
            .json::<CursorListDTO<ProfileActivityDTO>>()
            .await?)
    }

    async fn get_my_profile_activities(
        &self,
        year: Option<u32>,
        limit: Option<u32>,
    ) -> Result<CursorListDTO<ProfileActivityDTO>, Error> {
        let mut params: Vec<(&str, String)> = Vec::new();
        push_query(&mut params, "year", year.map(|v| v.to_string()));
        push_query(&mut params, "limit", limit.map(|v| v.to_string()));

        let mut request = self
            .client
            .get(format!("{}/api/v1/profile-activities", self.base_url));
        if !params.is_empty() {
            request = request.query(&params);
        }
        Ok(request
            .send()
            .await?
            .json::<CursorListDTO<ProfileActivityDTO>>()
            .await?)
    }

    async fn get_user_level_contributions(
        &self,
        user_id: &str,
        year: u32,
    ) -> Result<LevelContributionsDTO, Error> {
        Ok(self
            .client
            .get(format!(
                "{}/api/v1/users/{}/level-contributions/{}",
                self.base_url, user_id, year
            ))
            .send()
            .await?
            .json::<LevelContributionsDTO>()
            .await?)
    }

    async fn get_my_level_contributions(&self, year: u32) -> Result<LevelContributionsDTO, Error> {
        Ok(self
            .client
            .get(format!(
                "{}/api/v1/level-contributions/{}",
                self.base_url, year
            ))
            .send()
            .await?
            .json::<LevelContributionsDTO>()
            .await?)
    }

    async fn get_level_progress(&self) -> Result<LevelProgressDTO, Error> {
        Ok(self
            .client
            .get(format!("{}/api/v1/level-progress", self.base_url))
            .send()
            .await?
            .json::<LevelProgressDTO>()
            .await?)
    }

    async fn get_content_length_limit(&self) -> Result<ContentLengthLimitDTO, Error> {
        Ok(self
            .client
            .get(format!("{}/api/v1/content-length-limit", self.base_url))
            .send()
            .await?
            .json::<ContentLengthLimitDTO>()
            .await?)
    }

    async fn get_topic_collection_usage(&self) -> Result<TopicCollectionUsageDTO, Error> {
        Ok(self
            .client
            .get(format!("{}/api/v1/topic-collection-usage", self.base_url))
            .send()
            .await?
            .json::<TopicCollectionUsageDTO>()
            .await?)
    }

    async fn get_file_storage(&self) -> Result<FileStorageDTO, Error> {
        Ok(self
            .client
            .get(format!("{}/api/v1/file-storage", self.base_url))
            .send()
            .await?
            .json::<FileStorageDTO>()
            .await?)
    }

    async fn get_bookmarks(
        &self,
        limit: Option<u32>,
    ) -> Result<CursorListDTO<serde_json::Value>, Error> {
        let mut request = self
            .client
            .get(format!("{}/api/v1/bookmarks", self.base_url));
        if let Some(limit) = limit {
            request = request.query(&[("limit", limit)]);
        }
        Ok(request
            .send()
            .await?
            .json::<CursorListDTO<serde_json::Value>>()
            .await?)
    }

    async fn get_avatar_presets(&self) -> Result<AvatarPresetsDTO, Error> {
        Ok(self
            .client
            .get(format!("{}/api/v1/avatar-presets", self.base_url))
            .send()
            .await?
            .json::<AvatarPresetsDTO>()
            .await?)
    }
}
