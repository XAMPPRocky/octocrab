use http::StatusCode;
use wiremock::{
    matchers::{body_json, method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::hooks::{Config, ContentType, DeliveryDetail, Hook};
use octocrab::models::webhook_events::WebhookEventType;
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "owner";
const REPO: &str = "repo";
const HOOK_ID: u64 = 1;
const DELIVERY_ID: u64 = 42;

async fn setup_hooks_mock(
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
async fn should_list_repository_webhooks() {
    let mocked_response: Vec<Hook> =
        serde_json::from_str(include_str!("resources/repo_hooks.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_hooks_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/hooks").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .hooks()
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
    let hooks = page.items;
    assert_eq!(hooks.len(), 2);
    assert_eq!(hooks[0].id, 1);
    assert_eq!(hooks[0].name, "web");
    assert!(hooks[0].active);
    assert_eq!(hooks[1].id, 2);
    assert!(!hooks[1].active);
}

#[tokio::test]
async fn should_get_repository_webhook() {
    let mocked_response: Hook =
        serde_json::from_str(include_str!("resources/repo_hook.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_hooks_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/hooks/{HOOK_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.repos(OWNER, REPO).hooks().get(HOOK_ID).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let hook = result.unwrap();
    assert_eq!(hook.id, 1);
    assert_eq!(hook.name, "web");
    assert_eq!(
        hook.events,
        vec![WebhookEventType::Push, WebhookEventType::PullRequest]
    );
}

#[tokio::test]
async fn should_create_repository_webhook() {
    let mocked_response: Hook =
        serde_json::from_str(include_str!("resources/repo_hook.json")).unwrap();
    let template = ResponseTemplate::new(201).set_body_json(&mocked_response);
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/hooks")))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "POST on /repos/{OWNER}/{REPO}/hooks was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let hook_to_create = mocked_response.clone();
    let result = client
        .repos(OWNER, REPO)
        .hooks()
        .create(hook_to_create)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let hook = result.unwrap();
    assert_eq!(hook.id, 1);
}

#[tokio::test]
async fn should_update_repository_webhook() {
    let mocked_response: Hook =
        serde_json::from_str(include_str!("resources/repo_hook.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path(format!("/repos/{OWNER}/{REPO}/hooks/{HOOK_ID}")))
        .and(body_json(serde_json::json!({
            "active": true,
            "config": {
                "url": "https://example.com/updated-webhook",
                "content_type": "json"
            }
        })))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "PATCH on /repos/{OWNER}/{REPO}/hooks/{HOOK_ID} was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .hooks()
        .update(HOOK_ID)
        .active(true)
        .url("https://example.com/updated-webhook")
        .content_type(ContentType::Json)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_delete_repository_webhook() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_hooks_mock(
        "DELETE",
        format!("/repos/{OWNER}/{REPO}/hooks/{HOOK_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.repos(OWNER, REPO).hooks().delete(HOOK_ID).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_get_webhook_config() {
    let mocked_response: Config =
        serde_json::from_str(include_str!("resources/repo_hook_config.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_hooks_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/hooks/{HOOK_ID}/config").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.repos(OWNER, REPO).hooks().get_config(HOOK_ID).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let config = result.unwrap();
    assert_eq!(config.url, "https://example.com/webhook");
    assert_eq!(config.content_type, Some(ContentType::Json));
}

#[tokio::test]
async fn should_update_webhook_config() {
    let mocked_response: Config =
        serde_json::from_str(include_str!("resources/repo_hook_config.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/hooks/{HOOK_ID}/config"
        )))
        .and(body_json(serde_json::json!({
            "url": "https://example.com/webhook",
            "content_type": "json"
        })))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "PATCH on /repos/{OWNER}/{REPO}/hooks/{HOOK_ID}/config was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .hooks()
        .update_config(HOOK_ID)
        .url("https://example.com/webhook")
        .content_type(ContentType::Json)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_ping_repository_webhook() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_hooks_mock(
        "POST",
        format!("/repos/{OWNER}/{REPO}/hooks/{HOOK_ID}/pings").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.repos(OWNER, REPO).hooks().ping(HOOK_ID).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_test_repository_webhook() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_hooks_mock(
        "POST",
        format!("/repos/{OWNER}/{REPO}/hooks/{HOOK_ID}/tests").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.repos(OWNER, REPO).hooks().test(HOOK_ID).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_get_webhook_delivery_detail() {
    let mocked_response: DeliveryDetail =
        serde_json::from_str(include_str!("resources/repo_hook_delivery_detail.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_hooks_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/hooks/{HOOK_ID}/deliveries/{DELIVERY_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .hooks()
        .deliveries(HOOK_ID)
        .get(DELIVERY_ID)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let delivery = result.unwrap();
    assert_eq!(delivery.id, DELIVERY_ID.into());
    assert_eq!(delivery.status_code, 200);
    assert_eq!(delivery.event, "push");
    assert!(delivery.request.headers.is_some());
    assert!(delivery.response.payload.is_some());
}

#[tokio::test]
async fn should_get_webhook_delivery_detail_via_global_hooks() {
    let mocked_response: DeliveryDetail =
        serde_json::from_str(include_str!("resources/repo_hook_delivery_detail.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_hooks_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/hooks/{HOOK_ID}/deliveries/{DELIVERY_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .hooks(OWNER)
        .repo(REPO.to_string())
        .get_delivery(HOOK_ID.into(), DELIVERY_ID.into())
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let delivery = result.unwrap();
    assert_eq!(delivery.id, DELIVERY_ID.into());
}

#[tokio::test]
async fn should_redeliver_webhook_delivery() {
    let template = ResponseTemplate::new(202);
    let mock_server = setup_hooks_mock(
        "POST",
        format!("/repos/{OWNER}/{REPO}/hooks/{HOOK_ID}/deliveries/{DELIVERY_ID}/attempts").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .hooks()
        .deliveries(HOOK_ID)
        .redeliver(DELIVERY_ID)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_handle_webhook_errors() {
    let template = ResponseTemplate::new(StatusCode::NOT_FOUND.as_u16());
    let mock_server = setup_hooks_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/hooks/999").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.repos(OWNER, REPO).hooks().get(999u64).await;
    assert!(result.is_err());
}
