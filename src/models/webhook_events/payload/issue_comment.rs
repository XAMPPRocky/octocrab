use serde::{Deserialize, Serialize};

use crate::models::{
    issues::{Comment, Issue},
    Author, InstallationLite, Repository,
};

use super::OldValue;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueCommentWebhookEventPayload {
    pub action: IssueCommentWebhookEventAction,
    pub changes: Option<IssueCommentWebhookEventChanges>,
    pub comment: Comment,
    pub enterprise: Option<serde_json::Value>,
    pub issue: Issue,
    /// The repository of the GitHub App that triggered the event
    pub repository: Repository,
    /// The installation of the GitHub App that triggered the event
    pub installation: Option<InstallationLite>,
    /// The sender of the event
    pub sender: Author,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum IssueCommentWebhookEventAction {
    Created,
    Deleted,
    Edited,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueCommentWebhookEventChanges {
    pub body: OldValue<String>,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_deserialize_with_correct_payload() {
        let json = include_str!("../../../../tests/resources/issue_comment_webhook_webhook.json");
        let payload: IssueCommentWebhookEventPayload = serde_json::from_str(json).unwrap();

        assert_eq!(payload.action, IssueCommentWebhookEventAction::Created);
        assert_eq!(payload.issue.id.0, 3460488379);
        assert_eq!(payload.comment.id.0, 3342057776);
        assert_eq!(
            payload.comment.body.as_deref(),
            Some("This is a test comment message.")
        );
        assert_eq!(payload.issue.number, 1);
        assert_eq!(payload.issue.title, "feat: make unreasonable promises");
        assert_eq!(payload.repository.id.0, 1065276716);
        assert_eq!(
            payload.repository.name,
            "sandcastle-monorepo-test"
        );
        assert_eq!(payload.sender.id.0, 72556205);
        assert_eq!(payload.sender.login, "mmoreiradj");
        assert!(payload.changes.is_none());
    }
}
