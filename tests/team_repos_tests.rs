// Tests for calls to the /orgs/{org}/teams/{team}/members API.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::models::{Repository, RepositoryId};
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[expect(dead_code)]
#[derive(Serialize, Deserialize)]
struct FakePage<T> {
    items: Vec<T>,
}

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let org = "org";
    let team = "team-name";
    let repo = "testing";
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!("/orgs/{org}/teams/{team}/repos/{org}/{repo}")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    Mock::given(method("PUT"))
        .and(path(format!("/orgs/{org}/teams/{team}/repos/{org}/{repo}")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path(format!("/orgs/{org}/teams/{team}/repos")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("DELETE on /orgs/{org}/teams/{team}/repos/{org}/{repo} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const ORG: &str = "org";
const TEAM: &str = "team-name";
const REPO: &str = "testing";

#[tokio::test]
async fn should_remove_team_repo() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let teams = client.teams(ORG.to_owned());

    let result = teams
        .repos(TEAM.to_owned())
        .remove(ORG.to_owned(), REPO.to_owned())
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_add_or_update_team_repo() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let teams = client.teams(ORG.to_owned());

    let result = teams
        .repos(TEAM.to_owned())
        .add_or_update(
            ORG.to_owned(),
            REPO.to_owned(),
            Some(octocrab::params::teams::Permission::Push),
        )
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    eprintln!("Result: {result:#?}");
}

#[tokio::test]
async fn should_list_team_repos() {
    let mocked_response: Vec<Repository> =
        serde_json::from_str(include_str!("resources/user_repositories.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let teams = client.teams(ORG.to_owned());

    let result = teams.repos(TEAM.to_owned()).list().send().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let items = result.unwrap().items;
    assert_eq!(items.len(), 2);
    assert_eq!(RepositoryId(566109822), items[0].id);
    assert_eq!("actix-examples", items[0].name);
    assert_eq!(RepositoryId(292435601), items[1].id);
    assert_eq!("amazon-sqs-java-temporary-queues-client", items[1].name);
}
