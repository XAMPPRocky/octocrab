mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{
    auth::{self, ExchangeWebFlowCodeBuilder},
    Octocrab,
};
use secrecy::SecretString;
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_post_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!("/login/oauth/access_token")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("POST on /login/oauth/access_token was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const CLIENT_SECRET: &str = "some_secret";
const CLIENT_ID: &str = "some_client_id";
const CODE: &str = "a_code";
const REDIRECT_URI: &str = "https://yourapp/auth/callback-example";

#[tokio::test]
async fn should_return_oauth_response() {
    let expected_response = json!({
        "access_token":"gho_16C7e42F292c6912E7710c838347Ae178B4a",
        "scope":"repo,gist",
        "token_type":"bearer"
        }
    );
    let template = ResponseTemplate::new(201).set_body_json(expected_response);
    let mock_server = setup_post_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = auth::ExchangeWebFlowCodeBuilder::new(
        &client,
        &SecretString::from(CLIENT_ID),
        &SecretString::from(CLIENT_SECRET),
    )
    .code(CODE)
    .redirect_uri(REDIRECT_URI.to_owned())
    .send()
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn should_fail_when_receving_a_server_error() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_post_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = auth::ExchangeWebFlowCodeBuilder::new(
        &client,
        &SecretString::from(CLIENT_ID),
        &SecretString::from(CLIENT_SECRET),
    )
    .send()
    .await;

    assert!(result.is_err());
}
