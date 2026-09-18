use octocrab::{Error, Octocrab};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_handle_504_gateway_timeout_html_body() {
    let mock_server = MockServer::start().await;

    let html_body = "<html><body><h1>504 Gateway Time-out</h1>\nThe server didn't respond in time.\n</body></html>";

    Mock::given(method("GET"))
        .and(path("/gateway_timeout_endpoint"))
        .respond_with(
            ResponseTemplate::new(504)
                .insert_header("content-type", "text/html")
                .insert_header("x-ratelimit-limit", "60")
                .insert_header("x-ratelimit-remaining", "55")
                .insert_header("x-ratelimit-reset", "1746664469")
                .insert_header("x-github-request-id", "C216:1D5A9C:83F6E5:A3B9AA:681BEE10")
                .set_body_string(html_body),
        )
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let result: Result<serde_json::Value, _> =
        client.get("/gateway_timeout_endpoint", None::<&()>).await;

    match result.unwrap_err() {
        Error::GitHub { source, .. } => {
            assert_eq!(source.status_code, http::StatusCode::GATEWAY_TIMEOUT);
            assert_eq!(source.message, html_body);
            assert_eq!(source.documentation_url, None);
            assert_eq!(source.errors, None);
            assert_eq!(source.rate_limit_reset, Some(1746664469));

            let headers = source.headers.expect("headers should be present");
            assert_eq!(
                headers.get("x-ratelimit-reset").unwrap().to_str().unwrap(),
                "1746664469"
            );
            assert_eq!(
                headers
                    .get("x-github-request-id")
                    .unwrap()
                    .to_str()
                    .unwrap(),
                "C216:1D5A9C:83F6E5:A3B9AA:681BEE10"
            );
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[tokio::test]
async fn should_handle_502_bad_gateway_plain_text_body() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/bad_gateway_endpoint"))
        .respond_with(
            ResponseTemplate::new(502)
                .insert_header("content-type", "text/plain")
                .set_body_string("Bad Gateway\n"),
        )
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let result: Result<serde_json::Value, _> =
        client.get("/bad_gateway_endpoint", None::<&()>).await;

    match result.unwrap_err() {
        Error::GitHub { source, .. } => {
            assert_eq!(source.status_code, http::StatusCode::BAD_GATEWAY);
            assert_eq!(source.message, "Bad Gateway");
            assert_eq!(source.documentation_url, None);
            assert_eq!(source.errors, None);
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[tokio::test]
async fn should_handle_500_empty_body() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/empty_error_endpoint"))
        .respond_with(ResponseTemplate::new(500).set_body_bytes(vec![]))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let result: Result<serde_json::Value, _> =
        client.get("/empty_error_endpoint", None::<&()>).await;

    match result.unwrap_err() {
        Error::GitHub { source, .. } => {
            assert_eq!(source.status_code, http::StatusCode::INTERNAL_SERVER_ERROR);
            assert_eq!(source.message, "Internal Server Error");
            assert_eq!(source.documentation_url, None);
            assert_eq!(source.errors, None);
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[tokio::test]
async fn should_handle_non_utf8_error_body() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/non_utf8_endpoint"))
        .respond_with(ResponseTemplate::new(504).set_body_bytes(vec![0x66, 0x6f, 0x80, 0x6f]))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let result: Result<serde_json::Value, _> = client.get("/non_utf8_endpoint", None::<&()>).await;

    match result.unwrap_err() {
        Error::GitHub { source, .. } => {
            assert_eq!(source.status_code, http::StatusCode::GATEWAY_TIMEOUT);
            assert!(source.message.contains("fo"));
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[tokio::test]
async fn should_handle_json_error_without_message() {
    let mock_server = MockServer::start().await;

    let json_body = r#"{"error": "something_went_wrong"}"#;

    Mock::given(method("GET"))
        .and(path("/json_no_message_endpoint"))
        .respond_with(
            ResponseTemplate::new(400)
                .insert_header("content-type", "application/json")
                .set_body_string(json_body),
        )
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let result: Result<serde_json::Value, _> =
        client.get("/json_no_message_endpoint", None::<&()>).await;

    match result.unwrap_err() {
        Error::GitHub { source, .. } => {
            assert_eq!(source.status_code, http::StatusCode::BAD_REQUEST);
            assert_eq!(source.message, json_body);
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}
