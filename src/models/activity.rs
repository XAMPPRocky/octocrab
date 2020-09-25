use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Notification {
    #[serde(deserialize_with = "parse_u32")]
    pub id: u64,
    pub repository: Repository,
    pub subject: Subject,
    pub reason: String,
    pub unread: bool,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub last_read_at: Option<chrono::DateTime<chrono::Utc>>,
    pub url: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Reason {
    Assign,
    Author,
    Comment,
    Invitation,
    Manual,
    Mention,
    #[serde(rename = "review_requested")]
    ReviewRequested,
    #[serde(rename = "security_alert")]
    SecurityAlert,
    #[serde(rename = "state_change")]
    StateChange,
    Subscribed,
    #[serde(rename = "team_mention")]
    TeamMention,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Subject {
    pub title: String,
    pub url: Option<Url>,
    pub latest_comment_url: Option<Url>,
    #[serde(rename = "type")]
    pub type_: String,
}

fn parse_u32<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;

    let raw = String::deserialize(deserializer)?;
    match raw.parse() {
        Ok(val) => Ok(val),
        Err(_) => Err(D::Error::custom("expected `id` to be a number")),
    }
}
