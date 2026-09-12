mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{
    etag::{EntityTag, Etagged},
    models::events,
    Octocrab,
};
use wiremock::{
    matchers::{header, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

fn sample_event() -> events::Event {
    serde_json::from_str(include_str!("resources/watch_event.json")).unwrap()
}

#[tokio::test]
async fn test_network_events() {
    let mock_server = MockServer::start().await;
    let expected = vec![sample_event()];

    Mock::given(method("GET"))
        .and(path("/networks/owner/repo/events"))
        .and(query_param("per_page", "30"))
        .and(query_param("page", "1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(&expected)
                .insert_header("etag", "\"1234\""),
        )
        .mount(&mock_server)
        .await;

    setup_error_handler(&mock_server, "GET /networks/owner/repo/events not matched").await;

    let client = setup_octocrab(&mock_server.uri());

    // Via activity().events()
    let result = client
        .activity()
        .events()
        .network_events("owner", "repo")
        .per_page(30u8)
        .page(1u32)
        .send()
        .await
        .unwrap();

    assert!(result.value.is_some());
    assert_eq!(result.value.unwrap().items.len(), 1);
    assert_eq!(result.etag, Some(EntityTag::strong("1234".to_string())));

    // Via repos()
    let result2 = client
        .repos("owner", "repo")
        .network_events()
        .per_page(30u8)
        .page(1u32)
        .send()
        .await
        .unwrap();

    assert!(result2.value.is_some());
    assert_eq!(result2.value.unwrap().items.len(), 1);
}

#[tokio::test]
async fn test_user_events_endpoints() {
    let mock_server = MockServer::start().await;
    let expected = vec![sample_event()];

    // /users/octocat/events
    Mock::given(method("GET"))
        .and(path("/users/octocat/events"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    // /users/octocat/events/public
    Mock::given(method("GET"))
        .and(path("/users/octocat/events/public"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    // /users/octocat/received_events
    Mock::given(method("GET"))
        .and(path("/users/octocat/received_events"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    // /users/octocat/received_events/public
    Mock::given(method("GET"))
        .and(path("/users/octocat/received_events/public"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    // /users/octocat/events/orgs/github
    Mock::given(method("GET"))
        .and(path("/users/octocat/events/orgs/github"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());

    // 1. user_events
    let res = client
        .activity()
        .events()
        .user_events("octocat")
        .send()
        .await
        .unwrap();
    assert_eq!(res.value.unwrap().items.len(), 1);
    let res = client.users("octocat").events().send().await.unwrap();
    assert_eq!(res.value.unwrap().items.len(), 1);

    // 2. public_events
    let res = client
        .activity()
        .events()
        .public_user_events("octocat")
        .send()
        .await
        .unwrap();
    assert_eq!(res.value.unwrap().items.len(), 1);
    let res = client
        .users("octocat")
        .public_events()
        .send()
        .await
        .unwrap();
    assert_eq!(res.value.unwrap().items.len(), 1);

    // 3. received_events
    let res = client
        .activity()
        .events()
        .received_events("octocat")
        .send()
        .await
        .unwrap();
    assert_eq!(res.value.unwrap().items.len(), 1);
    let res = client
        .users("octocat")
        .received_events()
        .send()
        .await
        .unwrap();
    assert_eq!(res.value.unwrap().items.len(), 1);

    // 4. public_received_events
    let res = client
        .activity()
        .events()
        .public_received_events("octocat")
        .send()
        .await
        .unwrap();
    assert_eq!(res.value.unwrap().items.len(), 1);
    let res = client
        .users("octocat")
        .public_received_events()
        .send()
        .await
        .unwrap();
    assert_eq!(res.value.unwrap().items.len(), 1);

    // 5. user_org_events
    let res = client
        .activity()
        .events()
        .user_org_events("octocat", "github")
        .send()
        .await
        .unwrap();
    assert_eq!(res.value.unwrap().items.len(), 1);
    let res = client
        .users("octocat")
        .org_events("github")
        .send()
        .await
        .unwrap();
    assert_eq!(res.value.unwrap().items.len(), 1);
}

#[tokio::test]
async fn test_events_etag_handling() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/octocat/events"))
        .and(header("If-None-Match", "\"my-etag\""))
        .respond_with(ResponseTemplate::new(304).insert_header("etag", "\"my-etag\""))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());

    let res = client
        .users("octocat")
        .events()
        .etag(Some(EntityTag::strong("my-etag".to_string())))
        .send()
        .await
        .unwrap();

    match res {
        Etagged {
            etag: Some(etag),
            value: None,
        } => {
            assert_eq!(etag, EntityTag::strong("my-etag".to_string()));
        }
        other => panic!("expected 304 with None value, got {:?}", other),
    }
}
