#![cfg(feature = "jwt")]

use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use futures::future::BoxFuture;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use octocrab::auth::AppSigner;
use octocrab::models::{AppId, InstallationId};
use octocrab::Octocrab;
use serde::Deserialize;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

// These are dummy test keys and are never used as real credentials.
fn private_key() -> EncodingKey {
    EncodingKey::from_rsa_pem(include_bytes!("resources/test_app_private_key.pem")).unwrap()
}

fn public_key() -> DecodingKey {
    DecodingKey::from_rsa_pem(include_bytes!("resources/test_app_public_key.pem")).unwrap()
}

#[derive(Deserialize)]
struct Claims {
    iss: u64,
    iat: u64,
    exp: u64,
}

/// Checks that `jwt` authenticates as app 42 and is signed with the test key.
fn assert_valid_app_jwt(jwt: &str) {
    let claims =
        jsonwebtoken::decode::<Claims>(jwt, &public_key(), &Validation::new(Algorithm::RS256))
            .unwrap()
            .claims;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    assert_eq!(claims.iss, 42);
    assert!(claims.iat.abs_diff(now - 60) <= 5);
    assert_eq!(claims.exp - claims.iat, 10 * 60);
}

async fn mock_server() -> MockServer {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/app/installations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .expect(1)
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/app/installations/1/access_tokens"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "token": "installation-token",
            "expires_at": "2099-01-01T00:00:00Z",
            "permissions": {}
        })))
        .expect(1)
        .mount(&mock_server)
        .await;
    mock_server
}

/// Makes an app-authenticated request and requests an installation token, and
/// checks that both carry a valid app JWT.
async fn assert_requests_use_valid_app_jwts(mock_server: &MockServer, octocrab: Octocrab) {
    octocrab.apps().installations().send().await.unwrap();
    octocrab
        .installation_and_token(InstallationId(1))
        .await
        .unwrap();

    let requests = mock_server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 2);
    for request in &requests {
        let authorization = request.headers.get("authorization").unwrap();
        let jwt = authorization.to_str().unwrap().strip_prefix("Bearer ");
        assert_valid_app_jwt(jwt.unwrap());
    }
}

#[test]
fn create_jwt_creates_a_valid_app_jwt() {
    assert_valid_app_jwt(&octocrab::auth::create_jwt(AppId(42), &private_key()).unwrap());
}

#[tokio::test]
async fn rsa_key_requests_use_valid_app_jwts() {
    let mock_server = mock_server().await;
    let octocrab = Octocrab::builder()
        .base_uri(mock_server.uri())
        .unwrap()
        .app(AppId(42), private_key())
        .build()
        .unwrap();

    assert_requests_use_valid_app_jwts(&mock_server, octocrab).await;
}

/// Signs with the test key the way a KMS holding it would, so octocrab never
/// sees the key.
struct TestKeySigner;

impl AppSigner for TestKeySigner {
    fn sign<'a>(
        &'a self,
        message: &'a [u8],
    ) -> BoxFuture<'a, Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>> {
        Box::pin(async move {
            let signature = jsonwebtoken::crypto::sign(message, &private_key(), Algorithm::RS256)?;
            Ok(BASE64_URL_SAFE_NO_PAD.decode(signature)?)
        })
    }
}

struct FailingSigner;

impl AppSigner for FailingSigner {
    fn sign<'a>(
        &'a self,
        _message: &'a [u8],
    ) -> BoxFuture<'a, Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>> {
        Box::pin(async { Err("signer unavailable".into()) })
    }
}

#[tokio::test]
async fn create_jwt_with_signer_creates_a_valid_app_jwt() {
    let jwt = octocrab::auth::create_jwt_with_signer(AppId(42), &TestKeySigner)
        .await
        .unwrap();
    assert_valid_app_jwt(&jwt);
}

#[tokio::test]
async fn signer_requests_use_valid_app_jwts() {
    let mock_server = mock_server().await;
    let octocrab = Octocrab::builder()
        .base_uri(mock_server.uri())
        .unwrap()
        .app_with_signer(AppId(42), TestKeySigner)
        .build()
        .unwrap();

    assert_requests_use_valid_app_jwts(&mock_server, octocrab).await;
}

#[tokio::test]
async fn signer_errors_are_returned() {
    let mock_server = MockServer::start().await;
    let octocrab = Octocrab::builder()
        .base_uri(mock_server.uri())
        .unwrap()
        .app_with_signer(AppId(42), FailingSigner)
        .build()
        .unwrap();

    let error = octocrab.apps().installations().send().await.unwrap_err();

    assert!(matches!(error, octocrab::Error::AppSigner { .. }));
    assert_eq!(
        error.to_string(),
        "GitHub App signer error: signer unavailable"
    );
    assert!(mock_server.received_requests().await.unwrap().is_empty());
}
