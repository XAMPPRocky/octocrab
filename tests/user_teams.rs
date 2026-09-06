use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::teams::FullTeam;
use octocrab::Octocrab;

/// Tests API calls related to teams for the authenticated user.
mod mock_error;

async fn setup_user_teams_mock(
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
async fn should_respond_to_list_all_teams_for_auth_user() {
    // https://docs.github.com/en/rest/teams/teams?apiVersion=2022-11-28#list-teams-for-the-authenticated-user
    let mocked_response: Vec<FullTeam> =
        serde_json::from_str(include_str!("resources/user_teams.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_user_teams_mock("GET", "/user/teams", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .current()
        .list_all_teams_for_auth_user()
        .send()
        .await
        .unwrap();
    let items = result.items;
    assert_eq!(items.len(), 1); // expected 1 team in example list
    assert!(items
        .first()
        .is_some_and(|team| team.slug == "justice-league"
            && team
                .organization
                .as_ref()
                .is_some_and(|org| org.login == "github")));
}
