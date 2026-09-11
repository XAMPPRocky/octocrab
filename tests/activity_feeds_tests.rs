mod mock_error;

use mock_error::setup_error_handler;
use octocrab::Octocrab;
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_get_feeds() {
    let mock_server = MockServer::start().await;
    let expected = json!({
        "timeline_url": "https://github.com/timeline",
        "user_url": "https://github.com/{user}",
        "current_user_public_url": "https://github.com/octocat.private.atom?token=abc",
        "current_user_url": "https://github.com/octocat.private.atom?token=abc",
        "current_user_actor_url": "https://github.com/octocat.private.actor.atom?token=abc",
        "current_user_organization_url": "https://github.com/organizations/org/octocat.private.atom?token=abc",
        "current_user_organization_urls": [
            "https://github.com/organizations/org/octocat.private.atom?token=abc"
        ],
        "security_advisories_url": "https://github.com/security-advisories",
        "_links": {
            "timeline": {
                "href": "https://github.com/timeline",
                "type": "application/atom+xml"
            },
            "user": {
                "href": "https://github.com/{user}",
                "type": "application/atom+xml"
            },
            "security_advisories": {
                "href": "https://github.com/security-advisories",
                "type": "application/atom+xml"
            }
        }
    });

    Mock::given(method("GET"))
        .and(path("/feeds"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    setup_error_handler(&mock_server, "GET /feeds not matched").await;

    let client = setup_octocrab(&mock_server.uri());
    let feeds = client.activity().feeds().await.unwrap();

    assert_eq!(feeds.timeline_url, "https://github.com/timeline");
    assert_eq!(feeds.user_url, "https://github.com/{user}");
    assert_eq!(
        feeds.current_user_public_url.as_deref(),
        Some("https://github.com/octocat.private.atom?token=abc")
    );
    assert_eq!(feeds.links.timeline.href, "https://github.com/timeline");
    assert_eq!(feeds.links.timeline.r#type, "application/atom+xml");
}
