mod mock_error;

use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::Octocrab;

async fn setup_mock_http_server(
    http_method: &str,
    mocked_path: &str,
    template: ResponseTemplate,
) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method(http_method))
        .and(path(mocked_path))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("http method {http_method} on {mocked_path} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_get_api_root() {
    let root_json = r#"{
   "current_user_url": "https://api.github.com/user",
   "current_user_authorizations_html_url": "https://github.com/settings/connections/applications{/client_id}",
   "authorizations_url": "https://api.github.com/authorizations",
   "code_search_url": "https://api.github.com/search/code?q={query}{&page,per_page,sort,order}",
   "commit_search_url": "https://api.github.com/search/commits?q={query}{&page,per_page,sort,order}",
   "emails_url": "https://api.github.com/user/emails",
   "emojis_url": "https://api.github.com/emojis",
   "events_url": "https://api.github.com/events",
   "feeds_url": "https://api.github.com/feeds",
   "followers_url": "https://api.github.com/user/followers",
   "following_url": "https://api.github.com/user/following{/target}",
   "gists_url": "https://api.github.com/gists{/gist_id}",
   "hub_url": "https://api.github.com/hub",
   "issue_search_url": "https://api.github.com/search/issues?q={query}{&page,per_page,sort,order}",
   "issues_url": "https://api.github.com/issues",
   "keys_url": "https://api.github.com/user/keys",
   "label_search_url": "https://api.github.com/search/labels?q={query}&repository_id={repository_id}{&page,per_page}",
   "notifications_url": "https://api.github.com/notifications",
   "organization_url": "https://api.github.com/orgs/{org}",
   "organization_repositories_url": "https://api.github.com/orgs/{org}/repos{?type,page,per_page,sort}",
   "organization_teams_url": "https://api.github.com/orgs/{org}/teams",
   "public_gists_url": "https://api.github.com/gists/public",
   "rate_limit_url": "https://api.github.com/rate_limit",
   "repository_url": "https://api.github.com/repos/{owner}/{repo}",
   "repository_search_url": "https://api.github.com/search/repositories?q={query}{&page,per_page,sort,order}",
   "current_user_repositories_url": "https://api.github.com/user/repos{?type,page,per_page,sort}",
   "starred_url": "https://api.github.com/user/starred{/owner}{/repo}",
   "starred_gists_url": "https://api.github.com/gists/starred",
   "topic_search_url": "https://api.github.com/search/topics?q={query}{&page,per_page}",
   "user_url": "https://api.github.com/users/{user}",
   "user_organizations_url": "https://api.github.com/user/orgs",
   "user_repositories_url": "https://api.github.com/users/{user}/repos{?type,page,per_page,sort}",
   "user_search_url": "https://api.github.com/search/users?q={query}{&page,per_page,sort,order}"
}"#;
    let template = ResponseTemplate::new(200)
        .set_body_json(serde_json::from_str::<serde_json::Value>(root_json).unwrap());
    let mock_server = setup_mock_http_server("GET", "/", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.meta().get_api_root().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let root = result.unwrap();
    assert_eq!(root.current_user_url, "https://api.github.com/user");
    assert_eq!(root.hub_url.as_deref(), Some("https://api.github.com/hub"));
}

#[tokio::test]
async fn should_get_github_meta_information() {
    let meta_json = r#"{
  "verifiable_password_authentication": true,
  "ssh_key_fingerprints": {
    "SHA256_RSA": "SHA256:abc"
  },
  "ssh_keys": [
    "ssh-rsa ABC"
  ],
  "hooks": [
    "192.0.2.1"
  ],
  "domains": {
    "website": [
      "*.github.com"
    ]
  }
}"#;
    let template = ResponseTemplate::new(200)
        .set_body_json(serde_json::from_str::<serde_json::Value>(meta_json).unwrap());
    let mock_server = setup_mock_http_server("GET", "/meta", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.meta().get_github_meta_information().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let meta = result.unwrap();
    assert!(meta.verifiable_password_authentication);
    assert_eq!(
        meta.ssh_key_fingerprints.unwrap().sha256_rsa.as_deref(),
        Some("SHA256:abc")
    );
    assert_eq!(meta.ssh_keys.unwrap()[0], "ssh-rsa ABC");
    assert_eq!(meta.hooks.unwrap()[0], "192.0.2.1");
    assert_eq!(meta.domains.unwrap().website.unwrap()[0], "*.github.com");
}

#[tokio::test]
async fn should_get_octocat() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/octocat"))
        .and(query_param("s", "hello"))
        .respond_with(ResponseTemplate::new(200).set_body_string("octocat art"))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.meta().get_octocat(Some("hello")).await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    assert_eq!(result.unwrap(), "octocat art");
}

#[tokio::test]
async fn should_get_api_versions() {
    let versions_json = r#"["2022-11-28"]"#;
    let template = ResponseTemplate::new(200)
        .set_body_json(serde_json::from_str::<serde_json::Value>(versions_json).unwrap());
    let mock_server = setup_mock_http_server("GET", "/versions", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.meta().get_api_versions().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let versions = result.unwrap();
    assert_eq!(versions.len(), 1);
    assert_eq!(versions[0], "2022-11-28");
}

#[tokio::test]
async fn should_get_zen() {
    let template = ResponseTemplate::new(200).set_body_string("Responsive is better than fast");
    let mock_server = setup_mock_http_server("GET", "/zen", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.meta().zen().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    assert_eq!(result.unwrap(), "Responsive is better than fast");
}
