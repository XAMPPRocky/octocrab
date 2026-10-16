use octocrab::Octocrab;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_private_registries_all_endpoints() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 1. List private registries
    Mock::given(method("GET"))
        .and(path("/orgs/my-org/private-registries"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "configurations": [
                {
                    "name": "MAVEN_REPOSITORY_SECRET",
                    "registry_type": "maven_repository",
                    "username": "monalisa",
                    "visibility": "selected"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let list = octocrab
        .orgs("my-org")
        .private_registries()
        .list()
        .per_page(10)
        .send()
        .await
        .unwrap();
    assert_eq!(list.total_count, 1);
    assert_eq!(list.configurations[0].name, "MAVEN_REPOSITORY_SECRET");

    // 2. Create private registry
    Mock::given(method("POST"))
        .and(path("/orgs/my-org/private-registries"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "name": "NPM_REGISTRY",
            "registry_type": "npm_registry",
            "username": "monalisa",
            "visibility": "all"
        })))
        .mount(&mock_server)
        .await;

    let created = octocrab
        .orgs("my-org")
        .private_registries()
        .create(&json!({
            "registry_type": "npm_registry",
            "username": "monalisa"
        }))
        .await
        .unwrap();
    assert_eq!(created.name, "NPM_REGISTRY");

    // 3. Get public key
    Mock::given(method("GET"))
        .and(path("/orgs/my-org/private-registries/public-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "key_id": "key123",
            "key": "base64key"
        })))
        .mount(&mock_server)
        .await;

    let pk = octocrab
        .orgs("my-org")
        .private_registries()
        .public_key()
        .await
        .unwrap();
    assert_eq!(pk.key_id, "key123");

    // 4. Get specific private registry
    Mock::given(method("GET"))
        .and(path("/orgs/my-org/private-registries/NPM_REGISTRY"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "name": "NPM_REGISTRY",
            "registry_type": "npm_registry",
            "visibility": "all"
        })))
        .mount(&mock_server)
        .await;

    let reg = octocrab
        .orgs("my-org")
        .private_registries()
        .get("NPM_REGISTRY")
        .await
        .unwrap();
    assert_eq!(reg.name, "NPM_REGISTRY");

    // 5. Update private registry
    Mock::given(method("PATCH"))
        .and(path("/orgs/my-org/private-registries/NPM_REGISTRY"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    assert!(octocrab
        .orgs("my-org")
        .private_registries()
        .update("NPM_REGISTRY", &json!({ "visibility": "private" }))
        .await
        .is_ok());

    // 6. Delete private registry
    Mock::given(method("DELETE"))
        .and(path("/orgs/my-org/private-registries/NPM_REGISTRY"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    assert!(octocrab
        .orgs("my-org")
        .private_registries()
        .delete("NPM_REGISTRY")
        .await
        .is_ok());
}
