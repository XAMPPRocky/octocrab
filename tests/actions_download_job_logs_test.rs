use octocrab::Octocrab;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_download_job_logs_api(template: ResponseTemplate) -> MockServer {
    let owner: &str = "org";
    let repo: &str = "some-repo";
    let job_id: u64 = 456;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{owner}/{repo}/actions/jobs/{job_id}/logs"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const OWNER: &str = "org";
const REPO: &str = "some-repo";
const JOB_ID: u64 = 456;

#[tokio::test]
async fn should_download_job_logs() {
    let logs = b"job logs";
    let template = ResponseTemplate::new(200).set_body_bytes(logs);
    let mock_server = setup_download_job_logs_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let actions = client.actions();

    let result = actions
        .download_job_logs(&OWNER.to_owned(), &REPO.to_owned(), JOB_ID.into())
        .await;

    assert_eq!(logs.as_slice(), result.unwrap());
}

#[tokio::test]
async fn should_download_job_logs_from_location() {
    let logs = b"job logs from location";
    let mock_server = MockServer::start().await;
    let location = format!("{}/download/job-logs", mock_server.uri());

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/actions/jobs/{JOB_ID}/logs"
        )))
        .respond_with(ResponseTemplate::new(302).append_header("location", location))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/download/job-logs"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(logs))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let actions = client.actions();

    let result = actions
        .download_job_logs(&OWNER.to_owned(), &REPO.to_owned(), JOB_ID.into())
        .await;

    assert_eq!(logs.as_slice(), result.unwrap());
}
