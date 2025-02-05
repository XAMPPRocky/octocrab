// Tests for calls to the /users/{username}/projects API.
mod test_common;

use octocrab::{models::Project, Page};
use serde::{Deserialize, Serialize};
use test_common::{setup_error_handler, setup_octocrab};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

const USERNAME: &str = "octocat";

#[derive(Serialize, Deserialize)]
struct FakePage<T> {
    items: Vec<T>,
}

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/users/{USERNAME}/projects")))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /users/{USERNAME}/projects was not received"),
    )
    .await;
    mock_server
}

#[tokio::test]
async fn should_list_user_projects() {
    let org_project: Vec<Project> =
        serde_json::from_str(include_str!("resources/user_projects.json")).unwrap();

    let page_response = FakePage { items: org_project };

    let template = ResponseTemplate::new(200).set_body_json(&page_response);
    let mock_server = setup_api(template).await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.projects().list_user_projects(USERNAME).send().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let Page { items, .. } = result.unwrap();

    assert_eq!(items[0].name, "My Projects");
    assert_eq!(items[1].creator.login, "octocat");
}
