mod mock_error;

use mock_error::setup_error_handler;
use octocrab::Octocrab;
use serde_json::json;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

fn issue_event_json(id: u64, event_name: &str) -> serde_json::Value {
    json!({
        "id": id,
        "node_id": format!("MDEyOklzc3VlRXZlbnQ{id}"),
        "url": format!("https://api.github.com/repos/owner/repo/issues/events/{id}"),
        "actor": {
            "login": "octocat",
            "id": 1,
            "node_id": "MDQ6VXNlcjE=",
            "avatar_url": "https://github.com/images/error/octocat_happy.gif",
            "gravatar_id": "",
            "url": "https://api.github.com/users/octocat",
            "html_url": "https://github.com/octocat",
            "followers_url": "https://api.github.com/users/octocat/followers",
            "following_url": "https://api.github.com/users/octocat/following{/other_user}",
            "gists_url": "https://api.github.com/users/octocat/gists{/gist_id}",
            "starred_url": "https://api.github.com/users/octocat/starred{/owner}{/repo}",
            "subscriptions_url": "https://api.github.com/users/octocat/subscriptions",
            "organizations_url": "https://api.github.com/users/octocat/orgs",
            "repos_url": "https://api.github.com/users/octocat/repos",
            "events_url": "https://api.github.com/users/octocat/events{/privacy}",
            "received_events_url": "https://api.github.com/users/octocat/received_events",
            "type": "User",
            "site_admin": false
        },
        "event": event_name,
        "commit_id": "6dcb09b5b57875f334f61aebed695e2e4193db5e",
        "commit_url": "https://api.github.com/repos/owner/repo/commits/6dcb09b5b57875f334f61aebed695e2e4193db5e",
        "created_at": "2011-04-14T16:00:49Z"
    })
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn list_issue_events_for_repo() {
    let mock_server = MockServer::start().await;
    let expected = vec![issue_event_json(1, "closed")];

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/issues/events"))
        .and(query_param("per_page", "30"))
        .and(query_param("page", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        "GET /repos/owner/repo/issues/events not matched",
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let page = client
        .issues("owner", "repo")
        .events()
        .list()
        .per_page(30)
        .page(1u32)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].id.unwrap().0, 1);
    assert_eq!(page.items[0].event, Some(octocrab::models::Event::Closed));
}

#[tokio::test]
async fn get_issue_event() {
    let mock_server = MockServer::start().await;
    let expected = issue_event_json(42, "closed");

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/issues/events/42"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        "GET /repos/owner/repo/issues/events/42 not matched",
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let event = client
        .issues("owner", "repo")
        .events()
        .get(42u64)
        .await
        .unwrap();

    assert_eq!(event.id.unwrap().0, 42);
    assert_eq!(event.event, Some(octocrab::models::Event::Closed));
}

#[tokio::test]
async fn list_events_for_issue() {
    let mock_server = MockServer::start().await;
    let expected = vec![issue_event_json(100, "closed")];

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/issues/7/events"))
        .and(query_param("per_page", "50"))
        .and(query_param("page", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        "GET /repos/owner/repo/issues/7/events not matched",
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let page = client
        .issues("owner", "repo")
        .events()
        .list_for_issue(7)
        .per_page(50)
        .page(2u32)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].id.unwrap().0, 100);
}
