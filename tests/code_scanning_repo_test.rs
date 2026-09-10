use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::code_scannings::{
    AlertInstance, Analysis, CodeScanningAlert, CodeqlDatabase, DefaultSetup, UpdateDefaultSetup,
    UploadSarif,
};
use octocrab::params;
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "test-owner";
const REPO: &str = "test-repo";

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn check_repo_code_scanning_alerts_list_200() {
    let mock_server = MockServer::start().await;
    let s = include_str!("resources/codescanning_alerts_multiple.json");
    let alerts: Vec<CodeScanningAlert> = serde_json::from_str(s).unwrap();

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/code-scanning/alerts")))
        .respond_with(ResponseTemplate::new(200).set_body_json(&alerts))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/code-scanning/alerts was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .code_scanning()
        .list()
        .send()
        .await;

    assert!(result.is_ok(), "error: {:?}", result);
    let page = result.unwrap();
    assert_eq!(page.items.len(), 2);
}

#[tokio::test]
async fn check_repo_code_scanning_alert_get_200() {
    let mock_server = MockServer::start().await;
    let s = include_str!("resources/codescanning_alert_single.json");
    let alert: CodeScanningAlert = serde_json::from_str(s).unwrap();

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/code-scanning/alerts/42"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(&alert))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/code-scanning/alerts/42 was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).code_scanning().get(42).await;

    assert!(result.is_ok(), "error: {:?}", result);
    let item = result.unwrap();
    assert_eq!(item.number, 42);
}

#[tokio::test]
async fn check_repo_code_scanning_alert_update_200() {
    let mock_server = MockServer::start().await;
    let s = include_str!("resources/codescanning_alert_single.json");
    let alert: CodeScanningAlert = serde_json::from_str(s).unwrap();

    Mock::given(method("PATCH"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/code-scanning/alerts/42"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(&alert))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("PATCH on /repos/{OWNER}/{REPO}/code-scanning/alerts/42 was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .code_scanning()
        .update(42)
        .state(params::AlertState::Dismissed)
        .dismissed_reason("false positive")
        .dismissed_comment("test comment")
        .send()
        .await;

    assert!(result.is_ok(), "error: {:?}", result);
    assert_eq!(result.unwrap().number, 42);
}

#[tokio::test]
async fn check_repo_code_scanning_alert_instances_list_200() {
    let mock_server = MockServer::start().await;
    let s = include_str!("resources/code_scanning_alert_instances.json");
    let instances: Vec<AlertInstance> = serde_json::from_str(s).unwrap();

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/code-scanning/alerts/42/instances"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(&instances))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/code-scanning/alerts/42/instances was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .code_scanning()
        .list_instances(42)
        .send()
        .await;

    assert!(result.is_ok(), "error: {:?}", result);
    let page = result.unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].ref_field, "refs/heads/main");
}

#[tokio::test]
async fn check_repo_code_scanning_analyses_list_200() {
    let mock_server = MockServer::start().await;
    let s = include_str!("resources/code_scanning_analyses.json");
    let analyses: Vec<Analysis> = serde_json::from_str(s).unwrap();

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/code-scanning/analyses"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(&analyses))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/code-scanning/analyses was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .code_scannings() // testing plural alias
        .list_analyses()
        .send()
        .await;

    assert!(result.is_ok(), "error: {:?}", result);
    let page = result.unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].id, 12345);
}

#[tokio::test]
async fn check_repo_code_scanning_analysis_get_200() {
    let mock_server = MockServer::start().await;
    let s = include_str!("resources/code_scanning_analysis.json");
    let analysis: Analysis = serde_json::from_str(s).unwrap();

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/code-scanning/analyses/12345"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(&analysis))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/code-scanning/analyses/12345 was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .code_scanning()
        .get_analysis(12345)
        .await;

    assert!(result.is_ok(), "error: {:?}", result);
    assert_eq!(result.unwrap().id, 12345);
}

#[tokio::test]
async fn check_repo_code_scanning_analysis_delete_200() {
    let mock_server = MockServer::start().await;
    let delete_resp = serde_json::json!({
        "next_analysis_url": null,
        "confirm_delete_url": null,
    });

    Mock::given(method("DELETE"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/code-scanning/analyses/12345"
        )))
        .and(query_param("confirm_delete", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&delete_resp))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("DELETE on /repos/{OWNER}/{REPO}/code-scanning/analyses/12345 was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .code_scanning()
        .delete_analysis(12345, Some("true"))
        .await;

    assert!(result.is_ok(), "error: {:?}", result);
}

#[tokio::test]
async fn check_repo_code_scanning_codeql_databases_list_200() {
    let mock_server = MockServer::start().await;
    let s = include_str!("resources/code_scanning_codeql_databases.json");
    let dbs: Vec<CodeqlDatabase> = serde_json::from_str(s).unwrap();

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/code-scanning/codeql/databases"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(&dbs))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/code-scanning/codeql/databases was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .code_scanning()
        .list_codeql_databases()
        .await;

    assert!(result.is_ok(), "error: {:?}", result);
    let list = result.unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].language, "javascript");
}

#[tokio::test]
async fn check_repo_code_scanning_codeql_database_get_200() {
    let mock_server = MockServer::start().await;
    let s = include_str!("resources/code_scanning_codeql_databases.json");
    let dbs: Vec<CodeqlDatabase> = serde_json::from_str(s).unwrap();
    let db = &dbs[0];

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/code-scanning/codeql/databases/javascript"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(db))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/code-scanning/codeql/databases/javascript was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .code_scanning()
        .get_codeql_database("javascript")
        .await;

    assert!(result.is_ok(), "error: {:?}", result);
    assert_eq!(result.unwrap().language, "javascript");
}

#[tokio::test]
async fn check_repo_code_scanning_codeql_database_delete_204() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/code-scanning/codeql/databases/javascript"
        )))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("DELETE on /repos/{OWNER}/{REPO}/code-scanning/codeql/databases/javascript was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .code_scanning()
        .delete_codeql_database("javascript")
        .await;

    assert!(result.is_ok(), "error: {:?}", result);
}

#[tokio::test]
async fn check_repo_code_scanning_default_setup_get_200() {
    let mock_server = MockServer::start().await;
    let s = include_str!("resources/code_scanning_default_setup.json");
    let setup: DefaultSetup = serde_json::from_str(s).unwrap();

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/code-scanning/default-setup"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(&setup))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/code-scanning/default-setup was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .code_scanning()
        .get_default_setup()
        .await;

    assert!(result.is_ok(), "error: {:?}", result);
    assert_eq!(result.unwrap().state, Some("configured".to_string()));
}

#[tokio::test]
async fn check_repo_code_scanning_default_setup_update_200() {
    let mock_server = MockServer::start().await;
    let resp = serde_json::json!({
        "run_id": 999,
        "run_url": "https://api.github.com/runs/999",
    });

    Mock::given(method("PATCH"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/code-scanning/default-setup"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(&resp))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("PATCH on /repos/{OWNER}/{REPO}/code-scanning/default-setup was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let body = UpdateDefaultSetup {
        state: Some("configured".to_string()),
        threat_model: Some("remote_and_local".to_string()),
        ..Default::default()
    };
    let result = client
        .repos(OWNER, REPO)
        .code_scanning()
        .update_default_setup(&body)
        .await;

    assert!(result.is_ok(), "error: {:?}", result);
    assert_eq!(result.unwrap().run_id, Some(999));
}

#[tokio::test]
async fn check_repo_code_scanning_upload_sarif_202() {
    let mock_server = MockServer::start().await;
    let resp = serde_json::json!({
        "id": "sarif-receipt-123",
        "url": null,
    });

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/code-scanning/sarifs")))
        .respond_with(ResponseTemplate::new(202).set_body_json(&resp))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("POST on /repos/{OWNER}/{REPO}/code-scanning/sarifs was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let body = UploadSarif {
        commit_sha: "4b6472266afd7b471e86085a6659e8c7f2b119da".to_string(),
        ref_field: "refs/heads/main".to_string(),
        sarif: "H4sIC...".to_string(),
        checkout_uri: None,
        started_at: None,
        tool_name: None,
        validate: None,
    };
    let result = client
        .repos(OWNER, REPO)
        .code_scanning()
        .upload_sarif(&body)
        .await;

    assert!(result.is_ok(), "error: {:?}", result);
    assert_eq!(result.unwrap().id, "sarif-receipt-123");
}

#[tokio::test]
async fn check_repo_code_scanning_sarif_get_200() {
    let mock_server = MockServer::start().await;
    let resp = serde_json::json!({
        "processing_status": "complete",
        "analyses_url": null,
        "errors": null,
    });

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/code-scanning/sarifs/sarif-receipt-123"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(&resp))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!(
            "GET on /repos/{OWNER}/{REPO}/code-scanning/sarifs/sarif-receipt-123 was not received"
        ),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .code_scanning()
        .get_sarif("sarif-receipt-123")
        .await;

    assert!(result.is_ok(), "error: {:?}", result);
    assert_eq!(result.unwrap().processing_status, "complete");
}
