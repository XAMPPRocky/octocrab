use http::StatusCode;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::hooks::{Config, ContentType, Hook, UpdateHookConfig};
use octocrab::models::webhook_events::WebhookEventType;
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "owner";
const HOOK_ID: u64 = 1;

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
        &format!(
            "http method {} on {} was not received",
            http_method, mocked_path
        ),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_list_org_webhooks() {
    let mocked_response: Vec<Hook> =
        serde_json::from_str(include_str!("resources/repo_hooks.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server =
        setup_hooks_mock("GET", format!("/orgs/{}/hooks", OWNER).as_str(), template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .hooks()
        .list()
        .per_page(10)
        .page(1u32)
        .send()
        .await;

    assert!(result.is_ok(), "result: {:?}", result);
    let page = result.unwrap();
    assert_eq!(page.items.len(), 2);
}

#[tokio::test]
async fn should_get_org_webhook() {
    let mocked_response: Hook =
        serde_json::from_str(include_str!("resources/repo_hook.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_hooks_mock(
        "GET",
        format!("/orgs/{}/hooks/{}", OWNER, HOOK_ID).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.orgs(OWNER).hooks().get(HOOK_ID).await;

    assert!(result.is_ok(), "result: {:?}", result);
    assert_eq!(result.unwrap().id, HOOK_ID);
}

#[tokio::test]
async fn should_create_org_webhook() {
    let mocked_response: Hook =
        serde_json::from_str(include_str!("resources/repo_hook.json")).unwrap();
    let template = ResponseTemplate::new(201).set_body_json(&mocked_response);
    let mock_server =
        setup_hooks_mock("POST", format!("/orgs/{}/hooks", OWNER).as_str(), template).await;
    let client = setup_octocrab(&mock_server.uri());

    let hook = Hook {
        name: "web".to_string(),
        config: Config {
            url: "https://example.com/webhook".to_string(),
            content_type: Some(ContentType::Json),
            insecure_ssl: None,
            secret: None,
        },
        ..Hook::default()
    };

    let result = client.orgs(OWNER).hooks().create(hook).await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_update_org_webhook() {
    let mocked_response: Hook =
        serde_json::from_str(include_str!("resources/repo_hook.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_hooks_mock(
        "PATCH",
        format!("/orgs/{}/hooks/{}", OWNER, HOOK_ID).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .hooks()
        .update(HOOK_ID)
        .active(true)
        .events(vec![WebhookEventType::Push])
        .send()
        .await;

    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_delete_org_webhook() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_hooks_mock(
        "DELETE",
        format!("/orgs/{}/hooks/{}", OWNER, HOOK_ID).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.orgs(OWNER).hooks().delete(HOOK_ID).await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_ping_org_webhook() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_hooks_mock(
        "POST",
        format!("/orgs/{}/hooks/{}/pings", OWNER, HOOK_ID).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.orgs(OWNER).hooks().ping(HOOK_ID).await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_get_org_webhook_config() {
    let mocked_response: Config =
        serde_json::from_str(include_str!("resources/repo_hook_config.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_hooks_mock(
        "GET",
        format!("/orgs/{}/hooks/{}/config", OWNER, HOOK_ID).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.orgs(OWNER).hooks().get_config(HOOK_ID).await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_update_org_webhook_config() {
    let mocked_response: Config =
        serde_json::from_str(include_str!("resources/repo_hook_config.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_hooks_mock(
        "PATCH",
        format!("/orgs/{}/hooks/{}/config", OWNER, HOOK_ID).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let config = UpdateHookConfig {
        url: Some("https://example.com/webhook".to_string()),
        content_type: Some(ContentType::Json),
        insecure_ssl: None,
        secret: None,
    };

    let result = client
        .orgs(OWNER)
        .hooks()
        .update_config(HOOK_ID, config)
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}
