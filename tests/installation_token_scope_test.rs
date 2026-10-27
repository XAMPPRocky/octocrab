use jsonwebtoken::EncodingKey;
use octocrab::models::{AppId, InstallationId, RepositoryId};
use octocrab::Octocrab;
use serde_json::json;
use wiremock::{
    matchers::{body_json, header, method, path},
    Mock, MockServer, ResponseTemplate,
};

const INSTALLATION_TOKEN_PATH: &str = "/app/installations/1/access_tokens";

fn app_client(mock_server: &MockServer) -> Octocrab {
    // This is a dummy test key and is never used as a real credential.
    let key =
        EncodingKey::from_rsa_pem(include_bytes!("resources/test_app_private_key.pem")).unwrap();

    Octocrab::builder()
        .base_uri(mock_server.uri())
        .unwrap()
        .app(AppId(42), key)
        .build()
        .unwrap()
}

fn token_response(expires_at: &str) -> ResponseTemplate {
    ResponseTemplate::new(201).set_body_json(json!({
        "token": "dummy-installation-token",
        "expires_at": expires_at,
        "permissions": {}
    }))
}

#[tokio::test]
async fn scoped_installation_token_sends_repository_scope() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(INSTALLATION_TOKEN_PATH))
        .and(header("content-type", "application/json"))
        .and(body_json(json!({
            "repositories": ["my-repo"],
            "repository_ids": [123]
        })))
        .respond_with(token_response("2099-01-01T00:00:00Z"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let installation = app_client(&mock_server)
        .installation_builder(InstallationId(1))
        .repositories(vec!["my-repo".to_string()])
        .repository_ids(vec![RepositoryId(123)])
        .build()
        .unwrap();

    installation.installation_token().await.unwrap();
}

#[tokio::test]
async fn installation_token_without_scope_sends_empty_body() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(INSTALLATION_TOKEN_PATH))
        .and(header("content-type", "application/json"))
        .and(body_json(json!({})))
        .respond_with(token_response("2099-01-01T00:00:00Z"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let installation = app_client(&mock_server)
        .installation(InstallationId(1))
        .unwrap();

    installation.installation_token().await.unwrap();
}

#[tokio::test]
async fn automatic_refresh_retains_repository_scope() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(INSTALLATION_TOKEN_PATH))
        .and(header("content-type", "application/json"))
        .and(body_json(json!({
            "repositories": ["my-repo"],
            "repository_ids": [123]
        })))
        .respond_with(token_response("2000-01-01T00:00:00Z"))
        .expect(2..)
        .mount(&mock_server)
        .await;

    let installation = app_client(&mock_server)
        .installation_builder(InstallationId(1))
        .repositories(vec!["my-repo".to_string()])
        .repository_ids(vec![RepositoryId(123)])
        .build()
        .unwrap();

    installation.installation_token().await.unwrap();
    installation.installation_token().await.unwrap();
}
