//! Checks that the client tries to re-fetch an installation token if the contained token has
//! expired.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::models::{AppId, InstallationId, InstallationToken};
use octocrab::Octocrab;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_api(
    token_template: ResponseTemplate,
    secret_template: ResponseTemplate,
) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/app/installations/123/access_tokens"))
        .respond_with(token_template)
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/repos/foo/bar/actions/secrets/GH_TOKEN"))
        .and(wiremock::matchers::header(
            "Authorization",
            "Bearer NEW_TOKEN",
        ))
        .respond_with(secret_template)
        .expect(1)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        "POST on /app/installations/123/access_tokens was not received",
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    let client = Octocrab::builder()
        .base_uri(uri)
        .unwrap()
        .app(
            AppId(456),
            jsonwebtoken::EncodingKey::from_rsa_pem(include_bytes!("resources/sample_app.key"))
                .unwrap(),
        )
        .build()
        .unwrap();

    // Set an expired installation token on this app client
    client
        .installation(InstallationId(123))
        .with_token(gen_installation_access_token(
            "EXPIRED_TOKEN",
            chrono::Utc::now() - chrono::Duration::minutes(1),
        ))
        .build()
}

#[tokio::test]
async fn will_refetch_installation_token() {
    let new_token_response = ResponseTemplate::new(200).set_body_json(
        // New token that expires in the future.
        gen_installation_access_token("NEW_TOKEN", chrono::Utc::now() + chrono::Duration::hours(1)),
    );

    // Some other response to return.
    let other_endpoint_response = ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "name": "GH_TOKEN",
        "created_at": "2019-08-10T14:59:22Z",
        "updated_at": "2019-08-10T14:59:22Z",
    }));

    let mock_server = setup_api(new_token_response, other_endpoint_response).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos("foo", "bar")
        .secrets()
        .get_secret("GH_TOKEN")
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

// Create a sample access token for an installation,
fn gen_installation_access_token(
    token: &str,
    expiration: chrono::DateTime<chrono::Utc>,
) -> InstallationToken {
    // Constructing this from JSON because it's a non-exhaustive struct type.
    serde_json::from_value(serde_json::json!({
        "token": token,
        "expires_at": expiration.to_rfc3339(),
        "permissions":  {
            "actions": "read",
            "checks": "write",
            "contents": "read",
            "issues": "write",
            "metadata": "read",
            "single_file": "write",
            "statuses": "write",
        },
    }))
    .unwrap()
}
