#![allow(deprecated)]

use http::StatusCode;
use wiremock::{
    matchers::{body_json, method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::repos::TagProtection;
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "owner";
const REPO: &str = "repo";
const TAG_PROTECTION_ID: u64 = 1;

async fn setup_tag_protection_mock(
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
async fn should_list_tag_protections() {
    let mocked_response: Vec<TagProtection> =
        serde_json::from_str(include_str!("resources/repo_tag_protections.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_tag_protection_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/tags/protection").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.repos(OWNER, REPO).tags().protection().list().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let protections = result.unwrap();
    assert_eq!(protections.len(), 2);
    assert_eq!(protections[0].id, 1.into());
    assert_eq!(protections[0].pattern, "v1.*");
    assert_eq!(protections[0].enabled, Some(true));
    assert_eq!(protections[1].id, 2.into());
    assert_eq!(protections[1].pattern, "release-*");
    assert_eq!(protections[1].enabled, Some(false));
}

#[tokio::test]
async fn should_list_tag_protections_via_shortcuts() {
    let mocked_response: Vec<TagProtection> =
        serde_json::from_str(include_str!("resources/repo_tag_protections.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_tag_protection_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/tags/protection").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let res1 = client.repos(OWNER, REPO).tag_protection().list().await;
    assert!(res1.is_ok());
    assert_eq!(res1.unwrap().len(), 2);

    let res2 = client.repos(OWNER, REPO).tags().list_protection().await;
    assert!(res2.is_ok());
    assert_eq!(res2.unwrap().len(), 2);
}

#[tokio::test]
async fn should_create_tag_protection() {
    let mocked_response: TagProtection =
        serde_json::from_str(include_str!("resources/repo_tag_protection.json")).unwrap();
    let template = ResponseTemplate::new(201).set_body_json(&mocked_response);
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/tags/protection")))
        .and(body_json(serde_json::json!({
            "pattern": "v1.*"
        })))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "POST on /repos/{OWNER}/{REPO}/tags/protection was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .tags()
        .protection()
        .create("v1.*")
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let protection = result.unwrap();
    assert_eq!(protection.id, 1.into());
    assert_eq!(protection.pattern, "v1.*");
    assert_eq!(protection.enabled, Some(true));
}

#[tokio::test]
async fn should_create_tag_protection_via_shortcuts() {
    let mocked_response: TagProtection =
        serde_json::from_str(include_str!("resources/repo_tag_protection.json")).unwrap();
    let template = ResponseTemplate::new(201).set_body_json(&mocked_response);
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/tags/protection")))
        .and(body_json(serde_json::json!({
            "pattern": "v1.*"
        })))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let res1 = client
        .repos(OWNER, REPO)
        .tag_protection()
        .create("v1.*")
        .await;
    assert!(res1.is_ok());

    let res2 = client
        .repos(OWNER, REPO)
        .tags()
        .create_protection("v1.*")
        .await;
    assert!(res2.is_ok());
}

#[tokio::test]
async fn should_delete_tag_protection() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_tag_protection_mock(
        "DELETE",
        format!("/repos/{OWNER}/{REPO}/tags/protection/{TAG_PROTECTION_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .tags()
        .protection()
        .delete(TAG_PROTECTION_ID)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_delete_tag_protection_via_shortcuts() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_tag_protection_mock(
        "DELETE",
        format!("/repos/{OWNER}/{REPO}/tags/protection/{TAG_PROTECTION_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let res1 = client
        .repos(OWNER, REPO)
        .tag_protection()
        .delete(TAG_PROTECTION_ID)
        .await;
    assert!(res1.is_ok());

    let res2 = client
        .repos(OWNER, REPO)
        .tags()
        .delete_protection(TAG_PROTECTION_ID)
        .await;
    assert!(res2.is_ok());
}

#[tokio::test]
async fn should_handle_tag_protection_errors() {
    let template = ResponseTemplate::new(StatusCode::NOT_FOUND.as_u16());
    let mock_server = setup_tag_protection_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/tags/protection").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.repos(OWNER, REPO).tags().protection().list().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn should_handle_delete_tag_protection_errors() {
    let template = ResponseTemplate::new(StatusCode::FORBIDDEN.as_u16());
    let mock_server = setup_tag_protection_mock(
        "DELETE",
        format!("/repos/{OWNER}/{REPO}/tags/protection/{TAG_PROTECTION_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .tags()
        .protection()
        .delete(TAG_PROTECTION_ID)
        .await;
    assert!(result.is_err());
}
