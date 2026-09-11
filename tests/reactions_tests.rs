mod mock_error;

use http::StatusCode;
use mock_error::setup_error_handler;
use octocrab::{
    models::{reactions::ReactionContent, CommentId, ReactionId, ReleaseId, TeamId},
    Octocrab,
};
use wiremock::{
    matchers::{body_json, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

fn mock_reaction(id: u64, content: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "node_id": format!("MDEzOlJlYWN0aW9u{id}"),
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
        },
        "content": content,
        "created_at": "2020-01-01T00:00:00Z"
    })
}

// ===========================================================================
// Commit Comments Reactions
// ===========================================================================

#[tokio::test]
async fn should_list_commit_comment_reactions() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let reaction_json = mock_reaction(101, "+1");
    let response_json = serde_json::json!([reaction_json]);

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/comments/42/reactions"))
        .and(query_param("content", "+1"))
        .and(query_param("per_page", "10"))
        .and(query_param("page", "1"))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&response_json))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        "GET on /repos/owner/repo/comments/42/reactions was not received",
    )
    .await;

    let page = octocrab
        .repos("owner", "repo")
        .comments()
        .list_reactions(CommentId(42))
        .content(ReactionContent::PlusOne)
        .per_page(10)
        .page(1u32)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].id, ReactionId(101));
    assert_eq!(page.items[0].content, ReactionContent::PlusOne);
}

#[tokio::test]
async fn should_create_commit_comment_reaction() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let response_json = mock_reaction(102, "heart");

    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/comments/42/reactions"))
        .and(body_json(serde_json::json!({ "content": "heart" })))
        .respond_with(ResponseTemplate::new(StatusCode::CREATED).set_body_json(&response_json))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        "POST on /repos/owner/repo/comments/42/reactions was not received",
    )
    .await;

    let reaction = octocrab
        .repos("owner", "repo")
        .comments()
        .create_reaction(CommentId(42), ReactionContent::Heart)
        .await
        .unwrap();

    assert_eq!(reaction.id, ReactionId(102));
    assert_eq!(reaction.content, ReactionContent::Heart);
}

#[tokio::test]
async fn should_delete_commit_comment_reaction() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    Mock::given(method("DELETE"))
        .and(path("/repos/owner/repo/comments/42/reactions/102"))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        "DELETE on /repos/owner/repo/comments/42/reactions/102 was not received",
    )
    .await;

    let result = octocrab
        .repos("owner", "repo")
        .comments()
        .delete_reaction(CommentId(42), ReactionId(102))
        .await;

    assert!(result.is_ok());
}

// ===========================================================================
// Pull Request Review Comments Reactions
// ===========================================================================

#[tokio::test]
async fn should_list_pull_comment_reactions() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let reaction_json = mock_reaction(201, "laugh");
    let response_json = serde_json::json!([reaction_json]);

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/pulls/comments/55/reactions"))
        .and(query_param("content", "laugh"))
        .and(query_param("per_page", "20"))
        .and(query_param("page", "2"))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&response_json))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        "GET on /repos/owner/repo/pulls/comments/55/reactions was not received",
    )
    .await;

    let page = octocrab
        .pulls("owner", "repo")
        .list_comment_reactions(CommentId(55))
        .content(ReactionContent::Laugh)
        .per_page(20)
        .page(2u32)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].id, ReactionId(201));
    assert_eq!(page.items[0].content, ReactionContent::Laugh);
}

#[tokio::test]
async fn should_create_and_delete_pull_comment_reaction_via_comment_builder() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let response_json = mock_reaction(202, "hooray");

    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/pulls/comments/55/reactions"))
        .and(body_json(serde_json::json!({ "content": "hooray" })))
        .respond_with(ResponseTemplate::new(StatusCode::CREATED).set_body_json(&response_json))
        .mount(&server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/repos/owner/repo/pulls/comments/55/reactions/202"))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        "POST or DELETE on /repos/owner/repo/pulls/comments/55/reactions was not received",
    )
    .await;

    let pulls = octocrab.pulls("owner", "repo");
    let comment_builder = pulls.comment(CommentId(55));
    let created = comment_builder
        .create_reaction(ReactionContent::Hooray)
        .await
        .unwrap();

    assert_eq!(created.id, ReactionId(202));
    assert_eq!(created.content, ReactionContent::Hooray);

    let delete_res = comment_builder.delete_reaction(ReactionId(202)).await;
    assert!(delete_res.is_ok());
}

// ===========================================================================
// Release Reactions
// ===========================================================================

#[tokio::test]
async fn should_list_release_reactions() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let reaction_json = mock_reaction(301, "rocket");
    let response_json = serde_json::json!([reaction_json]);

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/releases/77/reactions"))
        .and(query_param("content", "rocket"))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&response_json))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        "GET on /repos/owner/repo/releases/77/reactions was not received",
    )
    .await;

    let page = octocrab
        .repos("owner", "repo")
        .releases()
        .list_reactions(ReleaseId(77))
        .content(ReactionContent::Rocket)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].id, ReactionId(301));
    assert_eq!(page.items[0].content, ReactionContent::Rocket);
}

#[tokio::test]
async fn should_create_and_delete_release_reaction() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let response_json = mock_reaction(302, "eyes");

    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/releases/77/reactions"))
        .and(body_json(serde_json::json!({ "content": "eyes" })))
        .respond_with(ResponseTemplate::new(StatusCode::CREATED).set_body_json(&response_json))
        .mount(&server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/repos/owner/repo/releases/77/reactions/302"))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        "POST or DELETE on /repos/owner/repo/releases/77/reactions was not received",
    )
    .await;

    let repos = octocrab.repos("owner", "repo");
    let releases = repos.releases();

    let reaction = releases
        .create_reaction(ReleaseId(77), ReactionContent::Eyes)
        .await
        .unwrap();
    assert_eq!(reaction.id, ReactionId(302));
    assert_eq!(reaction.content, ReactionContent::Eyes);

    let delete_res = releases
        .delete_reaction(ReleaseId(77), ReactionId(302))
        .await;
    assert!(delete_res.is_ok());
}

// ===========================================================================
// Team Discussions Reactions (Deprecated)
// ===========================================================================

#[allow(deprecated)]
#[tokio::test]
async fn should_handle_org_team_discussion_reactions() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let reaction_json = mock_reaction(401, "+1");
    let response_json = serde_json::json!([reaction_json]);

    // List discussion reactions
    Mock::given(method("GET"))
        .and(path("/orgs/myorg/teams/devs/discussions/1/reactions"))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&response_json))
        .mount(&server)
        .await;

    // Create discussion reaction
    Mock::given(method("POST"))
        .and(path("/orgs/myorg/teams/devs/discussions/1/reactions"))
        .and(body_json(serde_json::json!({ "content": "+1" })))
        .respond_with(ResponseTemplate::new(StatusCode::CREATED).set_body_json(&reaction_json))
        .mount(&server)
        .await;

    // Delete discussion reaction
    Mock::given(method("DELETE"))
        .and(path("/orgs/myorg/teams/devs/discussions/1/reactions/401"))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&server)
        .await;

    // List discussion comment reactions
    Mock::given(method("GET"))
        .and(path(
            "/orgs/myorg/teams/devs/discussions/1/comments/2/reactions",
        ))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&response_json))
        .mount(&server)
        .await;

    // Create discussion comment reaction
    Mock::given(method("POST"))
        .and(path(
            "/orgs/myorg/teams/devs/discussions/1/comments/2/reactions",
        ))
        .and(body_json(serde_json::json!({ "content": "+1" })))
        .respond_with(ResponseTemplate::new(StatusCode::CREATED).set_body_json(&reaction_json))
        .mount(&server)
        .await;

    // Delete discussion comment reaction
    Mock::given(method("DELETE"))
        .and(path(
            "/orgs/myorg/teams/devs/discussions/1/comments/2/reactions/401",
        ))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&server)
        .await;

    let teams = octocrab.teams("myorg");

    let list_disc = teams
        .list_discussion_reactions("devs", 1)
        .send()
        .await
        .unwrap();
    assert_eq!(list_disc.items.len(), 1);

    let create_disc = teams
        .create_discussion_reaction("devs", 1, ReactionContent::PlusOne)
        .await
        .unwrap();
    assert_eq!(create_disc.id, ReactionId(401));

    let del_disc = teams
        .delete_discussion_reaction("devs", 1, ReactionId(401))
        .await;
    assert!(del_disc.is_ok());

    let list_comment = teams
        .list_discussion_comment_reactions("devs", 1, 2)
        .send()
        .await
        .unwrap();
    assert_eq!(list_comment.items.len(), 1);

    let create_comment = teams
        .create_discussion_comment_reaction("devs", 1, 2, ReactionContent::PlusOne)
        .await
        .unwrap();
    assert_eq!(create_comment.id, ReactionId(401));

    let del_comment = teams
        .delete_discussion_comment_reaction("devs", 1, 2, ReactionId(401))
        .await;
    assert!(del_comment.is_ok());
}

#[allow(deprecated)]
#[tokio::test]
async fn should_handle_team_by_id_discussion_reactions() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let reaction_json = mock_reaction(501, "heart");
    let response_json = serde_json::json!([reaction_json]);

    // List discussion reactions by team ID
    Mock::given(method("GET"))
        .and(path("/teams/99/discussions/1/reactions"))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&response_json))
        .mount(&server)
        .await;

    // Create discussion reaction by team ID
    Mock::given(method("POST"))
        .and(path("/teams/99/discussions/1/reactions"))
        .and(body_json(serde_json::json!({ "content": "heart" })))
        .respond_with(ResponseTemplate::new(StatusCode::CREATED).set_body_json(&reaction_json))
        .mount(&server)
        .await;

    // Delete discussion reaction by team ID
    Mock::given(method("DELETE"))
        .and(path("/teams/99/discussions/1/reactions/501"))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&server)
        .await;

    // List discussion comment reactions by team ID
    Mock::given(method("GET"))
        .and(path("/teams/99/discussions/1/comments/2/reactions"))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&response_json))
        .mount(&server)
        .await;

    // Create discussion comment reaction by team ID
    Mock::given(method("POST"))
        .and(path("/teams/99/discussions/1/comments/2/reactions"))
        .and(body_json(serde_json::json!({ "content": "heart" })))
        .respond_with(ResponseTemplate::new(StatusCode::CREATED).set_body_json(&reaction_json))
        .mount(&server)
        .await;

    // Delete discussion comment reaction by team ID
    Mock::given(method("DELETE"))
        .and(path("/teams/99/discussions/1/comments/2/reactions/501"))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&server)
        .await;

    let team_handler = octocrab.teams_by_id(TeamId(99));

    let list_disc = team_handler
        .list_discussion_reactions(1)
        .send()
        .await
        .unwrap();
    assert_eq!(list_disc.items.len(), 1);

    let create_disc = team_handler
        .create_discussion_reaction(1, ReactionContent::Heart)
        .await
        .unwrap();
    assert_eq!(create_disc.id, ReactionId(501));

    let del_disc = team_handler
        .delete_discussion_reaction(1, ReactionId(501))
        .await;
    assert!(del_disc.is_ok());

    let list_comment = team_handler
        .list_discussion_comment_reactions(1, 2)
        .send()
        .await
        .unwrap();
    assert_eq!(list_comment.items.len(), 1);

    let create_comment = team_handler
        .create_discussion_comment_reaction(1, 2, ReactionContent::Heart)
        .await
        .unwrap();
    assert_eq!(create_comment.id, ReactionId(501));

    let del_comment = team_handler
        .delete_discussion_comment_reaction(1, 2, ReactionId(501))
        .await;
    assert!(del_comment.is_ok());
}

// ===========================================================================
// Builder Serialization Tests
// ===========================================================================

#[tokio::test]
async fn test_builder_serialization() {
    let octocrab = Octocrab::default();

    let repos = octocrab.repos("owner", "repo");
    let commit_builder = repos
        .comments()
        .list_reactions(CommentId(1))
        .content(ReactionContent::PlusOne)
        .per_page(50)
        .page(2u32);

    assert_eq!(
        serde_json::to_value(commit_builder).unwrap(),
        serde_json::json!({
            "content": "+1",
            "per_page": 50,
            "page": 2,
        })
    );

    let pulls = octocrab.pulls("owner", "repo");
    let pull_builder = pulls
        .list_comment_reactions(CommentId(1))
        .content(ReactionContent::Heart)
        .per_page(30)
        .page(1u32);

    assert_eq!(
        serde_json::to_value(pull_builder).unwrap(),
        serde_json::json!({
            "content": "heart",
            "per_page": 30,
            "page": 1,
        })
    );

    let releases = repos.releases();
    let release_builder = releases
        .list_reactions(ReleaseId(1))
        .content(ReactionContent::Rocket)
        .per_page(100)
        .page(5u32);

    assert_eq!(
        serde_json::to_value(release_builder).unwrap(),
        serde_json::json!({
            "content": "rocket",
            "per_page": 100,
            "page": 5,
        })
    );

    #[allow(deprecated)]
    let teams = octocrab.teams("owner");
    #[allow(deprecated)]
    let disc_builder = teams
        .list_discussion_reactions("devs", 1)
        .content(ReactionContent::Laugh)
        .per_page(25)
        .page(3u32);

    assert_eq!(
        serde_json::to_value(disc_builder).unwrap(),
        serde_json::json!({
            "content": "laugh",
            "per_page": 25,
            "page": 3,
        })
    );
}
