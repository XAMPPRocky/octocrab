use octocrab::Octocrab;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_issue_artifacts() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    let artifact_json = json!({
        "id": 1,
        "number": 1,
        "type": "summary",
        "name": "Investigation Summary",
        "body": "Root cause analysis details"
    });

    // 1. List issue artifacts
    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/issues/10/artifacts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([artifact_json])))
        .mount(&mock_server)
        .await;

    let artifacts = octocrab
        .issues("owner", "repo")
        .artifacts(10)
        .list()
        .send()
        .await
        .unwrap();
    assert_eq!(artifacts.items.len(), 1);
    assert_eq!(artifacts.items[0].name, "Investigation Summary");

    // 2. Create issue artifact
    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/issues/10/artifacts"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&artifact_json))
        .mount(&mock_server)
        .await;

    let created = octocrab
        .issues("owner", "repo")
        .artifacts(10)
        .create("Investigation Summary", "Root cause analysis details")
        .artifact_type("summary")
        .send()
        .await
        .unwrap();
    assert_eq!(created.number, 1);
    assert_eq!(created.name, "Investigation Summary");

    // 3. Get issue artifact
    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/issues/10/artifacts/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&artifact_json))
        .mount(&mock_server)
        .await;

    let fetched = octocrab
        .issues("owner", "repo")
        .artifacts(10)
        .get(1)
        .await
        .unwrap();
    assert_eq!(fetched.artifact().name, "Investigation Summary");

    // 4. Update issue artifact
    let mut updated_json = artifact_json.clone();
    updated_json["body"] = json!("Updated details");
    Mock::given(method("PATCH"))
        .and(path("/repos/owner/repo/issues/10/artifacts/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&updated_json))
        .mount(&mock_server)
        .await;

    let updated = octocrab
        .issues("owner", "repo")
        .artifacts(10)
        .update(1)
        .body("Updated details")
        .send()
        .await
        .unwrap();
    assert_eq!(updated.body, "Updated details");

    // 5. Delete issue artifact
    Mock::given(method("DELETE"))
        .and(path("/repos/owner/repo/issues/10/artifacts/1"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .issues("owner", "repo")
        .artifacts(10)
        .delete(1)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_issue_artifacts_with_type_filter() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    let artifact_json = json!({
        "id": 2,
        "number": 2,
        "type": "plan",
        "name": "Sprint Plan",
        "body": "Sprint goals"
    });

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/issues/10/artifacts"))
        .and(wiremock::matchers::query_param("type", "plan"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([artifact_json])))
        .mount(&mock_server)
        .await;

    let artifacts = octocrab
        .issues("owner", "repo")
        .artifacts(10)
        .list()
        .artifact_type("plan")
        .send()
        .await
        .unwrap();
    assert_eq!(artifacts.items.len(), 1);
    assert_eq!(artifacts.items[0].artifact_type, "plan");
    assert_eq!(artifacts.items[0].name, "Sprint Plan");
}

#[tokio::test]
async fn test_issue_artifacts_detailed_with_versions() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    let detailed_json = json!({
        "artifact": {
            "id": 1,
            "number": 1,
            "type": "generic",
            "name": "Design Spec",
            "body": "Latest version"
        },
        "versions": [
            {
                "version": 2,
                "name": "Design Spec",
                "body": "Latest version",
                "actor": {
                    "login": "octocat",
                    "id": 1
                }
            },
            {
                "version": 1,
                "name": "Design Spec",
                "body": "Initial draft",
                "actor": {
                    "login": "octocat",
                    "id": 1
                }
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/issues/10/artifacts/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&detailed_json))
        .mount(&mock_server)
        .await;

    let response = octocrab
        .issues("owner", "repo")
        .artifacts(10)
        .get(1)
        .await
        .unwrap();

    match response {
        octocrab::models::issues::IssueArtifactResponse::Detailed { artifact, versions } => {
            assert_eq!(artifact.name, "Design Spec");
            assert_eq!(versions.len(), 2);
            assert_eq!(versions[0].version, 2);
            assert_eq!(versions[1].version, 1);
        }
        octocrab::models::issues::IssueArtifactResponse::Simple(_) => {
            panic!("Expected Detailed artifact response");
        }
    }
}
