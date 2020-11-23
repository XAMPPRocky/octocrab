// Tests for calls to the /repos/{owner}/{repo}/events API.
use octocrab::{
    etag::{Etag, Etagged},
    models::events,
    Octocrab, OctocrabBuilder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::convert::TryFrom;
use wiremock::{
    matchers::{method, path, path_regex},
    Mock, MockServer, ResponseTemplate,
};

#[derive(Serialize, Deserialize)]
struct FakePage<T> {
    items: Vec<T>,
}

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let owner = "owner";
    let repo = "repo";
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("/repos/{}/{}/events", owner, repo)))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /repo/{}/{}/events was not received", owner, repo),
    )
    .await;
    mock_server
}

// Sets up a handler on the mock server which will return a 500 with the given message. This
// will be mapped internally into a GitHub json error, making it much easier to identify the cause
// of these test failures.
//
// This handler should always come after your real expectations as it will match any GET request.
async fn setup_error_handler(mock_server: &MockServer, message: &str) {
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(500).set_body_json(json!( {
            "documentation_url": "",
            "errors": None::<Vec<serde_json::Value>>,
            "message": message,
        })))
        .mount(&mock_server)
        .await;
}

fn setup_octocrab(uri: &str) -> Octocrab {
    OctocrabBuilder::new()
        .base_url(uri)
        .unwrap()
        .build()
        .unwrap()
}

const OWNER: &str = "owner";
const REPO: &str = "repo";

#[tokio::test]
async fn should_return_page_with_events_and_etag() {
    let event: events::Event =
        serde_json::from_str(include_str!("resources/create_event.json")).unwrap();
    let page_response = FakePage { items: vec![event] };
    let expected_etag = "12345";
    let template = ResponseTemplate::new(200)
        .set_body_json(&page_response)
        .insert_header("etag", expected_etag);
    let mock_server = setup_api(template).await;
    let octo = setup_octocrab(&mock_server.uri());
    let repos = octo.repos(OWNER.to_owned(), REPO.to_owned());
    let result = repos.events().send().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    match result.unwrap() {
        Etagged {
            etag: Some(etag),
            value: Some(mut page),
        } => {
            assert_eq!(page.take_items(), page_response.items);
            assert_eq!(etag, Etag::try_from(expected_etag).unwrap());
        }
        unexpected => panic!("expected a page and an etag, got {:#?}", unexpected),
    }
}

#[tokio::test]
async fn should_return_no_page_with_events_and_etag_when_response_is_304() {
    let expected_etag = "12345";
    let template = ResponseTemplate::new(304).append_header("etag", expected_etag);
    let mock_server = setup_api(template).await;
    let octo = setup_octocrab(&mock_server.uri());
    let repos = octo.repos(OWNER.to_owned(), REPO.to_owned());
    let result = repos.events().send().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    match result.unwrap() {
        Etagged {
            etag: Some(etag),
            value: None,
        } => {
            assert_eq!(etag, Etag::try_from(expected_etag).unwrap());
        }
        unexpected => panic!("expected no page and an etag, got {:#?}", unexpected),
    }
}

#[tokio::test]
async fn should_return_no_etag_if_response_contains_none() {
    let event: events::Event =
        serde_json::from_str(include_str!("resources/create_event.json")).unwrap();
    let page_response = FakePage { items: vec![event] };
    let template = ResponseTemplate::new(200).set_body_json(&page_response);
    let mock_server = setup_api(template).await;
    let octo = setup_octocrab(&mock_server.uri());
    let repos = octo.repos(OWNER.to_owned(), REPO.to_owned());
    let result = repos.events().send().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    match result.unwrap() {
        Etagged {
            etag: None,
            value: Some(mut page),
        } => {
            assert_eq!(page.take_items(), page_response.items);
        }
        unexpected => panic!("expected a page with no etag, got {:#?}", unexpected),
    }
}
