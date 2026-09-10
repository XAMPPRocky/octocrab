use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::repos::secret_scanning_alert::{Resolution, SecretScanningAlert, State};
use octocrab::Octocrab;

mod mock_error;

const ENTERPRISE: &str = "test-enterprise";

async fn setup_enterprise_secrets_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/enterprises/{ENTERPRISE}/secret-scanning/alerts"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /enterprises/{ENTERPRISE}/secret-scanning/alerts was not received"),
    )
    .await;

    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn check_enterprise_secrets_alert_list_200() {
    let s: &str = include_str!("resources/check_org_secrets_alerts.json");
    let alert: Vec<SecretScanningAlert> = serde_json::from_str(s).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&alert);
    let mock_server = setup_enterprise_secrets_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .enterprises(ENTERPRISE.to_owned())
        .secret_scanning()
        .get_alerts()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:?}",
        result
    );

    let response = result.unwrap();
    let items = response.items;

    assert_eq!(items.len(), 1);

    let item = &items[0];
    assert_eq!(2, item.number);
    assert_eq!(State::Resolved, item.state);
    assert_eq!(Some(Resolution::FalsePositive), item.resolution);

    let repo = item
        .repository
        .as_ref()
        .expect("repository should be present");
    assert_eq!("Hello-World", repo.name);
}

#[tokio::test]
async fn check_enterprise_secrets_alert_query_params() {
    let s: &str = include_str!("resources/check_org_secrets_alerts.json");
    let alert: Vec<SecretScanningAlert> = serde_json::from_str(s).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&alert);
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/enterprises/{ENTERPRISE}/secret-scanning/alerts"
        )))
        .and(query_param("state", "resolved"))
        .and(query_param("per_page", "25"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .enterprises(ENTERPRISE.to_owned())
        .secrets_scanning() // Testing plural alias
        .state("resolved")
        .per_page(25u8)
        .get_alerts()
        .await;

    assert!(result.is_ok());
}
