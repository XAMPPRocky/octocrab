pub mod payload;

use self::payload::{
    CommitCommentEventPayload, CreateEventPayload, DeleteEventPayload, EventPayload,
    ForkEventPayload, GollumEventPayload, IssueCommentEventPayload, IssuesEventPayload,
    PullRequestEventPayload, PullRequestReviewCommentEventPayload, PushEventPayload,
};
use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::{de::Error, Deserialize, Serialize};

/// A GitHub event.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct Event {
    pub id: String,
    pub r#type: EventType,
    pub actor: Actor,
    pub repo: Repository,
    pub public: bool,
    pub created_at: DateTime<Utc>,
    pub payload: Option<EventPayload>,
    pub org: Option<Org>,
}

/// The type of an event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[non_exhaustive]
pub enum EventType {
    PushEvent,
    CreateEvent,
    DeleteEvent,
    IssuesEvent,
    IssueCommentEvent,
    CommitCommentEvent,
    ForkEvent,
    GollumEvent,
    PullRequestEvent,
    PullRequestReviewCommentEvent,
    UnknownEvent(String),
}

/// The repository an [`Event`] belongs to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub url: Url,
}

/// The organization an [`Event`] belongs to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Org {
    pub id: u64,
    pub login: String,
    pub gravatar_id: String,
    pub url: Url,
    pub avatar_url: Url,
}

/// The actor that created this [`Event`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Actor {
    pub id: u64,
    pub login: String,
    pub display_login: String,
    pub gravatar_id: String,
    pub url: Url,
    pub avatar_url: Url,
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Intermediate {
            id: String,
            #[serde(rename = "type")]
            typ: String,
            actor: Actor,
            repo: Repository,
            public: bool,
            created_at: DateTime<Utc>,
            org: Option<Org>,
            payload: Option<serde_json::Value>,
        }
        let intermediate = Intermediate::deserialize(deserializer)?;
        let event_type = deserialize_event_type(intermediate.typ.as_ref());
        let payload = intermediate.payload.map_or(Ok(None), |data| {
            deserialize_payload(&event_type, data).map_err(|e| Error::custom(e.to_string()))
        })?;
        let event = Event {
            id: intermediate.id,
            r#type: event_type,
            actor: intermediate.actor,
            repo: intermediate.repo,
            public: intermediate.public,
            created_at: intermediate.created_at,
            org: intermediate.org,
            payload,
        };
        Ok(event)
    }
}

fn deserialize_event_type(event_type: &str) -> EventType {
    match event_type {
        "CreateEvent" => EventType::CreateEvent,
        "PushEvent" => EventType::PushEvent,
        "DeleteEvent" => EventType::DeleteEvent,
        "IssuesEvent" => EventType::IssuesEvent,
        "IssueCommentEvent" => EventType::IssueCommentEvent,
        "CommitCommentEvent" => EventType::CommitCommentEvent,
        "ForkEvent" => EventType::ForkEvent,
        "GollumEvent" => EventType::GollumEvent,
        "PullRequestEvent" => EventType::PullRequestEvent,
        "PullRequestReviewCommentEvent" => EventType::PullRequestReviewCommentEvent,
        unknown => EventType::UnknownEvent(unknown.to_owned()),
    }
}

fn deserialize_payload(
    event_type: &EventType,
    data: serde_json::Value,
) -> Result<Option<EventPayload>, serde_json::Error> {
    let maybe_payload = match event_type {
        EventType::PushEvent => {
            serde_json::from_value::<PushEventPayload>(data).map(EventPayload::PushEvent)?
        }
        EventType::CreateEvent => {
            serde_json::from_value::<CreateEventPayload>(data).map(EventPayload::CreateEvent)?
        }
        EventType::DeleteEvent => {
            serde_json::from_value::<DeleteEventPayload>(data).map(EventPayload::DeleteEvent)?
        }
        EventType::IssuesEvent => {
            serde_json::from_value::<IssuesEventPayload>(data).map(EventPayload::IssuesEvent)?
        }
        EventType::IssueCommentEvent => serde_json::from_value::<IssueCommentEventPayload>(data)
            .map(EventPayload::IssueCommentEvent)?,
        EventType::CommitCommentEvent => serde_json::from_value::<CommitCommentEventPayload>(data)
            .map(EventPayload::CommitCommentEvent)?,
        EventType::ForkEvent => {
            serde_json::from_value::<ForkEventPayload>(data).map(EventPayload::ForkEvent)?
        }
        EventType::GollumEvent => {
            serde_json::from_value::<GollumEventPayload>(data).map(EventPayload::GollumEvent)?
        }
        EventType::PullRequestEvent => serde_json::from_value::<PullRequestEventPayload>(data)
            .map(|payload| EventPayload::PullRequestEvent(Box::new(payload)))?,
        EventType::PullRequestReviewCommentEvent => {
            serde_json::from_value::<PullRequestReviewCommentEventPayload>(data)
                .map(|payload| EventPayload::PullRequestReviewCommentEvent(Box::new(payload)))?
        }
        _ => EventPayload::UnknownEvent(data),
    };
    Ok(Some(maybe_payload))
}

#[cfg(test)]
mod test {
    use super::{Event, EventPayload, EventType};
    use reqwest::Url;

    #[test]
    fn should_deserialize_push_event() {
        let json = include_str!("../../tests/resources/push_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::PushEvent);
    }

    #[test]
    fn should_deserialize_create_event() {
        let json = include_str!("../../tests/resources/create_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::CreateEvent);
    }

    #[test]
    fn should_deserialize_issues_event() {
        let json = include_str!("../../tests/resources/issues_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::IssuesEvent);
    }

    #[test]
    fn should_deserialize_issue_comment_event() {
        let json = include_str!("../../tests/resources/issue_comment_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::IssueCommentEvent);
    }

    #[test]
    fn should_deserialize_pull_request_event() {
        let json = include_str!("../../tests/resources/pull_request_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::PullRequestEvent);
    }

    #[test]
    fn should_deserialize_pull_request_review_comment_event() {
        let json = include_str!("../../tests/resources/pull_request_review_comment_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::PullRequestReviewCommentEvent);
    }

    #[test]
    fn should_deserialize_commit_comment_event() {
        let json = include_str!("../../tests/resources/commit_comment_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::CommitCommentEvent);
    }

    #[test]
    fn should_deserialize_delete_event() {
        let json = include_str!("../../tests/resources/delete_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::DeleteEvent);
    }

    #[test]
    fn should_deserialize_fork_event() {
        let json = include_str!("../../tests/resources/fork_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::ForkEvent);
    }

    #[test]
    fn should_deserialize_gollum_event() {
        let json = include_str!("../../tests/resources/gollum_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::GollumEvent);
    }

    #[test]
    fn should_deserialize_with_org_when_present() {
        let json = include_str!("../../tests/resources/create_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert!(event.org.is_some());
        let org = event.org.unwrap();
        assert_eq!(org.id, 1243215);
        assert_eq!(org.login, "hypothesis");
        assert_eq!(org.gravatar_id, "");
        assert_eq!(
            org.url,
            Url::parse("https://api.github.com/orgs/hypothesis").unwrap()
        );
        assert_eq!(
            org.avatar_url,
            Url::parse("https://avatars.githubusercontent.com/u/1243215?").unwrap()
        );
    }

    #[test]
    fn should_deserialize_unknown_event_payload() {
        let json = include_str!("../../tests/resources/unknown_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert!(event.payload.is_some());
        let payload = event.payload.unwrap();
        match payload {
            EventPayload::UnknownEvent(json) => {
                assert!(json.is_object());
                let map = json.as_object().unwrap();
                assert_eq!(map.get("ref").unwrap(), "Core.GetText");
                assert_eq!(map.get("ref_type").unwrap(), "branch");
                assert_eq!(map.get("pusher_type").unwrap(), "user");
            }
            _ => panic!("unexpected event deserialized"),
        }
    }

    #[test]
    fn should_capture_event_name_if_we_dont_currently_handle_this_event() {
        let json = include_str!("../../tests/resources/unknown_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        match event.r#type {
            EventType::UnknownEvent(typ) => assert_eq!(typ, "AmazingEvent"),
            _ => panic!("unexpected event deserialized"),
        }
    }

    #[test]
    fn event_deserialize_and_serialize_should_be_isomorphic() {
        let json = include_str!("../../tests/resources/create_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&event).unwrap();
        // do it again so we can compare Events, otherwise we are comparing strings which may have
        // different whitespace characteristics.
        let deserialized = serde_json::from_str::<Event>(&serialized);
        assert!(
            deserialized.is_ok(),
            "expected deserialized result to be ok, got error instead {:?}",
            deserialized
        );
        let deserialized = deserialized.unwrap();
        assert_eq!(deserialized, event);
    }
}
