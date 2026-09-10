mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{
    models::{
        repos::{Activity, ActivityType},
        ActivityId,
    },
    params::{repos::ActivityTimePeriod, Direction},
    Octocrab,
};
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

const OWNER: &str = "owner";
const REPO: &str = "repo";

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_list_repo_activities() {
    let mock_server = MockServer::start().await;
    let mocked_response: Vec<Activity> =
        serde_json::from_str(include_str!("resources/repo_activities.json")).unwrap();

    let template = ResponseTemplate::new(200)
        .set_body_json(&mocked_response)
        .append_header(
            "Link",
            "<https://api.github.com/repositories/123/activity?after=cursor_123&per_page=3>; rel=\"next\"",
        );

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/activity")))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/activity was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).activity().list().send().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let page = result.unwrap();
    assert_eq!(page.items.len(), 3);

    let first = &page.items[0];
    assert_eq!(first.id, ActivityId::from(43180726230u64));
    assert_eq!(first.node_id, "PSH_kwLOD4Ga3s8AAAAKDcWX1g");
    assert_eq!(first.activity_type, ActivityType::PrMerge);
    assert_eq!(first.r#ref, "refs/heads/main");
    assert!(first.actor.is_some());
    assert_eq!(first.actor.as_ref().unwrap().login, "dmgorsky");

    let second = &page.items[1];
    assert_eq!(second.id, ActivityId::from(43178147663u64));
    assert_eq!(second.activity_type, ActivityType::Push);
    assert_eq!(second.actor.as_ref().unwrap().login, "octocat");

    let third = &page.items[2];
    assert_eq!(third.id, ActivityId::from(43170000000u64));
    assert_eq!(third.activity_type, ActivityType::BranchCreation);
    assert!(third.actor.is_none());

    assert!(page.next.is_some());
}

#[tokio::test]
async fn should_filter_activities_with_query_parameters() {
    let mock_server = MockServer::start().await;
    let mocked_response: Vec<Activity> =
        serde_json::from_str(include_str!("resources/repo_activities.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/activity")))
        .and(query_param("direction", "asc"))
        .and(query_param("per_page", "10"))
        .and(query_param("ref", "refs/heads/main"))
        .and(query_param("actor", "octocat"))
        .and(query_param("time_period", "week"))
        .and(query_param("activity_type", "push"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/activity with query params was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .list_activities()
        .direction(Direction::Ascending)
        .per_page(10)
        .r#ref("refs/heads/main")
        .actor("octocat")
        .time_period(ActivityTimePeriod::Week)
        .activity_type(ActivityType::Push)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_paginate_with_cursor() {
    let mock_server = MockServer::start().await;
    let mocked_response: Vec<Activity> =
        serde_json::from_str(include_str!("resources/repo_activities.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/activity")))
        .and(query_param("before", "cursor_before"))
        .and(query_param("after", "cursor_after"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/activity with before/after was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(OWNER, REPO)
        .activity()
        .list()
        .before("cursor_before")
        .after("cursor_after")
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_handle_error_response() {
    let mock_server = MockServer::start().await;
    let template = ResponseTemplate::new(422).set_body_json(serde_json::json!({
        "message": "Validation Failed",
        "documentation_url": "https://docs.github.com/rest/repos/repos#list-repository-activities"
    }));

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/activity")))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).activity().list().send().await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}
