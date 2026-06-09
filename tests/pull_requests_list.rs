use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::pulls::PullRequest;
use octocrab::Octocrab;

/// Tests API calls related to check runs of a specific commit.
mod mock_error;

async fn setup_pull_requests_mock(
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
        &format!("http method {http_method} on {mocked_path} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_respond_to_pull_requests_list() {
    // https://docs.github.com/en/rest/pulls/pulls?apiVersion=2026-03-10#list-pull-requests
    const OWNER: &str = "XAMPPRocky";
    const REPO: &str = "octocrab";

    let mocked_response: Vec<PullRequest> =
        serde_json::from_str(include_str!("resources/pull_requests_list.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_pull_requests_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/pulls").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.pulls(OWNER, REPO).list().send().await.unwrap();
    let items = result.items;
    assert_eq!(items.len(), 1); // expected 1 pull request in example list
    assert!(items.first().is_some_and(|i| i
        .assignees
        .clone()
        .is_some_and(|a| a.first().is_some_and(|f| f.login == "octocat"))))
}
