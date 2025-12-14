use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodeOfConduct {
    pub key: String,
    pub name: String,
    pub url: String,
    pub html_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}
