mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{
    models::{
        repos::{InvitationPermission, RepositoryInvitation},
        InvitationId,
    },
    Octocrab,
};
use wiremock::{
    matchers::{body_json, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

const OWNER: &str = "owner";
const REPO: &str = "repo";

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_list_repo_invitations() {
    let mock_server = MockServer::start().await;
    let mocked_response: Vec<RepositoryInvitation> =
        serde_json::from_str(include_str!("resources/repo_invitations.json")).unwrap();

    let template = ResponseTemplate::new(200)
        .set_body_json(&mocked_response)
        .append_header(
            "Link",
            "<https://api.github.com/repositories/123/invitations?page=2>; rel=\"next\"",
        );

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/invitations")))
        .and(query_param("per_page", "10"))
        .and(query_param("page", "1"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/invitations was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .invitations()
        .list()
        .per_page(10)
        .page(1u32)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let page = result.unwrap();
    assert_eq!(page.items.len(), 2);

    let first = &page.items[0];
    assert_eq!(first.id, InvitationId::from(1u64));
    assert_eq!(first.permissions, InvitationPermission::Write);
    assert_eq!(first.repository.name, "Hello-World");
    assert!(first.invitee.is_some());
    assert_eq!(first.invitee.as_ref().unwrap().login, "hubot");
    assert_eq!(first.inviter.as_ref().unwrap().login, "octocat");
    assert_eq!(first.expired, Some(false));

    let second = &page.items[1];
    assert_eq!(second.id, InvitationId::from(2u64));
    assert_eq!(second.permissions, InvitationPermission::Read);
    assert!(second.invitee.is_none());
    assert_eq!(second.expired, Some(true));

    assert!(page.next.is_some());
}

#[tokio::test]
async fn should_update_repo_invitation() {
    let mock_server = MockServer::start().await;
    let mocked_response: RepositoryInvitation =
        serde_json::from_str(include_str!("resources/repo_invitation.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);

    Mock::given(method("PATCH"))
        .and(path(format!("/repos/{OWNER}/{REPO}/invitations/1")))
        .and(body_json(serde_json::json!({ "permissions": "admin" })))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("PATCH on /repos/{OWNER}/{REPO}/invitations/1 was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .invitations()
        .update(1u64)
        .permissions(InvitationPermission::Admin)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let invitation = result.unwrap();
    assert_eq!(invitation.id, InvitationId::from(1u64));
}

#[tokio::test]
async fn should_delete_repo_invitation() {
    let mock_server = MockServer::start().await;
    let template = ResponseTemplate::new(204);

    Mock::given(method("DELETE"))
        .and(path(format!("/repos/{OWNER}/{REPO}/invitations/1")))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("DELETE on /repos/{OWNER}/{REPO}/invitations/1 was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).invitations().delete(1u64).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_list_user_repository_invitations() {
    let mock_server = MockServer::start().await;
    let mocked_response: Vec<RepositoryInvitation> =
        serde_json::from_str(include_str!("resources/repo_invitations.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);

    Mock::given(method("GET"))
        .and(path("/user/repository_invitations"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        "GET on /user/repository_invitations was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .current()
        .repository_invitations()
        .list()
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let page = result.unwrap();
    assert_eq!(page.items.len(), 2);
}

#[tokio::test]
async fn should_accept_user_repository_invitation() {
    let mock_server = MockServer::start().await;
    let template = ResponseTemplate::new(204);

    Mock::given(method("PATCH"))
        .and(path("/user/repository_invitations/1"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        "PATCH on /user/repository_invitations/1 was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.current().accept_repository_invitation(1u64).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_decline_user_repository_invitation() {
    let mock_server = MockServer::start().await;
    let template = ResponseTemplate::new(204);

    Mock::given(method("DELETE"))
        .and(path("/user/repository_invitations/1"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        "DELETE on /user/repository_invitations/1 was not received",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .current()
        .repository_invitations()
        .decline(1u64)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}
