use serde_json::json;
use crate::model::*;
use crate::PaperMcClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ProjectRequest<'a> {
    pub(crate) http_client: &'a PaperMcClient,
    pub project: String,
}
impl<'a> ProjectRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ProjectResponse> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/v2/projects/{project}", project = self.project));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for ProjectRequest<'a> {
    type Output = httpclient::InMemoryResult<ProjectResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}