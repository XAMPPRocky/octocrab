

use crate::mock_error::setup_error_handler;
use octocrab::models::apps::{
    CreateInstallationAccessToken, CreateScopedAccessToken, UpdateWebhookConfig,
};
use octocrab::models::InstallationId;
use octocrab::Octocrab;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

fn sample_app_json() -> serde_json::Value {
    json!({
        "id": 1,
        "slug": "octoapp",
        "node_id": "MDZ6QXBwMQ==",
        "owner": {
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
        "name": "Octocat App",
        "description": "The Octocat App",
        "external_url": "https://octocat.github.io",
        "html_url": "https://github.com/apps/octocat-app",
        "created_at": "2022-07-08T16:18:44Z",
        "updated_at": "2022-07-08T16:18:44Z",
        "permissions": {
            "metadata": "read",
            "contents": "read",
            "issues": "write"
        },
        "events": ["push", "pull_request"]
    })
}

fn sample_installation_json() -> serde_json::Value {
    serde_json::from_str::<serde_json::Value>(include_str!("resources/installation_event.json"))
        .unwrap()["installation"]
        .clone()
}

#[tokio::test]
async fn test_get_authenticated_app() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/app"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_app_json()))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "GET on /app failed").await;

    let client = setup_octocrab(&mock_server.uri());
    let app = client.apps().get().await.unwrap();
    assert_eq!(app.name, "Octocat App");
    assert_eq!(app.slug.as_deref(), Some("octoapp"));
}

#[tokio::test]
async fn test_create_from_manifest() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/app-manifests/test-code/conversions"))
        .respond_with(ResponseTemplate::new(201).set_body_json(sample_app_json()))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let app = client
        .apps()
        .create_from_manifest("test-code")
        .await
        .unwrap();
    assert_eq!(app.name, "Octocat App");
}

#[tokio::test]
async fn test_get_app_by_slug() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/apps/octoapp"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_app_json()))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let app = client.apps().get_app("octoapp").await.unwrap();
    assert_eq!(app.name, "Octocat App");
}

#[tokio::test]
async fn test_get_user_installation() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/octocat/installation"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_installation_json()))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let inst1 = client
        .apps()
        .get_user_installation("octocat")
        .await
        .unwrap();
    assert_eq!(inst1.id, InstallationId(7777777));

    let inst2 = client.users("octocat").installation().await.unwrap();
    assert_eq!(inst2.id, InstallationId(7777777));
}

#[tokio::test]
async fn test_get_org_installation() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/orgs/octo-org/installation"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_installation_json()))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let inst1 = client
        .apps()
        .get_org_installation("octo-org")
        .await
        .unwrap();
    assert_eq!(inst1.id, InstallationId(7777777));

    let inst2 = client.orgs("octo-org").installation().await.unwrap();
    assert_eq!(inst2.id, InstallationId(7777777));
}

#[tokio::test]
async fn test_get_repo_installation() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/repos/octocat/hello-world/installation"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_installation_json()))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let inst1 = client
        .apps()
        .get_repository_installation("octocat", "hello-world")
        .await
        .unwrap();
    assert_eq!(inst1.id, InstallationId(7777777));

    let inst2 = client
        .repos("octocat", "hello-world")
        .installation()
        .await
        .unwrap();
    assert_eq!(inst2.id, InstallationId(7777777));
}

#[tokio::test]
async fn test_installation_requests() {
    let mock_server = MockServer::start().await;
    let reqs_json = json!([
        {
            "id": 42,
            "node_id": "MDExOkluc3RhbGxhdGlvblJlcXVlc3Q0Mg==",
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
            "requester": {
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
            "request_type": "user_to_organization",
            "created_at": "2022-07-08T16:18:44Z"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/app/installation-requests"))
        .respond_with(ResponseTemplate::new(200).set_body_json(reqs_json))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let page = client
        .apps()
        .installation_requests()
        .per_page(10)
        .page(1u32)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].id.0, 42);
}

#[tokio::test]
async fn test_delete_and_suspend_installation() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/app/installations/123"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/app/installations/123/suspended"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/app/installations/123/suspended"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    client.apps().delete_installation(123).await.unwrap();
    client.apps().suspend_installation(123).await.unwrap();
    client.apps().unsuspend_installation(123).await.unwrap();
}

#[tokio::test]
async fn test_create_installation_access_token() {
    let mock_server = MockServer::start().await;
    let token_json = json!({
        "token": "ghs_test12345",
        "expires_at": "2026-09-13T16:00:00Z",
        "permissions": {
            "issues": "write"
        },
        "repository_selection": "selected"
    });

    Mock::given(method("POST"))
        .and(path("/app/installations/123/access_tokens"))
        .respond_with(ResponseTemplate::new(201).set_body_json(token_json))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let body = CreateInstallationAccessToken::default();
    let token = client
        .apps()
        .create_installation_access_token(123, &body)
        .await
        .unwrap();

    assert_eq!(token.token, "ghs_test12345");
    assert_eq!(token.repository_selection.as_deref(), Some("selected"));
}

#[tokio::test]
async fn test_installation_repositories() {
    let mock_server = MockServer::start().await;
    let repos_json = json!({
        "total_count": 0,
        "repository_selection": "all",
        "repositories": []
    });

    Mock::given(method("GET"))
        .and(path("/installation/repositories"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&repos_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/user/installations/123/repositories"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&repos_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/user/installations/123/repositories/456"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/user/installations/123/repositories/456"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let res1 = client
        .apps()
        .installation_repositories()
        .send()
        .await
        .unwrap();
    assert_eq!(res1.total_count, 0);

    let res2 = client
        .current()
        .installation_repositories(123)
        .send()
        .await
        .unwrap();
    assert_eq!(res2.total_count, 0);

    client
        .current()
        .add_repository_to_installation(123, 456)
        .await
        .unwrap();
    client
        .current()
        .remove_repository_from_installation(123, 456)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_revoke_installation_token() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/installation/token"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    client.apps().revoke_installation_token().await.unwrap();
    client.current().revoke_installation_token().await.unwrap();
}

#[tokio::test]
async fn test_app_webhook() {
    let mock_server = MockServer::start().await;
    let config_json = json!({
        "url": "https://example.com/webhook",
        "content_type": "json",
        "secret": "secret123",
        "insecure_ssl": "0"
    });

    Mock::given(method("GET"))
        .and(path("/app/hook/config"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&config_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("PATCH"))
        .and(path("/app/hook/config"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&config_json))
        .mount(&mock_server)
        .await;

    let deliveries_json = json!([
        {
            "id": 1,
            "guid": "guid-1",
            "delivered_at": "2024-07-25T11:50:32Z",
            "redelivery": false,
            "duration": 0.2,
            "status": "OK",
            "status_code": 200,
            "event": "push",
            "action": null,
            "installation_id": null,
            "repository_id": null,
            "url": "",
            "throttled_at": null
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/app/hook/deliveries"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&deliveries_json))
        .mount(&mock_server)
        .await;

    let delivery_detail_json = json!({
        "id": 1,
        "guid": "guid-1",
        "delivered_at": "2024-07-25T11:50:32Z",
        "redelivery": false,
        "duration": 0.2,
        "status": "OK",
        "status_code": 200,
        "event": "push",
        "action": null,
        "installation_id": null,
        "repository_id": null,
        "url": "",
        "request": {
            "headers": {},
            "payload": {}
        },
        "response": {
            "headers": {},
            "payload": "OK"
        }
    });

    Mock::given(method("GET"))
        .and(path("/app/hook/deliveries/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&delivery_detail_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/app/hook/deliveries/1/attempts"))
        .respond_with(ResponseTemplate::new(202))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());

    // Via webhook() handler
    let cfg = client.apps().webhook().get_config().await.unwrap();
    assert_eq!(cfg.url, "https://example.com/webhook");

    let update = UpdateWebhookConfig {
        url: Some("https://example.com/webhook".into()),
        ..Default::default()
    };
    let cfg2 = client
        .apps()
        .webhook()
        .update_config(&update)
        .await
        .unwrap();
    assert_eq!(cfg2.url, "https://example.com/webhook");

    let dels = client.apps().webhook().deliveries().send().await.unwrap();
    assert_eq!(dels.len(), 1);

    let del = client.apps().webhook().delivery(1).await.unwrap();
    assert_eq!(del.id.0, 1);

    client.apps().webhook().redeliver(1).await.unwrap();

    // Via convenience methods on AppsRequestHandler
    let _ = client.apps().webhook_config().await.unwrap();
    let _ = client.apps().update_webhook_config(&update).await.unwrap();
    let _ = client.apps().webhook_deliveries().send().await.unwrap();
    let _ = client.apps().get_webhook_delivery(1).await.unwrap();
    client.apps().redeliver_webhook_delivery(1).await.unwrap();
}

#[tokio::test]
async fn test_oauth_applications() {
    let mock_server = MockServer::start().await;
    let auth_json = json!({
        "id": 1,
        "url": "https://api.github.com/authorizations/1",
        "scopes": ["public_repo"],
        "token": "ghp_tok123",
        "token_last_eight": "tok123",
        "hashed_token": "hash123",
        "app": {
            "url": "https://api.github.com/apps/octoapp",
            "name": "OctoApp",
            "client_id": "client123"
        },
        "note": null,
        "note_url": null,
        "updated_at": "2024-01-01T00:00:00Z",
        "created_at": "2024-01-01T00:00:00Z",
        "fingerprint": null
    });

    Mock::given(method("DELETE"))
        .and(path("/applications/client123/grant"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/applications/client123/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&auth_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("PATCH"))
        .and(path("/applications/client123/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&auth_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/applications/client123/token"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/applications/client123/token/scoped"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&auth_json))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());

    // Via apps().application()
    client
        .apps()
        .application("client123")
        .delete_grant("ghp_tok123")
        .await
        .unwrap();
    let auth1 = client
        .apps()
        .application("client123")
        .check_token("ghp_tok123")
        .await
        .unwrap();
    assert_eq!(auth1.token, "ghp_tok123");

    let auth2 = client
        .apps()
        .application("client123")
        .reset_token("ghp_tok123")
        .await
        .unwrap();
    assert_eq!(auth2.token, "ghp_tok123");

    client
        .apps()
        .application("client123")
        .delete_token("ghp_tok123")
        .await
        .unwrap();

    let scoped_body = CreateScopedAccessToken {
        access_token: "ghp_tok123".into(),
        target: Some("octocat".into()),
        target_id: None,
        repositories: None,
        repository_ids: None,
        permissions: None,
    };
    let auth3 = client
        .apps()
        .application("client123")
        .scoped_token(&scoped_body)
        .await
        .unwrap();
    assert_eq!(auth3.token, "ghp_tok123");

    // Via Octocrab::applications
    let auth4 = client
        .applications("client123")
        .check_token("ghp_tok123")
        .await
        .unwrap();
    assert_eq!(auth4.token, "ghp_tok123");
}
