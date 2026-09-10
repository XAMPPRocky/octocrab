use http::StatusCode;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::repos::Autolink;
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "owner";
const REPO: &str = "repo";
const AUTOLINK_ID: u64 = 1;

async fn setup_autolinks_mock(
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
async fn should_list_autolinks() {
    let mocked_response: Vec<Autolink> =
        serde_json::from_str(include_str!("resources/repo_autolinks.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_autolinks_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/autolinks").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.repos(OWNER, REPO).autolinks().list().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.len(), 2);
    let first = &response[0];
    assert_eq!(first.id.0, 1);
    assert_eq!(first.key_prefix, "TICKET-");
    assert_eq!(first.url_template, "https://example.com/TICKET?query=<num>");
    assert!(first.is_alphanumeric);

    let second = &response[1];
    assert_eq!(second.id.0, 2);
    assert_eq!(second.key_prefix, "ISSUE-");
    assert!(!second.is_alphanumeric);
}

#[tokio::test]
async fn should_get_autolink() {
    let mocked_response: Autolink =
        serde_json::from_str(include_str!("resources/repo_autolink.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_autolinks_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/autolinks/{AUTOLINK_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.repos(OWNER, REPO).autolinks().get(AUTOLINK_ID).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.id.0, 1);
    assert_eq!(response.key_prefix, "TICKET-");
    assert_eq!(
        response.url_template,
        "https://example.com/TICKET?query=<num>"
    );
    assert!(response.is_alphanumeric);
}

#[tokio::test]
async fn should_create_autolink() {
    let mocked_response: Autolink =
        serde_json::from_str(include_str!("resources/repo_autolink.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::CREATED).set_body_json(&mocked_response);
    let mock_server = setup_autolinks_mock(
        "POST",
        format!("/repos/{OWNER}/{REPO}/autolinks").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .autolinks()
        .create("TICKET-", "https://example.com/TICKET?query=<num>")
        .is_alphanumeric(true)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.id.0, 1);
    assert_eq!(response.key_prefix, "TICKET-");
    assert!(response.is_alphanumeric);
}

#[tokio::test]
async fn should_delete_autolink() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_autolinks_mock(
        "DELETE",
        format!("/repos/{OWNER}/{REPO}/autolinks/{AUTOLINK_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .autolinks()
        .delete(AUTOLINK_ID)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}
