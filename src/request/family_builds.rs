use serde_json::json;
use crate::model::*;
use crate::PaperMcClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct FamilyBuildsRequest<'a> {
    pub(crate) http_client: &'a PaperMcClient,
    pub family: String,
    pub project: String,
}
impl<'a> FamilyBuildsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<VersionFamilyBuildsResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v2/projects/{project}/version_group/{family}/builds", family = self
                    .family, project = self.project
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for FamilyBuildsRequest<'a> {
    type Output = httpclient::InMemoryResult<VersionFamilyBuildsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}