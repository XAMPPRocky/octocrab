use octocrab::{
    models::actions::{AllowedActions, EnabledRepositories, SelectedActions},
    Octocrab,
};

use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_org_actions_permissions() {
    let mock_server = MockServer::start().await;
    let perms_json = json!({
        "enabled_repositories": "all",
        "allowed_actions": "all",
        "sha_pinning_required": false
    });

    Mock::given(method("GET"))
        .and(path("/orgs/my-org/actions/permissions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&perms_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/orgs/my-org/actions/permissions"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let perms = client
        .actions()
        .get_org_actions_permissions("my-org")
        .await
        .unwrap();

    assert_eq!(perms.enabled_repositories, EnabledRepositories::All);

    let set_res = client
        .actions()
        .set_org_actions_permissions(
            "my-org",
            EnabledRepositories::All,
            Some(AllowedActions::All),
            Some(false),
        )
        .await;
    assert!(set_res.is_ok());
}

#[tokio::test]
async fn test_org_selected_repositories() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/orgs/my-org/actions/permissions/repositories"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/orgs/my-org/actions/permissions/repositories/123"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/orgs/my-org/actions/permissions/repositories/123"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    assert!(client
        .actions()
        .set_org_selected_repositories("my-org", &[123u64.into()])
        .await
        .is_ok());
    assert!(client
        .actions()
        .enable_org_selected_repository("my-org", 123u64.into())
        .await
        .is_ok());
    assert!(client
        .actions()
        .disable_org_selected_repository("my-org", 123u64.into())
        .await
        .is_ok());
}

#[tokio::test]
async fn test_org_selected_actions() {
    let mock_server = MockServer::start().await;
    let selected_json = json!({
        "github_owned_allowed": true,
        "verified_allowed": true,
        "patterns_allowed": ["actions/*"]
    });

    Mock::given(method("GET"))
        .and(path("/orgs/my-org/actions/permissions/selected-actions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&selected_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/orgs/my-org/actions/permissions/selected-actions"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let actions = client
        .actions()
        .get_org_selected_actions("my-org")
        .await
        .unwrap();
    assert!(actions.github_owned_allowed);

    let update: SelectedActions = serde_json::from_value(json!({
        "github_owned_allowed": true,
        "verified_allowed": true,
        "patterns_allowed": ["actions/*"]
    }))
    .unwrap();

    assert!(client
        .actions()
        .set_org_selected_actions("my-org", &update)
        .await
        .is_ok());
}

#[tokio::test]
async fn test_org_default_workflow_permissions() {
    let mock_server = MockServer::start().await;
    let default_perms_json = json!({
        "default_workflow_permissions": "read",
        "can_approve_pull_request_reviews": true
    });

    Mock::given(method("GET"))
        .and(path("/orgs/my-org/actions/permissions/workflow"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&default_perms_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/orgs/my-org/actions/permissions/workflow"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let perms = client
        .actions()
        .get_org_default_workflow_permissions("my-org")
        .await
        .unwrap();
    assert_eq!(perms.default_workflow_permissions, "read");
    assert!(perms.can_approve_pull_request_reviews);

    assert!(client
        .actions()
        .set_org_default_workflow_permissions("my-org", &perms)
        .await
        .is_ok());
}

#[tokio::test]
async fn test_repo_actions_permissions() {
    let mock_server = MockServer::start().await;
    let repo_perms_json = json!({
        "enabled": true,
        "allowed_actions": "all",
        "sha_pinning_required": false
    });

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/actions/permissions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&repo_perms_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/repos/owner/repo/actions/permissions"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let perms = client
        .actions()
        .get_repo_actions_permissions("owner", "repo")
        .await
        .unwrap();
    assert!(perms.enabled);

    assert!(client
        .actions()
        .set_repo_actions_permissions(
            "owner",
            "repo",
            true,
            Some(AllowedActions::All),
            Some(false)
        )
        .await
        .is_ok());
}

#[tokio::test]
async fn test_repo_access_permissions() {
    let mock_server = MockServer::start().await;
    let access_json = json!({
        "access_level": "organization"
    });

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/actions/permissions/access"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&access_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/repos/owner/repo/actions/permissions/access"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let access = client
        .actions()
        .get_repo_actions_access_permissions("owner", "repo")
        .await
        .unwrap();
    assert_eq!(access.access_level, "organization");

    assert!(client
        .actions()
        .set_repo_actions_access_permissions("owner", "repo", "organization")
        .await
        .is_ok());
}
