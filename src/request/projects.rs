use serde_json::json;
use crate::model::*;
use crate::PaperMcClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ProjectsRequest<'a> {
    pub(crate) http_client: &'a PaperMcClient,
}
impl<'a> ProjectsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ProjectsResponse> {
        let mut r = self.http_client.client.get("/v2/projects");
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for ProjectsRequest<'a> {
    type Output = httpclient::InMemoryResult<ProjectsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}