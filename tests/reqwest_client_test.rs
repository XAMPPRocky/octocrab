// Tests for `OctocrabBuilder::build_with_reqwest`.
#![cfg(feature = "reqwest")]

mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{Octocrab, ReqwestClientConfig};
use serde_json::{json, Value};
use std::time::Duration;
use wiremock::{
    matchers::{header, method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/user"))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "GET on /user was not received").await;
    mock_server
}

#[tokio::test]
async fn build_with_reqwest_round_trips_a_request() {
    let template = ResponseTemplate::new(200).set_body_json(json!({ "login": "octocat" }));
    let mock_server = setup_api(template).await;

    let http_client = reqwest::Client::builder().build().unwrap();

    let octocrab = Octocrab::builder()
        .base_uri(mock_server.uri())
        .unwrap()
        .personal_token(String::from("some-token"))
        .build_with_reqwest(http_client)
        .unwrap();

    let body: Value = octocrab.get("/user", None::<&()>).await.unwrap();

    assert_eq!(body["login"], "octocat");
}

#[tokio::test]
async fn build_with_reqwest_default_config_builds_a_client() {
    let template = ResponseTemplate::new(200).set_body_json(json!({ "login": "octocat" }));
    let mock_server = setup_api(template).await;

    let octocrab = Octocrab::builder()
        .base_uri(mock_server.uri())
        .unwrap()
        .set_connect_timeout(Some(Duration::from_secs(5)))
        .personal_token(String::from("some-token"))
        .build_with_reqwest(ReqwestClientConfig::Default)
        .unwrap();

    let body: Value = octocrab.get("/user", None::<&()>).await.unwrap();

    assert_eq!(body["login"], "octocat");
}

#[tokio::test]
async fn build_with_reqwest_sends_auth_header() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/user"))
        .and(header("Authorization", "Bearer some-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "login": "octocat" })))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "authenticated GET on /user was not received").await;

    let http_client = reqwest::Client::builder().build().unwrap();

    let octocrab = Octocrab::builder()
        .base_uri(mock_server.uri())
        .unwrap()
        .personal_token(String::from("some-token"))
        .build_with_reqwest(http_client)
        .unwrap();

    let body: Value = octocrab.get("/user", None::<&()>).await.unwrap();

    assert_eq!(body["login"], "octocat");
}
