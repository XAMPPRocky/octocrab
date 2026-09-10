mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{
    models::repos::{
        CodeFrequency, CommitActivity, ContributorActivity, ParticipationStats, PunchCard,
    },
    Octocrab,
};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

const OWNER: &str = "owner";
const REPO: &str = "repo";

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_get_code_frequency_stats() {
    let mock_server = MockServer::start().await;
    let mocked_response: Vec<CodeFrequency> =
        serde_json::from_str(include_str!("resources/repo_stats_code_frequency.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/stats/code_frequency")))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/stats/code_frequency was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).stats().code_frequency().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let stats = result.unwrap();
    assert!(stats.is_some());
    let stats = stats.unwrap();
    assert_eq!(stats.len(), 2);

    assert_eq!(stats[0].week(), 1302998400);
    assert_eq!(stats[0].additions(), 1124);
    assert_eq!(stats[0].deletions(), -435);
    assert_eq!(stats[0].0, 1302998400);
    assert_eq!(stats[0].1, 1124);
    assert_eq!(stats[0].2, -435);

    assert_eq!(stats[1].week(), 1303603200);
    assert_eq!(stats[1].additions(), 500);
    assert_eq!(stats[1].deletions(), -200);
}

#[tokio::test]
async fn should_handle_code_frequency_202_accepted() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/stats/code_frequency")))
        .respond_with(ResponseTemplate::new(202).set_body_json(serde_json::json!({})))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/stats/code_frequency was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).stats().code_frequency().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    assert_eq!(result.unwrap(), None);
}

#[tokio::test]
async fn should_get_commit_activity_stats() {
    let mock_server = MockServer::start().await;
    let mocked_response: Vec<CommitActivity> =
        serde_json::from_str(include_str!("resources/repo_stats_commit_activity.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/stats/commit_activity")))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/stats/commit_activity was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).stats().commit_activity().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let stats = result.unwrap();
    assert!(stats.is_some());
    let stats = stats.unwrap();
    assert_eq!(stats.len(), 1);
    assert_eq!(stats[0].total, 89);
    assert_eq!(stats[0].week, 1336280400);
    assert_eq!(stats[0].days, vec![0, 3, 26, 20, 39, 1, 0]);
}

#[tokio::test]
async fn should_handle_commit_activity_202_accepted() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/stats/commit_activity")))
        .respond_with(ResponseTemplate::new(202).set_body_json(serde_json::json!({})))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/stats/commit_activity was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).stats().commit_activity().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    assert_eq!(result.unwrap(), None);
}

#[tokio::test]
async fn should_get_contributors_stats() {
    let mock_server = MockServer::start().await;
    let mocked_response: Vec<ContributorActivity> =
        serde_json::from_str(include_str!("resources/repo_stats_contributors.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/stats/contributors")))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/stats/contributors was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).stats().contributors().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let stats = result.unwrap();
    assert!(stats.is_some());
    let stats = stats.unwrap();
    assert_eq!(stats.len(), 1);
    assert_eq!(stats[0].total, 135);
    assert_eq!(stats[0].author.as_ref().unwrap().login, "octocat");
    assert_eq!(stats[0].weeks.len(), 1);
    assert_eq!(stats[0].weeks[0].week, 1367712000);
    assert_eq!(stats[0].weeks[0].additions, 6898);
    assert_eq!(stats[0].weeks[0].deletions, 77);
    assert_eq!(stats[0].weeks[0].commits, 10);
}

#[tokio::test]
async fn should_handle_contributors_202_accepted() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/stats/contributors")))
        .respond_with(ResponseTemplate::new(202).set_body_json(serde_json::json!({})))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/stats/contributors was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).stats().contributors().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    assert_eq!(result.unwrap(), None);
}

#[tokio::test]
async fn should_get_participation_stats() {
    let mock_server = MockServer::start().await;
    let mocked_response: ParticipationStats =
        serde_json::from_str(include_str!("resources/repo_stats_participation.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/stats/participation")))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/stats/participation was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).stats().participation().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let stats = result.unwrap();
    assert_eq!(stats.all.len(), 52);
    assert_eq!(stats.owner.len(), 52);
    assert_eq!(stats.all[0], 11);
    assert_eq!(stats.owner[0], 3);
}

#[tokio::test]
async fn should_get_punch_card_stats() {
    let mock_server = MockServer::start().await;
    let mocked_response: Vec<PunchCard> =
        serde_json::from_str(include_str!("resources/repo_stats_punch_card.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/stats/punch_card")))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/stats/punch_card was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).stats().punch_card().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let cards = result.unwrap();
    assert_eq!(cards.len(), 3);
    assert_eq!(cards[0].day(), 0);
    assert_eq!(cards[0].hour(), 0);
    assert_eq!(cards[0].commits(), 5);
    assert_eq!(cards[0].0, 0);
    assert_eq!(cards[0].1, 0);
    assert_eq!(cards[0].2, 5);

    assert_eq!(cards[1].day(), 0);
    assert_eq!(cards[1].hour(), 1);
    assert_eq!(cards[1].commits(), 43);

    assert_eq!(cards[2].day(), 2);
    assert_eq!(cards[2].hour(), 14);
    assert_eq!(cards[2].commits(), 25);
}

#[tokio::test]
async fn should_handle_stats_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/stats/code_frequency")))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "message": "Not Found",
            "documentation_url": "https://docs.github.com/rest"
        })))
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/stats/code_frequency was not received"),
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).stats().code_frequency().await;

    assert!(result.is_err());
}
