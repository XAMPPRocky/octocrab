use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::repos::RepoTopics;
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "owner";
const REPO: &str = "repo";

async fn setup_topics_mock(
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
async fn should_get_topics() {
    let mocked_response: RepoTopics =
        serde_json::from_str(include_str!("resources/repo_topics.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_topics_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/topics").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.repos(OWNER, REPO).topics().get().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let topics = result.unwrap();
    assert_eq!(topics.names.len(), 4);
    assert_eq!(&topics[..], &["octocat", "atom", "electron", "api"]);
}

#[tokio::test]
async fn should_list_topics_with_pagination() {
    let mocked_response: RepoTopics =
        serde_json::from_str(include_str!("resources/repo_topics.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_topics_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/topics").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .topics()
        .list()
        .per_page(10)
        .page(2u32)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let topics = result.unwrap();
    assert_eq!(topics.names.len(), 4);
}

#[tokio::test]
async fn should_replace_topics() {
    let mocked_response: RepoTopics =
        serde_json::from_str(include_str!("resources/repo_topics.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_topics_mock(
        "PUT",
        format!("/repos/{OWNER}/{REPO}/topics").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .topics()
        .replace(["octocat", "atom", "electron", "api"])
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let topics = result.unwrap();
    assert_eq!(topics.names, vec!["octocat", "atom", "electron", "api"]);
}
