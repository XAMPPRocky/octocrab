use octocrab::Octocrab;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

fn sample_config_json() -> serde_json::Value {
    json!({
        "id": 1325,
        "target_type": "enterprise",
        "name": "High risk settings",
        "description": "This is a code security configuration",
        "advanced_security": "enabled",
        "dependency_graph": "enabled",
        "dependabot_alerts": "enabled",
        "dependabot_security_updates": "not_set",
        "code_scanning_default_setup": "disabled",
        "secret_scanning": "enabled",
        "secret_scanning_push_protection": "disabled",
        "private_vulnerability_reporting": "disabled",
        "enforcement": "enforced",
        "created_at": "2024-05-01T00:00:00Z",
        "updated_at": "2024-05-01T00:00:00Z"
    })
}

#[tokio::test]
async fn test_enterprise_code_security_configurations() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 1. List enterprise configs
    Mock::given(method("GET"))
        .and(path("/enterprises/my-ent/code-security/configurations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([sample_config_json()])))
        .mount(&mock_server)
        .await;

    let list = octocrab
        .enterprises("my-ent")
        .code_security()
        .list()
        .send()
        .await
        .unwrap();
    assert_eq!(list.items.len(), 1);

    // 2. Create enterprise config
    Mock::given(method("POST"))
        .and(path("/enterprises/my-ent/code-security/configurations"))
        .respond_with(ResponseTemplate::new(201).set_body_json(sample_config_json()))
        .mount(&mock_server)
        .await;

    let created = octocrab
        .enterprises("my-ent")
        .code_security()
        .create(&json!({ "name": "High risk settings" }))
        .await
        .unwrap();
    assert_eq!(created.id, 1325);

    // 3. Get defaults
    Mock::given(method("GET"))
        .and(path(
            "/enterprises/my-ent/code-security/configurations/defaults",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "default_for_new_repos": "all",
                "configuration": sample_config_json()
            }
        ])))
        .mount(&mock_server)
        .await;

    let defaults = octocrab
        .enterprises("my-ent")
        .code_security()
        .get_defaults()
        .await
        .unwrap();
    assert_eq!(defaults.len(), 1);

    // 4. Get by ID
    Mock::given(method("GET"))
        .and(path(
            "/enterprises/my-ent/code-security/configurations/1325",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_config_json()))
        .mount(&mock_server)
        .await;

    let config = octocrab
        .enterprises("my-ent")
        .code_security()
        .get(1325)
        .await
        .unwrap();
    assert_eq!(config.name, "High risk settings");

    // 5. Update
    Mock::given(method("PATCH"))
        .and(path(
            "/enterprises/my-ent/code-security/configurations/1325",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_config_json()))
        .mount(&mock_server)
        .await;

    let updated = octocrab
        .enterprises("my-ent")
        .code_security()
        .update(1325, &json!({ "description": "Updated" }))
        .await
        .unwrap();
    assert_eq!(updated.id, 1325);

    // 6. Delete
    Mock::given(method("DELETE"))
        .and(path(
            "/enterprises/my-ent/code-security/configurations/1325",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    assert!(octocrab
        .enterprises("my-ent")
        .code_security()
        .delete(1325)
        .await
        .is_ok());

    // 7. Attach
    Mock::given(method("POST"))
        .and(path(
            "/enterprises/my-ent/code-security/configurations/1325/attach",
        ))
        .respond_with(ResponseTemplate::new(202))
        .mount(&mock_server)
        .await;

    assert!(octocrab
        .enterprises("my-ent")
        .code_security()
        .attach(1325, &json!({ "scope": "all" }))
        .await
        .is_ok());

    // 8. Set defaults
    Mock::given(method("PUT"))
        .and(path(
            "/enterprises/my-ent/code-security/configurations/1325/defaults",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "default_for_new_repos": "all"
        })))
        .mount(&mock_server)
        .await;

    let set_def = octocrab
        .enterprises("my-ent")
        .code_security()
        .set_defaults(1325, &json!({ "default_for_new_repos": "all" }))
        .await
        .unwrap();
    assert_eq!(set_def.default_for_new_repos.as_deref(), Some("all"));

    // 9. Repositories
    Mock::given(method("GET"))
        .and(path(
            "/enterprises/my-ent/code-security/configurations/1325/repositories",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "status": "attached",
                "repository": { "id": 123 }
            }
        ])))
        .mount(&mock_server)
        .await;

    let repos = octocrab
        .enterprises("my-ent")
        .code_security()
        .repositories(1325)
        .send()
        .await
        .unwrap();
    assert_eq!(repos.items.len(), 1);
}

#[tokio::test]
async fn test_org_and_repo_code_security_configurations() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 10. List org configs
    Mock::given(method("GET"))
        .and(path("/orgs/my-org/code-security/configurations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([sample_config_json()])))
        .mount(&mock_server)
        .await;

    let list = octocrab
        .orgs("my-org")
        .code_security()
        .list()
        .send()
        .await
        .unwrap();
    assert_eq!(list.items.len(), 1);

    // 11. Create org config
    Mock::given(method("POST"))
        .and(path("/orgs/my-org/code-security/configurations"))
        .respond_with(ResponseTemplate::new(201).set_body_json(sample_config_json()))
        .mount(&mock_server)
        .await;

    let created = octocrab
        .orgs("my-org")
        .code_security()
        .create(&json!({ "name": "Org settings" }))
        .await
        .unwrap();
    assert_eq!(created.id, 1325);

    // 12. Get defaults
    Mock::given(method("GET"))
        .and(path("/orgs/my-org/code-security/configurations/defaults"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "default_for_new_repos": "all",
                "configuration": sample_config_json()
            }
        ])))
        .mount(&mock_server)
        .await;

    let defaults = octocrab
        .orgs("my-org")
        .code_security()
        .get_defaults()
        .await
        .unwrap();
    assert_eq!(defaults.len(), 1);

    // 13. Detach
    Mock::given(method("DELETE"))
        .and(path("/orgs/my-org/code-security/configurations/detach"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    assert!(octocrab
        .orgs("my-org")
        .code_security()
        .detach(&json!({ "selected_repository_ids": [123] }))
        .await
        .is_ok());

    // 14. Get by ID
    Mock::given(method("GET"))
        .and(path("/orgs/my-org/code-security/configurations/1325"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_config_json()))
        .mount(&mock_server)
        .await;

    let config = octocrab
        .orgs("my-org")
        .code_security()
        .get(1325)
        .await
        .unwrap();
    assert_eq!(config.id, 1325);

    // 15. Update
    Mock::given(method("PATCH"))
        .and(path("/orgs/my-org/code-security/configurations/1325"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_config_json()))
        .mount(&mock_server)
        .await;

    let updated = octocrab
        .orgs("my-org")
        .code_security()
        .update(1325, &json!({ "description": "Updated" }))
        .await
        .unwrap();
    assert_eq!(updated.id, 1325);

    // 16. Delete
    Mock::given(method("DELETE"))
        .and(path("/orgs/my-org/code-security/configurations/1325"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    assert!(octocrab
        .orgs("my-org")
        .code_security()
        .delete(1325)
        .await
        .is_ok());

    // 17. Attach
    Mock::given(method("POST"))
        .and(path(
            "/orgs/my-org/code-security/configurations/1325/attach",
        ))
        .respond_with(ResponseTemplate::new(202))
        .mount(&mock_server)
        .await;

    assert!(octocrab
        .orgs("my-org")
        .code_security()
        .attach(1325, &json!({ "scope": "all" }))
        .await
        .is_ok());

    // 18. Set defaults
    Mock::given(method("PUT"))
        .and(path(
            "/orgs/my-org/code-security/configurations/1325/defaults",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "default_for_new_repos": "all"
        })))
        .mount(&mock_server)
        .await;

    let set_def = octocrab
        .orgs("my-org")
        .code_security()
        .set_defaults(1325, &json!({ "default_for_new_repos": "all" }))
        .await
        .unwrap();
    assert_eq!(set_def.default_for_new_repos.as_deref(), Some("all"));

    // 19. Repositories
    Mock::given(method("GET"))
        .and(path(
            "/orgs/my-org/code-security/configurations/1325/repositories",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "status": "attached",
                "repository": { "id": 123 }
            }
        ])))
        .mount(&mock_server)
        .await;

    let repos = octocrab
        .orgs("my-org")
        .code_security()
        .repositories(1325)
        .send()
        .await
        .unwrap();
    assert_eq!(repos.items.len(), 1);

    // 20. Repo code security configuration
    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/code-security-configuration"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "attached",
            "configuration": sample_config_json()
        })))
        .mount(&mock_server)
        .await;

    let repo_config = octocrab
        .repos("owner", "repo")
        .code_security_configuration()
        .await
        .unwrap();
    assert_eq!(repo_config.status, "attached");
    assert_eq!(repo_config.configuration.unwrap().id, 1325);
}
