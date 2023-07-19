use serde_json::json;
use crate::model::*;
use crate::PaperMcClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DownloadRequest<'a> {
    pub(crate) http_client: &'a PaperMcClient,
    pub build: i64,
    pub download: String,
    pub project: String,
    pub version: String,
}
impl<'a> DownloadRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v2/projects/{project}/versions/{version}/builds/{build}/downloads/{download}",
                    build = self.build, download = self.download, project = self.project,
                    version = self.version
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
pub struct DownloadRequired<'a> {
    pub build: i64,
    pub download: &'a str,
    pub project: &'a str,
    pub version: &'a str,
}
impl<'a> DownloadRequired<'a> {}
impl<'a> ::std::future::IntoFuture for DownloadRequest<'a> {
    type Output = httpclient::InMemoryResult<serde_json::Value>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}