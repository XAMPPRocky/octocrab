pub mod payload;

use self::payload::{CreateEventPayload, EventPayload, PushEventPayload};
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
#[non_exhaustive]
pub enum EventType {
    PushEvent,
    CreateEvent,
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
        unknown => EventType::UnknownEvent(unknown.to_owned()),
    }
}

fn deserialize_payload(
    event_type: &EventType,
    data: serde_json::Value,
) -> Result<Option<EventPayload>, serde_json::Error> {
    let maybe_payload = match event_type {
        EventType::PushEvent => serde_json::from_value::<PushEventPayload>(data)
            .map(|payload| Some(EventPayload::PushEvent(payload)))?,
        EventType::CreateEvent => serde_json::from_value::<CreateEventPayload>(data)
            .map(|payload| Some(EventPayload::CreateEvent(payload)))?,
        _ => serde_json::from_value::<serde_json::Value>(data)
            .map(|payload| Some(EventPayload::UnknownEvent(payload)))?,
    };
    Ok(maybe_payload)
}

#[cfg(test)]
mod test {
    use super::{Actor, Event, EventPayload, EventType, Repository};
    use crate::models::repos::GitUser;
    use chrono::DateTime;
    use reqwest::Url;

    #[test]
    fn should_deserialize_push_event() {
        let json = include_str!("../../tests/resources/push_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.id, "14289834535".to_owned());
        assert_eq!(event.r#type, EventType::PushEvent);
        assert_eq!(
            event.actor,
            Actor {
                id: 8739360,
                login: "orhanarifoglu".to_string(),
                display_login: "orhanarifoglu".to_string(),
                gravatar_id: "".to_string(),
                url: Url::parse("https://api.github.com/users/orhanarifoglu").unwrap(),
                avatar_url: Url::parse("https://avatars.githubusercontent.com/u/8739360?").unwrap(),
            }
        );
        assert_eq!(
            event.repo,
            Repository {
                id: 291596188,
                name: "orhanarifoglu/orhanarifoglu".to_string(),
                url: Url::parse("https://api.github.com/repos/orhanarifoglu/orhanarifoglu")
                    .unwrap()
            }
        );
        assert!(event.public);
        assert_eq!(
            event.created_at,
            DateTime::parse_from_rfc3339("2020-11-23T19:54:09Z").unwrap()
        );
    }

    #[test]
    fn should_deserialize_push_event_with_correct_payload() {
        let json = include_str!("../../tests/resources/push_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert!(event.payload.is_some());
        let payload = event.payload.unwrap();
        match payload {
            EventPayload::PushEvent(payload) => {
                assert_eq!(payload.push_id, 6080608029);
                assert_eq!(payload.size, 1);
                assert_eq!(payload.distinct_size, 1);
                assert_eq!(payload.r#ref, "refs/heads/master");
                assert_eq!(payload.head, "eb1a60c03544dcea290f2d57bb66ae188ce25778");
                assert_eq!(payload.before, "9b2afb3a8e03fb30cc09e5efb64823bde802cf59");
                assert_eq!(payload.commits.len(), 1);
                let commit = payload.commits.get(0).unwrap();
                assert_eq!(commit.sha, "eb1a60c03544dcea290f2d57bb66ae188ce25778");
                assert_eq!(
                    commit.author,
                    GitUser {
                        name: "readme-bot".to_string(),
                        email: "readme-bot@example.com".to_string()
                    }
                );
                assert_eq!(commit.message, "Charts Updated");
                assert_eq!(commit.distinct, true);
                assert_eq!(
                    commit.url,
                    Url::parse("https://api.github.com/repos/user/user/commits/12345").unwrap()
                );
            }
            _ => panic!("unexpected event deserialized"),
        }
    }

    #[test]
    fn should_deserialize_create_event_with_correct_payload() {
        let json = include_str!("../../tests/resources/create_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert!(event.payload.is_some());
        let payload = event.payload.unwrap();
        match payload {
            EventPayload::CreateEvent(payload) => {
                assert_eq!(payload.r#ref, Some("url-normalisation".to_string()));
                assert_eq!(payload.ref_type, "branch");
                assert_eq!(payload.master_branch, "main");
                assert_eq!(
                    payload.description,
                    Some("Your friendly URL vetting service".to_string())
                );
                assert_eq!(payload.pusher_type, "user");
            }
            _ => panic!("unexpected event deserialized"),
        }
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
        let json = include_str!("../../tests/resources/delete_event.json");
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
    fn should_deserialize_null_description_as_none() {
        let json = include_str!("../../tests/resources/create_event_with_null_description.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert!(event.payload.is_some());
        let payload = event.payload.unwrap();
        match payload {
            EventPayload::CreateEvent(payload) => assert_eq!(payload.description, None),
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
