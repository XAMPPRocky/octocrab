use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::code_scannings::CodeScanningAlert;
use octocrab::Octocrab;

mod mock_error;

const ORG: &str = "test-org";

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn check_org_code_scanning_alerts_list_200() {
    let mock_server = MockServer::start().await;
    let s = include_str!("resources/codescanning_alerts_multiple.json");
    let alerts: Vec<CodeScanningAlert> = serde_json::from_str(s).unwrap();

    Mock::given(method("GET"))
        .and(path(format!("/orgs/{ORG}/code-scanning/alerts")))
        .respond_with(ResponseTemplate::new(200).set_body_json(&alerts))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /orgs/{ORG}/code-scanning/alerts was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.orgs(ORG).code_scanning().list().send().await;

    assert!(result.is_ok(), "error: {:?}", result);
    let page = result.unwrap();
    assert_eq!(page.items.len(), 2);
}

#[tokio::test]
async fn check_org_code_scanning_alerts_query_params() {
    let mock_server = MockServer::start().await;
    let s = include_str!("resources/codescanning_alerts_multiple.json");
    let alerts: Vec<CodeScanningAlert> = serde_json::from_str(s).unwrap();

    Mock::given(method("GET"))
        .and(path(format!("/orgs/{ORG}/code-scanning/alerts")))
        .and(query_param("tool_name", "CodeQL"))
        .and(query_param("per_page", "50"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&alerts))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /orgs/{ORG}/code-scanning/alerts with query params was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .orgs(ORG)
        .code_scannings() // testing plural alias
        .list_alerts() // testing list_alerts alias
        .tool_name("CodeQL")
        .per_page(50u8)
        .send()
        .await;

    assert!(result.is_ok(), "error: {:?}", result);
}
