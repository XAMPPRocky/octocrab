use octocrab::Octocrab;
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_get_artifact() {
    let mock_server = MockServer::start().await;
    let artifact_json = json!({
        "id": 123,
        "node_id": "MDg6QXJ0aWZhY3QxMjM=",
        "name": "adventure-time-logs",
        "size_in_bytes": 1024,
        "url": "https://api.github.com/repos/owner/repo/actions/artifacts/123",
        "archive_download_url": "https://api.github.com/repos/owner/repo/actions/artifacts/123/zip",
        "expired": false,
        "created_at": "2023-01-01T00:00:00Z",
        "updated_at": "2023-01-01T00:01:00Z",
        "expires_at": "2023-04-01T00:00:00Z",
    });

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/actions/artifacts/123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&artifact_json))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let artifact = client
        .actions()
        .get_artifact("owner", "repo", 123u64.into())
        .await
        .unwrap();

    assert_eq!(artifact.id.0, 123);
    assert_eq!(artifact.name, "adventure-time-logs");
}

#[tokio::test]
async fn test_delete_artifact() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/repos/owner/repo/actions/artifacts/123"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .actions()
        .delete_artifact("owner", "repo", 123u64.into())
        .await;

    assert!(result.is_ok());
}
