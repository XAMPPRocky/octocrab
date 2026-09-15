use crate::mock_error::setup_error_handler;
use octocrab::models::dependabot::{
    DependabotAlert, DependabotAlertDismissedReason, DependabotAlertState,
    DependabotAlertUpdateState, DependabotDefaultRepositoryAccessLevel, DependabotRepositoryAccess,
    SelectedRepositories,
};
use octocrab::models::orgs::secrets::{
    CreateOrganizationSecret, CreateOrganizationSecretResponse, Visibility,
};
use octocrab::models::repos::secrets::{CreateRepositorySecret, CreateRepositorySecretResponse};
use octocrab::models::RepositoryId;
use octocrab::params::Direction;
use octocrab::Octocrab;
use serde_json::json;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

fn sample_user() -> serde_json::Value {
    json!({
        "login": "octocat",
        "id": 1,
        "node_id": "MDQ6VXNlcjE=",
        "avatar_url": "https://github.com/images/error/octocat_happy.gif",
        "gravatar_id": "",
        "url": "https://api.github.com/users/octocat",
        "html_url": "https://github.com/octocat",
        "followers_url": "https://api.github.com/users/octocat/followers",
        "following_url": "https://api.github.com/users/octocat/following{/other_user}",
        "gists_url": "https://api.github.com/users/octocat/gists{/gist_id}",
        "starred_url": "https://api.github.com/users/octocat/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/octocat/subscriptions",
        "organizations_url": "https://api.github.com/users/octocat/orgs",
        "repos_url": "https://api.github.com/users/octocat/repos",
        "events_url": "https://api.github.com/users/octocat/events{/privacy}",
        "received_events_url": "https://api.github.com/users/octocat/received_events",
        "type": "User",
        "site_admin": false
    })
}

fn sample_repo(id: u64, name: &str) -> serde_json::Value {
    json!({
        "id": id,
        "node_id": "MDEwOlJlcG9zaXRvcnkxMjk2MjY5",
        "name": name,
        "full_name": format!("octocat/{name}"),
        "owner": sample_user(),
        "private": false,
        "html_url": format!("https://github.com/octocat/{name}"),
        "description": "A sample repo",
        "fork": false,
        "url": format!("https://api.github.com/repos/octocat/{name}")
    })
}

fn sample_dependabot_alert(number: u64) -> serde_json::Value {
    json!({
        "number": number,
        "state": "open",
        "dependency": {
            "package": {
                "ecosystem": "npm",
                "name": "lodash"
            },
            "manifest_path": "package.json",
            "scope": "runtime",
            "relationship": "direct"
        },
        "security_advisory": {
            "ghsa_id": "GHSA-xxxx-yyyy-zzzz",
            "cve_id": "CVE-2021-12345",
            "summary": "Prototype Pollution in lodash",
            "description": "Vulnerable to prototype pollution",
            "vulnerabilities": [
                {
                    "package": {
                        "ecosystem": "npm",
                        "name": "lodash"
                    },
                    "severity": "high",
                    "vulnerable_version_range": "< 4.17.21",
                    "first_patched_version": {
                        "identifier": "4.17.21"
                    }
                }
            ],
            "severity": "high",
            "cvss": {
                "vector_string": "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:N/I:N/A:H",
                "score": 7.5
            },
            "cvss_severities": {
                "cvss_v3": {
                    "vector_string": "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:N/I:N/A:H",
                    "score": 7.5
                },
                "cvss_v4": null
            },
            "epss": {
                "percentage": 0.05,
                "percentile": 0.85
            },
            "cwes": [
                {
                    "cwe_id": "CWE-1321",
                    "name": "Improperly Controlled Modification of Object Prototype Attributes ('Prototype Pollution')"
                }
            ],
            "identifiers": [
                {
                    "type": "GHSA",
                    "value": "GHSA-xxxx-yyyy-zzzz"
                }
            ],
            "references": [
                {
                    "url": "https://nvd.nist.gov/vuln/detail/CVE-2021-12345"
                }
            ],
            "published_at": "2021-05-01T00:00:00Z",
            "updated_at": "2021-05-02T00:00:00Z",
            "withdrawn_at": null
        },
        "security_vulnerability": {
            "package": {
                "ecosystem": "npm",
                "name": "lodash"
            },
            "severity": "high",
            "vulnerable_version_range": "< 4.17.21",
            "first_patched_version": {
                "identifier": "4.17.21"
            }
        },
        "url": format!("https://api.github.com/repos/octocat/hello-world/dependabot/alerts/{number}"),
        "html_url": format!("https://github.com/octocat/hello-world/security/dependabot/{number}"),
        "created_at": "2021-05-01T00:00:00Z",
        "updated_at": "2021-05-02T00:00:00Z",
        "dismissed_at": null,
        "dismissed_by": null,
        "dismissed_reason": null,
        "dismissed_comment": null,
        "fixed_at": null,
        "auto_dismissed_at": null,
        "assignees": [],
        "repository": sample_repo(12345, "hello-world")
    })
}

fn sample_secret(name: &str) -> serde_json::Value {
    json!({
        "name": name,
        "created_at": "2021-01-01T00:00:00Z",
        "updated_at": "2021-01-02T00:00:00Z",
        "visibility": "all"
    })
}

fn sample_public_key() -> serde_json::Value {
    json!({
        "key_id": "key-12345",
        "key": "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE="
    })
}

// 1. List repository dependabot secrets
#[tokio::test]
async fn should_list_repository_dependabot_secrets() {
    let mock_server = MockServer::start().await;
    let expected_path = "/repos/octocat/hello-world/dependabot/secrets";

    Mock::given(method("GET"))
        .and(path(expected_path))
        .and(query_param("per_page", "10"))
        .and(query_param("page", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "secrets": [sample_secret("SECRET_TOKEN")]
        })))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, expected_path).await;

    let client = setup_octocrab(&mock_server.uri());
    let page = client
        .repos("octocat", "hello-world")
        .dependabot()
        .secrets()
        .list()
        .per_page(10u8)
        .page(1u32)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].name, "SECRET_TOKEN");
}

// 2. Get repository dependabot public key
#[tokio::test]
async fn should_get_repository_dependabot_public_key() {
    let mock_server = MockServer::start().await;
    let expected_path = "/repos/octocat/hello-world/dependabot/secrets/public-key";

    Mock::given(method("GET"))
        .and(path(expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_public_key()))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, expected_path).await;

    let client = setup_octocrab(&mock_server.uri());
    let key = client
        .repos("octocat", "hello-world")
        .dependabot()
        .secrets()
        .get_public_key()
        .await
        .unwrap();

    assert_eq!(key.key_id, "key-12345");
}

// 3. Get repository dependabot secret
#[tokio::test]
async fn should_get_repository_dependabot_secret() {
    let mock_server = MockServer::start().await;
    let expected_path = "/repos/octocat/hello-world/dependabot/secrets/MY_SECRET";

    Mock::given(method("GET"))
        .and(path(expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_secret("MY_SECRET")))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, expected_path).await;

    let client = setup_octocrab(&mock_server.uri());
    let secret = client
        .repos("octocat", "hello-world")
        .dependabot()
        .secrets()
        .get("MY_SECRET")
        .await
        .unwrap();

    assert_eq!(secret.name, "MY_SECRET");
}

// 4. Create or update repository dependabot secret (201 Created and 204 Updated)
#[tokio::test]
async fn should_create_or_update_repository_dependabot_secret() {
    let mock_server = MockServer::start().await;
    let expected_path = "/repos/octocat/hello-world/dependabot/secrets/MY_SECRET";

    Mock::given(method("PUT"))
        .and(path(expected_path))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, expected_path).await;

    let client = setup_octocrab(&mock_server.uri());
    let secret_body = CreateRepositorySecret {
        encrypted_value: "c2VjcmV0",
        key_id: "key-12345",
    };

    let res = client
        .repos("octocat", "hello-world")
        .dependabot()
        .secrets()
        .create_or_update("MY_SECRET", &secret_body)
        .await
        .unwrap();

    assert_eq!(res, CreateRepositorySecretResponse::Created);
}

// 5. Delete repository dependabot secret
#[tokio::test]
async fn should_delete_repository_dependabot_secret() {
    let mock_server = MockServer::start().await;
    let expected_path = "/repos/octocat/hello-world/dependabot/secrets/MY_SECRET";

    Mock::given(method("DELETE"))
        .and(path(expected_path))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, expected_path).await;

    let client = setup_octocrab(&mock_server.uri());
    client
        .repos("octocat", "hello-world")
        .dependabot()
        .secrets()
        .delete("MY_SECRET")
        .await
        .unwrap();
}

// 6. List repository dependabot alerts
#[tokio::test]
async fn should_list_repository_dependabot_alerts() {
    let mock_server = MockServer::start().await;
    let expected_path = "/repos/octocat/hello-world/dependabot/alerts";

    Mock::given(method("GET"))
        .and(path(expected_path))
        .and(query_param("state", "open"))
        .and(query_param("severity", "high"))
        .and(query_param("ecosystem", "npm"))
        .and(query_param("package", "lodash"))
        .and(query_param("scope", "runtime"))
        .and(query_param("sort", "created"))
        .and(query_param("direction", "desc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([sample_dependabot_alert(1)])))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, expected_path).await;

    let client = setup_octocrab(&mock_server.uri());
    let page: octocrab::Page<DependabotAlert> = client
        .repos("octocat", "hello-world")
        .dependabot()
        .alerts()
        .list()
        .state("open")
        .severity("high")
        .ecosystem("npm")
        .package("lodash")
        .scope("runtime")
        .sort("created")
        .direction(Direction::Descending)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].number, 1);
    assert_eq!(page.items[0].state, DependabotAlertState::Open);
}

// 7. Get repository dependabot alert
#[tokio::test]
async fn should_get_repository_dependabot_alert() {
    let mock_server = MockServer::start().await;
    let expected_path = "/repos/octocat/hello-world/dependabot/alerts/42";

    Mock::given(method("GET"))
        .and(path(expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_dependabot_alert(42)))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, expected_path).await;

    let client = setup_octocrab(&mock_server.uri());
    let alert = client
        .repos("octocat", "hello-world")
        .dependabot()
        .alerts()
        .get(42)
        .await
        .unwrap();

    assert_eq!(alert.number, 42);
}

// 8. Update repository dependabot alert
#[tokio::test]
async fn should_update_repository_dependabot_alert() {
    let mock_server = MockServer::start().await;
    let expected_path = "/repos/octocat/hello-world/dependabot/alerts/42";

    let mut updated_alert = sample_dependabot_alert(42);
    updated_alert["state"] = json!("dismissed");
    updated_alert["dismissed_reason"] = json!("no_bandwidth");
    updated_alert["dismissed_comment"] = json!("Postponed");

    Mock::given(method("PATCH"))
        .and(path(expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(updated_alert))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, expected_path).await;

    let client = setup_octocrab(&mock_server.uri());
    let alert = client
        .repos("octocat", "hello-world")
        .dependabot()
        .alerts()
        .update(42)
        .state(DependabotAlertUpdateState::Dismissed)
        .dismissed_reason(DependabotAlertDismissedReason::NoBandwidth)
        .dismissed_comment("Postponed")
        .send()
        .await
        .unwrap();

    assert_eq!(alert.state, DependabotAlertState::Dismissed);
    assert_eq!(
        alert.dismissed_reason,
        Some(DependabotAlertDismissedReason::NoBandwidth)
    );
}

// 9. List org dependabot secrets
#[tokio::test]
async fn should_list_org_dependabot_secrets() {
    let mock_server = MockServer::start().await;
    let expected_path = "/orgs/my-org/dependabot/secrets";

    Mock::given(method("GET"))
        .and(path(expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "secrets": [sample_secret("ORG_SECRET")]
        })))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, expected_path).await;

    let client = setup_octocrab(&mock_server.uri());
    let page = client
        .orgs("my-org")
        .dependabot()
        .secrets()
        .list()
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].name, "ORG_SECRET");
}

// 10. Get org dependabot public key
#[tokio::test]
async fn should_get_org_dependabot_public_key() {
    let mock_server = MockServer::start().await;
    let expected_path = "/orgs/my-org/dependabot/secrets/public-key";

    Mock::given(method("GET"))
        .and(path(expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_public_key()))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, expected_path).await;

    let client = setup_octocrab(&mock_server.uri());
    let key = client
        .orgs("my-org")
        .dependabot()
        .secrets()
        .get_public_key()
        .await
        .unwrap();

    assert_eq!(key.key_id, "key-12345");
}

// 11. Get org dependabot secret
#[tokio::test]
async fn should_get_org_dependabot_secret() {
    let mock_server = MockServer::start().await;
    let expected_path = "/orgs/my-org/dependabot/secrets/ORG_SECRET";

    Mock::given(method("GET"))
        .and(path(expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_secret("ORG_SECRET")))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, expected_path).await;

    let client = setup_octocrab(&mock_server.uri());
    let secret = client
        .orgs("my-org")
        .dependabot()
        .secrets()
        .get("ORG_SECRET")
        .await
        .unwrap();

    assert_eq!(secret.name, "ORG_SECRET");
}

// 12. Create or update org dependabot secret
#[tokio::test]
async fn should_create_or_update_org_dependabot_secret() {
    let mock_server = MockServer::start().await;
    let expected_path = "/orgs/my-org/dependabot/secrets/ORG_SECRET";

    Mock::given(method("PUT"))
        .and(path(expected_path))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, expected_path).await;

    let client = setup_octocrab(&mock_server.uri());
    let body = CreateOrganizationSecret {
        encrypted_value: "c2VjcmV0",
        key_id: "key-12345",
        visibility: Visibility::All,
        selected_repository_ids: None,
    };

    let res = client
        .orgs("my-org")
        .dependabot()
        .secrets()
        .create_or_update("ORG_SECRET", &body)
        .await
        .unwrap();

    assert_eq!(res, CreateOrganizationSecretResponse::Created);
}

// 13. Delete org dependabot secret
#[tokio::test]
async fn should_delete_org_dependabot_secret() {
    let mock_server = MockServer::start().await;
    let expected_path = "/orgs/my-org/dependabot/secrets/ORG_SECRET";

    Mock::given(method("DELETE"))
        .and(path(expected_path))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, expected_path).await;

    let client = setup_octocrab(&mock_server.uri());
    client
        .orgs("my-org")
        .dependabot()
        .secrets()
        .delete("ORG_SECRET")
        .await
        .unwrap();
}

// 14. Org dependabot secret selected repositories (list, set, add, remove)
#[tokio::test]
async fn should_manage_org_dependabot_secret_repositories() {
    let mock_server = MockServer::start().await;
    let base_path = "/orgs/my-org/dependabot/secrets/MY_SECRET/repositories";

    // List
    Mock::given(method("GET"))
        .and(path(base_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "repositories": [sample_repo(123, "my-repo")]
        })))
        .mount(&mock_server)
        .await;

    // Set
    Mock::given(method("PUT"))
        .and(path(base_path))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    // Add
    Mock::given(method("PUT"))
        .and(path(format!("{base_path}/456")))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    // Remove
    Mock::given(method("DELETE"))
        .and(path(format!("{base_path}/456")))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let repos_handler = client
        .orgs("my-org")
        .dependabot()
        .secrets()
        .repositories("MY_SECRET");

    // List
    let selected: SelectedRepositories = repos_handler.list().await.unwrap();
    assert_eq!(selected.total_count, 1);
    assert_eq!(selected.repositories[0].name, "my-repo");

    // Set
    repos_handler
        .set(&[RepositoryId(123), RepositoryId(456)])
        .await
        .unwrap();

    // Add
    repos_handler.add(RepositoryId(456)).await.unwrap();

    // Remove
    repos_handler.remove(RepositoryId(456)).await.unwrap();
}

// 15. List org dependabot alerts
#[tokio::test]
async fn should_list_org_dependabot_alerts() {
    let mock_server = MockServer::start().await;
    let expected_path = "/orgs/my-org/dependabot/alerts";

    Mock::given(method("GET"))
        .and(path(expected_path))
        .and(query_param("state", "open"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!([sample_dependabot_alert(10)])),
        )
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, expected_path).await;

    let client = setup_octocrab(&mock_server.uri());
    let page = client
        .orgs("my-org")
        .dependabot()
        .alerts()
        .list()
        .state("open")
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].number, 10);
}

// 16. Manage org dependabot repository access
#[tokio::test]
async fn should_manage_org_dependabot_repository_access() {
    let mock_server = MockServer::start().await;
    let base_path = "/orgs/my-org/dependabot/repository-access";

    // List
    Mock::given(method("GET"))
        .and(path(base_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "default_level": "public",
            "accessible_repositories": [sample_repo(101, "accessible-repo")]
        })))
        .mount(&mock_server)
        .await;

    // Update
    Mock::given(method("PATCH"))
        .and(path(base_path))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    // Set default level
    Mock::given(method("PUT"))
        .and(path(format!("{base_path}/default-level")))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let access_handler = client.orgs("my-org").dependabot().repository_access();

    // List
    let access: DependabotRepositoryAccess = access_handler.list().send().await.unwrap();
    assert_eq!(
        access.default_level,
        DependabotDefaultRepositoryAccessLevel::Public
    );
    assert_eq!(access.accessible_repositories.len(), 1);
    assert_eq!(access.accessible_repositories[0].name, "accessible-repo");

    // Update
    access_handler
        .update()
        .add_repository(RepositoryId(202))
        .remove_repository(RepositoryId(303))
        .send()
        .await
        .unwrap();

    // Set default level
    access_handler
        .set_default_level(DependabotDefaultRepositoryAccessLevel::Internal)
        .await
        .unwrap();
}

// 17. List enterprise dependabot alerts
#[tokio::test]
async fn should_list_enterprise_dependabot_alerts() {
    let mock_server = MockServer::start().await;
    let expected_path = "/enterprises/my-corp/dependabot/alerts";

    Mock::given(method("GET"))
        .and(path(expected_path))
        .and(query_param("state", "open"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!([sample_dependabot_alert(99)])),
        )
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, expected_path).await;

    let client = setup_octocrab(&mock_server.uri());
    let page = client
        .enterprises("my-corp")
        .dependabot()
        .alerts()
        .list()
        .state("open")
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].number, 99);
}

// 18. Manage enterprise dependabot repository access
#[tokio::test]
async fn should_manage_enterprise_dependabot_repository_access() {
    let mock_server = MockServer::start().await;
    let base_path = "/enterprises/my-corp/dependabot/repository-access";

    // List
    Mock::given(method("GET"))
        .and(path(base_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "default_level": "internal",
            "accessible_repositories": [sample_repo(555, "enterprise-repo")]
        })))
        .mount(&mock_server)
        .await;

    // Update
    Mock::given(method("PATCH"))
        .and(path(base_path))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    // Set default level
    Mock::given(method("PUT"))
        .and(path(format!("{base_path}/default-level")))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let access_handler = client
        .enterprises("my-corp")
        .dependabot()
        .repository_access();

    // List
    let access: DependabotRepositoryAccess = access_handler.list().send().await.unwrap();
    assert_eq!(
        access.default_level,
        DependabotDefaultRepositoryAccessLevel::Internal
    );
    assert_eq!(access.accessible_repositories.len(), 1);
    assert_eq!(access.accessible_repositories[0].name, "enterprise-repo");

    // Update
    access_handler
        .update()
        .add_repository(RepositoryId(777))
        .remove_repository(RepositoryId(888))
        .send()
        .await
        .unwrap();

    // Set default level
    access_handler
        .set_default_level(DependabotDefaultRepositoryAccessLevel::Public)
        .await
        .unwrap();
}
