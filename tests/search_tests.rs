mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{models, Octocrab};
use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FakePage<T> {
    pub items: Vec<T>,
    pub incomplete_results: Option<bool>,
    pub total_count: Option<u64>,
}

async fn setup_mock_http_server(
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
async fn should_search_labels() {
    let mocked_response: FakePage<models::search::LabelSearchResultItem> =
        serde_json::from_str(include_str!("resources/search_labels.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/search/labels"))
        .and(query_param("repository_id", "260152030"))
        .and(query_param("q", "bug"))
        .and(query_param("sort", "created"))
        .and(query_param("order", "desc"))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "http method GET on /search/labels was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .search()
        .labels(260152030u64, "bug")
        .sort("created")
        .order("desc")
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let mut response = result.unwrap();
    let items = response.take_items();

    assert_eq!(items.len(), 1);
    let item = items.first().unwrap();
    assert_eq!(item.name, "bug");
    assert_eq!(item.color, "d73a4a");
    assert_eq!(item.score, 1.0);
}

#[tokio::test]
async fn should_search_topics() {
    let mocked_response: FakePage<models::search::TopicSearchResultItem> =
        serde_json::from_str(include_str!("resources/search_topics.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/search/topics"))
        .and(query_param("q", "rust"))
        .and(query_param("per_page", "10"))
        .and(query_param("page", "1"))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "http method GET on /search/topics was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .search()
        .topics("rust")
        .per_page(10u8)
        .page(1u32)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let mut response = result.unwrap();
    let items = response.take_items();

    assert_eq!(items.len(), 1);
    let item = items.first().unwrap();
    assert_eq!(item.name, "rust");
    assert_eq!(item.display_name, Some("Rust".to_string()));
    assert!(item.curated);
    assert!(!item.featured);
}

#[tokio::test]
async fn should_search_issues() {
    let mocked_response: FakePage<models::issues::Issue> =
        serde_json::from_str(include_str!("resources/search_issues.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_mock_http_server("GET", "/search/issues", template).await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.search().issues("octocrab").send().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let mut response = result.unwrap();
    let items = response.take_items();

    assert_eq!(items.len(), 1);
    let item = items.first().unwrap();
    assert_eq!(
        item.title,
        "new dismiss reason type for code scanning alerts: mitigated"
    );
}

#[tokio::test]
async fn should_search_repositories() {
    let mocked_response: FakePage<models::Repository> =
        serde_json::from_str(include_str!("resources/search_repositories.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_mock_http_server("GET", "/search/repositories", template).await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .search()
        .repositories("octocrab")
        .sort("stars")
        .order("desc")
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let mut response = result.unwrap();
    let items = response.take_items();

    assert_eq!(items.len(), 1);
    let item = items.first().unwrap();
    assert_eq!(item.name, "octocrab");
}

#[tokio::test]
async fn should_search_users() {
    let mocked_response: FakePage<models::Author> =
        serde_json::from_str(include_str!("resources/search_users.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_mock_http_server("GET", "/search/users", template).await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .search()
        .users("octocat")
        .sort("followers")
        .order("desc")
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let mut response = result.unwrap();
    let items = response.take_items();

    assert_eq!(items.len(), 1);
    let item = items.first().unwrap();
    assert_eq!(item.login, "octocat");
}

#[tokio::test]
async fn should_search_code() {
    let mocked_response: FakePage<models::Code> =
        serde_json::from_str(include_str!("resources/search_code.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_mock_http_server("GET", "/search/code", template).await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .search()
        .code("README.md repo:XAMPPRocky/octocrab")
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let mut response = result.unwrap();
    let items = response.take_items();

    assert_eq!(items.len(), 1);
    let item = items.first().unwrap();
    assert_eq!(item.name, "README.md");
}
