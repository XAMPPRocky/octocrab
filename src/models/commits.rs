use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Comment {
    pub html_url: Url,
    pub url: Url,
    pub id: CommentId,
    pub node_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u64>,
    pub commit_id: String,
    pub user: Author,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub author_association: String,
}
