use octocrab::models::JobId;
use octocrab::Octocrab;
use wiremock::{
    matchers::{header, method, path},
    Mock, MockServer, ResponseTemplate,
};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_download_workflow_job_logs_follows_redirect() {
    let mock_server = MockServer::start().await;
    let log_url = format!("{}/logs/job-1234", mock_server.uri());
    let expected_logs = b"2026-01-01T00:00:00Z job started\n2026-01-01T00:00:01Z job finished\n";

    Mock::given(method("GET"))
        .and(path("/repos/test-owner/test-repo/actions/jobs/1234/logs"))
        .and(header("x-github-api-version", "2022-11-28"))
        .respond_with(ResponseTemplate::new(302).insert_header("Location", log_url.as_str()))
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/logs/job-1234"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(expected_logs.to_vec()))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let logs = client
        .actions()
        .download_workflow_job_logs("test-owner", "test-repo", JobId(1234))
        .await
        .unwrap();

    assert_eq!(&logs[..], expected_logs);
}

#[tokio::test]
async fn test_download_workflow_job_logs_direct_response() {
    let mock_server = MockServer::start().await;
    let expected_logs = b"direct log output\n";

    Mock::given(method("GET"))
        .and(path("/repos/octocat/hello-world/actions/jobs/5678/logs"))
        .and(header("x-github-api-version", "2022-11-28"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(expected_logs.to_vec()))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let logs = client
        .actions()
        .download_workflow_job_logs("octocat", "hello-world", 5678u64.into())
        .await
        .unwrap();

    assert_eq!(&logs[..], expected_logs);
}
