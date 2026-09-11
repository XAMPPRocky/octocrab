use http::StatusCode;
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "owner";
const USERNAME: &str = "octocat";

async fn setup_mock(
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
async fn should_list_public_members() {
    let template = ResponseTemplate::new(200).set_body_json(json!([]));
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/public_members", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .list_public_members()
        .per_page(10)
        .send()
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_check_public_membership() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/public_members/{}", OWNER, USERNAME).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let is_member = client.orgs(OWNER).check_public_membership(USERNAME).await;
    assert!(is_member.is_ok(), "result: {:?}", is_member);
    assert!(is_member.unwrap());
}

#[tokio::test]
async fn should_publicize_membership() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "PUT",
        format!("/orgs/{}/public_members/{}", OWNER, USERNAME).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.orgs(OWNER).publicize_membership(USERNAME).await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_conceal_membership() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "DELETE",
        format!("/orgs/{}/public_members/{}", OWNER, USERNAME).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.orgs(OWNER).conceal_membership(USERNAME).await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_list_outside_collaborators() {
    let template = ResponseTemplate::new(200).set_body_json(json!([]));
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/outside_collaborators", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .list_outside_collaborators()
        .filter("all")
        .send()
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_convert_to_outside_collaborator() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "PUT",
        format!("/orgs/{}/outside_collaborators/{}", OWNER, USERNAME).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .convert_to_outside_collaborator(USERNAME)
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_remove_outside_collaborator() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "DELETE",
        format!("/orgs/{}/outside_collaborators/{}", OWNER, USERNAME).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .remove_outside_collaborator(USERNAME)
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_list_blocked_users() {
    let template = ResponseTemplate::new(200).set_body_json(json!([]));
    let mock_server = setup_mock("GET", format!("/orgs/{}/blocks", OWNER).as_str(), template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.orgs(OWNER).list_blocked_users().send().await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_check_blocked_user() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/blocks/{}", OWNER, USERNAME).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let is_blocked = client.orgs(OWNER).check_blocked_user(USERNAME).await;
    assert!(is_blocked.is_ok(), "result: {:?}", is_blocked);
    assert!(is_blocked.unwrap());
}

#[tokio::test]
async fn should_block_user() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "PUT",
        format!("/orgs/{}/blocks/{}", OWNER, USERNAME).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.orgs(OWNER).block_user(USERNAME).await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_unblock_user() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "DELETE",
        format!("/orgs/{}/blocks/{}", OWNER, USERNAME).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.orgs(OWNER).unblock_user(USERNAME).await;
    assert!(result.is_ok(), "result: {:?}", result);
}
