use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "owner";

async fn setup_mock(
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
        &format!(
            "http method {} on {} was not received",
            http_method, mocked_path
        ),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_list_org_installations() {
    let template = ResponseTemplate::new(200).set_body_json(json!({
        "total_count": 0,
        "installations": []
    }));
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/installations", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.orgs(OWNER).list_installations().send().await;
    assert!(result.is_ok(), "result: {:?}", result);
    let page = result.unwrap();
    assert_eq!(page.items.len(), 0);
}
