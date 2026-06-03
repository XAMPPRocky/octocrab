// Tests for calls to the actions "list artifacts for a repository" API:
// - /repos/{owner}/{repo}/actions/artifacts
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{models::workflows::WorkflowListArtifact, Octocrab, Page};
use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

const OWNER: &str = "owner";
const REPO: &str = "repo";

#[derive(Clone, Serialize, Deserialize)]
struct FakePage {
    total_count: u64,
    artifacts: Vec<WorkflowListArtifact>,
}

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;
    let uri = format!("/repos/{OWNER}/{REPO}/actions/artifacts");

    Mock::given(method("GET"))
        .and(path(&uri))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(&mock_server, &format!("GET on {uri} was not received")).await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_return_page_with_repository_artifacts() {
    let expected: FakePage =
        serde_json::from_str(include_str!("resources/repository_artifacts.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(expected.clone());
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .actions()
        .list_repository_artifacts(OWNER, REPO)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let etagged = result.unwrap();
    let Page {
        total_count, items, ..
    } = etagged.value.expect("expected a page, got None");
    assert_eq!(total_count.unwrap(), expected.total_count);
    assert_eq!(items, expected.artifacts);

    let first = &items[0];
    let workflow_run = first
        .workflow_run
        .as_ref()
        .expect("expected workflow_run on the first artifact");
    assert_eq!(workflow_run.head_branch, "main");
}

#[tokio::test]
async fn should_forward_query_parameters() {
    let mock_server = MockServer::start().await;
    let uri = format!("/repos/{OWNER}/{REPO}/actions/artifacts");
    let expected: FakePage =
        serde_json::from_str(include_str!("resources/repository_artifacts.json")).unwrap();

    Mock::given(method("GET"))
        .and(path(&uri))
        .and(query_param("name", "my-artifact"))
        .and(query_param("per_page", "10"))
        .and(query_param("page", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(expected))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on {uri} with expected query parameters was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .actions()
        .list_repository_artifacts(OWNER, REPO)
        .name("my-artifact")
        .per_page(10u8)
        .page(2u32)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_be_err_with_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .actions()
        .list_repository_artifacts(OWNER, REPO)
        .send()
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}
