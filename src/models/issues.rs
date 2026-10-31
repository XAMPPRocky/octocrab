use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Issue {
    pub id: IssueId,
    pub node_id: String,
    pub url: Url,
    pub repository_url: Url,
    pub labels_url: Url,
    pub comments_url: Url,
    pub events_url: Url,
    pub html_url: Url,
    pub number: u64,
    pub state: IssueState,
    pub state_reason: Option<IssueStateReason>,
    pub title: String,
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    pub user: Author,
    pub labels: Vec<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Author>,
    pub assignees: Vec<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_association: Option<AuthorAssociation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Milestone>,
    pub locked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_lock_reason: Option<String>,
    pub comments: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequestLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_by: Option<Author>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Comment {
    pub id: CommentId,
    pub node_id: String,
    pub url: Url,
    pub html_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_association: Option<AuthorAssociation>,
    pub user: Author,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin: Option<Pin>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum IssueStateReason {
    Completed,
    NotPlanned,
    Reopened,
    Duplicate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestLink {
    pub url: Url,
    pub html_url: Url,
    pub diff_url: Url,
    pub patch_url: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pin {
    pinned_at: chrono::DateTime<chrono::Utc>,
    pinned_by: Author,
}

pub enum SubIssuePriority {
    Before(IssueId),
    After(IssueId),
}

/// Option entry for a single-select or multi-select issue field value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueFieldSelectOption {
    pub id: u64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// An issue field value associated with an issue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueFieldValue {
    pub issue_field_id: u64,
    pub issue_field_name: String,
    pub node_id: String,
    pub data_type: String,
    pub value: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub single_select_option: Option<IssueFieldSelectOption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_select_options: Option<Vec<IssueFieldSelectOption>>,
}

/// Request entry for adding or setting an issue field value on an issue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueFieldValueInput {
    pub field_id: u64,
    pub value: serde_json::Value,
}

impl IssueFieldValueInput {
    pub fn new(field_id: u64, value: impl Into<serde_json::Value>) -> Self {
        Self {
            field_id,
            value: value.into(),
        }
    }
}

/// An issue suggestion proposed for an issue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueSuggestion {
    pub id: u64,
    pub issue_id: IssueId,
    pub action: String,
    pub state: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<ActorId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_event_id: Option<IssueEventId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_by: Option<UserId>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// An artifact attached to an issue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueArtifact {
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<Url>,
    pub number: u64,
    #[serde(rename = "type")]
    pub artifact_type: String,
    pub name: String,
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<IssueArtifactActor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by_actor: Option<IssueArtifactActor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Actor who created or updated an issue artifact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueArtifactActor {
    pub login: String,
    pub id: u64,
}

/// Historical version of an issue artifact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueArtifactVersion {
    pub version: u64,
    pub name: String,
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<IssueArtifactActor>,
}

/// Response format for getting an issue artifact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IssueArtifactResponse {
    Detailed {
        artifact: IssueArtifact,
        #[serde(default)]
        versions: Vec<IssueArtifactVersion>,
    },
    Simple(IssueArtifact),
}

impl IssueArtifactResponse {
    pub fn into_artifact(self) -> IssueArtifact {
        match self {
            Self::Detailed { artifact, .. } => artifact,
            Self::Simple(artifact) => artifact,
        }
    }

    pub fn artifact(&self) -> &IssueArtifact {
        match self {
            Self::Detailed { artifact, .. } => artifact,
            Self::Simple(artifact) => artifact,
        }
    }
}
