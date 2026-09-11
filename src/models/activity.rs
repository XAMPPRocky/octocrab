use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Notification {
    pub id: NotificationId,
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
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ThreadSubscription {
    pub subscribed: bool,
    pub ignored: bool,
    pub reason: Option<Reason>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub url: Url,
    pub thread_url: Url,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StarredRepository {
    pub repo: Repository,
    pub starred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepositorySubscription {
    pub subscribed: bool,
    pub ignored: bool,
    pub reason: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub url: Url,
    pub repository_url: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Feeds {
    pub timeline_url: String,
    pub user_url: String,
    pub current_user_public_url: Option<String>,
    pub current_user_url: Option<String>,
    pub current_user_actor_url: Option<String>,
    pub current_user_organization_url: Option<String>,
    #[serde(default)]
    pub current_user_organization_urls: Vec<String>,
    pub security_advisories_url: Option<String>,
    pub repository_discussions_url: Option<String>,
    pub repository_discussions_category_url: Option<String>,
    #[serde(rename = "_links")]
    pub links: FeedLinks,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FeedLink {
    pub href: String,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FeedLinks {
    pub timeline: FeedLink,
    pub user: FeedLink,
    pub security_advisories: Option<FeedLink>,
    pub current_user: Option<FeedLink>,
    pub current_user_public: Option<FeedLink>,
    pub current_user_actor: Option<FeedLink>,
    pub current_user_organization: Option<FeedLink>,
    #[serde(default)]
    pub current_user_organizations: Vec<FeedLink>,
    pub repository_discussions: Option<FeedLink>,
    pub repository_discussions_category: Option<FeedLink>,
}

