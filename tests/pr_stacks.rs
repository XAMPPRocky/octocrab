//! Integration tests for the Stacked Pull Requests API handler.
//!
//! See [`octocrab::repos::pr_stacks::StackedPrsHandler`].

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use octocrab::models::pr_stacks::PrStack;
use octocrab::Octocrab;

mod mock_error;

use crate::mock_error::setup_error_handler;

const OWNER: &str = "XAMPPRocky";
const REPO: &str = "octocrab";
const STACK_NUMBER: u64 = 1;

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

fn stack_fixture() -> PrStack {
    serde_json::from_str(include_str!("resources/pr_stacks.json")).unwrap()
}

/// Wraps a single PrStack in a GitHub page envelope so it can be
/// returned by the list endpoint (which is paginated).
fn page_body(stack: &PrStack) -> serde_json::Value {
    serde_json::json!([stack])
}

#[tokio::test]
async fn should_list_stacks() {
    let stack = stack_fixture();
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/stacks")))
        .respond_with(ResponseTemplate::new(200).set_body_json(page_body(&stack)))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET /repos/{OWNER}/{REPO}/stacks was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let page = client
        .repos(OWNER, REPO)
        .stacks()
        .list()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0], stack);
}

#[tokio::test]
async fn should_create_stack() {
    let stack = stack_fixture();
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/stacks")))
        .respond_with(ResponseTemplate::new(201).set_body_json(&stack))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("POST /repos/{OWNER}/{REPO}/stacks was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .stacks()
        .create(vec![101, 102])
        .await
        .unwrap();

    assert_eq!(result, stack);
}

#[tokio::test]
async fn should_get_stack() {
    let stack = stack_fixture();
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/stacks/{STACK_NUMBER}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(&stack))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET /repos/{OWNER}/{REPO}/stacks/{STACK_NUMBER} was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .stacks()
        .get(STACK_NUMBER)
        .await
        .unwrap();

    assert_eq!(result, stack);
}

#[tokio::test]
async fn should_add_to_stack() {
    let stack = stack_fixture();
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/stacks/{STACK_NUMBER}/add")))
        .respond_with(ResponseTemplate::new(200).set_body_json(&stack))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("POST /repos/{OWNER}/{REPO}/stacks/{STACK_NUMBER}/add was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .stacks()
        .add_to_stack(STACK_NUMBER, vec![103])
        .await
        .unwrap();

    assert_eq!(result, stack);
}

/// When some PRs remain after unstacking, the API returns 200 with the
/// updated stack body. `remove_from_stack` should return `Some(stack)`.
#[tokio::test]
async fn should_remove_from_stack_partial() {
    let stack = stack_fixture();
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/stacks/{STACK_NUMBER}/unstack")))
        .respond_with(ResponseTemplate::new(200).set_body_json(&stack))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("POST /repos/{OWNER}/{REPO}/stacks/{STACK_NUMBER}/unstack was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .stacks()
        .remove_from_stack(STACK_NUMBER)
        .await
        .unwrap();

    assert_eq!(result, Some(stack));
}

/// When the stack is fully dissolved, the API returns 204 No Content.
/// `remove_from_stack` should return `None`.
#[tokio::test]
async fn should_remove_from_stack_dissolved() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/stacks/{STACK_NUMBER}/unstack")))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("POST /repos/{OWNER}/{REPO}/stacks/{STACK_NUMBER}/unstack was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .stacks()
        .remove_from_stack(STACK_NUMBER)
        .await
        .unwrap();

    assert_eq!(result, None);
}
