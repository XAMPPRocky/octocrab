//! Integration tests for the asynchronous pull request merge API.

use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use octocrab::models::pr_stacks::{
    AsyncMergeAction, AsyncMergeDetails, AsyncMergeMethod, AsyncMergeResult, AsyncMergeStatus,
};
use octocrab::Octocrab;

const OWNER: &str = "XAMPPRocky";
const REPO: &str = "octocrab";
const PULL_NUMBER: u64 = 42;
const UUID: &str = "a1b2c3d4-e5f6-7890-abcd-ef1234567890";

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

fn pending_fixture() -> AsyncMergeResult {
    serde_json::from_str(include_str!("resources/pr_stacks_merge_async.json")).unwrap()
}

fn merged_fixture() -> AsyncMergeResult {
    serde_json::from_str(include_str!("resources/pr_stacks_merge_async_merged.json")).unwrap()
}

#[tokio::test]
async fn should_submit_merge_async_and_get_pending() {
    let fixture = pending_fixture();
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/pulls/{PULL_NUMBER}/merge-async"
        )))
        .respond_with(ResponseTemplate::new(202).set_body_json(&fixture))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .pulls(OWNER, REPO)
        .merge_async(PULL_NUMBER)
        .send()
        .await
        .unwrap();

    assert_eq!(result.status, AsyncMergeStatus::Pending);
    match &result.details {
        AsyncMergeDetails::Pending {
            uuid, bypass_rules, ..
        } => {
            assert_eq!(uuid, UUID);
            assert!(!bypass_rules);
        }
        other => panic!("Expected Pending details, got {:?}", other),
    }
}

#[tokio::test]
async fn should_submit_merge_async_with_bypass_rules() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "status": "pending",
        "details": {
            "message": "Merge in progress",
            "uuid": UUID,
            "merge_method": "squash",
            "merge_action": "direct_merge",
            "expected_head_sha": "abc123def456",
            "bypass_rules": true
        }
    });

    Mock::given(method("PUT"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/pulls/{PULL_NUMBER}/merge-async"
        )))
        .and(body_json(serde_json::json!({
            "merge_method": "squash",
            "merge_action": "direct_merge",
            "bypass_rules": true
        })))
        .respond_with(ResponseTemplate::new(202).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .pulls(OWNER, REPO)
        .merge_async(PULL_NUMBER)
        .merge_method(AsyncMergeMethod::Squash)
        .merge_action(AsyncMergeAction::DirectMerge)
        .bypass_rules(true)
        .send()
        .await
        .unwrap();

    assert_eq!(result.status, AsyncMergeStatus::Pending);
    match &result.details {
        AsyncMergeDetails::Pending {
            uuid, bypass_rules, ..
        } => {
            assert_eq!(uuid, UUID);
            assert!(bypass_rules);
        }
        other => panic!("Expected Pending details, got {:?}", other),
    }
}

#[tokio::test]
async fn should_submit_merge_async_already_merged() {
    let fixture = merged_fixture();
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/pulls/{PULL_NUMBER}/merge-async"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(&fixture))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .pulls(OWNER, REPO)
        .merge_async(PULL_NUMBER)
        .send()
        .await
        .unwrap();

    assert_eq!(result.status, AsyncMergeStatus::Merged);
    match &result.details {
        AsyncMergeDetails::Merged { sha, .. } => {
            assert_eq!(sha, "deadbeef1234567890abcdef");
        }
        other => panic!("Expected Merged details, got {:?}", other),
    }
}

#[tokio::test]
async fn should_get_merge_async_result() {
    let fixture = pending_fixture();
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/pulls/{PULL_NUMBER}/merge-async/{UUID}"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(&fixture))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .pulls(OWNER, REPO)
        .get_merge_async_result(PULL_NUMBER, UUID)
        .await
        .unwrap();

    assert_eq!(result, fixture);
}

#[tokio::test]
async fn should_deserialize_enqueued_and_failed_results() {
    let enqueued_json = serde_json::json!({
        "status": "enqueued",
        "details": {
            "message": "Pull request was added to the merge queue"
        }
    });

    let result: AsyncMergeResult = serde_json::from_value(enqueued_json).unwrap();
    assert_eq!(result.status, AsyncMergeStatus::Enqueued);
    match &result.details {
        AsyncMergeDetails::Failed { message } => {
            assert_eq!(message, "Pull request was added to the merge queue");
        }
        other => panic!("Expected message details, got {:?}", other),
    }

    let failed_json = serde_json::json!({
        "status": "failed",
        "details": {
            "message": "Branch protection rules violated"
        }
    });

    let result: AsyncMergeResult = serde_json::from_value(failed_json).unwrap();
    assert_eq!(result.status, AsyncMergeStatus::Failed);
    match &result.details {
        AsyncMergeDetails::Failed { message } => {
            assert_eq!(message, "Branch protection rules violated");
        }
        other => panic!("Expected message details, got {:?}", other),
    }
}
