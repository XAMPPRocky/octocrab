use http::StatusCode;
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::orgs::invitations::{FailedOrgInvitation, OrgInvitation};
use octocrab::models::teams::Team;
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "owner";
const INVITATION_ID: u64 = 42;

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

fn sample_invitation() -> OrgInvitation {
    serde_json::from_value(json!({
        "id": INVITATION_ID,
        "login": "mona",
        "email": "mona@github.com",
        "role": "direct_member",
        "created_at": "2022-11-28T00:00:00Z",
        "team_count": 1
    }))
    .unwrap()
}

#[tokio::test]
async fn should_list_invitations() {
    let template = ResponseTemplate::new(200).set_body_json(vec![sample_invitation()]);
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/invitations", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .invitations()
        .list()
        .role("direct_member")
        .per_page(10)
        .page(1u32)
        .send()
        .await;

    assert!(result.is_ok(), "result: {:?}", result);
    let page = result.unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].login, Some("mona".to_string()));
}

#[tokio::test]
async fn should_create_invitation() {
    let template = ResponseTemplate::new(201).set_body_json(sample_invitation());
    let mock_server = setup_mock(
        "POST",
        format!("/orgs/{}/invitations", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .invitations()
        .create()
        .email("mona@github.com")
        .role("direct_member")
        .send()
        .await;

    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_cancel_invitation() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "DELETE",
        format!("/orgs/{}/invitations/{}", OWNER, INVITATION_ID).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.orgs(OWNER).invitations().cancel(INVITATION_ID).await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_list_invitation_teams() {
    let template = ResponseTemplate::new(200).set_body_json(Vec::<Team>::new());
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/invitations/{}/teams", OWNER, INVITATION_ID).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .invitations()
        .list_teams(INVITATION_ID)
        .send()
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_list_failed_invitations() {
    let failed: FailedOrgInvitation = serde_json::from_value(json!({
        "id": INVITATION_ID,
        "login": "mona",
        "email": null,
        "role": "direct_member",
        "created_at": "2022-11-28T00:00:00Z",
        "failed_at": "2022-11-28T00:01:00Z",
        "failed_reason": "email invalid"
    }))
    .unwrap();

    let template = ResponseTemplate::new(200).set_body_json(vec![failed]);
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/failed_invitations", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.orgs(OWNER).invitations().list_failed().send().await;
    assert!(result.is_ok(), "result: {:?}", result);
    let page = result.unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(
        page.items[0].failed_reason,
        Some("email invalid".to_string())
    );
}
