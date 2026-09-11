mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{params, Octocrab};
use serde_json::json;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

fn milestone_json(number: i64, title: &str, state: &str) -> serde_json::Value {
    json!({
        "url": format!("https://api.github.com/repos/owner/repo/milestones/{number}"),
        "html_url": format!("https://github.com/owner/repo/milestone/{number}"),
        "labels_url": format!("https://api.github.com/repos/owner/repo/milestones/{number}/labels"),
        "id": 10026 + number,
        "node_id": "MDk6TWlsZXN0b25lMTAwMjY=",
        "number": number,
        "state": state,
        "title": title,
        "description": "Milestone description",
        "creator": {
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
        "open_issues": 4,
        "closed_issues": 8,
        "created_at": "2011-04-10T20:09:31Z",
        "updated_at": "2014-03-03T18:58:10Z",
        "closed_at": null,
        "due_on": "2026-10-09T23:39:01Z"
    })
}

fn label_json(name: &str) -> serde_json::Value {
    json!({
        "id": 208045946,
        "node_id": "MDU6TGFiZWwyMDgwNDU5NDY=",
        "url": format!("https://api.github.com/repos/owner/repo/labels/{name}"),
        "name": name,
        "description": "Something isn't working",
        "color": "f29513",
        "default": true
    })
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn list_milestones() {
    let mock_server = MockServer::start().await;
    let expected = vec![milestone_json(1, "v1.0", "open")];

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/milestones"))
        .and(query_param("state", "open"))
        .and(query_param("sort", "due_on"))
        .and(query_param("direction", "asc"))
        .and(query_param("per_page", "50"))
        .and(query_param("page", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    setup_error_handler(&mock_server, "GET /repos/owner/repo/milestones not matched").await;
    let client = setup_octocrab(&mock_server.uri());

    let page = client
        .issues("owner", "repo")
        .milestones()
        .list()
        .state(params::State::Open)
        .sort(params::milestones::Sort::DueOn)
        .direction(params::Direction::Ascending)
        .per_page(50)
        .page(1u32)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].title, "v1.0");
    assert_eq!(page.items[0].number, 1);
}

#[tokio::test]
async fn create_milestone() {
    let mock_server = MockServer::start().await;
    let expected = milestone_json(2, "v2.0", "open");

    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/milestones"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        "POST /repos/owner/repo/milestones not matched",
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let milestone = client
        .issues("owner", "repo")
        .milestones()
        .create("v2.0")
        .description("Version 2.0")
        .state(params::State::Open)
        .send()
        .await
        .unwrap();

    assert_eq!(milestone.title, "v2.0");
    assert_eq!(milestone.number, 2);
}

#[tokio::test]
async fn get_milestone() {
    let mock_server = MockServer::start().await;
    let expected = milestone_json(1, "v1.0", "open");

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/milestones/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        "GET /repos/owner/repo/milestones/1 not matched",
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let milestone = client
        .issues("owner", "repo")
        .milestones()
        .get(1)
        .await
        .unwrap();

    assert_eq!(milestone.title, "v1.0");
    assert_eq!(milestone.number, 1);
}

#[tokio::test]
async fn update_milestone() {
    let mock_server = MockServer::start().await;
    let expected = milestone_json(1, "v1.1", "closed");

    Mock::given(method("PATCH"))
        .and(path("/repos/owner/repo/milestones/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        "PATCH /repos/owner/repo/milestones/1 not matched",
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let milestone = client
        .issues("owner", "repo")
        .milestones()
        .update(1)
        .title("v1.1")
        .state(params::State::Closed)
        .send()
        .await
        .unwrap();

    assert_eq!(milestone.title, "v1.1");
    assert_eq!(milestone.state.as_deref(), Some("closed"));
}

#[tokio::test]
async fn delete_milestone() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/repos/owner/repo/milestones/1"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        "DELETE /repos/owner/repo/milestones/1 not matched",
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.issues("owner", "repo").milestones().delete(1).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn list_milestone_labels() {
    let mock_server = MockServer::start().await;
    let expected = vec![label_json("bug"), label_json("enhancement")];

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/milestones/1/labels"))
        .and(query_param("per_page", "30"))
        .and(query_param("page", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        "GET /repos/owner/repo/milestones/1/labels not matched",
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let page = client
        .issues("owner", "repo")
        .milestones()
        .list_labels(1)
        .per_page(30)
        .page(1u32)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 2);
    assert_eq!(page.items[0].name, "bug");
    assert_eq!(page.items[1].name, "enhancement");
}
