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
async fn test_runner_downloads() {
    let mock_server = MockServer::start().await;
    let downloads_json = json!([
        {
            "os": "osx",
            "architecture": "arm64",
            "download_url": "https://github.com/actions/runner/releases/download/v2.300.0/actions-runner-osx-arm64-2.300.0.tar.gz",
            "filename": "actions-runner-osx-arm64-2.300.0.tar.gz",
            "sha256_checksum": "abcdef123456"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/orgs/my-org/actions/runners/downloads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&downloads_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/actions/runners/downloads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&downloads_json))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let org_downloads = client
        .actions()
        .list_org_runner_downloads("my-org")
        .await
        .unwrap();
    assert_eq!(org_downloads.len(), 1);
    assert_eq!(org_downloads[0].os, "osx");

    let repo_downloads = client
        .actions()
        .list_repo_runner_downloads("owner", "repo")
        .await
        .unwrap();
    assert_eq!(repo_downloads.len(), 1);
    assert_eq!(repo_downloads[0].architecture, "arm64");
}

#[tokio::test]
async fn test_org_runner_labels() {
    let mock_server = MockServer::start().await;
    let labels_json = json!({
        "total_count": 1,
        "labels": [
            {
                "id": 1,
                "name": "custom-gpu",
                "type": "custom"
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/orgs/my-org/actions/runners/10/labels"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&labels_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/orgs/my-org/actions/runners/10/labels"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&labels_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/orgs/my-org/actions/runners/10/labels"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&labels_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/orgs/my-org/actions/runners/10/labels"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&labels_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/orgs/my-org/actions/runners/10/labels/custom-gpu"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&labels_json))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let labels = client
        .actions()
        .list_org_runner_labels("my-org", 10u64.into())
        .await
        .unwrap();
    assert_eq!(labels.total_count, 1);

    let added = client
        .actions()
        .add_org_runner_labels("my-org", 10u64.into(), &["custom-gpu"])
        .await
        .unwrap();
    assert_eq!(added.labels[0].name, "custom-gpu");

    let set = client
        .actions()
        .set_org_runner_labels("my-org", 10u64.into(), &["custom-gpu"])
        .await
        .unwrap();
    assert_eq!(set.total_count, 1);

    let removed = client
        .actions()
        .remove_org_runner_label("my-org", 10u64.into(), "custom-gpu")
        .await
        .unwrap();
    assert_eq!(removed.total_count, 1);

    let removed_all = client
        .actions()
        .remove_all_org_runner_labels("my-org", 10u64.into())
        .await
        .unwrap();
    assert_eq!(removed_all.total_count, 1);
}

#[tokio::test]
async fn test_repo_runner_labels() {
    let mock_server = MockServer::start().await;
    let labels_json = json!({
        "total_count": 1,
        "labels": [
            {
                "id": 2,
                "name": "fast-build",
                "type": "custom"
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/actions/runners/20/labels"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&labels_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/actions/runners/20/labels"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&labels_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/repos/owner/repo/actions/runners/20/labels"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&labels_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/repos/owner/repo/actions/runners/20/labels"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&labels_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path(
            "/repos/owner/repo/actions/runners/20/labels/fast-build",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(&labels_json))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let list = client
        .actions()
        .list_repo_runner_labels("owner", "repo", 20u64.into())
        .await
        .unwrap();
    assert_eq!(list.total_count, 1);

    let add = client
        .actions()
        .add_repo_runner_labels("owner", "repo", 20u64.into(), &["fast-build"])
        .await
        .unwrap();
    assert_eq!(add.labels[0].name, "fast-build");

    let set = client
        .actions()
        .set_repo_runner_labels("owner", "repo", 20u64.into(), &["fast-build"])
        .await
        .unwrap();
    assert_eq!(set.total_count, 1);

    let remove_one = client
        .actions()
        .remove_repo_runner_label("owner", "repo", 20u64.into(), "fast-build")
        .await
        .unwrap();
    assert_eq!(remove_one.total_count, 1);

    let remove_all = client
        .actions()
        .remove_all_repo_runner_labels("owner", "repo", 20u64.into())
        .await
        .unwrap();
    assert_eq!(remove_all.total_count, 1);
}
