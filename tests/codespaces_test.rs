mod mock_error;

use mock_error::setup_error_handler;
use octocrab::models::codespaces::{
    CreatePullRequestCodespace, CreateRepoCodespace, CreateUserCodespace,
    CreateUserCodespacesSecret, OrgCodespacesAccessVisibility, PublishCodespace, UpdateCodespace,
};
use octocrab::models::orgs::secrets::{CreateOrganizationSecret, Visibility};
use octocrab::models::repos::secrets::CreateRepositorySecret;
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

fn sample_repo() -> serde_json::Value {
    json!({
        "id": 1296269,
        "node_id": "MDEwOlJlcG9zaXRvcnkxMjk2MjY5",
        "name": "Hello-World",
        "full_name": "octocat/Hello-World",
        "owner": sample_user(),
        "private": false,
        "html_url": "https://github.com/octocat/Hello-World",
        "description": "This your first repo!",
        "fork": false,
        "url": "https://api.github.com/repos/octocat/Hello-World"
    })
}

fn sample_codespace(name: &str) -> serde_json::Value {
    json!({
        "id": 1,
        "name": name,
        "display_name": "my-codespace",
        "environment_id": "2600",
        "owner": sample_user(),
        "billable_owner": sample_user(),
        "repository": sample_repo(),
        "machine": {
            "name": "standardLinux",
            "display_name": "2 cores, 8 GB RAM, 32 GB storage",
            "operating_system": "linux",
            "storage_in_bytes": 34359738368u64,
            "memory_in_bytes": 8589934592u64,
            "cpus": 2,
            "prebuild_availability": "ready"
        },
        "created_at": "2020-11-28T00:00:00Z",
        "updated_at": "2020-11-28T00:00:00Z",
        "last_used_at": "2020-11-28T00:00:00Z",
        "state": "Available",
        "url": format!("https://api.github.com/user/codespaces/{name}"),
        "web_url": format!("https://github.com/codespaces/{name}"),
        "machines_url": format!("https://api.github.com/user/codespaces/{name}/machines"),
        "start_url": format!("https://api.github.com/user/codespaces/{name}/start"),
        "stop_url": format!("https://api.github.com/user/codespaces/{name}/stop"),
        "recent_folders": []
    })
}

#[tokio::test]
async fn should_list_user_codespaces() {
    let mock_server = MockServer::start().await;
    let expected_path = "/user/codespaces";

    Mock::given(method("GET"))
        .and(path(expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "codespaces": [sample_codespace("my-codespace")]
        })))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "GET /user/codespaces was not received").await;

    let client = setup_octocrab(&mock_server.uri());
    let page = client.codespaces().list().send().await.unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].name, "my-codespace");
    assert_eq!(page.total_count, Some(1));
}

#[tokio::test]
async fn should_create_and_manage_user_codespace() {
    let mock_server = MockServer::start().await;
    let name = "test-codespace";

    // Create
    Mock::given(method("POST"))
        .and(path("/user/codespaces"))
        .respond_with(ResponseTemplate::new(201).set_body_json(sample_codespace(name)))
        .mount(&mock_server)
        .await;

    // Get
    Mock::given(method("GET"))
        .and(path(format!("/user/codespaces/{name}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_codespace(name)))
        .mount(&mock_server)
        .await;

    // Update
    Mock::given(method("PATCH"))
        .and(path(format!("/user/codespaces/{name}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_codespace(name)))
        .mount(&mock_server)
        .await;

    // Start
    Mock::given(method("POST"))
        .and(path(format!("/user/codespaces/{name}/start")))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_codespace(name)))
        .mount(&mock_server)
        .await;

    // Stop
    Mock::given(method("POST"))
        .and(path(format!("/user/codespaces/{name}/stop")))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_codespace(name)))
        .mount(&mock_server)
        .await;

    // Publish
    Mock::given(method("POST"))
        .and(path(format!("/user/codespaces/{name}/publish")))
        .respond_with(ResponseTemplate::new(201).set_body_json(sample_codespace(name)))
        .mount(&mock_server)
        .await;

    // Export
    Mock::given(method("POST"))
        .and(path(format!("/user/codespaces/{name}/exports")))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "state": "exporting",
            "id": "export-123"
        })))
        .mount(&mock_server)
        .await;

    // Get export
    Mock::given(method("GET"))
        .and(path(format!("/user/codespaces/{name}/exports/latest")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "state": "completed",
            "id": "export-123"
        })))
        .mount(&mock_server)
        .await;

    // Machines
    Mock::given(method("GET"))
        .and(path(format!("/user/codespaces/{name}/machines")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "machines": [{
                "name": "standardLinux",
                "display_name": "2 cores",
                "operating_system": "linux",
                "storage_in_bytes": 34359738368u64,
                "memory_in_bytes": 8589934592u64,
                "cpus": 2,
                "prebuild_availability": "ready"
            }]
        })))
        .mount(&mock_server)
        .await;

    // Delete
    Mock::given(method("DELETE"))
        .and(path(format!("/user/codespaces/{name}")))
        .respond_with(ResponseTemplate::new(202))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let codespaces = client.current().codespaces();

    // Test create
    let cs = codespaces
        .create(&CreateUserCodespace {
            repository_id: Some(1296269u64.into()),
            display_name: Some("test-codespace".to_string()),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(cs.name, name);

    // Test get
    let cs = codespaces.get(name).await.unwrap();
    assert_eq!(cs.name, name);

    // Test update
    let cs = codespaces
        .update(
            name,
            &UpdateCodespace {
                display_name: Some("updated-codespace".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(cs.name, name);

    // Test start & stop
    let cs = codespaces.start(name).await.unwrap();
    assert_eq!(cs.name, name);
    let cs = codespaces.stop(name).await.unwrap();
    assert_eq!(cs.name, name);

    // Test publish
    let cs = codespaces
        .publish(
            name,
            &PublishCodespace {
                name: Some("new-repo".to_string()),
                private: Some(false),
            },
        )
        .await
        .unwrap();
    assert_eq!(cs.name, name);

    // Test export & get_export
    let export = codespaces.export(name).await.unwrap();
    assert_eq!(export.state.as_deref(), Some("exporting"));
    let export = codespaces.get_export(name, "latest").await.unwrap();
    assert_eq!(export.state.as_deref(), Some("completed"));

    // Test machines
    let machines = codespaces.machines(name).await.unwrap();
    assert_eq!(machines.items.len(), 1);
    assert_eq!(machines.items[0].name, "standardLinux");

    // Test delete
    codespaces.delete(name).await.unwrap();
}

#[tokio::test]
async fn should_manage_user_secrets() {
    let mock_server = MockServer::start().await;
    let secret_name = "MY_USER_SECRET";

    // List
    Mock::given(method("GET"))
        .and(path("/user/codespaces/secrets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "secrets": [{
                "name": secret_name,
                "created_at": "2020-11-28T00:00:00Z",
                "updated_at": "2020-11-28T00:00:00Z",
                "visibility": "selected",
                "selected_repositories_url": "https://api.github.com/user/codespaces/secrets/MY_USER_SECRET/repositories"
            }]
        })))
        .mount(&mock_server)
        .await;

    // Public key
    Mock::given(method("GET"))
        .and(path("/user/codespaces/secrets/public-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "key_id": "key-123",
            "key": "base64-key"
        })))
        .mount(&mock_server)
        .await;

    // Get
    Mock::given(method("GET"))
        .and(path(format!("/user/codespaces/secrets/{secret_name}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "name": secret_name,
            "created_at": "2020-11-28T00:00:00Z",
            "updated_at": "2020-11-28T00:00:00Z",
            "visibility": "all"
        })))
        .mount(&mock_server)
        .await;

    // Create (201)
    Mock::given(method("PUT"))
        .and(path(format!("/user/codespaces/secrets/{secret_name}")))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;

    // Delete (204)
    Mock::given(method("DELETE"))
        .and(path(format!("/user/codespaces/secrets/{secret_name}")))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    // Repositories: list
    Mock::given(method("GET"))
        .and(path(format!(
            "/user/codespaces/secrets/{secret_name}/repositories"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "repositories": [sample_repo()]
        })))
        .mount(&mock_server)
        .await;

    // Repositories: set
    Mock::given(method("PUT"))
        .and(path(format!(
            "/user/codespaces/secrets/{secret_name}/repositories"
        )))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    // Repositories: add
    Mock::given(method("PUT"))
        .and(path(format!(
            "/user/codespaces/secrets/{secret_name}/repositories/12345"
        )))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    // Repositories: remove
    Mock::given(method("DELETE"))
        .and(path(format!(
            "/user/codespaces/secrets/{secret_name}/repositories/12345"
        )))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let secrets = client.codespaces().secrets();

    // List
    let page = secrets.list().send().await.unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].name, secret_name);

    // Public key
    let pk = secrets.get_public_key().await.unwrap();
    assert_eq!(pk.key_id, "key-123");

    // Get
    let s = secrets.get(secret_name).await.unwrap();
    assert_eq!(s.name, secret_name);

    // Create
    let res = secrets
        .create_or_update(
            secret_name,
            &CreateUserCodespacesSecret {
                encrypted_value: "enc",
                key_id: "key-123",
                selected_repository_ids: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(
        res,
        octocrab::models::orgs::secrets::CreateOrganizationSecretResponse::Created
    );

    // Repositories operations
    let repos = secrets.repositories(secret_name);
    let list = repos.list().await.unwrap();
    assert_eq!(list.repositories.len(), 1);
    repos.set(&[12345u64.into()]).await.unwrap();
    repos.add(12345u64.into()).await.unwrap();
    repos.remove(12345u64.into()).await.unwrap();

    // Delete
    secrets.delete(secret_name).await.unwrap();
}

#[tokio::test]
async fn should_manage_org_codespaces() {
    let mock_server = MockServer::start().await;
    let org = "test-org";
    let user = "octocat";
    let cs_name = "org-codespace";

    // List org codespaces
    Mock::given(method("GET"))
        .and(path(format!("/orgs/{org}/codespaces")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "codespaces": [sample_codespace(cs_name)]
        })))
        .mount(&mock_server)
        .await;

    // Access control: get
    Mock::given(method("GET"))
        .and(path(format!("/orgs/{org}/codespaces/access")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "visibility": "selected_members"
        })))
        .mount(&mock_server)
        .await;

    // Access control: update
    Mock::given(method("PUT"))
        .and(path(format!("/orgs/{org}/codespaces/access")))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    // Access control: add selected users
    Mock::given(method("POST"))
        .and(path(format!(
            "/orgs/{org}/codespaces/access/selected_users"
        )))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    // Access control: remove selected users
    Mock::given(method("DELETE"))
        .and(path(format!(
            "/orgs/{org}/codespaces/access/selected_users"
        )))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    // List codespaces for user in org
    Mock::given(method("GET"))
        .and(path(format!("/orgs/{org}/members/{user}/codespaces")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "codespaces": [sample_codespace(cs_name)]
        })))
        .mount(&mock_server)
        .await;

    // Delete codespace for user in org
    Mock::given(method("DELETE"))
        .and(path(format!(
            "/orgs/{org}/members/{user}/codespaces/{cs_name}"
        )))
        .respond_with(ResponseTemplate::new(202))
        .mount(&mock_server)
        .await;

    // Stop codespace for user in org
    Mock::given(method("POST"))
        .and(path(format!(
            "/orgs/{org}/members/{user}/codespaces/{cs_name}/stop"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_codespace(cs_name)))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let org_codespaces = client.orgs(org).codespaces();

    // List
    let page = org_codespaces.list().send().await.unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].name, cs_name);

    // Access
    let access = org_codespaces.access().await.unwrap();
    assert_eq!(
        access.visibility,
        OrgCodespacesAccessVisibility::SelectedMembers
    );
    org_codespaces
        .update_access(
            OrgCodespacesAccessVisibility::SelectedMembers,
            Some(&["octocat"]),
        )
        .await
        .unwrap();

    // Add / remove selected users
    org_codespaces
        .add_selected_users(&["octocat"])
        .await
        .unwrap();
    org_codespaces
        .remove_selected_users(&["octocat"])
        .await
        .unwrap();

    // User codespaces in org
    let user_page = org_codespaces.list_for_user(user).send().await.unwrap();
    assert_eq!(user_page.items.len(), 1);

    // Stop & delete
    let cs = org_codespaces.stop_for_user(user, cs_name).await.unwrap();
    assert_eq!(cs.name, cs_name);
    org_codespaces.delete_for_user(user, cs_name).await.unwrap();
}

#[tokio::test]
async fn should_manage_org_codespaces_secrets() {
    let mock_server = MockServer::start().await;
    let org = "test-org";
    let secret_name = "ORG_CODESPACE_SECRET";

    // List
    Mock::given(method("GET"))
        .and(path(format!("/orgs/{org}/codespaces/secrets")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "secrets": [{
                "name": secret_name,
                "created_at": "2020-11-28T00:00:00Z",
                "updated_at": "2020-11-28T00:00:00Z",
                "visibility": "selected",
                "selected_repositories_url": format!("https://api.github.com/orgs/{org}/codespaces/secrets/{secret_name}/repositories")
            }]
        })))
        .mount(&mock_server)
        .await;

    // Public key
    Mock::given(method("GET"))
        .and(path(format!("/orgs/{org}/codespaces/secrets/public-key")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "key_id": "key-org",
            "key": "base64-key-org"
        })))
        .mount(&mock_server)
        .await;

    // Get
    Mock::given(method("GET"))
        .and(path(format!(
            "/orgs/{org}/codespaces/secrets/{secret_name}"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "name": secret_name,
            "created_at": "2020-11-28T00:00:00Z",
            "updated_at": "2020-11-28T00:00:00Z",
            "visibility": "all"
        })))
        .mount(&mock_server)
        .await;

    // Create (201)
    Mock::given(method("PUT"))
        .and(path(format!(
            "/orgs/{org}/codespaces/secrets/{secret_name}"
        )))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;

    // Delete (204)
    Mock::given(method("DELETE"))
        .and(path(format!(
            "/orgs/{org}/codespaces/secrets/{secret_name}"
        )))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    // Repositories: list
    Mock::given(method("GET"))
        .and(path(format!(
            "/orgs/{org}/codespaces/secrets/{secret_name}/repositories"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "repositories": [sample_repo()]
        })))
        .mount(&mock_server)
        .await;

    // Repositories: set
    Mock::given(method("PUT"))
        .and(path(format!(
            "/orgs/{org}/codespaces/secrets/{secret_name}/repositories"
        )))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    // Repositories: add
    Mock::given(method("PUT"))
        .and(path(format!(
            "/orgs/{org}/codespaces/secrets/{secret_name}/repositories/67890"
        )))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    // Repositories: remove
    Mock::given(method("DELETE"))
        .and(path(format!(
            "/orgs/{org}/codespaces/secrets/{secret_name}/repositories/67890"
        )))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let secrets = client.orgs(org).codespaces().secrets();

    // List
    let page = secrets.list().send().await.unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].name, secret_name);

    // Public key
    let pk = secrets.get_public_key().await.unwrap();
    assert_eq!(pk.key_id, "key-org");

    // Get
    let s = secrets.get(secret_name).await.unwrap();
    assert_eq!(s.name, secret_name);

    // Create
    let res = secrets
        .create_or_update(
            secret_name,
            &CreateOrganizationSecret {
                encrypted_value: "enc",
                key_id: "key-org",
                visibility: Visibility::Selected,
                selected_repository_ids: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(
        res,
        octocrab::models::orgs::secrets::CreateOrganizationSecretResponse::Created
    );

    // Repositories operations
    let repos = secrets.repositories(secret_name);
    let list = repos.list().await.unwrap();
    assert_eq!(list.repositories.len(), 1);
    repos.set(&[67890u64.into()]).await.unwrap();
    repos.add(67890u64.into()).await.unwrap();
    repos.remove(67890u64.into()).await.unwrap();

    // Delete
    secrets.delete(secret_name).await.unwrap();
}

#[tokio::test]
async fn should_manage_repo_codespaces() {
    let mock_server = MockServer::start().await;
    let owner = "owner";
    let repo = "repo";
    let cs_name = "repo-codespace";

    // List repo codespaces
    Mock::given(method("GET"))
        .and(path(format!("/repos/{owner}/{repo}/codespaces")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "codespaces": [sample_codespace(cs_name)]
        })))
        .mount(&mock_server)
        .await;

    // Create repo codespace
    Mock::given(method("POST"))
        .and(path(format!("/repos/{owner}/{repo}/codespaces")))
        .respond_with(ResponseTemplate::new(201).set_body_json(sample_codespace(cs_name)))
        .mount(&mock_server)
        .await;

    // Create codespace for PR (via pulls handler)
    Mock::given(method("POST"))
        .and(path(format!("/repos/{owner}/{repo}/pulls/42/codespaces")))
        .respond_with(ResponseTemplate::new(201).set_body_json(sample_codespace(cs_name)))
        .mount(&mock_server)
        .await;

    // Devcontainers
    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{owner}/{repo}/codespaces/devcontainers"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "devcontainers": [{
                "path": ".devcontainer/devcontainer.json",
                "name": "Rust Devcontainer"
            }]
        })))
        .mount(&mock_server)
        .await;

    // Default attributes
    Mock::given(method("GET"))
        .and(path(format!("/repos/{owner}/{repo}/codespaces/new")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "defaults": {
                "location": "WestUs2",
                "devcontainer_path": ".devcontainer/devcontainer.json"
            }
        })))
        .mount(&mock_server)
        .await;

    // Permissions check
    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{owner}/{repo}/codespaces/permissions_check"
        )))
        .and(query_param("ref", "main"))
        .and(query_param(
            "devcontainer_path",
            ".devcontainer/devcontainer.json",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "accepted": true
        })))
        .mount(&mock_server)
        .await;

    // Machines
    Mock::given(method("GET"))
        .and(path(format!("/repos/{owner}/{repo}/codespaces/machines")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "machines": [{
                "name": "standardLinux",
                "display_name": "2 cores",
                "operating_system": "linux",
                "storage_in_bytes": 34359738368u64,
                "memory_in_bytes": 8589934592u64,
                "cpus": 2,
                "prebuild_availability": "ready"
            }]
        })))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let repo_cs = client.repos(owner, repo).codespaces();

    // List
    let page = repo_cs.list().send().await.unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].name, cs_name);

    // Create
    let cs = repo_cs
        .create(&CreateRepoCodespace {
            ref_: Some("main".to_string()),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(cs.name, cs_name);

    // Create from PR via pulls
    let cs_pr = client
        .pulls(owner, repo)
        .create_codespace(42, &CreatePullRequestCodespace::default())
        .await
        .unwrap();
    assert_eq!(cs_pr.name, cs_name);

    // Devcontainers
    let devs = repo_cs.devcontainers().send().await.unwrap();
    assert_eq!(devs.items.len(), 1);
    assert_eq!(devs.items[0].path, ".devcontainer/devcontainer.json");

    // Default attributes
    let defaults = repo_cs.default_attributes().send().await.unwrap();
    assert_eq!(defaults.defaults.location, "WestUs2");

    // Permissions check
    let accepted = repo_cs
        .check_permissions("main", ".devcontainer/devcontainer.json")
        .await
        .unwrap();
    assert!(accepted);

    // Machines
    let machines = repo_cs.machines().await.unwrap();
    assert_eq!(machines.items.len(), 1);
    assert_eq!(machines.items[0].name, "standardLinux");
}

#[tokio::test]
async fn should_manage_repo_codespaces_secrets() {
    let mock_server = MockServer::start().await;
    let owner = "owner";
    let repo = "repo";
    let secret_name = "REPO_CODESPACE_SECRET";

    // List
    Mock::given(method("GET"))
        .and(path(format!("/repos/{owner}/{repo}/codespaces/secrets")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "secrets": [{
                "name": secret_name,
                "created_at": "2020-11-28T00:00:00Z",
                "updated_at": "2020-11-28T00:00:00Z"
            }]
        })))
        .mount(&mock_server)
        .await;

    // Public key
    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{owner}/{repo}/codespaces/secrets/public-key"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "key_id": "key-repo",
            "key": "base64-key-repo"
        })))
        .mount(&mock_server)
        .await;

    // Get
    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{owner}/{repo}/codespaces/secrets/{secret_name}"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "name": secret_name,
            "created_at": "2020-11-28T00:00:00Z",
            "updated_at": "2020-11-28T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    // Create (201)
    Mock::given(method("PUT"))
        .and(path(format!(
            "/repos/{owner}/{repo}/codespaces/secrets/{secret_name}"
        )))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;

    // Delete (204)
    Mock::given(method("DELETE"))
        .and(path(format!(
            "/repos/{owner}/{repo}/codespaces/secrets/{secret_name}"
        )))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let secrets = client.repos(owner, repo).codespaces().secrets();

    // List
    let page = secrets.list().send().await.unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].name, secret_name);

    // Public key
    let pk = secrets.get_public_key().await.unwrap();
    assert_eq!(pk.key_id, "key-repo");

    // Get
    let s = secrets.get(secret_name).await.unwrap();
    assert_eq!(s.name, secret_name);

    // Create
    let res = secrets
        .create_or_update(
            secret_name,
            &CreateRepositorySecret {
                encrypted_value: "enc",
                key_id: "key-repo",
            },
        )
        .await
        .unwrap();
    assert_eq!(
        res,
        octocrab::models::repos::secrets::CreateRepositorySecretResponse::Created
    );

    // Delete
    secrets.delete(secret_name).await.unwrap();
}
