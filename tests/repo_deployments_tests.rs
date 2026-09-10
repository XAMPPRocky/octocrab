use http::StatusCode;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::repos::{Deployment, DeploymentStatus, DeploymentStatusState};
use octocrab::Octocrab;
use octocrab::Page;

mod mock_error;

const OWNER: &str = "owner";
const REPO: &str = "repo";
const DEPLOYMENT_ID: u64 = 1;
const STATUS_ID: u64 = 1;

async fn setup_deployments_mock(
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
async fn should_list_deployments() {
    let mocked_response: Vec<Deployment> =
        serde_json::from_str(include_str!("resources/repo_deployments.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_deployments_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/deployments").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .deployments()
        .list()
        .environment("production")
        .per_page(10)
        .page(1u32)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let Page { items, .. } = result.unwrap();
    assert_eq!(items.len(), 2);
    let first = &items[0];
    assert_eq!(first.id.0, 1);
    assert_eq!(first.ref_field, "topic-branch");
    assert_eq!(first.task, "deploy");
    assert_eq!(first.environment, "production");
    assert_eq!(
        first.description.as_deref(),
        Some("Deploy request from hubot")
    );
    assert_eq!(first.creator.as_ref().unwrap().login, "octocat");
    assert_eq!(first.transient_environment, Some(false));
    assert_eq!(first.production_environment, Some(true));

    let second = &items[1];
    assert_eq!(second.id.0, 2);
    assert_eq!(second.ref_field, "main");
    assert_eq!(second.environment, "staging");
    assert!(second.creator.is_none());
}

#[tokio::test]
async fn should_get_deployment() {
    let mocked_response: Deployment =
        serde_json::from_str(include_str!("resources/repo_deployment.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_deployments_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/deployments/{DEPLOYMENT_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .deployments()
        .get(DEPLOYMENT_ID)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.id.0, 1);
    assert_eq!(response.sha, "a84d88e7554fc1fa21bcbc4efae3c782a70d2b9d");
    assert_eq!(response.ref_field, "topic-branch");
    assert_eq!(response.environment, "production");
}

#[tokio::test]
async fn should_create_deployment() {
    let mocked_response: Deployment =
        serde_json::from_str(include_str!("resources/repo_deployment.json")).unwrap();
    let template = ResponseTemplate::new(201).set_body_json(&mocked_response);
    let mock_server = setup_deployments_mock(
        "POST",
        format!("/repos/{OWNER}/{REPO}/deployments").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .deployments()
        .create("topic-branch")
        .task("deploy")
        .auto_merge(false)
        .required_contexts(vec!["ci/test"])
        .payload(serde_json::json!({"deploy": "migrate"}))
        .environment("production")
        .description("Deploy request from hubot")
        .transient_environment(false)
        .production_environment(true)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.id.0, 1);
    assert_eq!(response.ref_field, "topic-branch");
    assert_eq!(response.task, "deploy");
}

#[tokio::test]
async fn should_delete_deployment() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_deployments_mock(
        "DELETE",
        format!("/repos/{OWNER}/{REPO}/deployments/{DEPLOYMENT_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .deployments()
        .delete(DEPLOYMENT_ID)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_list_deployment_statuses() {
    let mocked_response: Vec<DeploymentStatus> =
        serde_json::from_str(include_str!("resources/repo_deployment_statuses.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_deployments_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/deployments/{DEPLOYMENT_ID}/statuses").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .deployments()
        .statuses(DEPLOYMENT_ID)
        .list()
        .per_page(10)
        .page(1u32)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let Page { items, .. } = result.unwrap();
    assert_eq!(items.len(), 2);
    let first = &items[0];
    assert_eq!(first.id.0, 1);
    assert_eq!(first.state, DeploymentStatusState::Success);
    assert_eq!(
        first.description.as_deref(),
        Some("Deployment finished successfully.")
    );
    assert_eq!(first.environment.as_deref(), Some("production"));
    assert_eq!(
        first.log_url.as_deref(),
        Some("https://example.com/deployment/1/log")
    );

    let second = &items[1];
    assert_eq!(second.id.0, 2);
    assert_eq!(second.state, DeploymentStatusState::InProgress);
}

#[tokio::test]
async fn should_get_deployment_status() {
    let mocked_response: DeploymentStatus =
        serde_json::from_str(include_str!("resources/repo_deployment_status.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_deployments_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/deployments/{DEPLOYMENT_ID}/statuses/{STATUS_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .deployments()
        .statuses(DEPLOYMENT_ID)
        .get(STATUS_ID)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.id.0, 1);
    assert_eq!(response.state, DeploymentStatusState::Success);
    assert_eq!(
        response.description.as_deref(),
        Some("Deployment finished successfully.")
    );
}

#[tokio::test]
async fn should_create_deployment_status() {
    let mocked_response: DeploymentStatus =
        serde_json::from_str(include_str!("resources/repo_deployment_status.json")).unwrap();
    let template = ResponseTemplate::new(201).set_body_json(&mocked_response);
    let mock_server = setup_deployments_mock(
        "POST",
        format!("/repos/{OWNER}/{REPO}/deployments/{DEPLOYMENT_ID}/statuses").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .deployments()
        .statuses(DEPLOYMENT_ID)
        .create(DeploymentStatusState::Success)
        .environment("production")
        .description("Deployment finished successfully.")
        .log_url("https://example.com/deployment/1/log")
        .environment_url("https://example.com")
        .auto_inactive(true)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.id.0, 1);
    assert_eq!(response.state, DeploymentStatusState::Success);
}
