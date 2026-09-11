mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{params, Octocrab};
use serde_json::json;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

fn issue_json(number: u64, title: &str) -> serde_json::Value {
    json!({
        "id": number,
        "node_id": format!("MDU6SXNzdWU{number}"),
        "url": format!("https://api.github.com/repos/owner/repo/issues/{number}"),
        "repository_url": "https://api.github.com/repos/owner/repo",
        "labels_url": format!("https://api.github.com/repos/owner/repo/issues/{number}/labels{{/name}}"),
        "comments_url": format!("https://api.github.com/repos/owner/repo/issues/{number}/comments"),
        "events_url": format!("https://api.github.com/repos/owner/repo/issues/{number}/events"),
        "html_url": format!("https://github.com/owner/repo/issues/{number}"),
        "number": number,
        "state": "open",
        "title": title,
        "body": "Issue body",
        "user": {
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
        "labels": [],
        "assignees": [],
        "locked": false,
        "comments": 0,
        "created_at": "2011-04-22T13:33:48Z",
        "updated_at": "2011-04-22T13:33:48Z"
    })
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn list_org_issues() {
    let mock_server = MockServer::start().await;
    let expected = vec![issue_json(1, "Org issue")];

    Mock::given(method("GET"))
        .and(path("/orgs/my-org/issues"))
        .and(query_param("filter", "assigned"))
        .and(query_param("state", "open"))
        .and(query_param("per_page", "25"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    setup_error_handler(&mock_server, "GET /orgs/my-org/issues not matched").await;
    let client = setup_octocrab(&mock_server.uri());

    let page = client
        .orgs("my-org")
        .list_issues()
        .filter(params::issues::IssueFilter::Assigned)
        .state(params::State::Open)
        .per_page(25)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].title, "Org issue");
}

#[tokio::test]
async fn list_user_issues() {
    let mock_server = MockServer::start().await;
    let expected = vec![issue_json(2, "User issue")];

    Mock::given(method("GET"))
        .and(path("/user/issues"))
        .and(query_param("filter", "created"))
        .and(query_param("state", "all"))
        .and(query_param("labels", "bug,help wanted"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    setup_error_handler(&mock_server, "GET /user/issues not matched").await;
    let client = setup_octocrab(&mock_server.uri());

    let labels = [String::from("bug"), String::from("help wanted")];
    let page = client
        .current()
        .list_issues_for_authenticated_user()
        .filter(params::issues::IssueFilter::Created)
        .state(params::State::All)
        .labels(&labels)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].title, "User issue");
}

#[tokio::test]
async fn list_all_issues() {
    let mock_server = MockServer::start().await;
    let expected = vec![issue_json(3, "Global issue")];

    Mock::given(method("GET"))
        .and(path("/issues"))
        .and(query_param("filter", "all"))
        .and(query_param("sort", "comments"))
        .and(query_param("direction", "desc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    setup_error_handler(&mock_server, "GET /issues not matched").await;
    let client = setup_octocrab(&mock_server.uri());

    let page = client
        .all_issues()
        .filter(params::issues::IssueFilter::All)
        .sort(params::issues::Sort::Comments)
        .direction(params::Direction::Descending)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].title, "Global issue");
}
