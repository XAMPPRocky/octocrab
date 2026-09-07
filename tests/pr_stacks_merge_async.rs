//! Integration tests for the asynchronous pull request merge API.
//!
//! These tests require the `stack-prs` feature.
//! Run with: `cargo test --features stack-prs --test pr_stacks_merge_async`

#[cfg(feature = "stack-prs")]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use octocrab::models::pr_stacks::{AsyncMergeDetails, AsyncMergeResult, AsyncMergeStatus};
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
            AsyncMergeDetails::Pending { uuid, .. } => {
                assert_eq!(uuid, UUID);
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
}
