use http::StatusCode;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "owner";
const REPO: &str = "repo";

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

// ---------------------------------------------------------------------------
// Vulnerability Alerts
// ---------------------------------------------------------------------------

#[tokio::test]
async fn should_check_vulnerability_alerts_enabled() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/vulnerability-alerts")))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("GET on /repos/{OWNER}/{REPO}/vulnerability-alerts was not received"),
    )
    .await;

    let enabled = octocrab
        .repos(OWNER, REPO)
        .vulnerability_alerts()
        .check()
        .await
        .unwrap();

    assert!(enabled);
}

#[tokio::test]
async fn should_check_vulnerability_alerts_disabled() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/vulnerability-alerts")))
        .respond_with(ResponseTemplate::new(StatusCode::NOT_FOUND))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("GET on /repos/{OWNER}/{REPO}/vulnerability-alerts was not received"),
    )
    .await;

    let enabled = octocrab
        .repos(OWNER, REPO)
        .vulnerability_alerts()
        .check()
        .await
        .unwrap();

    assert!(!enabled);
}

#[tokio::test]
async fn should_enable_vulnerability_alerts() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("PUT"))
        .and(path(format!("/repos/{OWNER}/{REPO}/vulnerability-alerts")))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("PUT on /repos/{OWNER}/{REPO}/vulnerability-alerts was not received"),
    )
    .await;

    let result = octocrab
        .repos(OWNER, REPO)
        .vulnerability_alerts()
        .enable()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn should_disable_vulnerability_alerts() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("DELETE"))
        .and(path(format!("/repos/{OWNER}/{REPO}/vulnerability-alerts")))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("DELETE on /repos/{OWNER}/{REPO}/vulnerability-alerts was not received"),
    )
    .await;

    let result = octocrab
        .repos(OWNER, REPO)
        .vulnerability_alerts()
        .disable()
        .await;

    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// Automated Security Fixes
// ---------------------------------------------------------------------------

#[tokio::test]
async fn should_get_automated_security_fixes() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/automated-security-fixes"
        )))
        .respond_with(
            ResponseTemplate::new(StatusCode::OK)
                .set_body_json(serde_json::json!({ "enabled": true, "paused": false })),
        )
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("GET on /repos/{OWNER}/{REPO}/automated-security-fixes was not received"),
    )
    .await;

    let status = octocrab
        .repos(OWNER, REPO)
        .automated_security_fixes()
        .get()
        .await
        .unwrap();

    assert!(status.enabled);
    assert!(!status.paused);
}

#[tokio::test]
async fn should_enable_automated_security_fixes() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("PUT"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/automated-security-fixes"
        )))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("PUT on /repos/{OWNER}/{REPO}/automated-security-fixes was not received"),
    )
    .await;

    let result = octocrab
        .repos(OWNER, REPO)
        .automated_security_fixes()
        .enable()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn should_disable_automated_security_fixes() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("DELETE"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/automated-security-fixes"
        )))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("DELETE on /repos/{OWNER}/{REPO}/automated-security-fixes was not received"),
    )
    .await;

    let result = octocrab
        .repos(OWNER, REPO)
        .automated_security_fixes()
        .disable()
        .await;

    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// Private Vulnerability Reporting
// ---------------------------------------------------------------------------

#[tokio::test]
async fn should_get_private_vulnerability_reporting() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/private-vulnerability-reporting"
        )))
        .respond_with(
            ResponseTemplate::new(StatusCode::OK)
                .set_body_json(serde_json::json!({ "enabled": true })),
        )
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("GET on /repos/{OWNER}/{REPO}/private-vulnerability-reporting was not received"),
    )
    .await;

    let status = octocrab
        .repos(OWNER, REPO)
        .private_vulnerability_reporting()
        .get()
        .await
        .unwrap();

    assert!(status.enabled);

    let is_enabled = octocrab
        .repos(OWNER, REPO)
        .private_vulnerability_reporting()
        .is_enabled()
        .await
        .unwrap();

    assert!(is_enabled);
}

#[tokio::test]
async fn should_enable_private_vulnerability_reporting() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("PUT"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/private-vulnerability-reporting"
        )))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("PUT on /repos/{OWNER}/{REPO}/private-vulnerability-reporting was not received"),
    )
    .await;

    let result = octocrab
        .repos(OWNER, REPO)
        .private_vulnerability_reporting()
        .enable()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn should_disable_private_vulnerability_reporting() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("DELETE"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/private-vulnerability-reporting"
        )))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!(
            "DELETE on /repos/{OWNER}/{REPO}/private-vulnerability-reporting was not received"
        ),
    )
    .await;

    let result = octocrab
        .repos(OWNER, REPO)
        .private_vulnerability_reporting()
        .disable()
        .await;

    assert!(result.is_ok());
}
