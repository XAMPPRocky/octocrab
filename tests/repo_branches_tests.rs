use http::StatusCode;
use wiremock::{
    matchers::{body_json, method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::apps::App;
use octocrab::models::repos::branches::{
    AdminEnforcement, BranchProtection, BranchProtectionRestrictions, DetailedBranch,
    ProtectionFlag, RequiredPullRequestReviews, RequiredStatusChecks, UpdateStatusChecks,
};
use octocrab::models::teams::Team;
use octocrab::models::Author;
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "owner";
const REPO: &str = "repo";
const BRANCH: &str = "main";

async fn setup_branches_mock(
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
        &format!("http method {http_method} on {mocked_path} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_get_branch() {
    let mocked_response: DetailedBranch =
        serde_json::from_str(include_str!("resources/repo_branch.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = setup_branches_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/branches/{BRANCH}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.repos(OWNER, REPO).branches().get(BRANCH).await;

    assert!(result.is_ok(), "error: {:#?}", result.err());
    let branch = result.unwrap();
    assert_eq!(branch.name, "main");
    assert!(branch.protected);
    assert_eq!(
        branch.commit.sha,
        "7fd1a60b01f91b314f59955a4e4d4e80d8edf11d"
    );
}

#[tokio::test]
async fn should_rename_branch() {
    let mocked_response: DetailedBranch =
        serde_json::from_str(include_str!("resources/repo_branch.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::CREATED).set_body_json(&mocked_response);
    let mock_server = MockServer::start().await;

    let expected_body = serde_json::json!({
        "new_name": "main"
    });

    Mock::given(method("POST"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/branches/master/rename"
        )))
        .and(body_json(&expected_body))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "POST on /branches/{branch}/rename failed").await;

    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .branches()
        .rename("master", "main")
        .await;

    assert!(result.is_ok(), "error: {:#?}", result.err());
    let branch = result.unwrap();
    assert_eq!(branch.name, "main");
}

#[tokio::test]
async fn should_get_branch_protection() {
    let mocked_response: BranchProtection =
        serde_json::from_str(include_str!("resources/repo_branch_protection.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = setup_branches_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/branches/{BRANCH}/protection").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .branches()
        .protection(BRANCH)
        .get()
        .await;

    assert!(result.is_ok(), "error: {:#?}", result.err());
    let protection = result.unwrap();
    assert!(protection.enforce_admins.unwrap().enabled);
    assert!(protection.required_linear_history.unwrap().enabled);
    assert_eq!(
        protection
            .required_pull_request_reviews
            .unwrap()
            .required_approving_review_count,
        Some(2)
    );
}

#[tokio::test]
async fn should_update_branch_protection() {
    let mocked_response: BranchProtection =
        serde_json::from_str(include_str!("resources/repo_branch_protection.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = MockServer::start().await;

    let expected_body = serde_json::json!({
        "required_status_checks": {
            "strict": true,
            "contexts": ["continuous-integration/travis-ci"]
        },
        "enforce_admins": true,
        "required_pull_request_reviews": null,
        "restrictions": null,
        "required_linear_history": true
    });

    Mock::given(method("PUT"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/branches/{BRANCH}/protection"
        )))
        .and(body_json(&expected_body))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "PUT on /branches/{branch}/protection failed").await;

    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .branches()
        .protection(BRANCH)
        .update()
        .required_status_checks(Some(UpdateStatusChecks {
            strict: true,
            contexts: vec!["continuous-integration/travis-ci".to_string()],
            checks: None,
        }))
        .enforce_admins(Some(true))
        .required_linear_history(true)
        .send()
        .await;

    assert!(result.is_ok(), "error: {:#?}", result.err());
}

#[tokio::test]
async fn should_delete_branch_protection() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_branches_mock(
        "DELETE",
        format!("/repos/{OWNER}/{REPO}/branches/{BRANCH}/protection").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .branches()
        .protection(BRANCH)
        .delete()
        .await;

    assert!(result.is_ok(), "error: {:#?}", result.err());
}

#[tokio::test]
async fn should_manage_admin_enforcement() {
    let mocked_response: AdminEnforcement =
        serde_json::from_str(include_str!("resources/repo_branch_admin_enforcement.json")).unwrap();

    let client_server = MockServer::start().await;
    let base_path = format!("/repos/{OWNER}/{REPO}/branches/{BRANCH}/protection/enforce_admins");

    // 1. GET
    Mock::given(method("GET"))
        .and(path(&base_path))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response))
        .mount(&client_server)
        .await;

    // 2. POST (set)
    Mock::given(method("POST"))
        .and(path(&base_path))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response))
        .mount(&client_server)
        .await;

    // 3. DELETE
    Mock::given(method("DELETE"))
        .and(path(&base_path))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&client_server)
        .await;

    let client = setup_octocrab(&client_server.uri());
    let repos = client.repos(OWNER, REPO);
    let handler = repos.branches().protection(BRANCH).enforce_admins();

    let get_res = handler.get().await;
    assert!(get_res.is_ok());
    assert!(get_res.unwrap().enabled);

    let set_res = handler.set().await;
    assert!(set_res.is_ok());
    assert!(set_res.unwrap().enabled);

    let del_res = handler.delete().await;
    assert!(del_res.is_ok());
}

#[tokio::test]
async fn should_manage_required_pr_reviews() {
    let mocked_response: RequiredPullRequestReviews =
        serde_json::from_str(include_str!("resources/repo_branch_pr_reviews.json")).unwrap();

    let client_server = MockServer::start().await;
    let base_path =
        format!("/repos/{OWNER}/{REPO}/branches/{BRANCH}/protection/required_pull_request_reviews");

    // 1. GET
    Mock::given(method("GET"))
        .and(path(&base_path))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response))
        .mount(&client_server)
        .await;

    // 2. PATCH
    let expected_patch = serde_json::json!({
        "dismiss_stale_reviews": true,
        "required_approving_review_count": 2
    });
    Mock::given(method("PATCH"))
        .and(path(&base_path))
        .and(body_json(&expected_patch))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response))
        .mount(&client_server)
        .await;

    // 3. DELETE
    Mock::given(method("DELETE"))
        .and(path(&base_path))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&client_server)
        .await;

    let client = setup_octocrab(&client_server.uri());
    let repos = client.repos(OWNER, REPO);
    let handler = repos.branches().protection(BRANCH).pull_request_reviews();

    let get_res = handler.get().await;
    assert!(get_res.is_ok());

    let patch_res = handler
        .update()
        .dismiss_stale_reviews(true)
        .required_approving_review_count(2)
        .send()
        .await;
    assert!(patch_res.is_ok());

    let del_res = handler.delete().await;
    assert!(del_res.is_ok());
}

#[tokio::test]
async fn should_manage_required_signatures() {
    let mocked_response: ProtectionFlag =
        serde_json::from_str(include_str!("resources/repo_branch_signatures.json")).unwrap();

    let client_server = MockServer::start().await;
    let base_path =
        format!("/repos/{OWNER}/{REPO}/branches/{BRANCH}/protection/required_signatures");

    // 1. GET
    Mock::given(method("GET"))
        .and(path(&base_path))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response))
        .mount(&client_server)
        .await;

    // 2. POST
    Mock::given(method("POST"))
        .and(path(&base_path))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response))
        .mount(&client_server)
        .await;

    // 3. DELETE
    Mock::given(method("DELETE"))
        .and(path(&base_path))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&client_server)
        .await;

    let client = setup_octocrab(&client_server.uri());
    let repos = client.repos(OWNER, REPO);
    let handler = repos.branches().protection(BRANCH).signatures();

    let get_res = handler.get().await;
    assert!(get_res.is_ok());
    assert!(get_res.unwrap().enabled);

    let create_res = handler.create().await;
    assert!(create_res.is_ok());
    assert!(create_res.unwrap().enabled);

    let del_res = handler.delete().await;
    assert!(del_res.is_ok());
}

#[tokio::test]
async fn should_manage_required_status_checks() {
    let mocked_response: RequiredStatusChecks =
        serde_json::from_str(include_str!("resources/repo_branch_status_checks.json")).unwrap();

    let client_server = MockServer::start().await;
    let base_path =
        format!("/repos/{OWNER}/{REPO}/branches/{BRANCH}/protection/required_status_checks");

    // 1. GET
    Mock::given(method("GET"))
        .and(path(&base_path))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response))
        .mount(&client_server)
        .await;

    // 2. PATCH
    let expected_patch = serde_json::json!({
        "strict": true,
        "contexts": ["continuous-integration/travis-ci"]
    });
    Mock::given(method("PATCH"))
        .and(path(&base_path))
        .and(body_json(&expected_patch))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response))
        .mount(&client_server)
        .await;

    // 3. DELETE
    Mock::given(method("DELETE"))
        .and(path(&base_path))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&client_server)
        .await;

    let client = setup_octocrab(&client_server.uri());
    let repos = client.repos(OWNER, REPO);
    let handler = repos.branches().protection(BRANCH).status_checks();

    let get_res = handler.get().await;
    assert!(get_res.is_ok());
    assert!(get_res.unwrap().strict);

    let patch_res = handler
        .update()
        .strict(true)
        .contexts(vec!["continuous-integration/travis-ci".to_string()])
        .send()
        .await;
    assert!(patch_res.is_ok());

    let del_res = handler.remove().await;
    assert!(del_res.is_ok());
}

#[tokio::test]
async fn should_manage_status_check_contexts() {
    let client_server = MockServer::start().await;
    let base_path = format!(
        "/repos/{OWNER}/{REPO}/branches/{BRANCH}/protection/required_status_checks/contexts"
    );

    let contexts = vec!["continuous-integration/travis-ci".to_string()];

    // 1. GET
    Mock::given(method("GET"))
        .and(path(&base_path))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&contexts))
        .mount(&client_server)
        .await;

    // 2. POST (add)
    let body_req = serde_json::json!({ "contexts": ["continuous-integration/travis-ci"] });
    Mock::given(method("POST"))
        .and(path(&base_path))
        .and(body_json(&body_req))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&contexts))
        .mount(&client_server)
        .await;

    // 3. PUT (set)
    Mock::given(method("PUT"))
        .and(path(&base_path))
        .and(body_json(&body_req))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&contexts))
        .mount(&client_server)
        .await;

    // 4. DELETE (remove)
    Mock::given(method("DELETE"))
        .and(path(&base_path))
        .and(body_json(&body_req))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&contexts))
        .mount(&client_server)
        .await;

    let client = setup_octocrab(&client_server.uri());
    let repos = client.repos(OWNER, REPO);
    let handler = repos
        .branches()
        .protection(BRANCH)
        .status_checks()
        .contexts();

    let get_res = handler.get().await;
    assert!(get_res.is_ok());
    assert_eq!(get_res.unwrap().len(), 1);

    let add_res = handler
        .add(vec!["continuous-integration/travis-ci".to_string()])
        .await;
    assert!(add_res.is_ok());

    let set_res = handler
        .set(vec!["continuous-integration/travis-ci".to_string()])
        .await;
    assert!(set_res.is_ok());

    let remove_res = handler
        .remove(vec!["continuous-integration/travis-ci".to_string()])
        .await;
    assert!(remove_res.is_ok());
}

#[tokio::test]
async fn should_manage_restrictions() {
    let mocked_response: BranchProtectionRestrictions =
        serde_json::from_str(include_str!("resources/repo_branch_restrictions.json")).unwrap();

    let client_server = MockServer::start().await;
    let base_path = format!("/repos/{OWNER}/{REPO}/branches/{BRANCH}/protection/restrictions");

    // 1. GET
    Mock::given(method("GET"))
        .and(path(&base_path))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response))
        .mount(&client_server)
        .await;

    // 2. DELETE
    Mock::given(method("DELETE"))
        .and(path(&base_path))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&client_server)
        .await;

    let client = setup_octocrab(&client_server.uri());
    let repos = client.repos(OWNER, REPO);
    let handler = repos.branches().protection(BRANCH).restrictions();

    let get_res = handler.get().await;
    assert!(get_res.is_ok());
    let r = get_res.unwrap();
    assert_eq!(r.users.len(), 1);
    assert_eq!(r.teams.len(), 1);
    assert_eq!(r.apps.len(), 1);

    let del_res = handler.delete().await;
    assert!(del_res.is_ok());
}

#[tokio::test]
async fn should_manage_user_restrictions() {
    let client_server = MockServer::start().await;
    let base_path =
        format!("/repos/{OWNER}/{REPO}/branches/{BRANCH}/protection/restrictions/users");

    let users_resp: Vec<Author> = serde_json::from_value(serde_json::json!([
        {
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
    ]))
    .unwrap();

    let body_req = serde_json::json!({ "users": ["octocat"] });

    // 1. GET
    Mock::given(method("GET"))
        .and(path(&base_path))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&users_resp))
        .mount(&client_server)
        .await;

    // 2. POST
    Mock::given(method("POST"))
        .and(path(&base_path))
        .and(body_json(&body_req))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&users_resp))
        .mount(&client_server)
        .await;

    // 3. PUT
    Mock::given(method("PUT"))
        .and(path(&base_path))
        .and(body_json(&body_req))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&users_resp))
        .mount(&client_server)
        .await;

    // 4. DELETE
    Mock::given(method("DELETE"))
        .and(path(&base_path))
        .and(body_json(&body_req))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&users_resp))
        .mount(&client_server)
        .await;

    let client = setup_octocrab(&client_server.uri());
    let repos = client.repos(OWNER, REPO);
    let handler = repos.branches().protection(BRANCH).restrictions().users();

    let list_res = handler.list().await;
    assert!(list_res.is_ok());
    assert_eq!(list_res.unwrap().len(), 1);

    let add_res = handler.add(vec!["octocat".to_string()]).await;
    assert!(add_res.is_ok());

    let set_res = handler.set(vec!["octocat".to_string()]).await;
    assert!(set_res.is_ok());

    let remove_res = handler.remove(vec!["octocat".to_string()]).await;
    assert!(remove_res.is_ok());
}

#[tokio::test]
async fn should_manage_team_restrictions() {
    let client_server = MockServer::start().await;
    let base_path =
        format!("/repos/{OWNER}/{REPO}/branches/{BRANCH}/protection/restrictions/teams");

    let teams_resp: Vec<Team> = serde_json::from_value(serde_json::json!([
        {
            "id": 1,
            "node_id": "MDQ6VGVhbTE=",
            "url": "https://api.github.com/teams/1",
            "html_url": "https://github.com/orgs/github/teams/justice-league",
            "name": "Justice League",
            "slug": "justice-league",
            "description": "A great team.",
            "privacy": "closed",
            "notification_setting": "notifications_enabled",
            "permission": "admin",
            "members_url": "https://api.github.com/teams/1/members{/member}",
            "repositories_url": "https://api.github.com/teams/1/repos"
        }
    ]))
    .unwrap();

    let body_req = serde_json::json!({ "teams": ["justice-league"] });

    // 1. GET
    Mock::given(method("GET"))
        .and(path(&base_path))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&teams_resp))
        .mount(&client_server)
        .await;

    // 2. POST
    Mock::given(method("POST"))
        .and(path(&base_path))
        .and(body_json(&body_req))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&teams_resp))
        .mount(&client_server)
        .await;

    // 3. PUT
    Mock::given(method("PUT"))
        .and(path(&base_path))
        .and(body_json(&body_req))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&teams_resp))
        .mount(&client_server)
        .await;

    // 4. DELETE
    Mock::given(method("DELETE"))
        .and(path(&base_path))
        .and(body_json(&body_req))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&teams_resp))
        .mount(&client_server)
        .await;

    let client = setup_octocrab(&client_server.uri());
    let repos = client.repos(OWNER, REPO);
    let handler = repos.branches().protection(BRANCH).restrictions().teams();

    let list_res = handler.list().await;
    assert!(list_res.is_ok());
    assert_eq!(list_res.unwrap().len(), 1);

    let add_res = handler.add(vec!["justice-league".to_string()]).await;
    assert!(add_res.is_ok());

    let set_res = handler.set(vec!["justice-league".to_string()]).await;
    assert!(set_res.is_ok());

    let remove_res = handler.remove(vec!["justice-league".to_string()]).await;
    assert!(remove_res.is_ok());
}

#[tokio::test]
async fn should_manage_app_restrictions() {
    let client_server = MockServer::start().await;
    let base_path = format!("/repos/{OWNER}/{REPO}/branches/{BRANCH}/protection/restrictions/apps");

    let apps_resp: Vec<App> = serde_json::from_value(serde_json::json!([
        {
            "id": 1,
            "slug": "octoapp",
            "node_id": "MDExOkludGVncmF0aW9uMQ==",
            "owner": {
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
            },
            "name": "Octocat App",
            "description": "",
            "external_url": "https://example.com",
            "html_url": "https://github.com/apps/octoapp",
            "created_at": "2017-07-08T16:18:44-04:00",
            "updated_at": "2017-07-08T16:18:44-04:00",
            "permissions": {},
            "events": []
        }
    ]))
    .unwrap();

    let body_req = serde_json::json!({ "apps": ["octoapp"] });

    // 1. GET
    Mock::given(method("GET"))
        .and(path(&base_path))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&apps_resp))
        .mount(&client_server)
        .await;

    // 2. POST
    Mock::given(method("POST"))
        .and(path(&base_path))
        .and(body_json(&body_req))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&apps_resp))
        .mount(&client_server)
        .await;

    // 3. PUT
    Mock::given(method("PUT"))
        .and(path(&base_path))
        .and(body_json(&body_req))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&apps_resp))
        .mount(&client_server)
        .await;

    // 4. DELETE
    Mock::given(method("DELETE"))
        .and(path(&base_path))
        .and(body_json(&body_req))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&apps_resp))
        .mount(&client_server)
        .await;

    let client = setup_octocrab(&client_server.uri());
    let repos = client.repos(OWNER, REPO);
    let handler = repos.branches().protection(BRANCH).restrictions().apps();

    let list_res = handler.list().await;
    assert!(list_res.is_ok());
    assert_eq!(list_res.unwrap().len(), 1);

    let add_res = handler.add(vec!["octoapp".to_string()]).await;
    assert!(add_res.is_ok());

    let set_res = handler.set(vec!["octoapp".to_string()]).await;
    assert!(set_res.is_ok());

    let remove_res = handler.remove(vec!["octoapp".to_string()]).await;
    assert!(remove_res.is_ok());
}
