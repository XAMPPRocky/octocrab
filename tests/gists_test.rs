

use crate::mock_error::setup_error_handler;
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

fn sample_gist_json(gist_id: &str, description: Option<&str>) -> serde_json::Value {
    serde_json::json!({
        "url": format!("https://api.github.com/gists/{gist_id}"),
        "forks_url": format!("https://api.github.com/gists/{gist_id}/forks"),
        "commits_url": format!("https://api.github.com/gists/{gist_id}/commits"),
        "id": gist_id,
        "node_id": "MDQ6R2lzdDEyYzU1YTk0YmQwMzE2NmZmMzNlZDA1OTYyNjNiNGM2",
        "git_pull_url": format!("https://gist.github.com/{gist_id}.git"),
        "git_push_url": format!("https://gist.github.com/{gist_id}.git"),
        "html_url": format!("https://gist.github.com/{gist_id}"),
        "files": {
            "hello_world.rs": {
                "filename": "hello_world.rs",
                "type": "text/plain",
                "language": "Rust",
                "raw_url": format!("https://gist.githubusercontent.com/raw/{gist_id}/hello_world.rs"),
                "size": 100
            }
        },
        "public": true,
        "created_at": "2010-04-14T02:15:15Z",
        "updated_at": "2011-06-20T11:34:56Z",
        "description": description,
        "comments": 0,
        "comments_url": format!("https://api.github.com/gists/{gist_id}/comments")
    })
}

async fn setup_update_gist_api(
    gist_id: &str,
    expected_body: serde_json::Value,
    template: ResponseTemplate,
) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path(format!("/gists/{gist_id}")))
        .and(wiremock::matchers::body_json(expected_body))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("PATCH on /gists/{gist_id} was not received"),
    )
    .await;
    mock_server
}

async fn setup_create_gist_api(
    expected_body: serde_json::Value,
    template: ResponseTemplate,
) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/gists"))
        .and(wiremock::matchers::body_json(expected_body))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(&mock_server, "POST on /gists was not received").await;
    mock_server
}

#[tokio::test]
async fn test_update_gist_files_batch() {
    let gist_response = sample_gist_json(GIST_ID, Some("updated gist"));
    let expected_body = serde_json::json!({
        "files": {
            "file1.rs": { "content": "fn main() { 1; }" },
            "file2.rs": { "content": "fn main() { 2; }" }
        }
    });

    let template = ResponseTemplate::new(200).set_body_json(&gist_response);
    let mock_server = setup_update_gist_api(GIST_ID, expected_body, template).await;
    let client = setup_octocrab(&mock_server.uri());

    let files = vec![
        ("file1.rs", "fn main() { 1; }"),
        ("file2.rs", "fn main() { 2; }"),
    ];

    let result = client.gists().update(GIST_ID).files(files).send().await;

    assert!(
        result.is_ok(),
        "expected successful result, got: {:#?}",
        result
    );
    let gist = result.unwrap();
    assert_eq!(gist.id, GIST_ID);
}

#[tokio::test]
async fn test_update_gist_files_in_loop_with_build() {
    let gist_response = sample_gist_json(GIST_ID, None);
    let expected_body = serde_json::json!({
        "files": {
            "a.txt": { "content": "alpha" },
            "b.txt": { "content": "beta" }
        }
    });

    let template = ResponseTemplate::new(200).set_body_json(&gist_response);
    let mock_server = setup_update_gist_api(GIST_ID, expected_body, template).await;
    let client = setup_octocrab(&mock_server.uri());

    let updates = vec![("a.txt", "alpha"), ("b.txt", "beta")];
    let mut builder = client.gists().update(GIST_ID);
    for (filename, content) in updates {
        builder = builder.file(filename).with_content(content).build();
    }

    let result = builder.send().await;
    assert!(
        result.is_ok(),
        "expected successful result, got: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_update_gist_files_mixed_operations() {
    let gist_response = sample_gist_json(GIST_ID, Some("new description"));
    let expected_body = serde_json::json!({
        "description": "new description",
        "files": {
            "old_name.rs": {
                "filename": "new_name.rs",
                "content": "renamed and modified"
            },
            "delete_me.rs": null
        }
    });

    let template = ResponseTemplate::new(200).set_body_json(&gist_response);
    let mock_server = setup_update_gist_api(GIST_ID, expected_body, template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .gists()
        .update(GIST_ID)
        .description("new description")
        .file("old_name.rs")
        .rename_to("new_name.rs")
        .with_content("renamed and modified")
        .build()
        .file("delete_me.rs")
        .delete()
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_update_gist_files_on_file_builder() {
    let gist_response = sample_gist_json(GIST_ID, None);
    let expected_body = serde_json::json!({
        "files": {
            "first.rs": { "content": "first content" },
            "second.rs": { "content": "second content" }
        }
    });

    let template = ResponseTemplate::new(200).set_body_json(&gist_response);
    let mock_server = setup_update_gist_api(GIST_ID, expected_body, template).await;
    let client = setup_octocrab(&mock_server.uri());

    // Call .files(...) directly on an UpdateGistFileBuilder
    let result = client
        .gists()
        .update(GIST_ID)
        .file("first.rs")
        .with_content("first content")
        .files([("second.rs", "second content")])
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_create_gist_files_batch() {
    let gist_response = sample_gist_json(GIST_ID, Some("batch create"));
    let expected_body = serde_json::json!({
        "description": "batch create",
        "public": false,
        "files": {
            "f1.rs": { "content": "code 1" },
            "f2.rs": { "content": "code 2" }
        }
    });

    let template = ResponseTemplate::new(201).set_body_json(&gist_response);
    let mock_server = setup_create_gist_api(expected_body, template).await;
    let client = setup_octocrab(&mock_server.uri());

    let files = vec![("f1.rs", "code 1"), ("f2.rs", "code 2")];
    let result = client
        .gists()
        .create()
        .description("batch create")
        .public(false)
        .files(files)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_update_gist_empty_files() {
    let gist_response = sample_gist_json(GIST_ID, None);
    let expected_body = serde_json::json!({});

    let template = ResponseTemplate::new(200).set_body_json(&gist_response);
    let mock_server = setup_update_gist_api(GIST_ID, expected_body, template).await;
    let client = setup_octocrab(&mock_server.uri());

    let empty_files: Vec<(&str, &str)> = Vec::new();
    let result = client
        .gists()
        .update(GIST_ID)
        .files(empty_files)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got: {:#?}",
        result
    );
}
