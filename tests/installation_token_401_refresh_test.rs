use jsonwebtoken::EncodingKey;
use octocrab::models::{AppId, InstallationId};
use octocrab::Octocrab;
use serde_json::json;
use wiremock::{
    matchers::{body_json, header, method, path},
    Mock, MockServer, ResponseTemplate,
};

const INSTALLATION_TOKEN_PATH: &str = "/app/installations/1/access_tokens";
const ISSUES_PATH: &str = "/repos/o/r/issues";

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

fn token_response() -> ResponseTemplate {
    ResponseTemplate::new(201).set_body_json(json!({
        "token": "dummy-installation-token",
        "expires_at": "2099-01-01T00:00:00Z",
        "permissions": {}
    }))
}

async fn mount_token_mock(mock_server: &MockServer) {
    Mock::given(method("POST"))
        .and(path(INSTALLATION_TOKEN_PATH))
        .and(header("content-type", "application/json"))
        .and(body_json(json!({})))
        .respond_with(token_response())
        .expect(2)
        .mount(mock_server)
        .await;
}

#[tokio::test]
async fn installation_token_is_refreshed_and_request_is_resent_after_unauthorized() {
    let mock_server = MockServer::start().await;
    mount_token_mock(&mock_server).await;

    Mock::given(method("GET"))
        .and(path(ISSUES_PATH))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({
            "message": "Bad credentials"
        })))
        .up_to_n_times(1)
        .expect(1)
        .with_priority(1)
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path(ISSUES_PATH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .expect(1)
        .with_priority(2)
        .mount(&mock_server)
        .await;

    let installation = app_client(&mock_server)
        .installation(InstallationId(1))
        .unwrap();

    let issues = installation.issues("o", "r").list().send().await.unwrap();

    assert!(issues.items.is_empty());
}

#[tokio::test]
async fn persistent_unauthorized_response_is_resent_only_once() {
    let mock_server = MockServer::start().await;
    mount_token_mock(&mock_server).await;

    Mock::given(method("GET"))
        .and(path(ISSUES_PATH))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({
            "message": "Bad credentials"
        })))
        .expect(2)
        .mount(&mock_server)
        .await;

    let installation = app_client(&mock_server)
        .installation(InstallationId(1))
        .unwrap();

    let error = installation
        .issues("o", "r")
        .list()
        .send()
        .await
        .unwrap_err();

    assert!(matches!(
        error,
        octocrab::Error::GitHub { ref source, .. }
            if source.status_code == http::StatusCode::UNAUTHORIZED
    ));
}
