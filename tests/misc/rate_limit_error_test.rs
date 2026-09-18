use chrono::DateTime;
use octocrab::{Error, Octocrab};
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_expose_rate_limit_reset_and_headers_on_error() {
    let mock_server = MockServer::start().await;

    let error_body = json!({
        "message": "API rate limit exceeded for 127.0.0.1. (But here's the good news: Authenticated requests get a higher rate limit. Check out the documentation for more details.)",
        "documentation_url": "https://docs.github.com/rest/overview/resources-in-the-rest-api#rate-limiting"
    });

    Mock::given(method("GET"))
        .and(path("/rate_limited_endpoint"))
        .respond_with(
            ResponseTemplate::new(403)
                .insert_header("x-ratelimit-reset", "1715000000")
                .insert_header("x-ratelimit-remaining", "0")
                .insert_header("x-ratelimit-limit", "60")
                .set_body_json(&error_body),
        )
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let result: Result<serde_json::Value, _> =
        client.get("/rate_limited_endpoint", None::<&()>).await;

    match result.unwrap_err() {
        Error::GitHub { source, .. } => {
            assert_eq!(source.status_code, http::StatusCode::FORBIDDEN);
            assert_eq!(
                source.message,
                "API rate limit exceeded for 127.0.0.1. (But here's the good news: Authenticated requests get a higher rate limit. Check out the documentation for more details.)"
            );
            assert_eq!(source.rate_limit_reset, Some(1715000000));
            assert_eq!(
                source.rate_limit_reset_at(),
                DateTime::from_timestamp(1715000000, 0)
            );

            let headers = source.headers.expect("headers should be present");
            assert_eq!(
                headers.get("x-ratelimit-reset").unwrap().to_str().unwrap(),
                "1715000000"
            );
            assert_eq!(
                headers
                    .get("x-ratelimit-remaining")
                    .unwrap()
                    .to_str()
                    .unwrap(),
                "0"
            );
            assert_eq!(
                headers.get("x-ratelimit-limit").unwrap().to_str().unwrap(),
                "60"
            );
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[tokio::test]
async fn should_handle_missing_rate_limit_headers() {
    let mock_server = MockServer::start().await;

    let error_body = json!({
        "message": "Not Found",
        "documentation_url": "https://docs.github.com/rest"
    });

    Mock::given(method("GET"))
        .and(path("/not_found_endpoint"))
        .respond_with(ResponseTemplate::new(404).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let result: Result<serde_json::Value, _> = client.get("/not_found_endpoint", None::<&()>).await;

    match result.unwrap_err() {
        Error::GitHub { source, .. } => {
            assert_eq!(source.status_code, http::StatusCode::NOT_FOUND);
            assert_eq!(source.rate_limit_reset, None);
            assert_eq!(source.rate_limit_reset_at(), None);
            assert!(source.headers.is_some());
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[tokio::test]
async fn should_handle_malformed_rate_limit_reset_header() {
    let mock_server = MockServer::start().await;

    let error_body = json!({
        "message": "API rate limit exceeded",
        "documentation_url": "https://docs.github.com/rest"
    });

    Mock::given(method("GET"))
        .and(path("/malformed_header_endpoint"))
        .respond_with(
            ResponseTemplate::new(403)
                .insert_header("x-ratelimit-reset", "invalid_timestamp")
                .set_body_json(&error_body),
        )
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let result: Result<serde_json::Value, _> =
        client.get("/malformed_header_endpoint", None::<&()>).await;

    match result.unwrap_err() {
        Error::GitHub { source, .. } => {
            assert_eq!(source.status_code, http::StatusCode::FORBIDDEN);
            assert_eq!(source.rate_limit_reset, None);
            assert_eq!(source.rate_limit_reset_at(), None);
            assert!(source.headers.is_some());
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}
