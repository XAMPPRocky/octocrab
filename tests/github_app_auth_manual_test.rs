use jsonwebtoken::EncodingKey;
use octocrab::models::{AppId, InstallationToken};
use octocrab::params::apps::CreateInstallationAccessToken;
use octocrab::service::middleware::retry::RetryConfig;
use octocrab::Octocrab;
use serde_json::json;
use wiremock::{
    matchers::{body_json, header_regex, method, path},
    Mock, MockServer, ResponseTemplate,
};

fn test_private_key() -> EncodingKey {
    // This is a dummy test key and is never used as a real credential.
    EncodingKey::from_rsa_pem(include_bytes!("resources/test_app_private_key.pem")).unwrap()
}

fn installation_json(access_tokens_url: &str) -> serde_json::Value {
    json!([{
        "id": 1,
        "account": {
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
        },
        "access_tokens_url": access_tokens_url,
        "permissions": {},
        "events": []
    }])
}

#[tokio::test]
async fn github_app_manual_auth_sends_authorization_to_base_uri_authority() {
    let mock_server = MockServer::start().await;
    let access_tokens_url = format!("{}/app/installations/1/access_tokens", mock_server.uri());

    Mock::given(method("GET"))
        .and(path("/app/installations"))
        .and(header_regex("authorization", r"^Bearer .+$"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(installation_json(&access_tokens_url)),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/app/installations/1/access_tokens"))
        .and(header_regex("authorization", r"^Bearer .+$"))
        .and(body_json(json!({
            "repositories": ["octocrab"],
            "repository_ids": []
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "token": "ghs_test_installation_token",
            "expires_at": "2099-01-01T00:00:00Z",
            "permissions": {}
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let octocrab = Octocrab::builder()
        .add_retry_config(RetryConfig::Simple(3))
        .app(AppId(42), test_private_key())
        .base_uri(mock_server.uri())
        .unwrap()
        .build()
        .unwrap();

    let installations = octocrab
        .apps()
        .installations()
        .send()
        .await
        .unwrap()
        .take_items();

    assert_eq!(installations.len(), 1);
    let url = installations[0].access_tokens_url.as_ref().unwrap();
    assert_eq!(url, &access_tokens_url);

    let mut create_access_token = CreateInstallationAccessToken::default();
    create_access_token.repositories = vec!["octocrab".to_string()];

    let access: InstallationToken = octocrab
        .post(url, Some(&create_access_token))
        .await
        .unwrap();

    assert_eq!(access.token, "ghs_test_installation_token");
}

#[tokio::test]
async fn execute_allows_upload_uri_and_withholds_auth_from_third_party_authority() {
    let base_server = MockServer::start().await;
    let upload_server = MockServer::start().await;
    let third_party_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/uploads"))
        .and(header_regex("authorization", r"^Bearer .+$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&upload_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/external"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&third_party_server)
        .await;

    let octocrab = Octocrab::builder()
        .add_retry_config(RetryConfig::Simple(1))
        .app(AppId(42), test_private_key())
        .base_uri(base_server.uri())
        .unwrap()
        .upload_uri(upload_server.uri())
        .unwrap()
        .build()
        .unwrap();

    let upload_res: serde_json::Value = octocrab
        .post(format!("{}/uploads", upload_server.uri()), None::<&()>)
        .await
        .unwrap();
    assert_eq!(upload_res, json!({}));

    let external_res: serde_json::Value = octocrab
        .get(
            format!("{}/external", third_party_server.uri()),
            None::<&()>,
        )
        .await
        .unwrap();
    assert_eq!(external_res, json!({}));

    let received = third_party_server.received_requests().await.unwrap();
    assert_eq!(received.len(), 1);
    assert!(
        !received[0]
            .headers
            .contains_key(&http::header::AUTHORIZATION),
        "Authorization header must not be sent to third-party authority"
    );
}
