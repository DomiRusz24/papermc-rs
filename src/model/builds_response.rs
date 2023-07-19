
use serde::{Serialize, Deserialize};
use super::VersionBuild;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BuildsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builds: Option<Vec<VersionBuild>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for BuildsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}