use http::StatusCode;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

mod mock_error;
use mock_error::setup_error_handler;
use octocrab::models::security_advisories::{
    CreatePackage, CreateRepositoryAdvisory, CreateVulnerability, ReportVulnerability,
    RepositoryAdvisory, SecurityAdvisory, UpdateRepositoryAdvisory,
};
use octocrab::models::Repository;
use octocrab::Octocrab;

const GHSA_ID: &str = "GHSA-xxxx-yyyy-zzzz";
const REPO_GHSA_ID: &str = "GHSA-aaaa-bbbb-cccc";
const OWNER: &str = "octocat";
const REPO: &str = "Hello-World";
const ORG: &str = "github";

async fn setup_mock_server(
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
        &format!("http method {http_method} on {mocked_path} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_list_global_security_advisories() {
    let mocked_response: Vec<SecurityAdvisory> =
        serde_json::from_str(include_str!("resources/global_security_advisories.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = setup_mock_server("GET", "/advisories", template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .security_advisories()
        .list_global_advisories()
        .severity("high")
        .per_page(10u8)
        .send()
        .await;

    assert!(result.is_ok(), "expected successful result, got: {:?}", result);
    let page = result.unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].ghsa_id, GHSA_ID);
    assert_eq!(page.items[0].severity, "high");
}

#[tokio::test]
async fn should_get_global_security_advisory() {
    let mocked_response: SecurityAdvisory =
        serde_json::from_str(include_str!("resources/global_security_advisory.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = setup_mock_server("GET", format!("/advisories/{GHSA_ID}").as_str(), template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .security_advisories()
        .get_global_advisory(GHSA_ID)
        .await;

    assert!(result.is_ok(), "expected successful result, got: {:?}", result);
    let advisory = result.unwrap();
    assert_eq!(advisory.ghsa_id, GHSA_ID);
    assert_eq!(advisory.summary, "Sample Global Security Advisory");
}

#[tokio::test]
async fn should_list_org_security_advisories() {
    let mocked_response: Vec<RepositoryAdvisory> =
        serde_json::from_str(include_str!("resources/org_security_advisories.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = setup_mock_server(
        "GET",
        format!("/orgs/{ORG}/security-advisories").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(ORG)
        .security_advisories()
        .state("published")
        .per_page(20u8)
        .send()
        .await;

    assert!(result.is_ok(), "expected successful result, got: {:?}", result);
    let page = result.unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].ghsa_id, REPO_GHSA_ID);
}

#[tokio::test]
async fn should_list_repo_security_advisories() {
    let mocked_response: Vec<RepositoryAdvisory> =
        serde_json::from_str(include_str!("resources/repo_security_advisories.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = setup_mock_server(
        "GET",
        format!("/repos/{OWNER}/{REPO}/security-advisories").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .security_advisories()
        .list()
        .state("published")
        .send()
        .await;

    assert!(result.is_ok(), "expected successful result, got: {:?}", result);
    let page = result.unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].ghsa_id, REPO_GHSA_ID);
}

#[tokio::test]
async fn should_get_repo_security_advisory() {
    let mocked_response: RepositoryAdvisory =
        serde_json::from_str(include_str!("resources/repo_security_advisory.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = setup_mock_server(
        "GET",
        format!("/repos/{OWNER}/{REPO}/security-advisories/{REPO_GHSA_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .security_advisories()
        .get(REPO_GHSA_ID)
        .await;

    assert!(result.is_ok(), "expected successful result, got: {:?}", result);
    let advisory = result.unwrap();
    assert_eq!(advisory.ghsa_id, REPO_GHSA_ID);
}

#[tokio::test]
async fn should_create_repo_security_advisory() {
    let mocked_response: RepositoryAdvisory =
        serde_json::from_str(include_str!("resources/repo_security_advisory.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::CREATED).set_body_json(&mocked_response);
    let mock_server = setup_mock_server(
        "POST",
        format!("/repos/{OWNER}/{REPO}/security-advisories").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let body = CreateRepositoryAdvisory {
        summary: "Sample Repository Security Advisory".to_string(),
        description: "Detailed description".to_string(),
        vulnerabilities: vec![CreateVulnerability {
            package: CreatePackage {
                ecosystem: "npm".to_string(),
                name: Some("sample-package".to_string()),
            },
            vulnerable_version_range: Some("< 1.0.0".to_string()),
            patched_versions: Some("1.0.0".to_string()),
            vulnerable_functions: None,
        }],
        severity: Some("high".to_string()),
        ..Default::default()
    };

    let result = client
        .repos(OWNER, REPO)
        .security_advisories()
        .create(&body)
        .await;

    assert!(result.is_ok(), "expected successful result, got: {:?}", result);
    let advisory = result.unwrap();
    assert_eq!(advisory.ghsa_id, REPO_GHSA_ID);
}

#[tokio::test]
async fn should_update_repo_security_advisory() {
    let mocked_response: RepositoryAdvisory =
        serde_json::from_str(include_str!("resources/repo_security_advisory.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = setup_mock_server(
        "PATCH",
        format!("/repos/{OWNER}/{REPO}/security-advisories/{REPO_GHSA_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let body = UpdateRepositoryAdvisory {
        summary: Some("Updated Summary".to_string()),
        state: Some("published".to_string()),
        ..Default::default()
    };

    let result = client
        .repos(OWNER, REPO)
        .security_advisories()
        .update(REPO_GHSA_ID, &body)
        .await;

    assert!(result.is_ok(), "expected successful result, got: {:?}", result);
    let advisory = result.unwrap();
    assert_eq!(advisory.ghsa_id, REPO_GHSA_ID);
}

#[tokio::test]
async fn should_report_security_vulnerability() {
    let mocked_response: RepositoryAdvisory =
        serde_json::from_str(include_str!("resources/repo_security_advisory.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::CREATED).set_body_json(&mocked_response);
    let mock_server = setup_mock_server(
        "POST",
        format!("/repos/{OWNER}/{REPO}/security-advisories/reports").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let body = ReportVulnerability {
        summary: "Privately Reported Vulnerability".to_string(),
        description: "Vulnerability details".to_string(),
        severity: Some("high".to_string()),
        ..Default::default()
    };

    let result = client
        .repos(OWNER, REPO)
        .security_advisories()
        .report(&body)
        .await;

    assert!(result.is_ok(), "expected successful result, got: {:?}", result);
    let advisory = result.unwrap();
    assert_eq!(advisory.ghsa_id, REPO_GHSA_ID);
}

#[tokio::test]
async fn should_request_cve_for_advisory() {
    let template = ResponseTemplate::new(StatusCode::ACCEPTED);
    let mock_server = setup_mock_server(
        "POST",
        format!("/repos/{OWNER}/{REPO}/security-advisories/{REPO_GHSA_ID}/cve").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .security_advisories()
        .request_cve(REPO_GHSA_ID)
        .await;

    assert!(result.is_ok(), "expected successful result, got: {:?}", result);
}

#[tokio::test]
async fn should_create_temporary_private_fork() {
    let mocked_response: Repository =
        serde_json::from_str(include_str!("resources/repo_security_advisory_fork.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::ACCEPTED).set_body_json(&mocked_response);
    let mock_server = setup_mock_server(
        "POST",
        format!("/repos/{OWNER}/{REPO}/security-advisories/{REPO_GHSA_ID}/forks").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .security_advisories()
        .create_fork(REPO_GHSA_ID)
        .await;

    assert!(result.is_ok(), "expected successful result, got: {:?}", result);
    let fork = result.unwrap();
    assert_eq!(fork.name, "actix-examples");
}
