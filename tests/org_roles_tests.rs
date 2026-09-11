use http::StatusCode;
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::orgs::roles::{OrgFineGrainedPermission, OrgRole, OrgRolesResponse};
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "owner";
const ROLE_ID: u64 = 1;

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

fn sample_role() -> OrgRole {
    serde_json::from_value(json!({
        "id": ROLE_ID,
        "name": "developer",
        "description": "Dev role",
        "permissions": ["read"]
    }))
    .unwrap()
}

fn sample_roles_response() -> OrgRolesResponse {
    serde_json::from_value(json!({
        "total_count": 1,
        "roles": [
            {
                "id": ROLE_ID,
                "name": "developer",
                "description": "Dev role",
                "permissions": ["read"]
            }
        ]
    }))
    .unwrap()
}

#[tokio::test]
async fn should_list_roles() {
    let response = sample_roles_response();
    let template = ResponseTemplate::new(200).set_body_json(&response);
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/organization-roles", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.orgs(OWNER).roles().list().await;
    assert!(result.is_ok(), "result: {:?}", result);
    let roles = result.unwrap();
    assert_eq!(roles.total_count, 1);
    assert_eq!(roles.roles[0].name, "developer");
}

#[tokio::test]
async fn should_create_role() {
    let role = sample_role();
    let template = ResponseTemplate::new(201).set_body_json(&role);
    let mock_server = setup_mock(
        "POST",
        format!("/orgs/{}/organization-roles", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .roles()
        .create("developer", Some("Dev role".to_string()), vec!["read"])
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_get_role() {
    let role = sample_role();
    let template = ResponseTemplate::new(200).set_body_json(&role);
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/organization-roles/{}", OWNER, ROLE_ID).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.orgs(OWNER).roles().get(ROLE_ID).await;
    assert!(result.is_ok(), "result: {:?}", result);
    assert_eq!(result.unwrap().id.0, ROLE_ID);
}

#[tokio::test]
async fn should_update_role() {
    let role = sample_role();
    let template = ResponseTemplate::new(200).set_body_json(&role);
    let mock_server = setup_mock(
        "PATCH",
        format!("/orgs/{}/organization-roles/{}", OWNER, ROLE_ID).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .roles()
        .update(ROLE_ID, Some("new_name".to_string()), None, None)
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_delete_role() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "DELETE",
        format!("/orgs/{}/organization-roles/{}", OWNER, ROLE_ID).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.orgs(OWNER).roles().delete(ROLE_ID).await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_list_teams_for_role() {
    let template = ResponseTemplate::new(200).set_body_json(json!([]));
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/organization-roles/{}/teams", OWNER, ROLE_ID).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .roles()
        .list_teams_for_role(ROLE_ID)
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_list_users_for_role() {
    let template = ResponseTemplate::new(200).set_body_json(json!([]));
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/organization-roles/{}/users", OWNER, ROLE_ID).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .roles()
        .list_users_for_role(ROLE_ID)
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_list_roles_for_team() {
    let response = sample_roles_response();
    let template = ResponseTemplate::new(200).set_body_json(&response);
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/organization-roles/teams/justice-league", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .roles()
        .list_roles_for_team("justice-league")
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_assign_to_team() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "PUT",
        format!(
            "/orgs/{}/organization-roles/teams/justice-league/{}",
            OWNER, ROLE_ID
        )
        .as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .roles()
        .assign_to_team("justice-league", ROLE_ID)
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_remove_from_team() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "DELETE",
        format!(
            "/orgs/{}/organization-roles/teams/justice-league/{}",
            OWNER, ROLE_ID
        )
        .as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .roles()
        .remove_from_team("justice-league", ROLE_ID)
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_list_roles_for_user() {
    let response = sample_roles_response();
    let template = ResponseTemplate::new(200).set_body_json(&response);
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/organization-roles/users/octocat", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .roles()
        .list_roles_for_user("octocat")
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_assign_to_user() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "PUT",
        format!(
            "/orgs/{}/organization-roles/users/octocat/{}",
            OWNER, ROLE_ID
        )
        .as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .roles()
        .assign_to_user("octocat", ROLE_ID)
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_remove_from_user() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "DELETE",
        format!(
            "/orgs/{}/organization-roles/users/octocat/{}",
            OWNER, ROLE_ID
        )
        .as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .roles()
        .remove_from_user("octocat", ROLE_ID)
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_list_fine_grained_permissions() {
    let permissions: Vec<OrgFineGrainedPermission> = serde_json::from_value(json!([
        {
            "name": "read_code",
            "description": "Can read code"
        }
    ]))
    .unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&permissions);
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/organization-fine-grained-permissions", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .roles()
        .list_fine_grained_permissions()
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
    let list = result.unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].name, "read_code");
}
