
use serde::{Serialize, Deserialize};
use super::VersionFamilyBuild;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VersionFamilyBuildsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builds: Option<Vec<VersionFamilyBuild>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<String>>,
}
impl std::fmt::Display for VersionFamilyBuildsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}