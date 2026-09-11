mod mock_error;

use mock_error::setup_error_handler;
use octocrab::models::Repository;
use octocrab::Octocrab;
use serde_json::json;
use wiremock::{
    matchers::{body_json, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

fn author_json(login: &str, id: u64) -> serde_json::Value {
    json!({
        "login": login,
        "id": id,
        "node_id": format!("MDQ6VXNlcj{id}"),
        "avatar_url": "https://github.com/images/error/octocat_happy.gif",
        "gravatar_id": "",
        "url": format!("https://api.github.com/users/{login}"),
        "html_url": format!("https://github.com/{login}"),
        "followers_url": format!("https://api.github.com/users/{login}/followers"),
        "following_url": format!("https://api.github.com/users/{login}/following{{/other_user}}"),
        "gists_url": format!("https://api.github.com/users/{login}/gists{{/gist_id}}"),
        "starred_url": format!("https://api.github.com/users/{login}/starred{{/owner}}{{/repo}}"),
        "subscriptions_url": format!("https://api.github.com/users/{login}/subscriptions"),
        "organizations_url": format!("https://api.github.com/users/{login}/orgs"),
        "repos_url": format!("https://api.github.com/users/{login}/repos"),
        "events_url": format!("https://api.github.com/users/{login}/events{{/privacy}}"),
        "received_events_url": format!("https://api.github.com/users/{login}/received_events"),
        "type": "User",
        "site_admin": false
    })
}

fn subscription_json(subscribed: bool, ignored: bool) -> serde_json::Value {
    json!({
        "subscribed": subscribed,
        "ignored": ignored,
        "reason": null,
        "created_at": "2012-10-06T21:34:12Z",
        "url": "https://api.github.com/repos/owner/repo/subscription",
        "repository_url": "https://api.github.com/repos/owner/repo"
    })
}

#[tokio::test]
async fn test_list_watchers() {
    let mock_server = MockServer::start().await;
    let expected = vec![author_json("octocat", 1)];

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/subscribers"))
        .and(query_param("per_page", "30"))
        .and(query_param("page", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    setup_error_handler(&mock_server, "GET /repos/owner/repo/subscribers not matched").await;

    let client = setup_octocrab(&mock_server.uri());

    // Test via activity().watching()
    let page = client
        .activity()
        .watching()
        .list_watchers("owner", "repo")
        .per_page(30)
        .page(1u32)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].login, "octocat");

    // Test via repos()
    let page2 = client
        .repos("owner", "repo")
        .list_watchers()
        .per_page(30)
        .page(1u32)
        .send()
        .await
        .unwrap();

    assert_eq!(page2.items.len(), 1);
    assert_eq!(page2.items[0].login, "octocat");
}

#[tokio::test]
async fn test_get_repository_subscription() {
    let mock_server = MockServer::start().await;
    let expected = subscription_json(true, false);

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/subscription"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    setup_error_handler(&mock_server, "GET /repos/owner/repo/subscription not matched").await;

    let client = setup_octocrab(&mock_server.uri());

    // Test via activity().watching()
    let sub = client
        .activity()
        .watching()
        .get_repository_subscription("owner", "repo")
        .await
        .unwrap();

    assert!(sub.subscribed);
    assert!(!sub.ignored);

    // Test via repos().subscription()
    let sub2 = client.repos("owner", "repo").subscription().get().await.unwrap();
    assert!(sub2.subscribed);
    assert!(!sub2.ignored);
}

#[tokio::test]
async fn test_set_repository_subscription() {
    let mock_server = MockServer::start().await;
    let expected = subscription_json(true, false);

    Mock::given(method("PUT"))
        .and(path("/repos/owner/repo/subscription"))
        .and(body_json(json!({ "subscribed": true, "ignored": false })))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());

    // Test via activity().watching()
    let sub = client
        .activity()
        .watching()
        .set_repository_subscription("owner", "repo")
        .subscribed(true)
        .ignored(false)
        .send()
        .await
        .unwrap();

    assert!(sub.subscribed);
    assert!(!sub.ignored);

    // Test via repos().subscription()
    let sub2 = client
        .repos("owner", "repo")
        .subscription()
        .set()
        .subscribed(true)
        .ignored(false)
        .send()
        .await
        .unwrap();

    assert!(sub2.subscribed);
    assert!(!sub2.ignored);
}

#[tokio::test]
async fn test_delete_repository_subscription() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/repos/owner/repo/subscription"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());

    // Test via activity().watching()
    client
        .activity()
        .watching()
        .delete_repository_subscription("owner", "repo")
        .await
        .unwrap();

    // Test via repos().subscription()
    client
        .repos("owner", "repo")
        .subscription()
        .delete()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_list_user_subscriptions() {
    let mock_server = MockServer::start().await;
    let repos: Vec<Repository> =
        serde_json::from_str(include_str!("resources/user_repositories.json")).unwrap();

    Mock::given(method("GET"))
        .and(path("/user/subscriptions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&repos))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/users/octocat/subscriptions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&repos))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());

    // Authenticated user subscriptions via activity().watching()
    let watched = client
        .activity()
        .watching()
        .list_watched_repos_for_authenticated_user()
        .send()
        .await
        .unwrap();
    assert_eq!(watched.items.len(), 2);

    // Authenticated user subscriptions via current()
    let watched_cur = client.current().list_watched_repos().send().await.unwrap();
    assert_eq!(watched_cur.items.len(), 2);

    // User subscriptions via activity().watching()
    let user_watched = client
        .activity()
        .watching()
        .list_watched_repos_for_user("octocat")
        .send()
        .await
        .unwrap();
    assert_eq!(user_watched.items.len(), 2);

    // User subscriptions via users()
    let user_watched2 = client.users("octocat").subscriptions().send().await.unwrap();
    assert_eq!(user_watched2.items.len(), 2);
}
