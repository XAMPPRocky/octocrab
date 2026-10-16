use octocrab::Octocrab;
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_mark_notification_done() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/notifications/threads/42"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let octo = setup_octocrab(&mock_server.uri());
    let res = octo
        .activity()
        .notifications()
        .mark_as_done(42u64.into())
        .await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_stargazer_count() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/stargazers/count"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "count": 1234 })))
        .mount(&mock_server)
        .await;

    let octo = setup_octocrab(&mock_server.uri());
    let count = octo.repos("owner", "repo").stargazer_count().await.unwrap();
    assert_eq!(count.count, 1234);
}

#[tokio::test]
async fn test_add_remove_collaborator() {
    let mock_server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/repos/owner/repo/collaborators/hubot"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/repos/owner/repo/collaborators/hubot"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let octo = setup_octocrab(&mock_server.uri());
    let add_res = octo
        .repos("owner", "repo")
        .add_collaborator("hubot")
        .send()
        .await
        .unwrap();
    assert!(add_res.is_none());

    let remove_res = octo
        .repos("owner", "repo")
        .remove_collaborator("hubot")
        .await;
    assert!(remove_res.is_ok());
}

#[tokio::test]
async fn test_get_and_rerequest_reviewers() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/pulls/101/requested_reviewers"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "users": [],
            "teams": []
        })))
        .mount(&mock_server)
        .await;

    let octo = setup_octocrab(&mock_server.uri());
    let reviewers = octo
        .pulls("owner", "repo")
        .get_requested_reviewers(101)
        .await
        .unwrap();
    assert!(reviewers.users.is_empty());
}

#[tokio::test]
async fn test_credentials_and_emojis() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/credentials/revoke"))
        .respond_with(ResponseTemplate::new(202))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/emojis"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "+1": "https://github.githubassets.com/images/icons/emoji/unicode/1f44d.png?v8"
        })))
        .mount(&mock_server)
        .await;

    let octo = setup_octocrab(&mock_server.uri());
    let revoke_res = octo.credentials().revoke(["token_abc"]).await;
    assert!(revoke_res.is_ok());

    let emojis = octo.emojis().list().await.unwrap();
    assert_eq!(
        emojis.get("+1").unwrap(),
        "https://github.githubassets.com/images/icons/emoji/unicode/1f44d.png?v8"
    );
}
