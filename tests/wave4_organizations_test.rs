use octocrab::Octocrab;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_org_insights_and_artifacts() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 1. Insights: summary-stats
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/insights/api/summary-stats"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_request_count": 12345
        })))
        .mount(&mock_server)
        .await;

    let summary = octocrab
        .orgs("test-org")
        .insights()
        .summary_stats()
        .await
        .unwrap();
    assert_eq!(summary["total_request_count"], 12345);

    // 2. Artifact metadata: create deployment record
    Mock::given(method("POST"))
        .and(path("/orgs/test-org/artifacts/metadata/deployment-record"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "total_count": 1,
            "deployment_records": []
        })))
        .mount(&mock_server)
        .await;

    let res = octocrab
        .orgs("test-org")
        .artifact_metadata()
        .create_deployment_record(&json!({"name": "deploy-1"}))
        .await
        .unwrap();
    assert_eq!(res.total_count, Some(1));

    // 3. Artifact metadata: list deployment records
    Mock::given(method("GET"))
        .and(path(
            "/orgs/test-org/artifacts/sha256:abc/metadata/deployment-records",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "deployment_records": []
        })))
        .mount(&mock_server)
        .await;

    let list_res = octocrab
        .orgs("test-org")
        .artifact_metadata()
        .list_deployment_records("sha256:abc")
        .await
        .unwrap();
    assert_eq!(list_res.total_count, Some(1));
}

#[tokio::test]
async fn test_org_attestations_and_properties() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 1. Attestations: delete by digest
    Mock::given(method("DELETE"))
        .and(path("/orgs/test-org/attestations/digest/sha256:testdigest"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .orgs("test-org")
        .attestations()
        .delete_by_digest("sha256:testdigest")
        .await
        .unwrap();

    // 2. External properties: list installations
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/properties/installations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {"id": 42, "app_slug": "custom-app"}
        ])))
        .mount(&mock_server)
        .await;

    let installations = octocrab
        .orgs("test-org")
        .external_properties()
        .list_installations()
        .await
        .unwrap();
    assert_eq!(installations.as_array().unwrap().len(), 1);

    // 3. Issue fields: list
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/issue-fields"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {"id": "field_1", "name": "Priority"}
        ])))
        .mount(&mock_server)
        .await;

    let fields = octocrab
        .orgs("test-org")
        .issue_fields()
        .list()
        .await
        .unwrap();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].name, "Priority");

    // 4. Issue types: list
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/issue-types"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {"id": "type_1", "name": "Bug"}
        ])))
        .mount(&mock_server)
        .await;

    let types = octocrab
        .orgs("test-org")
        .issue_types()
        .list()
        .await
        .unwrap();
    assert_eq!(types.len(), 1);
    assert_eq!(types[0].name, "Bug");
}

#[tokio::test]
async fn test_org_network_immutable_roles_and_verbs() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 1. Network configurations: list
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/settings/network-configurations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "network_configurations": [
                {"id": "nc_1", "name": "Default Config"}
            ]
        })))
        .mount(&mock_server)
        .await;

    let net_configs = octocrab
        .orgs("test-org")
        .network_configurations()
        .list()
        .await
        .unwrap();
    assert_eq!(net_configs.total_count, 1);
    assert_eq!(net_configs.network_configurations[0].name, "Default Config");

    // 2. Immutable releases: get settings
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/settings/immutable-releases"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "enforced_repositories": "all"
        })))
        .mount(&mock_server)
        .await;

    let imm_settings = octocrab
        .orgs("test-org")
        .immutable_releases()
        .get_settings()
        .await
        .unwrap();
    assert_eq!(imm_settings.enforced_repositories.as_deref(), Some("all"));

    // 3. Roles: remove all for team and user
    Mock::given(method("DELETE"))
        .and(path("/orgs/test-org/organization-roles/teams/devs"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .orgs("test-org")
        .roles()
        .remove_all_for_team("devs")
        .await
        .unwrap();

    Mock::given(method("DELETE"))
        .and(path("/orgs/test-org/organization-roles/users/octocat"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .orgs("test-org")
        .roles()
        .remove_all_for_user("octocat")
        .await
        .unwrap();

    // 4. PATs: batch update access
    Mock::given(method("POST"))
        .and(path("/orgs/test-org/personal-access-tokens"))
        .respond_with(ResponseTemplate::new(202))
        .mount(&mock_server)
        .await;

    octocrab
        .orgs("test-org")
        .personal_access_tokens()
        .update_access_for_tokens(
            [123u64, 456u64],
            octocrab::models::orgs::personal_access_tokens::PatAction::Revoke,
        )
        .await
        .unwrap();

    // 5. Rulesets: history and version
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/rulesets/1/history"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "version_id": 3,
                "actor": {"id": 1, "type": "User"},
                "updated_at": "2024-10-23T16:29:47Z"
            }
        ])))
        .mount(&mock_server)
        .await;

    let history = octocrab
        .orgs("test-org")
        .rulesets()
        .history(1u64)
        .await
        .unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].version_id, 3);

    // 6. Direct org methods: remove_member, remove_membership, delete
    Mock::given(method("DELETE"))
        .and(path("/orgs/test-org/members/octocat"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .orgs("test-org")
        .remove_member("octocat")
        .await
        .unwrap();

    Mock::given(method("DELETE"))
        .and(path("/orgs/test-org/memberships/octocat"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .orgs("test-org")
        .remove_membership("octocat")
        .await
        .unwrap();

    Mock::given(method("DELETE"))
        .and(path("/orgs/test-org"))
        .respond_with(ResponseTemplate::new(202))
        .mount(&mock_server)
        .await;

    octocrab.orgs("test-org").delete().await.unwrap();

    // 7. Verify HTTP verb fixes:
    // add_or_update_membership must use PUT
    Mock::given(method("PUT"))
        .and(path("/orgs/test-org/memberships/octocat"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "url": "https://api.github.com/orgs/test-org/memberships/octocat",
            "state": "active",
            "role": "member",
            "organization_url": "https://api.github.com/orgs/test-org",
            "user": {
                "login": "octocat",
                "id": 1,
                "node_id": "MDQ6VXNlcjE=",
                "avatar_url": "https://github.com/images/error/octocat_happy.gif",
                "gravatar_id": "",
                "url": "https://api.github.com/users/octocat",
                "html_url": "https://github.com/octocat",
                "followers_url": "https://api.github.com/users/octocat/followers",
                "following_url": "https://api.github.com/users/octocat/following{/other_user}",
                "gists_url": "https://api.github.com/users/octocat/gists{/gist_id}",
                "starred_url": "https://api.github.com/users/octocat/starred{/owner}{/repo}",
                "subscriptions_url": "https://api.github.com/users/octocat/subscriptions",
                "organizations_url": "https://api.github.com/users/octocat/orgs",
                "repos_url": "https://api.github.com/users/octocat/repos",
                "events_url": "https://api.github.com/users/octocat/events{/privacy}",
                "received_events_url": "https://api.github.com/users/octocat/received_events",
                "type": "User",
                "site_admin": false
            },
            "organization": {
                "login": "test-org",
                "id": 1978534,
                "node_id": "MDEyOk9yZ2FuaXphdGlvbjE5Nzg1MzQ=",
                "url": "https://api.github.com/orgs/test-org",
                "repos_url": "https://api.github.com/orgs/test-org/repos",
                "events_url": "https://api.github.com/orgs/test-org/events",
                "hooks_url": "https://api.github.com/orgs/test-org/hooks",
                "issues_url": "https://api.github.com/orgs/test-org/issues",
                "members_url": "https://api.github.com/orgs/test-org/members{/member}",
                "public_members_url": "https://api.github.com/orgs/test-org/public_members{/member}",
                "avatar_url": "https://avatars.githubusercontent.com/u/1978534?v=4",
                "description": "Test organization"
            }
        })))
        .mount(&mock_server)
        .await;

    octocrab
        .orgs("test-org")
        .add_or_update_membership("octocat", None)
        .await
        .unwrap();

    // set_security_product_enablement must use POST
    Mock::given(method("POST"))
        .and(path("/orgs/test-org/secret_scanning/enable_all"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .orgs("test-org")
        .set_security_product_enablement(
            octocrab::models::orgs::security::SecurityProduct::SecretScanning,
            octocrab::models::orgs::security::SecurityEnablement::EnableAll,
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn test_all_organizations() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    let orgs_json = json!([
        {
            "login": "github",
            "id": 1,
            "node_id": "MDEyOk9yZ2FuaXphdGlvbjE=",
            "url": "https://api.github.com/orgs/github",
            "repos_url": "https://api.github.com/orgs/github/repos",
            "events_url": "https://api.github.com/orgs/github/events",
            "hooks_url": "https://api.github.com/orgs/github/hooks",
            "issues_url": "https://api.github.com/orgs/github/issues",
            "members_url": "https://api.github.com/orgs/github/members{/member}",
            "public_members_url": "https://api.github.com/orgs/github/public_members{/member}",
            "avatar_url": "https://avatars.githubusercontent.com/u/1?v=4",
            "description": "A great organization"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/organizations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&orgs_json))
        .mount(&mock_server)
        .await;

    let page = octocrab
        .all_organizations()
        .since(0u64)
        .per_page(100)
        .send()
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].login, "github");
}
