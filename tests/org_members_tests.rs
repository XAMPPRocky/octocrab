// Tests for calls to the /repos/{owner}/members API.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{models::User, Octocrab, Page};
use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[derive(Serialize, Deserialize)]
struct FakePage<T> {
    items: Vec<T>,
}

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let org = "org";
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/orgs/{}/members", org)))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /orgs/{}/members was not received", org),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_url(uri).unwrap().build().unwrap()
}

const ORG: &str = "org";

#[tokio::test]
async fn should_return_page_with_users() {
    let org_members: User =
        serde_json::from_str(include_str!("resources/org_members.json")).unwrap();
    let login: String = org_members.login.clone();
    let page_response = FakePage {
        items: vec![org_members],
    };
    let template = ResponseTemplate::new(200).set_body_json(&page_response);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.list_members().send().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    match result.unwrap() {
        Page { items, .. } => {
            assert_eq!(items.len(), 1);
            assert_eq!(items[0].login, login);
        }
    }
}
