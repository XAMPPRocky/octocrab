mod mock_error;

use mock_error::setup_error_handler;
use octocrab::models::AuthorAssociation;
use octocrab::Octocrab;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_get_api(template: ResponseTemplate) -> MockServer {
    let gist_id: &str = "12c55a94bd03166ff33ed0596263b4c6";

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/gists/{gist_id}/star")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /gists/{gist_id}/star was not received"),
    )
    .await;
    mock_server
}

async fn setup_delete_star_api(template: ResponseTemplate) -> MockServer {
    let gist_id: &str = "12c55a94bd03166ff33ed0596263b4c6";

    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!("/gists/{gist_id}/star")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("DELETE on /gists/{gist_id}/star was not received"),
    )
    .await;
    mock_server
}

async fn setup_delete_gist_api(template: ResponseTemplate) -> MockServer {
    let gist_id: &str = "12c55a94bd03166ff33ed0596263b4c6";

    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!("/gists/{gist_id}")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("DELETE on /gists/{gist_id} was not received"),
    )
    .await;
    mock_server
}

async fn setup_put_api(template: ResponseTemplate) -> MockServer {
    let gist_id: &str = "12c55a94bd03166ff33ed0596263b4c6";

    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path(format!("/gists/{gist_id}/star")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("PUT on /gists/{gist_id}/star was not received"),
    )
    .await;
    mock_server
}

async fn setup_list_comments_api(template: ResponseTemplate) -> MockServer {
    let gist_id: &str = "12c55a94bd03166ff33ed0596263b4c6";

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/gists/{gist_id}/comments")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /gists/{gist_id}/comments was not received"),
    )
    .await;
    mock_server
}

async fn setup_create_comment_api(template: ResponseTemplate) -> MockServer {
    let gist_id: &str = "12c55a94bd03166ff33ed0596263b4c6";

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!("/gists/{gist_id}/comments")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("POST on /gists/{gist_id}/comments was not received"),
    )
    .await;
    mock_server
}

async fn setup_get_comment_api(template: ResponseTemplate) -> MockServer {
    let gist_id: &str = "12c55a94bd03166ff33ed0596263b4c6";
    let comment_id: u64 = 1;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/gists/{gist_id}/comments/{comment_id}")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /gists/{gist_id}/comments/{comment_id} was not received"),
    )
    .await;
    mock_server
}

async fn setup_update_comment_api(template: ResponseTemplate) -> MockServer {
    let gist_id: &str = "12c55a94bd03166ff33ed0596263b4c6";
    let comment_id: u64 = 1;

    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path(format!("/gists/{gist_id}/comments/{comment_id}")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("PATCH on /gists/{gist_id}/comments/{comment_id} was not received"),
    )
    .await;
    mock_server
}

async fn setup_delete_comment_api(template: ResponseTemplate) -> MockServer {
    let gist_id: &str = "12c55a94bd03166ff33ed0596263b4c6";
    let comment_id: u64 = 1;

    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!("/gists/{gist_id}/comments/{comment_id}")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("DELETE on /gists/{gist_id}/comments/{comment_id} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const GIST_ID: &str = "12c55a94bd03166ff33ed0596263b4c6";
const COMMENT_ID: u64 = 1;

fn sample_user_json() -> serde_json::Value {
    serde_json::json!({
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
    })
}

fn sample_comment_json(comment_id: u64, body: &str) -> serde_json::Value {
    serde_json::json!({
        "id": comment_id,
        "node_id": "MDExOkdpc3RDb21tZW50MQ==",
        "url": format!("https://api.github.com/gists/{GIST_ID}/comments/{comment_id}"),
        "body": body,
        "user": sample_user_json(),
        "created_at": "2011-04-18T23:23:56Z",
        "updated_at": "2011-04-18T23:23:56Z",
        "author_association": "COLLABORATOR"
    })
}

#[tokio::test]
async fn test_get_gists_star_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_get_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().is_starred(GIST_ID.to_owned()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let result = result.unwrap();
    assert!(result, "expected the result to be true: {}", result);
}

#[tokio::test]
async fn test_get_gists_star_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_get_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().is_starred(GIST_ID.to_owned()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let result = result.unwrap();
    assert!(!result, "expected the result to be false: {}", result);
}

#[tokio::test]
async fn test_get_gists_star_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_get_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().is_starred(GIST_ID.to_owned()).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_put_gists_star_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_put_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().star(GIST_ID.to_owned()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_put_gists_star_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_put_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().star(GIST_ID.to_owned()).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_put_gists_star_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_put_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().star(GIST_ID.to_owned()).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gists_star_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_delete_star_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().unstar(GIST_ID.to_owned()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gists_star_304() {
    let template = ResponseTemplate::new(304);
    let mock_server = setup_delete_star_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().unstar(GIST_ID.to_owned()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gists_star_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_delete_star_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().unstar(GIST_ID.to_owned()).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gists_star_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_delete_star_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().unstar(GIST_ID.to_owned()).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gist_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_delete_gist_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().delete(GIST_ID.to_owned()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gist_304() {
    let template = ResponseTemplate::new(304);
    let mock_server = setup_delete_gist_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().delete(GIST_ID.to_owned()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gist_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_delete_gist_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().delete(GIST_ID.to_owned()).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gist_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_delete_gist_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().delete(GIST_ID.to_owned()).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_list_gist_comments_200() {
    let template = ResponseTemplate::new(200).set_body_json(vec![sample_comment_json(
        COMMENT_ID,
        "Just commenting for the sake of commenting",
    )]);
    let mock_server = setup_list_comments_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().comments_for(GIST_ID).list_comments().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let page = result.unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].id.into_inner(), COMMENT_ID);
    assert_eq!(
        page.items[0].body,
        "Just commenting for the sake of commenting"
    );
    assert_eq!(
        page.items[0].author_association,
        AuthorAssociation::Collaborator
    );
}

#[tokio::test]
async fn test_list_gist_comments_403() {
    let template = ResponseTemplate::new(403);
    let mock_server = setup_list_comments_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().comments_for(GIST_ID).list_comments().await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_list_gist_comments_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_list_comments_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().comments_for(GIST_ID).list_comments().await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_create_gist_comment_201() {
    let template = ResponseTemplate::new(201).set_body_json(sample_comment_json(
        COMMENT_ID,
        "This is a comment to a gist",
    ));
    let mock_server = setup_create_comment_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .gists()
        .comments_for(GIST_ID)
        .create_comment("This is a comment to a gist")
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let comment = result.unwrap();
    assert_eq!(comment.id.into_inner(), COMMENT_ID);
    assert_eq!(comment.body, "This is a comment to a gist");
}

#[tokio::test]
async fn test_create_gist_comment_403() {
    let template = ResponseTemplate::new(403);
    let mock_server = setup_create_comment_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .gists()
        .comments_for(GIST_ID)
        .create_comment("This is a comment to a gist")
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_create_gist_comment_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_create_comment_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .gists()
        .comments_for(GIST_ID)
        .create_comment("This is a comment to a gist")
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_get_gist_comment_200() {
    let template = ResponseTemplate::new(200).set_body_json(sample_comment_json(
        COMMENT_ID,
        "Just commenting for the sake of commenting",
    ));
    let mock_server = setup_get_comment_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .gists()
        .comments_for(GIST_ID)
        .get_comment(COMMENT_ID)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let comment = result.unwrap();
    assert_eq!(comment.id.into_inner(), COMMENT_ID);
}

#[tokio::test]
async fn test_get_gist_comment_403() {
    let template = ResponseTemplate::new(403);
    let mock_server = setup_get_comment_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .gists()
        .comments_for(GIST_ID)
        .get_comment(COMMENT_ID)
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_get_gist_comment_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_get_comment_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .gists()
        .comments_for(GIST_ID)
        .get_comment(COMMENT_ID)
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_update_gist_comment_200() {
    let template = ResponseTemplate::new(200).set_body_json(sample_comment_json(
        COMMENT_ID,
        "This is an update to a comment in a gist",
    ));
    let mock_server = setup_update_comment_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .gists()
        .comments_for(GIST_ID)
        .update_comment(COMMENT_ID, "This is an update to a comment in a gist")
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let comment = result.unwrap();
    assert_eq!(comment.body, "This is an update to a comment in a gist");
}

#[tokio::test]
async fn test_update_gist_comment_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_update_comment_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .gists()
        .comments_for(GIST_ID)
        .update_comment(COMMENT_ID, "This is an update to a comment in a gist")
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gist_comment_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_delete_comment_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .gists()
        .comments_for(GIST_ID)
        .delete_comment(COMMENT_ID)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gist_comment_304() {
    let template = ResponseTemplate::new(304);
    let mock_server = setup_delete_comment_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .gists()
        .comments_for(GIST_ID)
        .delete_comment(COMMENT_ID)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gist_comment_403() {
    let template = ResponseTemplate::new(403);
    let mock_server = setup_delete_comment_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .gists()
        .comments_for(GIST_ID)
        .delete_comment(COMMENT_ID)
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gist_comment_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_delete_comment_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .gists()
        .comments_for(GIST_ID)
        .delete_comment(COMMENT_ID)
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}
