mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{
    models::repos::{
        PageBuild, PageBuildStatus, PagesBuildType, PagesDeployment, PagesDeploymentId,
        PagesDeploymentStatus, PagesDeploymentStatusState, PagesHealthCheck, PagesSite,
        PagesSource,
    },
    Octocrab,
};
use wiremock::{
    matchers::{body_json, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

const OWNER: &str = "owner";
const REPO: &str = "repo";

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_get_pages_site() {
    let mock_server = MockServer::start().await;
    let mocked_response: PagesSite =
        serde_json::from_str(include_str!("resources/repo_pages_site.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/pages")))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/pages was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).pages().get().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let site = result.unwrap();
    assert_eq!(site.status, Some("built".to_string()));
    assert_eq!(site.cname, Some("example.com".to_string()));
    assert_eq!(site.protected_domain_state, Some("verified".to_string()));
    assert_eq!(site.build_type, Some(PagesBuildType::Legacy));
    assert!(site.public);
    assert_eq!(site.https_enforced, Some(true));

    let source = site.source.unwrap();
    assert_eq!(source.branch, "main");
    assert_eq!(source.path, "/");

    let cert = site.https_certificate.unwrap();
    assert_eq!(cert.state, "approved");
    assert_eq!(cert.domains, vec!["example.com", "www.example.com"]);
}

#[tokio::test]
async fn should_create_pages_site() {
    let mock_server = MockServer::start().await;
    let mocked_response: PagesSite =
        serde_json::from_str(include_str!("resources/repo_pages_site.json")).unwrap();

    let template = ResponseTemplate::new(201).set_body_json(&mocked_response);

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/pages")))
        .and(body_json(serde_json::json!({
            "build_type": "legacy",
            "source": {
                "branch": "main",
                "path": "/"
            }
        })))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("POST on /repos/{OWNER}/{REPO}/pages was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .pages()
        .create()
        .build_type(PagesBuildType::Legacy)
        .source(PagesSource::new("main", "/"))
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let site = result.unwrap();
    assert_eq!(site.status, Some("built".to_string()));
}

#[tokio::test]
async fn should_update_pages_site() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path(format!("/repos/{OWNER}/{REPO}/pages")))
        .and(body_json(serde_json::json!({
            "cname": "example.com",
            "https_enforced": true,
            "build_type": "workflow",
            "source": {
                "branch": "gh-pages",
                "path": "/docs"
            }
        })))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("PUT on /repos/{OWNER}/{REPO}/pages was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .pages()
        .update()
        .cname("example.com")
        .https_enforced(true)
        .build_type(PagesBuildType::Workflow)
        .source(PagesSource::new("gh-pages", "/docs"))
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_update_pages_site_remove_cname() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path(format!("/repos/{OWNER}/{REPO}/pages")))
        .and(body_json(serde_json::json!({
            "cname": null
        })))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("PUT on /repos/{OWNER}/{REPO}/pages was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .pages()
        .update()
        .remove_cname()
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_delete_pages_site() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!("/repos/{OWNER}/{REPO}/pages")))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("DELETE on /repos/{OWNER}/{REPO}/pages was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).pages().delete().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_get_pages_health_check() {
    let mock_server = MockServer::start().await;
    let mocked_response: PagesHealthCheck =
        serde_json::from_str(include_str!("resources/repo_pages_health.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/pages/health")))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/pages/health was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).pages().health().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let health = result.unwrap();
    let domain = health.domain.unwrap();
    assert_eq!(domain.host, Some("example.com".to_string()));
    assert_eq!(domain.dns_resolves, Some(true));
    assert_eq!(domain.is_valid, Some(true));
    assert_eq!(domain.enforces_https, Some(true));
}

#[tokio::test]
async fn should_list_pages_builds() {
    let mock_server = MockServer::start().await;
    let mocked_response: Vec<PageBuild> =
        serde_json::from_str(include_str!("resources/repo_pages_builds.json")).unwrap();

    let template = ResponseTemplate::new(200)
        .set_body_json(&mocked_response)
        .append_header(
            "Link",
            "<https://api.github.com/repositories/123/pages/builds?page=2>; rel=\"next\"",
        );

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/pages/builds")))
        .and(query_param("per_page", "10"))
        .and(query_param("page", "1"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/pages/builds was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .pages()
        .builds()
        .list()
        .per_page(10)
        .page(1u32)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let page = result.unwrap();
    assert_eq!(page.items.len(), 1);
    let build = &page.items[0];
    assert_eq!(build.status, "built");
    assert_eq!(build.commit, "351391cdcb88ffae71ec3028c91f375a8036a26b");
    assert_eq!(build.duration, 2104);
    assert_eq!(build.pusher.as_ref().unwrap().login, "octocat");
}

#[tokio::test]
async fn should_get_latest_pages_build() {
    let mock_server = MockServer::start().await;
    let mocked_response: PageBuild =
        serde_json::from_str(include_str!("resources/repo_pages_build.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/pages/builds/latest")))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/pages/builds/latest was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).pages().latest_build().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let build = result.unwrap();
    assert_eq!(build.status, "built");
    assert_eq!(build.commit, "351391cdcb88ffae71ec3028c91f375a8036a26b");
}

#[tokio::test]
async fn should_get_pages_build_by_id() {
    let mock_server = MockServer::start().await;
    let mocked_response: PageBuild =
        serde_json::from_str(include_str!("resources/repo_pages_build.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/pages/builds/5472601")))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/pages/builds/5472601 was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .pages()
        .get_build(5472601u64)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let build = result.unwrap();
    assert_eq!(build.status, "built");
    assert_eq!(build.commit, "351391cdcb88ffae71ec3028c91f375a8036a26b");
}

#[tokio::test]
async fn should_request_pages_build() {
    let mock_server = MockServer::start().await;
    let mocked_response: PageBuildStatus =
        serde_json::from_str(include_str!("resources/repo_pages_build_status.json")).unwrap();

    let template = ResponseTemplate::new(201).set_body_json(&mocked_response);

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/pages/builds")))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("POST on /repos/{OWNER}/{REPO}/pages/builds was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).pages().request_build().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let status = result.unwrap();
    assert_eq!(status.status, "queued");
}

#[tokio::test]
async fn should_create_pages_deployment() {
    let mock_server = MockServer::start().await;
    let mocked_response: PagesDeployment =
        serde_json::from_str(include_str!("resources/repo_pages_deployment.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/pages/deployments")))
        .and(body_json(serde_json::json!({
            "artifact_id": 12345,
            "environment": "github-pages",
            "pages_build_version": "4fd754f7e594640989b406850d0bc8f06a121251",
            "oidc_token": "token123"
        })))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("POST on /repos/{OWNER}/{REPO}/pages/deployments was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .pages()
        .create_deployment("token123")
        .artifact_id(12345u64)
        .environment("github-pages")
        .pages_build_version("4fd754f7e594640989b406850d0bc8f06a121251")
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let deployment = result.unwrap();
    assert_eq!(
        deployment.id,
        PagesDeploymentId::from("4fd754f7e594640989b406850d0bc8f06a121251")
    );
    assert_eq!(deployment.page_url, "octocat.github.io/Hello-World");
    assert_eq!(
        deployment.preview_url,
        Some("octocat-123.drafts.github.io".to_string())
    );
}

#[tokio::test]
async fn should_get_pages_deployment_status() {
    let mock_server = MockServer::start().await;
    let mocked_response: PagesDeploymentStatus =
        serde_json::from_str(include_str!("resources/repo_pages_deployment_status.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/pages/deployments/4fd754f7e594640989b406850d0bc8f06a121251"
        )))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/pages/deployments/... was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .pages()
        .get_deployment_status("4fd754f7e594640989b406850d0bc8f06a121251")
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let status = result.unwrap();
    assert_eq!(status.status, Some(PagesDeploymentStatusState::Succeed));
}

#[tokio::test]
async fn should_cancel_pages_deployment() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/pages/deployments/4fd754f7e594640989b406850d0bc8f06a121251/cancel"
        )))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("POST on /repos/{OWNER}/{REPO}/pages/deployments/.../cancel was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .pages()
        .cancel_deployment("4fd754f7e594640989b406850d0bc8f06a121251")
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}
