use crate::dto::tag::{TagSidebarLinksDTO, TagsDTO};
use crate::{Error, PickcatAccound};

/// 分区（标签）接口，来自文档的「分区标签」章节
/// (<https://pickcat-docs.xiaole6324.fun/tag.html>)。
pub trait TagBehavior {
    /// `GET /api/v1/tags` —— 公开。`assignable = true` 时只返回可发帖分区。
    fn list_tags(
        &self,
        assignable: Option<bool>,
    ) -> impl std::future::Future<Output = Result<TagsDTO, Error>> + Send;

    /// `GET /api/v1/tags/{slug}/sidebar-links` —— 公开。
    fn get_tag_sidebar_links(
        &self,
        slug: &str,
    ) -> impl std::future::Future<Output = Result<TagSidebarLinksDTO, Error>> + Send;
}

impl TagBehavior for PickcatAccound {
    async fn list_tags(&self, assignable: Option<bool>) -> Result<TagsDTO, Error> {
        let mut request = self.client.get(format!("{}/api/v1/tags", self.base_url));
        if let Some(assignable) = assignable {
            request = request.query(&[("assignable", assignable)]);
        }
        Ok(request.send().await?.json::<TagsDTO>().await?)
    }

    async fn get_tag_sidebar_links(&self, slug: &str) -> Result<TagSidebarLinksDTO, Error> {
        Ok(self
            .client
            .get(format!(
                "{}/api/v1/tags/{}/sidebar-links",
                self.base_url, slug
            ))
            .send()
            .await?
            .json::<TagSidebarLinksDTO>()
            .await?)
    }
}
