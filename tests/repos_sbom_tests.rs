use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::repos::sbom::SbomDependencyGraph;
use octocrab::models::repos::sbom::SbomGenerateReportResponse;
use octocrab::{models::repos::sbom::SbomFetchResponse, Octocrab};
use serde_json::{json, Value};
use url::Url;

mod mock_error;

const OWNER: &str = "XAMPPRocky";
const REPO: &str = "octocrab";
// This is an invalid UUID on purpose
const SBOM_UUID: &str = "c0ffee-c0ff-c0ff-f0ff-c0ffeec0ffee";

async fn setup_sbom_generate_report_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/dependency-graph/sbom/generate-report"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!(
            "GET on /repos/{OWNER}/{REPO}/dependency-graph/sbom/generate-report was not received"
        ),
    )
    .await;

    mock_server
}

async fn setup_sbom_fetch_report_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/dependency-graph/sbom/fetch-report/{SBOM_UUID}"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!(
            "GET on /repos/{OWNER}/{REPO}/dependency-graph/sbom/fetch-report/{SBOM_UUID} was not received"
        ),
    )
    .await;

    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn generate_report_404() {
    let mocked_response = json!({
        "status": json!("404"),
        "documentation_url": json!("https://docs.github.com/rest"),
        "errors": Value::Null,
        "message": json!("Not Found")
    });
    let template = ResponseTemplate::new(404).set_body_json(&mocked_response);
    let mock_server = setup_sbom_generate_report_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER.to_owned(), REPO.to_owned())
        .sbom()
        .generate_report()
        .await;

    assert!(
        result.is_err(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn generate_report_201() {
    let s: &str = include_str!("resources/repos_sbom_tests_generatereport.json");
    let report_response: SbomGenerateReportResponse = serde_json::from_str(s).unwrap();
    let template = ResponseTemplate::new(201).set_body_json(&report_response);
    let mock_server = setup_sbom_generate_report_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER.to_owned(), REPO.to_owned())
        .sbom()
        .generate_report()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:?}",
        result
    );

    let response = result.unwrap();
    let report_url: Url = response.sbom_url;

    assert_eq!(
        report_url,
        Url::parse(format!("https://api.github.com/repos/{OWNER}/{REPO}/dependency-graph/sbom/fetch-report/{SBOM_UUID}").as_str()).unwrap()
    );
}

#[tokio::test]
async fn fetch_report_validation_fail() {
    let mocked_response = json!({
        "message": json!("SBOM report not found or has expired."),
        "documentation_url": json!("https://docs.github.com/rest/dependency-graph/sboms#fetch-a-software-bill-of-materials-sbom-for-a-repository"),
        "status": json!("404"),
        "errors": Value::Null,
    });
    let template = ResponseTemplate::new(404).set_body_json(&mocked_response);
    let mock_server = setup_sbom_fetch_report_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let report_url = SbomGenerateReportResponse {
        sbom_url: Url::parse(format!("https://api.github.com/repos/{OWNER}/{REPO}/dependency-graph/sbom/fetch-report/invaliduuid{SBOM_UUID}").as_str()).unwrap()
    };

    let result = client
        .repos(OWNER.to_owned(), REPO.to_owned())
        .sbom()
        .fetch_report(report_url)
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn fetch_report_404() {
    let mocked_response = json!({
        "message": json!("SBOM report not found or has expired."),
        "documentation_url": json!("https://docs.github.com/rest/dependency-graph/sboms#fetch-a-software-bill-of-materials-sbom-for-a-repository"),
        "status": json!("404"),
        "errors": Value::Null,
    });
    let template = ResponseTemplate::new(404).set_body_json(&mocked_response);
    let mock_server = setup_sbom_fetch_report_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let report_url = SbomGenerateReportResponse {
        sbom_url: Url::parse(format!("https://api.github.com/repos/{OWNER}/{REPO}/dependency-graph/sbom/fetch-report/{SBOM_UUID}").as_str()).unwrap()
    };

    let result = client
        .repos(OWNER.to_owned(), REPO.to_owned())
        .sbom()
        .fetch_report(report_url)
        .await;
    let response = result.unwrap();
    let expected = SbomFetchResponse::NotFound;
    assert_eq!(response, expected);
}

#[tokio::test]
async fn fetch_report_202() {
    let template = ResponseTemplate::new(202).set_body_json("{}");
    let mock_server = setup_sbom_fetch_report_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let report_url = SbomGenerateReportResponse {
        sbom_url: Url::parse(format!("https://api.github.com/repos/{OWNER}/{REPO}/dependency-graph/sbom/fetch-report/{SBOM_UUID}").as_str()).unwrap()
    };
    let result = client
        .repos(OWNER.to_owned(), REPO.to_owned())
        .sbom()
        .fetch_report(report_url)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:?}",
        result
    );

    let response = result.unwrap();
    let expected = SbomFetchResponse::NotReady;

    assert_eq!(response, expected);
}

#[tokio::test]
async fn fetch_report_200() {
    let s: &str = include_str!("resources/repos_sbom_tests_report.json");
    let report_content: SbomDependencyGraph = serde_json::from_str(s).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&report_content);
    let mock_server = setup_sbom_fetch_report_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let report_url = SbomGenerateReportResponse {
        sbom_url: Url::parse(format!("https://api.github.com/repos/{OWNER}/{REPO}/dependency-graph/sbom/fetch-report/{SBOM_UUID}").as_str()).unwrap()
    };

    let result = client
        .repos(OWNER.to_owned(), REPO.to_owned())
        .sbom()
        .fetch_report(report_url)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:?}",
        result
    );

    let response = result.unwrap();
    let expected = SbomFetchResponse::Ready {
        graph: Box::new(report_content.clone()),
    };
    assert_eq!(response, expected);
}
