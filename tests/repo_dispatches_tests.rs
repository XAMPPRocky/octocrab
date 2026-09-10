mod mock_error;

use mock_error::setup_error_handler;
use octocrab::Octocrab;
use serde::Serialize;
use serde_json::json;
use wiremock::{
    matchers::{body_json, method, path},
    Mock, MockServer, ResponseTemplate,
};

const OWNER: &str = "owner";
const REPO: &str = "repo";

async fn setup_post_api(template: ResponseTemplate, request: impl Serialize) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/dispatches")))
        .and(body_json(request))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("POST on /repos/{OWNER}/{REPO}/dispatches was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_create_repository_dispatch_with_payload() {
    let template = ResponseTemplate::new(204);
    let payload = json!({
        "unit": false,
        "integration": true
    });
    let mock_server = setup_post_api(
        template,
        json!({
            "event_type": "on-demand-test",
            "client_payload": payload,
        }),
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .create_dispatch("on-demand-test")
        .client_payload(payload)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_create_repository_dispatch_without_payload() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_post_api(
        template,
        json!({
            "event_type": "on-demand-test",
        }),
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .dispatches()
        .create("on-demand-test")
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_handle_error_response() {
    let template = ResponseTemplate::new(422).set_body_json(json!({
        "message": "Validation Failed",
        "documentation_url": "https://docs.github.com/rest/repos/repos#create-a-repository-dispatch-event"
    }));
    let mock_server = setup_post_api(
        template,
        json!({
            "event_type": "on-demand-test",
        }),
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .create_dispatch("on-demand-test")
        .send()
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}
