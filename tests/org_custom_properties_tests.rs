use http::StatusCode;
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::orgs::custom_properties::{
    CustomPropertyValue, OrgCustomPropertyDefinition, OrgRepoCustomPropertyValues,
};
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

fn sample_property() -> OrgCustomPropertyDefinition {
    serde_json::from_value(json!({
        "property_name": "env",
        "value_type": "string",
        "required": false,
        "default_value": "prod",
        "description": "Environment"
    }))
    .unwrap()
}

#[tokio::test]
async fn should_get_schema() {
    let property = sample_property();
    let template = ResponseTemplate::new(200).set_body_json(vec![property]);
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/properties/schema", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.orgs(OWNER).custom_properties().get_schema().await;
    assert!(result.is_ok(), "result: {:?}", result);
    let schema = result.unwrap();
    assert_eq!(schema.len(), 1);
    assert_eq!(schema[0].property_name, "env");
}

#[tokio::test]
async fn should_create_or_update_schema() {
    let property = sample_property();
    let template = ResponseTemplate::new(200).set_body_json(vec![property.clone()]);
    let mock_server = setup_mock(
        "PATCH",
        format!("/orgs/{}/properties/schema", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .custom_properties()
        .create_or_update_schema(vec![property])
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_get_property_schema() {
    let property = sample_property();
    let template = ResponseTemplate::new(200).set_body_json(&property);
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/properties/schema/env", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .custom_properties()
        .get_property_schema("env")
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
    assert_eq!(result.unwrap().property_name, "env");
}

#[tokio::test]
async fn should_create_or_update_property() {
    let property = sample_property();
    let template = ResponseTemplate::new(200).set_body_json(&property);
    let mock_server = setup_mock(
        "PUT",
        format!("/orgs/{}/properties/schema/env", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .custom_properties()
        .create_or_update_property("env", &property)
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_remove_property() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "DELETE",
        format!("/orgs/{}/properties/schema/env", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .custom_properties()
        .remove_property("env")
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}

#[tokio::test]
async fn should_get_values() {
    let values: Vec<OrgRepoCustomPropertyValues> = serde_json::from_value(json!([
        {
            "repository_id": 1,
            "repository_name": "repo",
            "repository_full_name": "owner/repo",
            "properties": [
                {
                    "property_name": "env",
                    "value": "prod"
                }
            ]
        }
    ]))
    .unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&values);
    let mock_server = setup_mock(
        "GET",
        format!("/orgs/{}/properties/values", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .custom_properties()
        .get_values()
        .per_page(10)
        .page(1u32)
        .send()
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
    let page = result.unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].repository_name, "repo");
}

#[tokio::test]
async fn should_create_or_update_values() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_mock(
        "PATCH",
        format!("/orgs/{}/properties/values", OWNER).as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .orgs(OWNER)
        .custom_properties()
        .create_or_update_values(
            vec!["repo"],
            vec![CustomPropertyValue::new("env", Some(json!("prod")))],
        )
        .await;
    assert!(result.is_ok(), "result: {:?}", result);
}
