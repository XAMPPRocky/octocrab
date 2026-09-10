use http::StatusCode;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::repos::DeployKey;
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "owner";
const REPO: &str = "repo";
const KEY_ID: u64 = 1;

async fn setup_deploy_keys_mock(
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
async fn should_list_deploy_keys() {
    let mocked_response: Vec<DeployKey> =
        serde_json::from_str(include_str!("resources/repo_deploy_keys.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_deploy_keys_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/keys").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .keys()
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
    let response = result.unwrap();
    assert_eq!(response.items.len(), 2);
    let first = &response.items[0];
    assert_eq!(first.id.0, 1);
    assert_eq!(first.title, "octocat@my-mac");
    assert!(first.read_only);
    assert!(first.verified);

    let second = &response.items[1];
    assert_eq!(second.id.0, 2);
    assert_eq!(second.title, "ci-runner");
    assert!(!second.read_only);
}

#[tokio::test]
async fn should_get_deploy_key() {
    let mocked_response: DeployKey =
        serde_json::from_str(include_str!("resources/repo_deploy_key.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_deploy_keys_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/keys/{KEY_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.repos(OWNER, REPO).keys().get(KEY_ID).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.id.0, 1);
    assert_eq!(response.title, "octocat@my-mac");
    assert!(response.read_only);
    assert_eq!(response.added_by.as_deref(), Some("octocat"));
}

#[tokio::test]
async fn should_create_deploy_key() {
    let mocked_response: DeployKey =
        serde_json::from_str(include_str!("resources/repo_deploy_key.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::CREATED).set_body_json(&mocked_response);
    let mock_server = setup_deploy_keys_mock(
        "POST",
        format!("/repos/{OWNER}/{REPO}/keys").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .keys()
        .create(
            "octocat@my-mac",
            "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQCb3Y2n...",
        )
        .read_only(true)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.id.0, 1);
    assert_eq!(response.title, "octocat@my-mac");
    assert!(response.read_only);
}

#[tokio::test]
async fn should_delete_deploy_key() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_deploy_keys_mock(
        "DELETE",
        format!("/repos/{OWNER}/{REPO}/keys/{KEY_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.repos(OWNER, REPO).keys().delete(KEY_ID).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}
