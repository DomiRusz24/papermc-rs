//! [`PaperMcClient`](struct.PaperMcClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;
mod serde;
pub struct PaperMcClient {
    pub client: httpclient::Client,
}
impl PaperMcClient {
    pub fn from_env() -> Self {
        Self {
            client: httpclient::Client::new().base_url("https://api.papermc.io"),
        }
    }
}
impl PaperMcClient {
    pub fn new(url: &str) -> Self {
        let client = httpclient::Client::new().base_url(url);
        Self { client }
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    ///Gets a list of all available projects.
    pub fn projects(&self) -> request::ProjectsRequest {
        request::ProjectsRequest {
            http_client: &self,
        }
    }
    ///Gets information about a project.
    pub fn project(&self, project: &str) -> request::ProjectRequest {
        request::ProjectRequest {
            http_client: &self,
            project: project.to_owned(),
        }
    }
    ///Gets information about a version.
    pub fn version(&self, project: &str, version: &str) -> request::VersionRequest {
        request::VersionRequest {
            http_client: &self,
            project: project.to_owned(),
            version: version.to_owned(),
        }
    }
    ///Gets all available builds for a project's version.
    pub fn builds(&self, project: &str, version: &str) -> request::BuildsRequest {
        request::BuildsRequest {
            http_client: &self,
            project: project.to_owned(),
            version: version.to_owned(),
        }
    }
    ///Gets information related to a specific build.
    pub fn build(
        &self,
        build: i64,
        project: &str,
        version: &str,
    ) -> request::BuildRequest {
        request::BuildRequest {
            http_client: &self,
            build,
            project: project.to_owned(),
            version: version.to_owned(),
        }
    }
    ///Downloads the given file from a build's data.
    pub fn download(&self, args: request::DownloadRequired) -> request::DownloadRequest {
        request::DownloadRequest {
            http_client: &self,
            build: args.build,
            download: args.download.to_owned(),
            project: args.project.to_owned(),
            version: args.version.to_owned(),
        }
    }
    ///Gets information about a project's version group.
    pub fn family(&self, family: &str, project: &str) -> request::FamilyRequest {
        request::FamilyRequest {
            http_client: &self,
            family: family.to_owned(),
            project: project.to_owned(),
        }
    }
    ///Gets all available builds for a project's version group.
    pub fn family_builds(
        &self,
        family: &str,
        project: &str,
    ) -> request::FamilyBuildsRequest {
        request::FamilyBuildsRequest {
            http_client: &self,
            family: family.to_owned(),
            project: project.to_owned(),
        }
    }
}