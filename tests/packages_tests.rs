use http::StatusCode;
use octocrab::models::packages::{PackageType, PackageVersionState, PackageVisibility};
use octocrab::Octocrab;
use serde_json::json;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

mod mock_error;
use mock_error::setup_error_handler;

const ORG: &str = "test-org";
const USER: &str = "test-user";
const PKG_NAME: &str = "my-package";

fn sample_package(name: &str) -> serde_json::Value {
    json!({
        "id": 101,
        "name": name,
        "package_type": "container",
        "url": format!("https://api.github.com/orgs/{ORG}/packages/container/{name}"),
        "html_url": format!("https://github.com/orgs/{ORG}/packages/container/package/{name}"),
        "version_count": 1,
        "visibility": "public",
        "owner": {
            "login": ORG,
            "id": 12345,
            "node_id": "MDEyOk9yZ2FuaXphdGlvbjEyMzQ1",
            "avatar_url": "https://avatars.githubusercontent.com/u/12345?v=4",
            "gravatar_id": "",
            "url": format!("https://api.github.com/users/{ORG}"),
            "html_url": format!("https://github.com/{ORG}"),
            "followers_url": format!("https://api.github.com/users/{ORG}/followers"),
            "following_url": format!("https://api.github.com/users/{ORG}/following{{/other_user}}"),
            "gists_url": format!("https://api.github.com/users/{ORG}/gists{{/gist_id}}"),
            "starred_url": format!("https://api.github.com/users/{ORG}/starred{{/owner}}{{/repo}}"),
            "subscriptions_url": format!("https://api.github.com/users/{ORG}/subscriptions"),
            "organizations_url": format!("https://api.github.com/users/{ORG}/orgs"),
            "repos_url": format!("https://api.github.com/users/{ORG}/repos"),
            "events_url": format!("https://api.github.com/users/{ORG}/events{{/privacy}}"),
            "received_events_url": format!("https://api.github.com/users/{ORG}/received_events"),
            "type": "Organization",
            "site_admin": false
        },
        "created_at": "2023-01-01T00:00:00Z",
        "updated_at": "2023-01-02T00:00:00Z"
    })
}

fn sample_package_version(id: u64, name: &str) -> serde_json::Value {
    json!({
        "id": id,
        "name": name,
        "url": format!("https://api.github.com/orgs/{ORG}/packages/container/{PKG_NAME}/versions/{id}"),
        "package_html_url": format!("https://github.com/orgs/{ORG}/packages/container/package/{PKG_NAME}"),
        "html_url": format!("https://github.com/{ORG}/{PKG_NAME}/pkgs/container/{PKG_NAME}/{id}"),
        "license": "MIT",
        "description": "A container package version",
        "created_at": "2023-01-01T00:00:00Z",
        "updated_at": "2023-01-02T00:00:00Z",
        "metadata": {
            "package_type": "container",
            "container": {
                "tags": ["latest", "v1.0.0"]
            }
        }
    })
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_list_packages_for_org() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/orgs/{ORG}/packages");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .and(query_param("package_type", "container"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([sample_package(PKG_NAME)])))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "GET /orgs/{org}/packages was not received").await;

    let client = setup_octocrab(&mock_server.uri());
    let page = client
        .orgs(ORG)
        .packages()
        .list(PackageType::Container)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].name, PKG_NAME);
    assert_eq!(page.items[0].package_type, PackageType::Container);
}

#[tokio::test]
async fn should_list_packages_for_user() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/users/{USER}/packages");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .and(query_param("package_type", "npm"))
        .and(query_param("visibility", "public"))
        .and(query_param("per_page", "50"))
        .and(query_param("page", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([sample_package(PKG_NAME)])))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "GET /users/{user}/packages was not received").await;

    let client = setup_octocrab(&mock_server.uri());
    let page = client
        .users(USER)
        .packages()
        .list(PackageType::Npm)
        .visibility(PackageVisibility::Public)
        .per_page(50u8)
        .page(2u32)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
}

#[tokio::test]
async fn should_list_packages_for_current_user() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/user/packages"))
        .and(query_param("package_type", "docker"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([sample_package(PKG_NAME)])))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "GET /user/packages was not received").await;

    let client = setup_octocrab(&mock_server.uri());
    let page = client
        .current()
        .packages()
        .list(PackageType::Docker)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
}

#[tokio::test]
async fn should_get_package() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/orgs/{ORG}/packages/container/{PKG_NAME}");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_package(PKG_NAME)))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "GET package was not received").await;

    let client = setup_octocrab(&mock_server.uri());
    let pkg = client
        .packages()
        .org(ORG)
        .get(PackageType::Container, PKG_NAME)
        .await
        .unwrap();

    assert_eq!(pkg.name, PKG_NAME);
    assert_eq!(pkg.package_type, PackageType::Container);
}

#[tokio::test]
async fn should_get_scoped_package_with_percent_encoding() {
    let mock_server = MockServer::start().await;
    let scoped_pkg = "@scope/my-pkg";
    let encoded_path = format!("/orgs/{ORG}/packages/npm/@scope%2Fmy-pkg");

    Mock::given(method("GET"))
        .and(path(&encoded_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_package(scoped_pkg)))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "GET scoped package was not received").await;

    let client = setup_octocrab(&mock_server.uri());
    let pkg = client
        .orgs(ORG)
        .packages()
        .get(PackageType::Npm, scoped_pkg)
        .await
        .unwrap();

    assert_eq!(pkg.name, scoped_pkg);
}

#[tokio::test]
async fn should_delete_package() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/orgs/{ORG}/packages/container/{PKG_NAME}");

    Mock::given(method("DELETE"))
        .and(path(&expected_path))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "DELETE package was not received").await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .orgs(ORG)
        .packages()
        .delete(PackageType::Container, PKG_NAME)
        .await;

    assert!(result.is_ok(), "delete failed: {:?}", result);
}

#[tokio::test]
async fn should_restore_package() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/orgs/{ORG}/packages/container/{PKG_NAME}/restore");

    Mock::given(method("POST"))
        .and(path(&expected_path))
        .and(query_param("token", "secret123"))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "POST restore package was not received").await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .orgs(ORG)
        .packages()
        .restore(PackageType::Container, PKG_NAME)
        .token("secret123")
        .send()
        .await;

    assert!(result.is_ok(), "restore package failed: {:?}", result);
}

#[tokio::test]
async fn should_list_package_versions() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/orgs/{ORG}/packages/container/{PKG_NAME}/versions");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .and(query_param("state", "active"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!([sample_package_version(42, "v1.0.0")])),
        )
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "GET package versions was not received").await;

    let client = setup_octocrab(&mock_server.uri());
    let page = client
        .orgs(ORG)
        .packages()
        .list_versions(PackageType::Container, PKG_NAME)
        .state(PackageVersionState::Active)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].id.0, 42);
    assert_eq!(page.items[0].name, "v1.0.0");
}

#[tokio::test]
async fn should_get_package_version() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/orgs/{ORG}/packages/container/{PKG_NAME}/versions/42");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(sample_package_version(42, "v1.0.0")),
        )
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "GET package version was not received").await;

    let client = setup_octocrab(&mock_server.uri());
    let version = client
        .orgs(ORG)
        .packages()
        .get_version(PackageType::Container, PKG_NAME, 42u64)
        .await
        .unwrap();

    assert_eq!(version.id.0, 42);
    assert_eq!(version.name, "v1.0.0");
    assert_eq!(version.license, Some("MIT".to_string()));
}

#[tokio::test]
async fn should_delete_package_version() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/orgs/{ORG}/packages/container/{PKG_NAME}/versions/42");

    Mock::given(method("DELETE"))
        .and(path(&expected_path))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "DELETE package version was not received").await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .orgs(ORG)
        .packages()
        .delete_version(PackageType::Container, PKG_NAME, 42u64)
        .await;

    assert!(
        result.is_ok(),
        "delete package version failed: {:?}",
        result
    );
}

#[tokio::test]
async fn should_restore_package_version() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/orgs/{ORG}/packages/container/{PKG_NAME}/versions/42/restore");

    Mock::given(method("POST"))
        .and(path(&expected_path))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "POST restore package version was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .orgs(ORG)
        .packages()
        .restore_version(PackageType::Container, PKG_NAME, 42u64)
        .await;

    assert!(
        result.is_ok(),
        "restore package version failed: {:?}",
        result
    );
}

#[tokio::test]
async fn should_get_docker_conflicts_for_all_namespaces() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/orgs/{ORG}/docker/conflicts")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([sample_package(PKG_NAME)])))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path(format!("/users/{USER}/docker/conflicts")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([sample_package(PKG_NAME)])))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/user/docker/conflicts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([sample_package(PKG_NAME)])))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());

    let org_conflicts = client
        .orgs(ORG)
        .packages()
        .docker_conflicts()
        .await
        .unwrap();
    assert_eq!(org_conflicts.len(), 1);

    let user_conflicts = client
        .users(USER)
        .packages()
        .docker_conflicts()
        .await
        .unwrap();
    assert_eq!(user_conflicts.len(), 1);

    let current_conflicts = client
        .current()
        .packages()
        .docker_conflicts()
        .await
        .unwrap();
    assert_eq!(current_conflicts.len(), 1);
}

#[tokio::test]
async fn should_map_errors() {
    let mock_server = MockServer::start().await;
    let expected_path = format!("/orgs/{ORG}/packages/container/{PKG_NAME}");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .respond_with(
            ResponseTemplate::new(StatusCode::NOT_FOUND).set_body_json(json!({
                "message": "Not Found",
                "documentation_url": "https://docs.github.com/rest/packages/packages"
            })),
        )
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let err = client
        .orgs(ORG)
        .packages()
        .get(PackageType::Container, PKG_NAME)
        .await
        .unwrap_err();

    match err {
        octocrab::Error::GitHub { source, .. } => {
            assert_eq!(source.status_code, StatusCode::NOT_FOUND);
            assert_eq!(source.message, "Not Found");
        }
        other => panic!("expected GitHub error, got: {:?}", other),
    }
}
