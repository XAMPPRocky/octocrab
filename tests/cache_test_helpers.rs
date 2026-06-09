use octocrab::models::UserProfile;
use octocrab::service::middleware::cache::CacheStorage;
use octocrab::Octocrab;
use serde::Serialize;
use serde_json::json;
use wiremock::{
    matchers::{any, header, method, path},
    Mock, MockServer, ResponseTemplate,
};

pub async fn no_match_header_request_mock<B: Serialize>(
    mocked_path: &str,
    body: &B,
    etag: &str,
    expect: u64,
) -> Mock {
    let template = ResponseTemplate::new(200)
        .append_header(http::header::ETAG, etag)
        .set_body_json(body);

    Mock::given(method("GET"))
        .and(path(mocked_path))
        .respond_with(template)
        .expect(expect)
}

pub async fn etag_hit_request_mock(mocked_path: &str, etag: &str, expect: u64) -> Mock {
    let template = ResponseTemplate::new(304).append_header(http::header::ETAG, etag);

    Mock::given(method("GET"))
        .and(path(mocked_path))
        .and(header(http::header::IF_NONE_MATCH, etag))
        .respond_with(template)
        .expect(expect)
}

pub async fn etag_miss_request_mock<B: Serialize>(
    mocked_path: &str,
    request_etag: &str,
    body: &B,
    response_etag: &str,
    expect: u64,
) -> Mock {
    let template = ResponseTemplate::new(200)
        .append_header(http::header::ETAG, response_etag)
        .set_body_json(body);

    Mock::given(method("GET"))
        .and(path(mocked_path))
        .and(header(http::header::IF_NONE_MATCH, request_etag))
        .respond_with(template)
        .expect(expect)
}

// Copied from mock_error.rs to avoid transitive mod
pub async fn error_mock(message: &str) -> Mock {
    Mock::given(method("GET"))
        .and(any())
        .respond_with(ResponseTemplate::new(500).set_body_json(json!( {
            "documentation_url": "",
            "errors": None::<Vec<serde_json::Value>>,
            "message": message,
        })))
}

pub fn setup_octocrab<C: CacheStorage + 'static>(uri: &str, cache: C) -> Octocrab {
    Octocrab::builder()
        .base_uri(uri)
        .unwrap()
        .cache(cache)
        .build()
        .unwrap()
}

async fn get_user_profile(client: &Octocrab, expected: &UserProfile) {
    let result = client.users("some-user").profile().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    assert_eq!(expected, &result.unwrap());
}

pub async fn should_cache_impl<C>(cache: C)
where
    C: CacheStorage + 'static,
{
    let mocked_path = "/users/some-user";

    let body: UserProfile = serde_json::from_str(include_str!("resources/user_data.json"))
        .expect("Failed to parse mocked user profile");
    let etag = "test-etag";

    let mock_server = MockServer::start().await;

    etag_hit_request_mock(mocked_path, etag, 2)
        .await
        .mount(&mock_server)
        .await;

    // Registered after etag mocks since there is no way to mock "Header is not present"
    no_match_header_request_mock(mocked_path, &body, etag, 1)
        .await
        .mount(&mock_server)
        .await;

    error_mock(&format!("GET on {mocked_path} was not received"))
        .await
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri(), cache);

    // First call should populate the cache
    get_user_profile(&client, &body).await;

    // Last calls should send the If-None-Match header and receive a 304 Not Modified response
    get_user_profile(&client, &body).await;
    get_user_profile(&client, &body).await;
}

pub async fn etag_update_cache_impl<C>(cache: C)
where
    C: CacheStorage + 'static,
{
    let mocked_path = "/users/some-user";

    let first_body: UserProfile = serde_json::from_str(include_str!("resources/user_data.json"))
        .expect("Failed to parse mocked user profile");
    let first_etag = "first-etag";

    let mut second_body = first_body.clone();
    second_body.name = Some("Updated Name".to_string());
    let second_etag = "second-etag";

    let mock_server = MockServer::start().await;

    etag_miss_request_mock(mocked_path, first_etag, &second_body, second_etag, 1)
        .await
        .mount(&mock_server)
        .await;

    etag_hit_request_mock(mocked_path, second_etag, 2)
        .await
        .mount(&mock_server)
        .await;

    // Registered after etag mocks since there is no way to mock "Header is not present"
    no_match_header_request_mock(mocked_path, &first_body, first_etag, 1)
        .await
        .mount(&mock_server)
        .await;

    error_mock(&format!("GET on {mocked_path} was not received"))
        .await
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri(), cache);

    // First call should populate the cache
    get_user_profile(&client, &first_body).await;

    // Second call will miss the cache with a different etag in the response
    get_user_profile(&client, &second_body).await;

    // Last calls should also send the If-None-Match header and receive a 304 Not Modified response
    get_user_profile(&client, &second_body).await;
    get_user_profile(&client, &second_body).await;
}
