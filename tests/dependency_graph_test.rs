mod mock_error;

use std::collections::HashMap;

use chrono::Utc;
use mock_error::setup_error_handler;
use octocrab::models::dependency_graph::{
    ChangeType, DependencyRelationship, DependencyScope, Snapshot, SnapshotDetector, SnapshotJob,
    SnapshotManifest, SnapshotManifestFile, SnapshotResolvedPackage,
};
use octocrab::Octocrab;
use serde_json::json;
use url::Url;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

fn sample_compare_response() -> serde_json::Value {
    json!([
        {
            "change_type": "added",
            "manifest": "package.json",
            "ecosystem": "npm",
            "name": "helmet",
            "version": "5.0.0",
            "package_url": "pkg:npm/helmet@5.0.0",
            "license": "MIT",
            "source_repository_url": "https://github.com/helmetjs/helmet",
            "vulnerabilities": [
                {
                    "severity": "critical",
                    "advisory_ghsa_id": "GHSA-fqfj-cmh6-hj49",
                    "advisory_summary": "Ruby OpenID",
                    "advisory_url": "https://github.com/advisories/GHSA-fqfj-cmh6-hj49"
                }
            ],
            "scope": "runtime"
        },
        {
            "change_type": "removed",
            "manifest": "package.json",
            "ecosystem": "npm",
            "name": "old-lib",
            "version": "1.0.0",
            "package_url": null,
            "license": null,
            "source_repository_url": null,
            "vulnerabilities": [],
            "scope": "development"
        }
    ])
}

fn sample_sbom_response() -> serde_json::Value {
    json!({
        "sbom": {
            "SPDXID": "SPDXRef-DOCUMENT",
            "spdxVersion": "SPDX-2.3",
            "creationInfo": {
                "created": "2021-09-01T00:00:00Z",
                "creators": ["Tool: GitHub.com-Dependency-Graph"]
            },
            "name": "owner/repo",
            "dataLicense": "CC0-1.0",
            "documentNamespace": "https://spdx.org/spdxdocs/protobom/15e41dd2-f961-4f4d-b8dc-f8f57ad70d57",
            "packages": [
                {
                    "name": "rails",
                    "SPDXID": "SPDXRef-Package",
                    "versionInfo": "1.0.0",
                    "downloadLocation": "NOASSERTION",
                    "filesAnalyzed": false,
                    "licenseConcluded": "MIT",
                    "licenseDeclared": "MIT",
                    "copyrightText": "Copyright (c) 1985 GitHub.com",
                    "externalRefs": [
                        {
                            "referenceCategory": "PACKAGE-MANAGER",
                            "referenceType": "purl",
                            "referenceLocator": "pkg:gem/rails@1.0.0"
                        }
                    ]
                }
            ],
            "relationships": [
                {
                    "relationshipType": "DEPENDS_ON",
                    "spdxElementId": "SPDXRef-Repository",
                    "relatedSpdxElement": "SPDXRef-Package"
                }
            ]
        }
    })
}

#[tokio::test]
async fn should_compare_dependencies() {
    let mock_server = MockServer::start().await;
    let expected_path = "/repos/owner/repo/dependency-graph/compare/base...head";

    Mock::given(method("GET"))
        .and(path(expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_compare_response()))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "compare failed").await;

    let octocrab = setup_octocrab(&mock_server.uri());
    let diff = octocrab
        .repos("owner", "repo")
        .dependency_graph()
        .compare("base...head")
        .send()
        .await
        .unwrap();

    assert_eq!(diff.len(), 2);
    assert_eq!(diff[0].change_type, ChangeType::Added);
    assert_eq!(diff[0].manifest, "package.json");
    assert_eq!(diff[0].ecosystem, "npm");
    assert_eq!(diff[0].name, "helmet");
    assert_eq!(diff[0].version, "5.0.0");
    assert_eq!(
        diff[0].package_url,
        Some("pkg:npm/helmet@5.0.0".to_string())
    );
    assert_eq!(diff[0].license, Some("MIT".to_string()));
    assert_eq!(
        diff[0].source_repository_url,
        Some(Url::parse("https://github.com/helmetjs/helmet").unwrap())
    );
    assert_eq!(diff[0].scope, DependencyScope::Runtime);
    assert_eq!(diff[0].vulnerabilities.len(), 1);
    assert_eq!(diff[0].vulnerabilities[0].severity, "critical");
    assert_eq!(
        diff[0].vulnerabilities[0].advisory_ghsa_id,
        "GHSA-fqfj-cmh6-hj49"
    );

    assert_eq!(diff[1].change_type, ChangeType::Removed);
    assert_eq!(diff[1].scope, DependencyScope::Development);
    assert_eq!(diff[1].package_url, None);
    assert_eq!(diff[1].license, None);
}

#[tokio::test]
async fn should_compare_dependencies_with_manifest_filter() {
    let mock_server = MockServer::start().await;
    let expected_path = "/repos/owner/repo/dependency-graph/compare/base...head";

    Mock::given(method("GET"))
        .and(path(expected_path))
        .and(query_param("name", "package-lock.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_compare_response()))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "compare failed").await;

    let octocrab = setup_octocrab(&mock_server.uri());
    // Also tests top-level octocrab.dependency_graph(owner, repo) shortcut
    let diff = octocrab
        .dependency_graph("owner", "repo")
        .compare("base...head")
        .name("package-lock.json")
        .send()
        .await
        .unwrap();

    assert_eq!(diff.len(), 2);
}

#[tokio::test]
async fn should_create_dependency_snapshot() {
    let mock_server = MockServer::start().await;
    let expected_path = "/repos/owner/repo/dependency-graph/snapshots";

    Mock::given(method("POST"))
        .and(path(expected_path))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": 12345,
            "created_at": "2026-06-14T20:25:00Z",
            "result": "SUCCESS",
            "message": "Snapshot successfully created."
        })))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "create snapshot failed").await;

    let mut manifests = HashMap::new();
    let mut resolved = HashMap::new();
    resolved.insert(
        "@actions/core".to_string(),
        SnapshotResolvedPackage {
            package_url: Some("pkg:/npm/%40actions/core@1.1.9".to_string()),
            metadata: None,
            relationship: Some(DependencyRelationship::Direct),
            scope: Some(DependencyScope::Runtime),
            dependencies: Some(vec!["@actions/http-client".to_string()]),
        },
    );

    manifests.insert(
        "package-lock.json".to_string(),
        SnapshotManifest {
            name: "package-lock.json".to_string(),
            file: Some(SnapshotManifestFile {
                source_location: Some("src/package-lock.json".to_string()),
            }),
            metadata: None,
            resolved: Some(resolved),
        },
    );

    let snapshot = Snapshot {
        version: 0,
        job: SnapshotJob {
            id: "job-1".to_string(),
            correlator: "workflow_name_job_name".to_string(),
            html_url: Some(Url::parse("https://github.com/owner/repo/actions/runs/1").unwrap()),
        },
        sha: "ce587453ced02b1526dfb4cb910479d431683101".to_string(),
        ref_field: "refs/heads/main".to_string(),
        detector: SnapshotDetector {
            name: "octo-detector".to_string(),
            version: "0.0.1".to_string(),
            url: Url::parse("https://github.com/octo-org/octo-repo").unwrap(),
        },
        metadata: None,
        manifests: Some(manifests),
        scanned: Utc::now(),
    };

    let octocrab = setup_octocrab(&mock_server.uri());
    let resp = octocrab
        .repos("owner", "repo")
        .dependency_graph()
        .create_snapshot(&snapshot)
        .await
        .unwrap();

    assert_eq!(resp.id, 12345);
    assert_eq!(resp.created_at, "2026-06-14T20:25:00Z");
    assert_eq!(resp.result, "SUCCESS");
    assert_eq!(resp.message, "Snapshot successfully created.");
}

#[tokio::test]
#[allow(deprecated)]
async fn should_export_sbom_via_dependency_graph() {
    let mock_server = MockServer::start().await;
    let expected_path = "/repos/owner/repo/dependency-graph/sbom";

    Mock::given(method("GET"))
        .and(path(expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_sbom_response()))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "export sbom failed").await;

    let octocrab = setup_octocrab(&mock_server.uri());
    let sbom_export = octocrab
        .repos("owner", "repo")
        .dependency_graph()
        .export_sbom()
        .await
        .unwrap();

    assert_eq!(sbom_export.sbom.spdxid, "SPDXRef-DOCUMENT");
    assert_eq!(sbom_export.sbom.spdx_version, "SPDX-2.3");
    assert_eq!(sbom_export.sbom.name, "owner/repo");
    assert_eq!(sbom_export.sbom.packages.len(), 1);
    assert_eq!(sbom_export.sbom.packages[0].name, "rails");
}

#[tokio::test]
#[allow(deprecated)]
async fn should_export_sbom_via_sbom_handler() {
    let mock_server = MockServer::start().await;
    let expected_path = "/repos/owner/repo/dependency-graph/sbom";

    Mock::given(method("GET"))
        .and(path(expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_sbom_response()))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "export sbom failed").await;

    let octocrab = setup_octocrab(&mock_server.uri());
    let sbom_export = octocrab
        .repos("owner", "repo")
        .sbom()
        .export()
        .await
        .unwrap();

    assert_eq!(sbom_export.sbom.spdxid, "SPDXRef-DOCUMENT");
    assert_eq!(sbom_export.sbom.name, "owner/repo");

    // Test get() alias
    let sbom_get = octocrab.repos("owner", "repo").sbom().get().await.unwrap();
    assert_eq!(sbom_get.sbom.spdxid, "SPDXRef-DOCUMENT");
}

#[tokio::test]
async fn should_handle_errors_on_compare() {
    let mock_server = MockServer::start().await;
    let expected_path = "/repos/owner/repo/dependency-graph/compare/base...head";

    Mock::given(method("GET"))
        .and(path(expected_path))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "message": "Not Found",
            "documentation_url": "https://docs.github.com/rest"
        })))
        .mount(&mock_server)
        .await;

    let octocrab = setup_octocrab(&mock_server.uri());
    let err = octocrab
        .repos("owner", "repo")
        .dependency_graph()
        .compare("base...head")
        .send()
        .await
        .unwrap_err();

    match err {
        octocrab::Error::GitHub { source, .. } => {
            assert_eq!(source.status_code, http::StatusCode::NOT_FOUND);
            assert_eq!(source.message, "Not Found");
        }
        other => panic!("Expected GitHub error, got {:?}", other),
    }
}
