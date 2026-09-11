mod mock_error;

use mock_error::setup_error_handler;
use octocrab::models::Repository;
use octocrab::Octocrab;
use serde_json::json;
use wiremock::{
    matchers::{header, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_check_starred_repository() {
    let mock_server = MockServer::start().await;

    // Starred -> 204
    Mock::given(method("GET"))
        .and(path("/user/starred/owner/starred-repo"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    // Not starred -> 404
    Mock::given(method("GET"))
        .and(path("/user/starred/owner/unstarred-repo"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());

    // Via activity().starring()
    assert!(client
        .activity()
        .starring()
        .check("owner", "starred-repo")
        .await
        .unwrap());
    assert!(!client
        .activity()
        .starring()
        .check("owner", "unstarred-repo")
        .await
        .unwrap());

    // Via current()
    assert!(client.current().is_starred("owner", "starred-repo").await.unwrap());
    assert!(!client.current().is_starred("owner", "unstarred-repo").await.unwrap());

    // Via repos()
    assert!(client.repos("owner", "starred-repo").is_starred().await.unwrap());
    assert!(!client.repos("owner", "unstarred-repo").is_starred().await.unwrap());
}

#[tokio::test]
async fn test_star_and_unstar_repository() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/user/starred/owner/repo"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/user/starred/owner/repo"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());

    // Via activity().starring()
    client.activity().starring().star("owner", "repo").await.unwrap();
    client.activity().starring().unstar("owner", "repo").await.unwrap();

    // Via current()
    client.current().star_repo("owner", "repo").await.unwrap();
    client.current().unstar_repo("owner", "repo").await.unwrap();

    // Via repos()
    client.repos("owner", "repo").star().await.unwrap();
    client.repos("owner", "repo").unstar().await.unwrap();
}

#[tokio::test]
async fn test_list_repos_starred_by_user() {
    let mock_server = MockServer::start().await;
    let repos: Vec<Repository> =
        serde_json::from_str(include_str!("resources/user_repositories.json")).unwrap();

    let expected = json!([
        {
            "starred_at": "2020-07-09T00:17:55Z",
            "repo": repos[0]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/users/octocat/starred"))
        .and(query_param("sort", "created"))
        .and(query_param("direction", "desc"))
        .and(query_param("per_page", "30"))
        .and(query_param("page", "1"))
        .and(header("Accept", "application/vnd.github.star+json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    setup_error_handler(&mock_server, "GET /users/octocat/starred not matched").await;

    let client = setup_octocrab(&mock_server.uri());

    // Test via activity().starring()
    let page = client
        .activity()
        .starring()
        .list_repos_starred_by_user("octocat")
        .sort("created")
        .direction("desc")
        .per_page(30u8)
        .page(1u8)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].repo.name, repos[0].name);

    // Test via users()
    let page2 = client
        .users("octocat")
        .starred()
        .sort("created")
        .direction("desc")
        .per_page(30u8)
        .page(1u8)
        .send()
        .await
        .unwrap();

    assert_eq!(page2.items.len(), 1);
    assert_eq!(page2.items[0].repo.name, repos[0].name);
}
