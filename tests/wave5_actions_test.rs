use octocrab::models::actions::{
    ActionsCacheRetentionLimit, ActionsCacheStorageLimit, ActionsPoliciesResponse, ActionsPolicy,
    ConcurrencyGroupDetails, ConcurrencyGroupsResponse, HostedRunner, HostedRunnersResponse,
    RunnerGroup, RunnerGroupsResponse, RunnerVersionDeprecation,
};
use octocrab::Octocrab;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_hosted_runners_and_runner_groups() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 1. List hosted runners
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/actions/hosted-runners"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "runners": [{
                "id": 42,
                "name": "my-hosted-runner",
                "runner_group_id": 1,
                "platform": "linux-x64",
                "image_details": {
                    "id": "ubuntu-22.04",
                    "size_gb": 64,
                    "display_name": "Ubuntu 22.04"
                },
                "machine_size_details": {
                    "id": "4-core",
                    "cpu_cores": 4,
                    "memory_gb": 16,
                    "storage_gb": 150
                },
                "status": "Ready",
                "maximum_runners": 10
            }]
        })))
        .mount(&mock_server)
        .await;

    let res: HostedRunnersResponse = octocrab
        .orgs("test-org")
        .actions()
        .hosted_runners()
        .list()
        .await
        .unwrap();
    assert_eq!(res.total_count, 1);
    assert_eq!(res.runners[0].name, "my-hosted-runner");

    // 2. Create hosted runner
    Mock::given(method("POST"))
        .and(path("/orgs/test-org/actions/hosted-runners"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": 43,
            "name": "new-runner",
            "runner_group_id": 1,
            "platform": "linux-x64",
            "image_details": { "id": "ubuntu-22.04" },
            "machine_size_details": { "id": "4-core" },
            "status": "Provisioning",
            "maximum_runners": 5
        })))
        .mount(&mock_server)
        .await;

    let created: HostedRunner = octocrab
        .orgs("test-org")
        .actions()
        .hosted_runners()
        .create(&json!({
            "name": "new-runner",
            "image": { "id": "ubuntu-22.04" },
            "size": "4-core",
            "runner_group_id": 1,
            "maximum_runners": 5
        }))
        .await
        .unwrap();
    assert_eq!(created.id, 43);
    assert_eq!(created.name, "new-runner");

    // 3. Delete hosted runner
    Mock::given(method("DELETE"))
        .and(path("/orgs/test-org/actions/hosted-runners/43"))
        .respond_with(ResponseTemplate::new(202))
        .mount(&mock_server)
        .await;

    octocrab
        .orgs("test-org")
        .actions()
        .hosted_runners()
        .delete(43)
        .await
        .unwrap();

    // 4. Runner groups list
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/actions/runner-groups"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "runner_groups": [{
                "id": 1,
                "name": "Default",
                "visibility": "all",
                "default": true,
                "runners_url": "https://api.github.com/orgs/test-org/actions/runner-groups/1/runners",
                "inherited": false,
                "allows_public_repositories": false
            }]
        })))
        .mount(&mock_server)
        .await;

    let groups: RunnerGroupsResponse = octocrab
        .orgs("test-org")
        .actions()
        .runner_groups()
        .list()
        .await
        .unwrap();
    assert_eq!(groups.total_count, 1);
    assert_eq!(groups.runner_groups[0].name, "Default");

    // 5. Runner group create
    Mock::given(method("POST"))
        .and(path("/orgs/test-org/actions/runner-groups"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": 2,
            "name": "custom-group",
            "visibility": "selected",
            "default": false,
            "runners_url": "https://api.github.com/orgs/test-org/actions/runner-groups/2/runners",
            "inherited": false,
            "allows_public_repositories": false
        })))
        .mount(&mock_server)
        .await;

    let new_group: RunnerGroup = octocrab
        .orgs("test-org")
        .actions()
        .runner_groups()
        .create(&json!({
            "name": "custom-group",
            "visibility": "selected"
        }))
        .await
        .unwrap();
    assert_eq!(new_group.id, octocrab::models::RunnerGroupId(2));
    assert_eq!(new_group.name, "custom-group");
}

#[tokio::test]
async fn test_actions_policies_and_concurrency_groups() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 1. Org Actions policies list
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/actions/policies"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "policies": [{
                "id": 101,
                "name": "org-policy-1",
                "enabled": true
            }]
        })))
        .mount(&mock_server)
        .await;

    let policies: ActionsPoliciesResponse = octocrab
        .orgs("test-org")
        .actions()
        .policies()
        .list()
        .await
        .unwrap();
    assert_eq!(policies.total_count, 1);
    assert_eq!(policies.policies[0].name, "org-policy-1");

    // 2. Org Actions policy get
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/actions/policies/101"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 101,
            "name": "org-policy-1",
            "enabled": true
        })))
        .mount(&mock_server)
        .await;

    let policy: ActionsPolicy = octocrab
        .orgs("test-org")
        .actions()
        .policies()
        .get(101)
        .await
        .unwrap();
    assert_eq!(policy.id, 101);

    // 3. Repo Concurrency groups list
    Mock::given(method("GET"))
        .and(path("/repos/test-owner/test-repo/actions/concurrency_groups"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "concurrency_groups": [{
                "group_name": "ci-concurrency",
                "group_url": "https://api.github.com/repos/test-owner/test-repo/actions/concurrency_groups/ci-concurrency",
                "last_acquired_at": "2026-10-17T00:00:00Z"
            }]
        })))
        .mount(&mock_server)
        .await;

    let cgroups: ConcurrencyGroupsResponse = octocrab
        .repos("test-owner", "test-repo")
        .actions()
        .concurrency_groups()
        .list()
        .await
        .unwrap();
    assert_eq!(cgroups.total_count, 1);
    assert_eq!(cgroups.concurrency_groups[0].group_name, "ci-concurrency");

    // 4. Repo Concurrency group get
    Mock::given(method("GET"))
        .and(path("/repos/test-owner/test-repo/actions/concurrency_groups/ci-concurrency"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "group_name": "ci-concurrency",
            "group_url": "https://api.github.com/repos/test-owner/test-repo/actions/concurrency_groups/ci-concurrency",
            "total_count": 1,
            "group_members": []
        })))
        .mount(&mock_server)
        .await;

    let cgroup: ConcurrencyGroupDetails = octocrab
        .repos("test-owner", "test-repo")
        .actions()
        .concurrency_groups()
        .get("ci-concurrency")
        .await
        .unwrap();
    assert_eq!(cgroup.group_name, "ci-concurrency");
    assert_eq!(cgroup.total_count, 1);
}

#[tokio::test]
async fn test_cache_oidc_and_deprecation() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 1. Enterprise cache retention limit
    Mock::given(method("GET"))
        .and(path("/enterprises/test-ent/actions/cache/retention-limit"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "max_cache_retention_days": 90
        })))
        .mount(&mock_server)
        .await;

    let ent_ret: ActionsCacheRetentionLimit = octocrab
        .enterprises("test-ent")
        .actions()
        .cache()
        .get_retention_limit()
        .await
        .unwrap();
    assert_eq!(ent_ret.max_cache_retention_days, 90);

    // 2. Org cache retention limit
    Mock::given(method("GET"))
        .and(path(
            "/organizations/test-org/actions/cache/retention-limit",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "max_cache_retention_days": 60
        })))
        .mount(&mock_server)
        .await;

    let org_ret: ActionsCacheRetentionLimit = octocrab
        .orgs("test-org")
        .actions()
        .cache()
        .get_retention_limit()
        .await
        .unwrap();
    assert_eq!(org_ret.max_cache_retention_days, 60);

    // 3. Repo cache storage limit
    Mock::given(method("GET"))
        .and(path(
            "/repos/test-owner/test-repo/actions/cache/storage-limit",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "max_cache_size_gb": 10
        })))
        .mount(&mock_server)
        .await;

    let repo_storage: ActionsCacheStorageLimit = octocrab
        .repos("test-owner", "test-repo")
        .actions()
        .cache()
        .get_storage_limit()
        .await
        .unwrap();
    assert_eq!(repo_storage.max_cache_size_gb, 10);

    // 4. Enterprise OIDC custom properties
    Mock::given(method("GET"))
        .and(path(
            "/enterprises/test-ent/actions/oidc/customization/properties/repo",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "include_claim_keys": ["repo", "context"]
        })))
        .mount(&mock_server)
        .await;

    let oidc_props = octocrab
        .enterprises("test-ent")
        .actions()
        .oidc()
        .list_repo_properties()
        .await
        .unwrap();
    assert_eq!(oidc_props["include_claim_keys"][0], "repo");

    // 5. Org runner deprecation schedule
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/actions/runners/deprecations/2.300.0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "runner_version": "2.300.0",
            "runtime_deprecates_at": "2026-12-31T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let deprecation: RunnerVersionDeprecation = octocrab
        .orgs("test-org")
        .actions()
        .get_runner_deprecation("2.300.0")
        .await
        .unwrap();
    assert!(deprecation.runtime_deprecates_at.is_some());
    assert_eq!(deprecation.runner_version, "2.300.0");
}

#[tokio::test]
async fn test_workflows_and_runs() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 1. Delete workflow run
    Mock::given(method("DELETE"))
        .and(path("/repos/test-owner/test-repo/actions/runs/123"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .workflows("test-owner", "test-repo")
        .delete_run(123u64)
        .await
        .unwrap();

    // 2. Review pending deployments
    Mock::given(method("POST"))
        .and(path(
            "/repos/test-owner/test-repo/actions/runs/123/pending_deployments",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&mock_server)
        .await;

    let deployments = octocrab
        .workflows("test-owner", "test-repo")
        .review_pending_deployments(123u64, &[1], "approved", "LGTM")
        .await
        .unwrap();
    assert_eq!(deployments, json!([]));
}
