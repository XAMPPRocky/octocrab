use http::StatusCode;
use wiremock::{
    matchers::{body_json, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::{
    models::{repos::CustomPropertyValue, TeamId},
    Octocrab,
};

mod mock_error;

const OWNER: &str = "owner";
const REPO: &str = "repo";

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

// ---------------------------------------------------------------------------
// CODEOWNERS Errors
// ---------------------------------------------------------------------------

#[tokio::test]
async fn should_list_codeowners_errors() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let response_json = serde_json::json!({
        "errors": [
            {
                "line": 5,
                "column": 3,
                "kind": "Unknown owner",
                "suggestion": "Did you mean @octocat?",
                "message": "The owner `@octocat` could not be found.",
                "path": ".github/CODEOWNERS"
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/codeowners/errors")))
        .and(query_param("ref", "main"))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&response_json))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("GET on /repos/{OWNER}/{REPO}/codeowners/errors was not received"),
    )
    .await;

    let errors = octocrab
        .repos(OWNER, REPO)
        .codeowners_errors()
        .r#ref("main")
        .send()
        .await
        .unwrap();

    assert_eq!(errors.errors.len(), 1);
    assert_eq!(errors.errors[0].line, 5);
    assert_eq!(errors.errors[0].column, 3);
    assert_eq!(errors.errors[0].kind, "Unknown owner");
    assert_eq!(
        errors.errors[0].suggestion.as_deref(),
        Some("Did you mean @octocat?")
    );
    assert_eq!(
        errors.errors[0].message,
        "The owner `@octocat` could not be found."
    );
    assert_eq!(errors.errors[0].path, ".github/CODEOWNERS");
}

// ---------------------------------------------------------------------------
// Custom Properties
// ---------------------------------------------------------------------------

#[tokio::test]
async fn should_get_custom_property_values() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let response_json = serde_json::json!([
        {
            "property_name": "environment",
            "value": "production"
        },
        {
            "property_name": "service_tier",
            "value": ["tier1", "tier2"]
        }
    ]);

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/properties/values")))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&response_json))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("GET on /repos/{OWNER}/{REPO}/properties/values was not received"),
    )
    .await;

    let values = octocrab
        .repos(OWNER, REPO)
        .custom_properties()
        .get_values()
        .await
        .unwrap();

    assert_eq!(values.len(), 2);
    assert_eq!(values[0].property_name, "environment");
    assert_eq!(
        values[0].value,
        Some(serde_json::Value::String("production".into()))
    );
    assert_eq!(values[1].property_name, "service_tier");
}

#[tokio::test]
async fn should_update_custom_property_values() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let expected_body = serde_json::json!({
        "properties": [
            {
                "property_name": "environment",
                "value": "production"
            }
        ]
    });

    Mock::given(method("PATCH"))
        .and(path(format!("/repos/{OWNER}/{REPO}/properties/values")))
        .and(body_json(&expected_body))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("PATCH on /repos/{OWNER}/{REPO}/properties/values was not received"),
    )
    .await;

    let props = vec![CustomPropertyValue::new(
        "environment",
        Some(serde_json::Value::String("production".into())),
    )];

    let result = octocrab
        .repos(OWNER, REPO)
        .custom_properties()
        .create_or_update_values(props)
        .await;

    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// Transfer Repository
// ---------------------------------------------------------------------------

#[tokio::test]
async fn should_transfer_repository() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let expected_body = serde_json::json!({
        "new_owner": "hubot",
        "new_name": "new-repo",
        "team_ids": [1]
    });

    let response_json = serde_json::json!({
        "id": 1296269,
        "name": "new-repo",
        "url": "https://api.github.com/repos/hubot/new-repo"
    });

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/transfer")))
        .and(body_json(&expected_body))
        .respond_with(ResponseTemplate::new(StatusCode::ACCEPTED).set_body_json(&response_json))
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("POST on /repos/{OWNER}/{REPO}/transfer was not received"),
    )
    .await;

    let repo = octocrab
        .repos(OWNER, REPO)
        .transfer("hubot")
        .new_name("new-repo")
        .team_ids(vec![TeamId(1)])
        .send()
        .await
        .unwrap();

    assert_eq!(repo.name, "new-repo");
    assert_eq!(repo.id.0, 1296269);
}

// ---------------------------------------------------------------------------
// Download Zipball
// ---------------------------------------------------------------------------

#[tokio::test]
async fn should_download_zipball() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let zip_bytes = b"PK\x03\x04mock_zip_content";

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/zipball/main")))
        .respond_with(
            ResponseTemplate::new(StatusCode::OK)
                .set_body_bytes(zip_bytes.to_vec())
                .insert_header("content-type", "application/zip"),
        )
        .mount(&server)
        .await;

    setup_error_handler(
        &server,
        &format!("GET on /repos/{OWNER}/{REPO}/zipball/main was not received"),
    )
    .await;

    let response = octocrab
        .repos(OWNER, REPO)
        .download_zipball("main")
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

// ---------------------------------------------------------------------------
// List Public Repositories
// ---------------------------------------------------------------------------

#[tokio::test]
async fn should_list_all_repositories() {
    let server = MockServer::start().await;
    let octocrab = setup_octocrab(&server.uri());

    let response_json = serde_json::json!([
        {
            "id": 101,
            "name": "repo-101",
            "url": "https://api.github.com/repos/user/repo-101"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/repositories"))
        .and(query_param("since", "100"))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&response_json))
        .mount(&server)
        .await;

    setup_error_handler(&server, "GET on /repositories was not received").await;

    let page = octocrab
        .all_repositories()
        .since(100u64)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].name, "repo-101");
    assert_eq!(page.items[0].id.0, 101);

    // Also verify alias repositories()
    let page_alias = octocrab.repositories().since(100u64).send().await.unwrap();

    assert_eq!(page_alias.items.len(), 1);
}
