mod mock_error;

use mock_error::setup_error_handler;
use octocrab::Octocrab;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_check_issue_assignee_api(template: ResponseTemplate) -> MockServer {
    let owner: &str = "org";
    let repo: &str = "some-repo";
    let issue_number: u64 = 42;
    let assignee: &str = "some-user";

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{owner}/{repo}/issues/{issue_number}/assignees/{assignee}"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!(
            "GET on /repos/{owner}/{repo}/issues/{issue_number}/assignees/{assignee} was not received"
        ),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const OWNER: &str = "org";
const REPO: &str = "some-repo";
const ISSUE_NUMBER: u64 = 42;
const ASSIGNEE: &str = "some-user";

#[tokio::test]
async fn check_assignee_for_issue_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_check_issue_assignee_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .issues(OWNER.to_owned(), REPO.to_owned())
        .check_assignee_for_issue(ISSUE_NUMBER, ASSIGNEE)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let result = result.unwrap();
    assert!(result, "expected the result to be true: {}", result);
}

#[tokio::test]
async fn check_assignee_for_issue_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_check_issue_assignee_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .issues(OWNER.to_owned(), REPO.to_owned())
        .check_assignee_for_issue(ISSUE_NUMBER, ASSIGNEE)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let result = result.unwrap();
    assert!(!result, "expected the result to be false: {}", result);
}

#[tokio::test]
async fn check_assignee_for_issue_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_check_issue_assignee_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .issues(OWNER.to_owned(), REPO.to_owned())
        .check_assignee_for_issue(ISSUE_NUMBER, ASSIGNEE)
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}
