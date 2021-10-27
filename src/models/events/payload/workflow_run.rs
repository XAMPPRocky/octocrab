use crate::models::{Repository, workflows::{WorkFlow, Run}, orgs::Organization, User};
use serde::{Deserialize, Serialize};

/// The payload in a [`super::EventPayload::WorkflowRunEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkflowRunEventPayload {
    pub action: WorkflowRunEventAction,
    pub workflow_run: Run,
    pub workflow: WorkFlow,
    pub organization: Organization,
    pub repository: Repository,
    pub sender: User,
}

/// The action on a pull request this event corresponds to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum WorkflowRunEventAction {
    Requested,
    Completed,
}

#[cfg(test)]
mod test {
    use crate::models::events::{payload::EventPayload, Event};

    #[test]
    fn should_deserialize_with_correct_payload() {
        let json = include_str!("../../../../tests/resources/workflow_run_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        if let Some(EventPayload::WorkflowRunEvent(payload)) = event.payload {
            assert_eq!(payload.workflow_run.run_number, 1185);
        } else {
            panic!("unexpected event payload encountered: {:#?}", event.payload);
        }
    }
}
