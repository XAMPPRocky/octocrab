use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Team {
    pub id: i64,
    pub node_id: String,
    pub url: Url,
    pub html_url: Url,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub privacy: String,
    pub permission: String,
    pub members_url: Url,
    pub repositories_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repos_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<orgs::Organization>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RequestedReviewers {
    pub users: Vec<User>,
    pub teams: Vec<Team>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RequestedTeam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    pub name: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub privacy: String,
    pub permission: String,
    pub members_url: Url,
    pub repositories_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Team>,
}
