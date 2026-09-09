use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::Hovercard;
use octocrab::params::users::hovercard::SubjectType;
use octocrab::Octocrab;

mod mock_error;

async fn setup_hovercard_mock(
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
async fn should_respond_to_hovercard() {
    let mocked_response: Hovercard =
        serde_json::from_str(include_str!("resources/user_hovercard.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_hovercard_mock("GET", "/users/some_user/hovercard", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.users("some_user").hovercard().send().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.contexts.len(), 1);
    assert_eq!(response.contexts[0].message, "Owns this repository");
    assert_eq!(response.contexts[0].octicon, "repo");
}

#[tokio::test]
async fn should_respond_to_hovercard_with_subject() {
    let mocked_response: Hovercard =
        serde_json::from_str(include_str!("resources/user_hovercard.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/some_user/hovercard"))
        .and(query_param("subject_type", "repository"))
        .and(query_param("subject_id", "1300192"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .users("some_user")
        .hovercard()
        .with_subject_type(SubjectType::Repository)
        .with_subject_id("1300192")
        .send()
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.contexts.len(), 1);
    assert_eq!(response.contexts[0].message, "Owns this repository");
    assert_eq!(response.contexts[0].octicon, "repo");
}
