use crate::mock_error::setup_error_handler;
use octocrab::{models::copilot::CopilotSeat, Octocrab};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

const ORG: &str = "org";
const USERNAME: &str = "octocat";

async fn setup_user_seat_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/orgs/{ORG}/members/{USERNAME}/copilot")))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /orgs/{ORG}/members/{USERNAME}/copilot was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_return_user_seat_info() {
    let seat: CopilotSeat =
        serde_json::from_str(include_str!("../resources/org_copilot_user_seat.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&seat);
    let mock_server = setup_user_seat_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());

    // Test OrgHandler::copilot().seat_assignment(...)
    let result = org.copilot().seat_assignment(USERNAME).await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let r = result.unwrap();
    assert_eq!(r.assignee.unwrap().login, "octocat");
    assert_eq!(r.plan_type.as_deref(), Some("business"));
    assert!(r.updated_at.is_some());
    assert!(r.last_authenticated_at.is_some());
    assert_eq!(r.assigning_team.unwrap().name, "Justice League");

    // Test OrgHandler::copilot().get_user_seat(...)
    let result_alias = org.copilot().get_user_seat(USERNAME).await;
    assert!(result_alias.is_ok());

    // Test CopilotSeatHandler::get_user(...)
    let result_manage = org.copilot().manage_seats().get_user(USERNAME).await;
    assert!(result_manage.is_ok());

    // Test CopilotSeatHandler::seat_assignment(...)
    let result_manage_alias = org.copilot().manage_seats().seat_assignment(USERNAME).await;
    assert!(result_manage_alias.is_ok());

    // Test Octocrab::copilot().org(...).seat_assignment(...)
    let result_top_level = client.copilot().org(ORG).seat_assignment(USERNAME).await;
    assert!(result_top_level.is_ok());
}

#[tokio::test]
async fn org_check_copilot_user_seat_401() {
    let template = ResponseTemplate::new(401);
    let mock_server = setup_user_seat_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.copilot().seat_assignment(USERNAME).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn org_check_copilot_user_seat_403() {
    let template = ResponseTemplate::new(403);
    let mock_server = setup_user_seat_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.copilot().seat_assignment(USERNAME).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn org_check_copilot_user_seat_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_user_seat_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.copilot().seat_assignment(USERNAME).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn org_check_copilot_user_seat_422() {
    let template = ResponseTemplate::new(422);
    let mock_server = setup_user_seat_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.copilot().seat_assignment(USERNAME).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn org_check_copilot_user_seat_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_user_seat_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.copilot().seat_assignment(USERNAME).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}
