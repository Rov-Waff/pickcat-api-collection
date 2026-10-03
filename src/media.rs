use crate::dto::media::{FileListDTO, UploadedFileDTO};
use crate::{Error, PickcatAccound};

/// 媒体资源接口，来自文档的「媒体资源」章节
/// (<https://pickcat-docs.xiaole6324.fun/media.html>)。
///
/// 二进制读取（文件/头像/表情）在浏览器里通常直接由 `<img>` 加载，这里返回
/// 原始字节，方便调用方自行处理缓存 / MIME。
pub trait MediaBehavior {
    /// `POST /api/v1/files` —— 需要会话，`multipart/form-data`。
    ///
    /// `idempotency_key` 会作为必填的 `Idempotency-Key` 请求头发送
    /// （可用 [`crate::topic::generate_idempotency_key`] 生成）。
    fn upload_file(
        &self,
        idempotency_key: &str,
        ownership: &str,
        file_name: &str,
        bytes: Vec<u8>,
    ) -> impl std::future::Future<Output = Result<UploadedFileDTO, Error>> + Send;

    /// `GET /api/v1/files` —— 需要会话，游标分页。
    fn list_files(
        &self,
        scope: Option<&str>,
        state: Option<&str>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> impl std::future::Future<Output = Result<FileListDTO, Error>> + Send;

    /// `GET /api/v1/files/{fileId}` —— 公开，返回图片字节。
    fn get_file(
        &self,
        file_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    /// `GET /api/v1/avatars/{id}` —— 公开，`id` 为预设编号或自定义头像 `fileId`。
    fn get_avatar(
        &self,
        id: &str,
        version: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    /// `GET /api/v1/emojis/{emojiName}/image` —— 公开，返回表情图片字节。
    fn get_emoji_image(
        &self,
        emoji_name: &str,
        emoji_read: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;
}

impl MediaBehavior for PickcatAccound {
    async fn upload_file(
        &self,
        idempotency_key: &str,
        ownership: &str,
        file_name: &str,
        bytes: Vec<u8>,
    ) -> Result<UploadedFileDTO, Error> {
        let part = reqwest::multipart::Part::bytes(bytes).file_name(file_name.to_string());
        let form = reqwest::multipart::Form::new()
            .text("ownership", ownership.to_string())
            .part("file", part);

        Ok(self
            .client
            .post(format!("{}/api/v1/files", self.base_url))
            .header("Idempotency-Key", idempotency_key)
            .header("origin", self.base_url.clone())
            .multipart(form)
            .send()
            .await?
            .json::<UploadedFileDTO>()
            .await?)
    }

    async fn list_files(
        &self,
        scope: Option<&str>,
        state: Option<&str>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<FileListDTO, Error> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(scope) = scope {
            params.push(("scope", scope.to_string()));
        }
        if let Some(state) = state {
            params.push(("state", state.to_string()));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit.to_string()));
        }
        if let Some(cursor) = cursor {
            params.push(("cursor", cursor.to_string()));
        }

        let mut request = self.client.get(format!("{}/api/v1/files", self.base_url));
        if !params.is_empty() {
            request = request.query(&params);
        }
        Ok(request.send().await?.json::<FileListDTO>().await?)
    }

    async fn get_file(&self, file_id: &str) -> Result<Vec<u8>, Error> {
        Ok(self
            .client
            .get(format!("{}/api/v1/files/{}", self.base_url, file_id))
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?
            .to_vec())
    }

    async fn get_avatar(&self, id: &str, version: Option<&str>) -> Result<Vec<u8>, Error> {
        let mut request = self
            .client
            .get(format!("{}/api/v1/avatars/{}", self.base_url, id));
        if let Some(version) = version {
            request = request.query(&[("v", version)]);
        }
        Ok(request
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?
            .to_vec())
    }

    async fn get_emoji_image(
        &self,
        emoji_name: &str,
        emoji_read: Option<&str>,
    ) -> Result<Vec<u8>, Error> {
        let mut request = self.client.get(format!(
            "{}/api/v1/emojis/{}/image",
            self.base_url, emoji_name
        ));
        if let Some(emoji_read) = emoji_read {
            request = request.query(&[("emoji-read", emoji_read)]);
        }
        Ok(request
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?
            .to_vec())
    }
}
