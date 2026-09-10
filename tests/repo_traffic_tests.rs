use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::repos::{Clones, PathTraffic, ReferrerTraffic, Views};
use octocrab::params::repos::TrafficInterval;
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "owner";
const REPO: &str = "repo";

async fn setup_traffic_mock(
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
async fn should_get_clones() {
    let mocked_response: Clones =
        serde_json::from_str(include_str!("resources/repo_traffic_clones.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/traffic/clones")))
        .and(query_param("per", "week"))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET on /repos/{OWNER}/{REPO}/traffic/clones was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .traffic()
        .clones()
        .per(TrafficInterval::Week)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.count, 123);
    assert_eq!(response.uniques, 45);
    assert_eq!(response.clones.len(), 2);
    assert_eq!(response.clones[0].count, 70);
    assert_eq!(response.clones[0].uniques, 25);
    assert_eq!(response.clones[1].count, 53);
}

#[tokio::test]
async fn should_get_views() {
    let mocked_response: Views =
        serde_json::from_str(include_str!("resources/repo_traffic_views.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/traffic/views")))
        .and(query_param("per", "day"))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET on /repos/{OWNER}/{REPO}/traffic/views was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .traffic()
        .views()
        .per(TrafficInterval::Day)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.count, 1234);
    assert_eq!(response.uniques, 456);
    assert_eq!(response.views.len(), 2);
    assert_eq!(response.views[0].count, 600);
    assert_eq!(response.views[0].uniques, 220);
    assert_eq!(response.views[1].count, 634);
}

#[tokio::test]
async fn should_get_popular_paths() {
    let mocked_response: Vec<PathTraffic> =
        serde_json::from_str(include_str!("resources/repo_traffic_paths.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_traffic_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/traffic/popular/paths").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.repos(OWNER, REPO).traffic().popular_paths().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.len(), 2);
    assert_eq!(response[0].path, "/octocat/Hello-World");
    assert_eq!(
        response[0].title,
        "octocat/Hello-World: My first repository"
    );
    assert_eq!(response[0].count, 350);
    assert_eq!(response[0].uniques, 120);

    // Test alias
    let alias_result = client.repos(OWNER, REPO).traffic().paths().await;
    assert!(alias_result.is_ok());
}

#[tokio::test]
async fn should_get_popular_referrers() {
    let mocked_response: Vec<ReferrerTraffic> =
        serde_json::from_str(include_str!("resources/repo_traffic_referrers.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_traffic_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/traffic/popular/referrers").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .traffic()
        .popular_referrers()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.len(), 2);
    assert_eq!(response[0].referrer, "Google");
    assert_eq!(response[0].count, 100);
    assert_eq!(response[0].uniques, 80);

    // Test alias
    let alias_result = client.repos(OWNER, REPO).traffic().referrers().await;
    assert!(alias_result.is_ok());
}
