use http::StatusCode;
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::orgs::security::{SecurityEnablement, SecurityProduct};
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "owner";
const TEAM_SLUG: &str = "justice-league";

async fn setup_mock(
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
        &format!(
            "http method {} on {} was not received",
            http_method, mocked_path
        ),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_list_security_managers() {
    let template = ResponseTemplate::new(200).set_body_json(json!([]));
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/security-managers", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.orgs(OWNER).list_security_managers().await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_add_security_manager_team() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "PUT",
        format!("/orgs/{}/security-managers/teams/{}", OWNER, TEAM_SLUG).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .add_security_manager_team(TEAM_SLUG)
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_remove_security_manager_team() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "DELETE",
        format!("/orgs/{}/security-managers/teams/{}", OWNER, TEAM_SLUG).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .remove_security_manager_team(TEAM_SLUG)
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_set_security_product_enablement() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "PATCH",
        format!("/orgs/{}/dependabot_alerts/enable_all", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .set_security_product_enablement(
            SecurityProduct::DependabotAlerts,
            SecurityEnablement::EnableAll,
        )
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}
