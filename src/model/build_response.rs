
use serde::{Serialize, Deserialize};
use super::Change;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BuildResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<Change>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloads: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promoted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for BuildResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}