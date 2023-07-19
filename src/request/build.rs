use serde_json::json;
use crate::model::*;
use crate::PaperMcClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct BuildRequest<'a> {
    pub(crate) http_client: &'a PaperMcClient,
    pub build: i64,
    pub project: String,
    pub version: String,
}
impl<'a> BuildRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<BuildResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v2/projects/{project}/versions/{version}/builds/{build}", build =
                    self.build, project = self.project, version = self.version
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for BuildRequest<'a> {
    type Output = httpclient::InMemoryResult<BuildResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}