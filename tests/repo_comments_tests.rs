use http::StatusCode;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::commits::Comment;
use octocrab::Octocrab;
use octocrab::Page;

mod mock_error;

const OWNER: &str = "owner";
const REPO: &str = "repo";
const COMMENT_ID: u64 = 1;
const COMMIT_SHA: &str = "6dcb09b5b57875f334f61aebed695e2e4193db5e";

async fn setup_comments_mock(
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
async fn should_list_commit_comments() {
    let mocked_response: Vec<Comment> =
        serde_json::from_str(include_str!("resources/repo_comments.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_comments_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/comments").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .comments()
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
    let Page { items, .. } = result.unwrap();
    assert_eq!(items.len(), 2);
    let first = &items[0];
    assert_eq!(first.id.0, 1);
    assert_eq!(first.body.as_deref(), Some("Great stuff"));
    assert_eq!(first.path.as_deref(), Some("file1.txt"));
    assert_eq!(first.position, Some(4));
    assert_eq!(first.line, Some(1));
    assert_eq!(first.user.as_ref().unwrap().login, "octocat");

    let second = &items[1];
    assert_eq!(second.id.0, 2);
    assert_eq!(second.body.as_deref(), Some("Another comment"));
    assert!(second.user.is_none());
}

#[tokio::test]
async fn should_list_commit_comments_for_specific_commit() {
    let mocked_response: Vec<Comment> =
        serde_json::from_str(include_str!("resources/repo_comments.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_comments_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/commits/{COMMIT_SHA}/comments").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .comments()
        .list()
        .commit_sha(COMMIT_SHA)
        .per_page(10)
        .page(1u32)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let Page { items, .. } = result.unwrap();
    assert_eq!(items.len(), 2);
}

#[tokio::test]
async fn should_get_commit_comment() {
    let mocked_response: Comment =
        serde_json::from_str(include_str!("resources/repo_comment.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_comments_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/comments/{COMMENT_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.repos(OWNER, REPO).comments().get(COMMENT_ID).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.id.0, 1);
    assert_eq!(response.body.as_deref(), Some("Great stuff"));
    assert_eq!(response.commit_id, COMMIT_SHA);
}

#[tokio::test]
async fn should_create_commit_comment() {
    let mocked_response: Comment =
        serde_json::from_str(include_str!("resources/repo_comment.json")).unwrap();
    let template = ResponseTemplate::new(201).set_body_json(&mocked_response);
    let mock_server = setup_comments_mock(
        "POST",
        format!("/repos/{OWNER}/{REPO}/commits/{COMMIT_SHA}/comments").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .comments()
        .create(COMMIT_SHA, "Great stuff")
        .path("file1.txt")
        .position(4u64)
        .line(1u64)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.id.0, 1);
    assert_eq!(response.body.as_deref(), Some("Great stuff"));
}

#[tokio::test]
async fn should_update_commit_comment() {
    let mocked_response: Comment =
        serde_json::from_str(include_str!("resources/repo_comment.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_comments_mock(
        "PATCH",
        format!("/repos/{OWNER}/{REPO}/comments/{COMMENT_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .comments()
        .update(COMMENT_ID, "Great stuff")
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.id.0, 1);
}

#[tokio::test]
async fn should_delete_commit_comment() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_comments_mock(
        "DELETE",
        format!("/repos/{OWNER}/{REPO}/comments/{COMMENT_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .comments()
        .delete(COMMENT_ID)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}
