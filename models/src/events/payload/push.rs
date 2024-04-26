use serde::{Deserialize, Serialize};

use super::Commit;
use crate::models::PushId;

/// The payload in a [`super::EventPayload::PushEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PushEventPayload {
    pub push_id: PushId,
    pub size: u64,
    pub distinct_size: u64,
    pub r#ref: String,
    pub head: String,
    pub before: String,
    pub commits: Vec<Commit>,
}

#[cfg(test)]
mod test {
    use crate::models::{
        events::{payload::EventPayload, Event},
        repos::CommitAuthor,
    };
    use url::Url;

    #[test]
    fn should_deserialize_push_event_with_correct_payload() {
        let json = include_str!("../../../../tests/resources/push_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert!(event.payload.is_some());
        let payload = event.payload.unwrap().specific.unwrap();
        match payload {
            EventPayload::PushEvent(payload) => {
                assert_eq!(payload.push_id.0, 6080608029);
                assert_eq!(payload.size, 1);
                assert_eq!(payload.distinct_size, 1);
                assert_eq!(payload.r#ref, "refs/heads/master");
                assert_eq!(payload.head, "eb1a60c03544dcea290f2d57bb66ae188ce25778");
                assert_eq!(payload.before, "9b2afb3a8e03fb30cc09e5efb64823bde802cf59");
                assert_eq!(payload.commits.len(), 1);
                let commit = payload.commits.first().unwrap();
                assert_eq!(commit.sha, "eb1a60c03544dcea290f2d57bb66ae188ce25778");
                assert_eq!(
                    commit.author,
                    CommitAuthor {
                        name: "readme-bot".to_string(),
                        email: "readme-bot@example.com".to_string(),
                        date: None,
                    }
                );
                assert_eq!(commit.message, "Charts Updated");
                assert!(commit.distinct);
                assert_eq!(
                    commit.url,
                    Url::parse("https://api.github.com/repos/user/user/commits/12345").unwrap()
                );
            }
            _ => panic!("unexpected event deserialized"),
        }
    }
}
