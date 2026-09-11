use http::StatusCode;
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::orgs::personal_access_tokens::{
    OrgPersonalAccessToken, OrgPersonalAccessTokenRequest, PatAction, PatReviewDecision,
};
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "owner";
const PAT_ID: u64 = 123;
const PAT_REQUEST_ID: u64 = 456;

async fn setup_mock(
    http_method: &str,
    mocked_path: &str,
    template: ResponseTemplate,
) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method(http_method))
        .and(path(mocked_path))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!(
            "http method {} on {} was not received",
            http_method, mocked_path
        ),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

fn sample_pat() -> OrgPersonalAccessToken {
    serde_json::from_value(json!({
        "id": PAT_ID,
        "owner": {
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
        "repository_selection": "all",
        "token_expired": false,
        "token_expires_at": "2023-01-01T00:00:00Z",
        "access_granted_at": "2022-01-01T00:00:00Z"
    }))
    .unwrap()
}

fn sample_pat_request() -> OrgPersonalAccessTokenRequest {
    serde_json::from_value(json!({
        "id": PAT_REQUEST_ID,
        "owner": {
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
        "repository_selection": "all",
        "token_id": 1234,
        "token_name": "test-token",
        "token_expired": false,
        "token_expires_at": "2023-01-01T00:00:00Z",
        "created_at": "2022-01-01T00:00:00Z"
    }))
    .unwrap()
}

#[tokio::test]
async fn should_list_pats() {
    let template = ResponseTemplate::new(200).set_body_json(vec![sample_pat()]);
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/personal-access-tokens", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .personal_access_tokens()
        .list()
        .per_page(10)
        .page(1u32)
        .send()
        .await;

    assert!(result.is_ok(), "result: {:?}", result);
    let page = result.unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].id.0, PAT_ID);
}

#[tokio::test]
async fn should_get_pat() {
    let template = ResponseTemplate::new(200).set_body_json(sample_pat());
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/personal-access-tokens/{}", OWNER, PAT_ID).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .personal_access_tokens()
        .get(PAT_ID)
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
    assert_eq!(result.unwrap().id.0, PAT_ID);
}

#[tokio::test]
async fn should_update_pat_access() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "POST",
        format!("/orgs/{}/personal-access-tokens/{}", OWNER, PAT_ID).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .personal_access_tokens()
        .update_access(PAT_ID, PatAction::Revoke)
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_list_pat_repositories() {
    let template = ResponseTemplate::new(200).set_body_json(json!([]));
    let mock_server = setup_mock(
        "GET",
        format!(
            "/orgs/{}/personal-access-tokens/{}/repositories",
            OWNER, PAT_ID
        )
        .as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .personal_access_tokens()
        .list_repositories(PAT_ID)
        .send()
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_list_pat_requests() {
    let template = ResponseTemplate::new(200).set_body_json(vec![sample_pat_request()]);
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/personal-access-token-requests", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .personal_access_tokens()
        .list_requests()
        .send()
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
    let page = result.unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].id.0, PAT_REQUEST_ID);
}

#[tokio::test]
async fn should_review_pat_requests_batch() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "POST",
        format!("/orgs/{}/personal-access-token-requests", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .personal_access_tokens()
        .review_requests(vec![PAT_REQUEST_ID], PatReviewDecision::Approve, None)
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_review_single_pat_request() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "POST",
        format!(
            "/orgs/{}/personal-access-token-requests/{}",
            OWNER, PAT_REQUEST_ID
        )
        .as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .personal_access_tokens()
        .review_request(
            PAT_REQUEST_ID,
            PatReviewDecision::Deny,
            Some("Not allowed".to_string()),
        )
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_list_pat_request_repositories() {
    let template = ResponseTemplate::new(200).set_body_json(json!([]));
    let mock_server = setup_mock(
        "GET",
        format!(
            "/orgs/{}/personal-access-token-requests/{}/repositories",
            OWNER, PAT_REQUEST_ID
        )
        .as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .personal_access_tokens()
        .list_request_repositories(PAT_REQUEST_ID)
        .send()
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}
