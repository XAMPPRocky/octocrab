use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::checks::CheckSuite;
use octocrab::Octocrab;

/// Unit test for calls to the `/repos/OWNER/REPO/contributors` endpoint
mod mock_error;

const OWNER: &str = "XAMPPRocky";
const REPO: &str = "octocrab";

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/check-suites")))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "POST on /repos/OWNER/REPO/check-suites not called",
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_create_check_suite() {
    let check_suite_response: CheckSuite =
        serde_json::from_str(include_str!("resources/check_suite.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&check_suite_response);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let head_sha = "d6fde92930d4715a2b49857d24b940956b26d2d3";
    let result = client
        .checks(OWNER, REPO)
        .create_check_suite(head_sha)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let check_suite = result.unwrap();

    assert_eq!(check_suite.head_sha, head_sha);
}
