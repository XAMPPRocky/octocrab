use http::StatusCode;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::{Followee, Follower};
use octocrab::Octocrab;

mod mock_error;

async fn setup_followers_mock(
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
async fn should_list_people_authenticated_user_follows() {
    let mocked_response: Vec<Followee> =
        serde_json::from_str(include_str!("resources/user_following.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_followers_mock("GET", "/user/following", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .current()
        .follows()
        .per_page(10u8)
        .page(1u32)
        .send()
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.items.len(), 1);
    assert_eq!(response.items[0].login, "hubot");
}

#[tokio::test]
async fn should_check_if_user_is_followed() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_followers_mock("GET", "/user/following/octocat", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.current().is_following("octocat").await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    assert!(result.unwrap());
}

#[tokio::test]
async fn should_check_if_user_is_not_followed() {
    let template = ResponseTemplate::new(StatusCode::NOT_FOUND);
    let mock_server = setup_followers_mock("GET", "/user/following/unknown_user", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.current().is_following("unknown_user").await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    assert!(!result.unwrap());
}

#[tokio::test]
async fn should_follow_user() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_followers_mock("PUT", "/user/following/octocat", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.current().follow("octocat").await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_unfollow_user() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_followers_mock("DELETE", "/user/following/octocat", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.current().unfollow("octocat").await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_list_followers_of_authenticated_user() {
    let mocked_response: Vec<Follower> =
        serde_json::from_str(include_str!("resources/user_followers.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_followers_mock("GET", "/user/followers", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .current()
        .list_followers()
        .per_page(10u8)
        .page(1u32)
        .send()
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.items.len(), 1);
    assert_eq!(response.items[0].login, "octocat");
}
