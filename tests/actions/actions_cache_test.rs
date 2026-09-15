use octocrab::Octocrab;
use serde_json::json;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_repo_cache_usage() {
    let mock_server = MockServer::start().await;
    let usage_json = json!({
        "full_name": "owner/repo",
        "active_caches_size_in_bytes": 1048576,
        "active_caches_count": 5
    });

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/actions/cache/usage"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&usage_json))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let usage = client
        .actions()
        .get_repo_cache_usage("owner", "repo")
        .await
        .unwrap();

    assert_eq!(usage.full_name, "owner/repo");
    assert_eq!(usage.active_caches_size_in_bytes, 1048576);
    assert_eq!(usage.active_caches_count, 5);
}

#[tokio::test]
async fn test_list_repo_caches() {
    let mock_server = MockServer::start().await;
    let caches_json = json!({
        "total_count": 1,
        "actions_caches": [
            {
                "id": 1,
                "ref": "refs/heads/main",
                "key": "Linux-node-v16",
                "version": "abc123",
                "last_accessed_at": "2023-01-01T00:00:00Z",
                "created_at": "2023-01-01T00:00:00Z",
                "size_in_bytes": 2048
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/actions/caches"))
        .and(query_param("key", "Linux-node"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&caches_json))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let list = client
        .actions()
        .list_repo_caches("owner", "repo")
        .key("Linux-node")
        .send()
        .await
        .unwrap();

    assert_eq!(list.total_count, 1);
    assert_eq!(list.actions_caches[0].key, "Linux-node-v16");
}

#[tokio::test]
async fn test_delete_repo_caches_by_key() {
    let mock_server = MockServer::start().await;
    let caches_json = json!({
        "total_count": 1,
        "actions_caches": [
            {
                "id": 1,
                "ref": "refs/heads/main",
                "key": "Linux-node-v16",
                "version": "abc123",
                "last_accessed_at": "2023-01-01T00:00:00Z",
                "created_at": "2023-01-01T00:00:00Z",
                "size_in_bytes": 2048
            }
        ]
    });

    Mock::given(method("DELETE"))
        .and(path("/repos/owner/repo/actions/caches"))
        .and(query_param("key", "Linux-node-v16"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&caches_json))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let deleted = client
        .actions()
        .delete_repo_caches_by_key("owner", "repo", "Linux-node-v16", None::<&str>)
        .await
        .unwrap();

    assert_eq!(deleted.total_count, 1);
}

#[tokio::test]
async fn test_delete_repo_cache_by_id() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/repos/owner/repo/actions/caches/42"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let res = client
        .actions()
        .delete_repo_cache("owner", "repo", 42)
        .await;

    assert!(res.is_ok());
}

#[tokio::test]
async fn test_org_cache_usage() {
    let mock_server = MockServer::start().await;
    let org_usage_json = json!({
        "total_active_caches_size_in_bytes": 5242880,
        "total_active_caches_count": 10
    });

    Mock::given(method("GET"))
        .and(path("/orgs/my-org/actions/cache/usage"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&org_usage_json))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let usage = client
        .actions()
        .get_org_cache_usage("my-org")
        .await
        .unwrap();

    assert_eq!(usage.total_active_caches_size_in_bytes, 5242880);
    assert_eq!(usage.total_active_caches_count, 10);
}

#[tokio::test]
async fn test_org_cache_usage_by_repo() {
    let mock_server = MockServer::start().await;
    let list_json = json!({
        "total_count": 1,
        "repository_cache_usages": [
            {
                "full_name": "my-org/repo1",
                "active_caches_size_in_bytes": 1048576,
                "active_caches_count": 2
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/orgs/my-org/actions/cache/usage-by-repository"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list_json))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let res = client
        .actions()
        .list_org_cache_usage_by_repo("my-org")
        .send()
        .await
        .unwrap();

    assert_eq!(res.total_count, 1);
    assert_eq!(res.repository_cache_usages[0].full_name, "my-org/repo1");
}
