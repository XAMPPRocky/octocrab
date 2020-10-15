use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequest {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub html_url: Url,
    pub diff_url: Url,
    pub patch_url: Url,
    pub issue_url: Url,
    pub commits_url: Url,
    pub review_comments_url: Url,
    pub review_comment_url: Url,
    pub comments_url: Url,
    pub statuses_url: Url,
    pub number: i64,
    pub state: IssueState,
    pub locked: bool,
    pub title: String,
    pub user: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Milestone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_lock_reason: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_at: Option<DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_commit_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<User>,
    pub assignees: Vec<User>,
    pub requested_reviewers: Vec<User>,
    pub requested_teams: Vec<teams::RequestedTeam>,
    pub head: Head,
    pub base: Base,
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    pub author_association: String,
    pub draft: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<Repository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Head {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub sha: String,
    pub user: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<Repository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Base {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub sha: String,
    pub user: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<Repository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Links {
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<SelfLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_link: Option<HtmlLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_link: Option<IssueLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_link: Option<CommentsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_comments_link: Option<ReviewCommentsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_comment_link: Option<ReviewCommentLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits_link: Option<CommitsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses_link: Option<StatusesLink>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SelfLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HtmlLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommentsLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReviewCommentsLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReviewCommentLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommitsLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StatusesLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Review {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    pub user: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ReviewState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum ReviewState {
    Approved,
    Pending,
    ChangesRequested,
}
