use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::repos::secret_scanning_alert::SecretScanningAlert;
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "org";
const REPO: &str = "some-repo";

async fn setup_secrets_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{owner}/{repo}/secret-scanning/alerts",
            owner = OWNER,
            repo = REPO
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!(
            "GET on /repos/{owner}/{repo}/secret-scanning/alerts was not received",
            owner = OWNER,
            repo = REPO
        ),
    )
    .await;

    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn check_list_200() {
    let s: &str = include_str!("resources/check_secrets_alerts.json");
    let alert: Vec<SecretScanningAlert> = serde_json::from_str(s).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&alert);
    let mock_server = setup_secrets_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER.to_owned(), REPO.to_owned())
        .secrets_scanning()
        .get_alerts()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:?}",
        result
    );

    let response = result.unwrap();
    let items = response.items;

    assert_eq!(items.len(), 2);

    {
        let item = &items[0];

        assert_eq!(2, item.number);
        assert_eq!(
            octocrab::models::repos::secret_scanning_alert::State::Open,
            item.state
        );
    }
}
