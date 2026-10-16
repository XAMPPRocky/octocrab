use octocrab::Octocrab;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_copilot_endpoints() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 1. Repo cloud agent configuration
    Mock::given(method("GET"))
        .and(path(
            "/repos/octocat/hello-world/copilot/cloud-agent/configuration",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "is_firewall_enabled": true,
            "is_automations_enabled": true
        })))
        .mount(&mock_server)
        .await;

    let config = octocrab
        .repos("octocat", "hello-world")
        .copilot()
        .get_cloud_agent_configuration()
        .await
        .unwrap();
    assert_eq!(config.is_firewall_enabled, Some(true));

    // 2. Enterprise coding agent policy: set, add orgs, remove orgs
    Mock::given(method("PUT"))
        .and(path("/enterprises/my-ent/copilot/policies/coding_agent"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .enterprises("my-ent")
        .copilot()
        .set_coding_agent_policy(&json!({ "policy": "enabled" }))
        .await
        .unwrap();

    Mock::given(method("POST"))
        .and(path(
            "/enterprises/my-ent/copilot/policies/coding_agent/organizations",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .enterprises("my-ent")
        .copilot()
        .add_organizations_to_coding_agent_policy(&json!({ "orgs": ["org1"] }))
        .await
        .unwrap();

    Mock::given(method("DELETE"))
        .and(path(
            "/enterprises/my-ent/copilot/policies/coding_agent/organizations",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .enterprises("my-ent")
        .copilot()
        .remove_organizations_from_coding_agent_policy(&json!({ "orgs": ["org1"] }))
        .await
        .unwrap();

    // 3. Enterprise metrics reports
    Mock::given(method("GET"))
        .and(path(
            "/enterprises/my-ent/copilot/metrics/reports/enterprise-1-day",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "report_day": "2026-10-15",
            "download_links": ["https://example.com/report.csv"]
        })))
        .mount(&mock_server)
        .await;

    let ent_1day = octocrab
        .enterprises("my-ent")
        .copilot()
        .metrics_reports_enterprise_1_day(Some("2026-10-15"))
        .await
        .unwrap();
    assert_eq!(ent_1day.report_day.as_deref(), Some("2026-10-15"));

    Mock::given(method("GET"))
        .and(path(
            "/enterprises/my-ent/copilot/metrics/reports/enterprise-28-day/latest",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "report_start_day": "2026-09-17",
            "report_end_day": "2026-10-15"
        })))
        .mount(&mock_server)
        .await;

    let ent_28day = octocrab
        .enterprises("my-ent")
        .copilot()
        .metrics_reports_enterprise_28_day_latest()
        .await
        .unwrap();
    assert_eq!(ent_28day.report_start_day.as_deref(), Some("2026-09-17"));

    // 4. Org coding agent permissions & content exclusion
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/copilot/coding-agent/permissions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "enabled_repositories": "all"
        })))
        .mount(&mock_server)
        .await;

    let perms = octocrab
        .orgs("test-org")
        .copilot()
        .get_coding_agent_permissions()
        .await
        .unwrap();
    assert_eq!(perms.enabled_repositories.as_deref(), Some("all"));

    Mock::given(method("PUT"))
        .and(path(
            "/orgs/test-org/copilot/coding-agent/permissions/repositories/42",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .orgs("test-org")
        .copilot()
        .enable_coding_agent_repository(42)
        .await
        .unwrap();

    Mock::given(method("DELETE"))
        .and(path(
            "/orgs/test-org/copilot/coding-agent/permissions/repositories/42",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .orgs("test-org")
        .copilot()
        .disable_coding_agent_repository(42)
        .await
        .unwrap();

    Mock::given(method("GET"))
        .and(path("/orgs/test-org/copilot/content_exclusion"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "paths": ["secret/**"]
        })))
        .mount(&mock_server)
        .await;

    let exclusion = octocrab
        .orgs("test-org")
        .copilot()
        .get_content_exclusion()
        .await
        .unwrap();
    assert!(exclusion.get("paths").is_some());
}

#[tokio::test]
async fn test_copilot_spaces_endpoints() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 1. List org spaces
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/copilot-spaces"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "spaces": [
                {
                    "id": 1,
                    "number": 10,
                    "name": "Design Space"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let spaces = octocrab
        .orgs("test-org")
        .copilot_spaces()
        .list()
        .await
        .unwrap();
    assert_eq!(spaces.spaces.len(), 1);
    assert_eq!(spaces.spaces[0].name, "Design Space");

    // 2. Create org space
    Mock::given(method("POST"))
        .and(path("/orgs/test-org/copilot-spaces"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": 2,
            "number": 11,
            "name": "Frontend Space"
        })))
        .mount(&mock_server)
        .await;

    let new_space = octocrab
        .orgs("test-org")
        .copilot_spaces()
        .create(&json!({ "name": "Frontend Space" }))
        .await
        .unwrap();
    assert_eq!(new_space.number, 11);

    // 3. Get org space
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/copilot-spaces/10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 1,
            "number": 10,
            "name": "Design Space"
        })))
        .mount(&mock_server)
        .await;

    let space = octocrab
        .copilot_spaces()
        .org("test-org")
        .get(10)
        .await
        .unwrap();
    assert_eq!(space.id, 1);

    // 4. Set org space
    Mock::given(method("PUT"))
        .and(path("/orgs/test-org/copilot-spaces/10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 1,
            "number": 10,
            "name": "Updated Space"
        })))
        .mount(&mock_server)
        .await;

    let updated = octocrab
        .copilot_spaces()
        .org("test-org")
        .set(10, &json!({ "name": "Updated Space" }))
        .await
        .unwrap();
    assert_eq!(updated.name, "Updated Space");

    // 5. Delete org space
    Mock::given(method("DELETE"))
        .and(path("/orgs/test-org/copilot-spaces/10"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .orgs("test-org")
        .copilot_spaces()
        .delete(10)
        .await
        .unwrap();

    // 6. Collaborators in user space
    Mock::given(method("GET"))
        .and(path("/users/octocat/copilot-spaces/5/collaborators"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "collaborators": [
                {
                    "actor_type": "User",
                    "role": "reader",
                    "login": "monalisa"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let collabs = octocrab
        .users("octocat")
        .copilot_spaces()
        .list_collaborators(5)
        .await
        .unwrap();
    assert_eq!(collabs.collaborators.len(), 1);
    assert_eq!(collabs.collaborators[0].role.as_deref(), Some("reader"));

    // 7. Resources in user space
    Mock::given(method("GET"))
        .and(path("/users/octocat/copilot-spaces/5/resources"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "resources": [
                {
                    "id": 101,
                    "resource_type": "free_text"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let resources = octocrab
        .users("octocat")
        .copilot_spaces()
        .list_resources(5)
        .await
        .unwrap();
    assert_eq!(resources.resources.len(), 1);
    assert_eq!(resources.resources[0].id, 101);
}
