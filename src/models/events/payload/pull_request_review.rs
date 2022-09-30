use crate::models::{
    pulls::{PullRequest, Review},
    Repository, User,
};
use serde::{Deserialize, Serialize};

/// The payload in a [`super::EventPayload::PullRequestReviewEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestReviewEventPayload {
    /// The action this event represents.
    pub action: PullRequestReviewEventAction,
    /// The pull request this event corresponds to.
    pub pull_request: PullRequest,
    /// The review that was affected.
    pub review: Review,
    /// The changes to body or title if this event is of type [`PullRequestReviewEventAction::Edited`].
    pub changes: Option<PullRequestReviewChanges>,
    /// The repository where the event occurred.
    pub repository: Repository,
    /// The user that triggered the event.
    pub sender: User,
}

/// The action on a pull request review this event corresponds to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum PullRequestReviewEventAction {
    Submitted,
    Edited,
    Dismissed,
}

/// The change which occurred in an event of type [`PullRequestReviewEventAction::Edited`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum PullRequestReviewChanges {
    Body(PullRequestReviewChangesFrom),
}

/// The previous value of the body of a review which has changed. Only
/// available in an event of type [`PullRequestReviewEventAction::Edited`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub struct PullRequestReviewChangesFrom {
    pub from: String,
}

#[cfg(test)]
mod test {
    use super::{
        PullRequestReviewChanges, PullRequestReviewChangesFrom, PullRequestReviewEventAction,
    };
    use crate::models::events::{payload::EventPayload, Event};
    use serde_json::json;

    #[test]
    fn should_deserialize_action_from_lowercase() {
        let actions = vec![
            (r#""submitted""#, PullRequestReviewEventAction::Submitted),
            (r#""edited""#, PullRequestReviewEventAction::Edited),
            (r#""dismissed""#, PullRequestReviewEventAction::Dismissed),
        ];
        for (action_str, action) in actions {
            let deserialized = serde_json::from_str(&action_str).unwrap();
            assert_eq!(action, deserialized);
        }
    }

    #[test]
    fn should_deserialize_body_changes() {
        let json = json!({
            "body": {
                "from": "test"
            }
        });
        let deserialized = serde_json::from_value::<PullRequestReviewChanges>(json).unwrap();
        assert_eq!(
            deserialized,
            PullRequestReviewChanges::Body(PullRequestReviewChangesFrom {
                from: "test".to_owned()
            })
        );
    }

    #[test]
    fn should_deserialize_with_correct_payload() {
        let json = include_str!("../../../../tests/resources/pull_request_review_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        if let Some(EventPayload::PullRequestReviewEvent(payload)) = event.payload {
            assert_eq!(payload.action, PullRequestReviewEventAction::Submitted);
            assert_eq!(payload.pull_request.id.0, 279147437);
        } else {
            panic!("unexpected event payload encountered: {:#?}", event.payload);
        }
    }
}
