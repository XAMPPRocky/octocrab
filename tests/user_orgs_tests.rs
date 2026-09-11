use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::orgs::{OrgMembership, OrgMembershipState};
use octocrab::Octocrab;

mod mock_error;

const USERNAME: &str = "octocat";
const ORG: &str = "github";

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
async fn should_list_orgs_for_authenticated_user() {
    let template = ResponseTemplate::new(200).set_body_json(serde_json::json!([]));
    let mock_server = setup_mock("GET", "/user/orgs", template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.current().list_orgs().per_page(10).send().await;
    assert!(result.is_ok(), "result: {:?}", result);
    let page = result.unwrap();
    assert_eq!(page.items.len(), 0);
}

#[tokio::test]
async fn should_get_org_membership_for_authenticated_user() {
    let org_json = serde_json::json!({
        "url": "https://api.github.com/user/memberships/orgs/github",
        "state": "active",
        "role": "admin",
        "organization_url": "https://api.github.com/orgs/github",
        "organization": {
            "login": "github",
            "id": 1,
            "node_id": "MDEyOk9yZ2FuaXphdGlvbjE=",
            "url": "https://api.github.com/orgs/github",
            "repos_url": "https://api.github.com/orgs/github/repos",
            "events_url": "https://api.github.com/orgs/github/events",
            "hooks_url": "https://api.github.com/orgs/github/hooks",
            "issues_url": "https://api.github.com/orgs/github/issues",
            "members_url": "https://api.github.com/orgs/github/members{/member}",
            "public_members_url": "https://api.github.com/orgs/github/public_members{/member}",
            "avatar_url": "https://github.com/images/error/octocat_happy.gif"
        },
        "user": {
            "login": "octocat",
            "id": 1,
            "node_id": "MDQ6VXNlcjE=",
            "avatar_url": "https://github.com/images/error/octocat_happy.gif",
            "gravatar_id": "",
            "url": "https://api.github.com/users/octocat",
            "html_url": "https://github.com/octocat",
            "followers_url": "https://api.github.com/users/octocat/followers",
            "following_url": "https://api.github.com/users/octocat/following{/other_user}",
            "gists_url": "https://api.github.com/users/octocat/gists{/gist_id}",
            "starred_url": "https://api.github.com/users/octocat/starred{/owner}{/repo}",
            "subscriptions_url": "https://api.github.com/users/octocat/subscriptions",
            "organizations_url": "https://api.github.com/users/octocat/orgs",
            "repos_url": "https://api.github.com/users/octocat/repos",
            "events_url": "https://api.github.com/users/octocat/events{/privacy}",
            "received_events_url": "https://api.github.com/users/octocat/received_events",
            "type": "User",
            "site_admin": false
        }
    });

    let template = ResponseTemplate::new(200).set_body_json(&org_json);
    let mock_server = setup_mock(
        "GET",
        format!("/user/memberships/orgs/{}", ORG).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.current().get_org_membership(ORG).await;
    assert!(result.is_ok(), "result: {:?}", result);
    let membership: OrgMembership = result.unwrap();
    assert_eq!(membership.state, OrgMembershipState::Active);
    assert_eq!(membership.role, "admin");
}

#[tokio::test]
async fn should_update_org_membership_for_authenticated_user() {
    let org_json = serde_json::json!({
        "url": "https://api.github.com/user/memberships/orgs/github",
        "state": "active",
        "role": "admin",
        "organization_url": "https://api.github.com/orgs/github",
        "organization": {
            "login": "github",
            "id": 1,
            "node_id": "MDEyOk9yZ2FuaXphdGlvbjE=",
            "url": "https://api.github.com/orgs/github",
            "repos_url": "https://api.github.com/orgs/github/repos",
            "events_url": "https://api.github.com/orgs/github/events",
            "hooks_url": "https://api.github.com/orgs/github/hooks",
            "issues_url": "https://api.github.com/orgs/github/issues",
            "members_url": "https://api.github.com/orgs/github/members{/member}",
            "public_members_url": "https://api.github.com/orgs/github/public_members{/member}",
            "avatar_url": "https://github.com/images/error/octocat_happy.gif"
        },
        "user": {
            "login": "octocat",
            "id": 1,
            "node_id": "MDQ6VXNlcjE=",
            "avatar_url": "https://github.com/images/error/octocat_happy.gif",
            "gravatar_id": "",
            "url": "https://api.github.com/users/octocat",
            "html_url": "https://github.com/octocat",
            "followers_url": "https://api.github.com/users/octocat/followers",
            "following_url": "https://api.github.com/users/octocat/following{/other_user}",
            "gists_url": "https://api.github.com/users/octocat/gists{/gist_id}",
            "starred_url": "https://api.github.com/users/octocat/starred{/owner}{/repo}",
            "subscriptions_url": "https://api.github.com/users/octocat/subscriptions",
            "organizations_url": "https://api.github.com/users/octocat/orgs",
            "repos_url": "https://api.github.com/users/octocat/repos",
            "events_url": "https://api.github.com/users/octocat/events{/privacy}",
            "received_events_url": "https://api.github.com/users/octocat/received_events",
            "type": "User",
            "site_admin": false
        }
    });

    let template = ResponseTemplate::new(200).set_body_json(&org_json);
    let mock_server = setup_mock(
        "PATCH",
        format!("/user/memberships/orgs/{}", ORG).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .current()
        .update_org_membership(ORG, OrgMembershipState::Active)
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
    let membership = result.unwrap();
    assert_eq!(membership.state, OrgMembershipState::Active);
}

#[tokio::test]
async fn should_list_orgs_for_user() {
    let template = ResponseTemplate::new(200).set_body_json(serde_json::json!([]));
    let mock_server = setup_mock(
        "GET",
        format!("/users/{}/orgs", USERNAME).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.users(USERNAME).list_orgs().send().await;
    assert!(result.is_ok(), "result: {:?}", result);
    let page = result.unwrap();
    assert_eq!(page.items.len(), 0);
}
