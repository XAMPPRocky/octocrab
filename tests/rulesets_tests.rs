use http::StatusCode;
use wiremock::{
    matchers::{body_json, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::rulesets::{
    BypassActorType, BypassMode, RuleSuiteId, RuleSuiteResult, RulesetEnforcement, RulesetId,
    RulesetTarget, UpdateRuleset,
};
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "owner";
const REPO: &str = "repo";
const ORG: &str = "org";

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

fn sample_ruleset_json(id: u64, name: &str, source: &str, source_type: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "name": name,
        "target": "branch",
        "source_type": source_type,
        "source": source,
        "enforcement": "active",
        "bypass_actors": [
            {
                "actor_id": 1,
                "actor_type": "Team",
                "bypass_mode": "always"
            }
        ],
        "current_user_can_bypass": "always",
        "node_id": "node123",
        "conditions": {
            "ref_name": {
                "include": ["refs/heads/main"],
                "exclude": []
            }
        },
        "rules": [
            {
                "type": "deletion"
            },
            {
                "type": "commit_author_email_pattern",
                "parameters": {
                    "operator": "contains",
                    "pattern": "github"
                }
            }
        ]
    })
}

fn sample_rule_suite_summary_json(id: u64) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "actor_id": 100,
        "actor_name": "octocat",
        "before_sha": "1111111111111111111111111111111111111111",
        "after_sha": "2222222222222222222222222222222222222222",
        "ref": "refs/heads/main",
        "repository_id": 123456,
        "repository_name": REPO,
        "pushed_at": "2022-11-28T00:00:00Z",
        "result": "pass",
        "evaluation_result": "pass"
    })
}

fn sample_rule_suite_json(id: u64) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "actor_id": 100,
        "actor_name": "octocat",
        "before_sha": "1111111111111111111111111111111111111111",
        "after_sha": "2222222222222222222222222222222222222222",
        "ref": "refs/heads/main",
        "repository_id": 123456,
        "repository_name": REPO,
        "pushed_at": "2022-11-28T00:00:00Z",
        "result": "pass",
        "evaluation_result": "pass",
        "rule_evaluations": [
            {
                "rule_source": {
                    "type": "repository",
                    "id": 1,
                    "name": "default-rules"
                },
                "enforcement": "active",
                "result": "pass",
                "rule_type": "deletion",
                "details": null
            }
        ]
    })
}

// 1. List Repository Rulesets
#[tokio::test]
async fn should_list_repo_rulesets() {
    let mock_server = MockServer::start().await;
    let expected = serde_json::json!([sample_ruleset_json(
        1,
        "default-rules",
        &format!("{OWNER}/{REPO}"),
        "Repository"
    )]);

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/rulesets")))
        .and(query_param("includes_parents", "true"))
        .and(query_param("targets", "branch,tag"))
        .and(query_param("per_page", "30"))
        .and(query_param("page", "1"))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&expected))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "list repo rulesets").await;

    let client = setup_octocrab(&mock_server.uri());
    let res = client
        .repos(OWNER, REPO)
        .rulesets()
        .list()
        .includes_parents(true)
        .targets("branch,tag")
        .per_page(30)
        .page(1u32)
        .send()
        .await
        .unwrap();

    assert_eq!(res.len(), 1);
    assert_eq!(res[0].id, RulesetId(1));
    assert_eq!(res[0].name, "default-rules");
    assert_eq!(res[0].target, Some(RulesetTarget::Branch));
    assert_eq!(res[0].bypass_actors.len(), 1);
    assert_eq!(res[0].bypass_actors[0].actor_type, BypassActorType::Team);
    assert_eq!(res[0].bypass_actors[0].bypass_mode, BypassMode::Always);
}

// 2. Get Repository Ruleset
#[tokio::test]
async fn should_get_repo_ruleset() {
    let mock_server = MockServer::start().await;
    let expected =
        sample_ruleset_json(1, "default-rules", &format!("{OWNER}/{REPO}"), "Repository");

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/rulesets/1")))
        .and(query_param("includes_parents", "true"))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&expected))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "get repo ruleset").await;

    let client = setup_octocrab(&mock_server.uri());
    let res = client
        .repos(OWNER, REPO)
        .rulesets()
        .get(1u64)
        .includes_parents(true)
        .send()
        .await
        .unwrap();

    assert_eq!(res.id, RulesetId(1));
    assert_eq!(res.name, "default-rules");
    assert_eq!(res.rules.len(), 2);
    assert_eq!(res.rules[0].rule_type, "deletion");
}

// 3. Create Repository Ruleset
#[tokio::test]
async fn should_create_repo_ruleset() {
    let mock_server = MockServer::start().await;
    let payload = UpdateRuleset::new()
        .name("new-rules")
        .target(RulesetTarget::Branch)
        .enforcement(RulesetEnforcement::Active);
    let expected = sample_ruleset_json(10, "new-rules", &format!("{OWNER}/{REPO}"), "Repository");

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/rulesets")))
        .and(body_json(&payload))
        .respond_with(ResponseTemplate::new(StatusCode::CREATED).set_body_json(&expected))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "create repo ruleset").await;

    let client = setup_octocrab(&mock_server.uri());
    let res = client
        .repos(OWNER, REPO)
        .rulesets()
        .create(&payload)
        .await
        .unwrap();

    assert_eq!(res.id, RulesetId(10));
    assert_eq!(res.name, "new-rules");
}

// 4. Update Repository Ruleset
#[tokio::test]
async fn should_update_repo_ruleset() {
    let mock_server = MockServer::start().await;
    let payload = UpdateRuleset::new()
        .name("updated-rules")
        .enforcement(RulesetEnforcement::Evaluate);
    let expected =
        sample_ruleset_json(1, "updated-rules", &format!("{OWNER}/{REPO}"), "Repository");

    Mock::given(method("PUT"))
        .and(path(format!("/repos/{OWNER}/{REPO}/rulesets/1")))
        .and(body_json(&payload))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&expected))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "update repo ruleset").await;

    let client = setup_octocrab(&mock_server.uri());
    let res = client
        .repos(OWNER, REPO)
        .rulesets()
        .update(1u64, &payload)
        .await
        .unwrap();

    assert_eq!(res.id, RulesetId(1));
    assert_eq!(res.name, "updated-rules");
}

// 5. Delete Repository Ruleset
#[tokio::test]
async fn should_delete_repo_ruleset() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!("/repos/{OWNER}/{REPO}/rulesets/1")))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "delete repo ruleset").await;

    let client = setup_octocrab(&mock_server.uri());
    client
        .repos(OWNER, REPO)
        .rulesets()
        .delete(1u64)
        .await
        .unwrap();
}

// 6. List Repository Rule Suites
#[tokio::test]
async fn should_list_repo_rule_suites() {
    let mock_server = MockServer::start().await;
    let expected = serde_json::json!([sample_rule_suite_summary_json(42)]);

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/rulesets/rule-suites")))
        .and(query_param("ref", "refs/heads/main"))
        .and(query_param("time_period", "day"))
        .and(query_param("rule_suite_result", "pass"))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&expected))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "list repo rule suites").await;

    let client = setup_octocrab(&mock_server.uri());
    let res = client
        .repos(OWNER, REPO)
        .rulesets()
        .rule_suites()
        .list()
        .ref_name("refs/heads/main")
        .time_period("day")
        .rule_suite_result("pass")
        .send()
        .await
        .unwrap();

    assert_eq!(res.len(), 1);
    assert_eq!(res[0].id, RuleSuiteId(42));
    assert_eq!(res[0].result, RuleSuiteResult::Pass);
    assert_eq!(res[0].ref_name, "refs/heads/main");
}

// 7. Get Repository Rule Suite
#[tokio::test]
async fn should_get_repo_rule_suite() {
    let mock_server = MockServer::start().await;
    let expected = sample_rule_suite_json(42);

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/rulesets/rule-suites/42"
        )))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&expected))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "get repo rule suite").await;

    let client = setup_octocrab(&mock_server.uri());
    let res = client
        .repos(OWNER, REPO)
        .rulesets()
        .rule_suites()
        .get(42u64)
        .await
        .unwrap();

    assert_eq!(res.id, RuleSuiteId(42));
    assert_eq!(res.rule_evaluations.len(), 1);
    assert_eq!(res.rule_evaluations[0].rule_type, "deletion");
}

// 8. Get Rules for a Branch
#[tokio::test]
async fn should_get_rules_for_branch() {
    let mock_server = MockServer::start().await;
    let expected = serde_json::json!([
        {
            "type": "deletion",
            "ruleset_source_type": "Repository",
            "ruleset_id": 1
        }
    ]);

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/rules/branches/main")))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&expected))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "get rules for branch").await;

    let client = setup_octocrab(&mock_server.uri());
    let res = client
        .repos(OWNER, REPO)
        .rules_for_branch("main")
        .send()
        .await
        .unwrap();

    assert_eq!(res.len(), 1);
    assert_eq!(res[0].rule_type, "deletion");
    assert_eq!(res[0].ruleset_id, Some(RulesetId(1)));
}

// 9. List Organization Rulesets
#[tokio::test]
async fn should_list_org_rulesets() {
    let mock_server = MockServer::start().await;
    let expected = serde_json::json!([sample_ruleset_json(2, "org-rules", ORG, "Organization")]);

    Mock::given(method("GET"))
        .and(path(format!("/orgs/{ORG}/rulesets")))
        .and(query_param("targets", "branch"))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&expected))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "list org rulesets").await;

    let client = setup_octocrab(&mock_server.uri());
    let res = client
        .orgs(ORG)
        .rulesets()
        .list()
        .targets("branch")
        .send()
        .await
        .unwrap();

    assert_eq!(res.len(), 1);
    assert_eq!(res[0].id, RulesetId(2));
    assert_eq!(res[0].name, "org-rules");
}

// 10. Get Organization Ruleset
#[tokio::test]
async fn should_get_org_ruleset() {
    let mock_server = MockServer::start().await;
    let expected = sample_ruleset_json(2, "org-rules", ORG, "Organization");

    Mock::given(method("GET"))
        .and(path(format!("/orgs/{ORG}/rulesets/2")))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&expected))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "get org ruleset").await;

    let client = setup_octocrab(&mock_server.uri());
    let res = client.orgs(ORG).rulesets().get(2u64).await.unwrap();

    assert_eq!(res.id, RulesetId(2));
    assert_eq!(res.name, "org-rules");
}

// 11. Create Organization Ruleset
#[tokio::test]
async fn should_create_org_ruleset() {
    let mock_server = MockServer::start().await;
    let payload = UpdateRuleset::new()
        .name("new-org-rules")
        .target(RulesetTarget::Branch)
        .enforcement(RulesetEnforcement::Active);
    let expected = sample_ruleset_json(20, "new-org-rules", ORG, "Organization");

    Mock::given(method("POST"))
        .and(path(format!("/orgs/{ORG}/rulesets")))
        .and(body_json(&payload))
        .respond_with(ResponseTemplate::new(StatusCode::CREATED).set_body_json(&expected))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "create org ruleset").await;

    let client = setup_octocrab(&mock_server.uri());
    let res = client.orgs(ORG).rulesets().create(&payload).await.unwrap();

    assert_eq!(res.id, RulesetId(20));
    assert_eq!(res.name, "new-org-rules");
}

// 12. Update Organization Ruleset
#[tokio::test]
async fn should_update_org_ruleset() {
    let mock_server = MockServer::start().await;
    let payload = UpdateRuleset::new()
        .name("updated-org-rules")
        .enforcement(RulesetEnforcement::Active);
    let expected = sample_ruleset_json(2, "updated-org-rules", ORG, "Organization");

    Mock::given(method("PUT"))
        .and(path(format!("/orgs/{ORG}/rulesets/2")))
        .and(body_json(&payload))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&expected))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "update org ruleset").await;

    let client = setup_octocrab(&mock_server.uri());
    let res = client
        .orgs(ORG)
        .rulesets()
        .update(2u64, &payload)
        .await
        .unwrap();

    assert_eq!(res.id, RulesetId(2));
    assert_eq!(res.name, "updated-org-rules");
}

// 13. Delete Organization Ruleset
#[tokio::test]
async fn should_delete_org_ruleset() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!("/orgs/{ORG}/rulesets/2")))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "delete org ruleset").await;

    let client = setup_octocrab(&mock_server.uri());
    client.orgs(ORG).rulesets().delete(2u64).await.unwrap();
}

// 14. List Organization Rule Suites
#[tokio::test]
async fn should_list_org_rule_suites() {
    let mock_server = MockServer::start().await;
    let expected = serde_json::json!([sample_rule_suite_summary_json(99)]);

    Mock::given(method("GET"))
        .and(path(format!("/orgs/{ORG}/rulesets/rule-suites")))
        .and(query_param("repository_name", REPO))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&expected))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "list org rule suites").await;

    let client = setup_octocrab(&mock_server.uri());
    let res = client
        .orgs(ORG)
        .rulesets()
        .rule_suites()
        .list()
        .repository_name(REPO)
        .send()
        .await
        .unwrap();

    assert_eq!(res.len(), 1);
    assert_eq!(res[0].id, RuleSuiteId(99));
}

// 15. Get Organization Rule Suite
#[tokio::test]
async fn should_get_org_rule_suite() {
    let mock_server = MockServer::start().await;
    let expected = sample_rule_suite_json(99);

    Mock::given(method("GET"))
        .and(path(format!("/orgs/{ORG}/rulesets/rule-suites/99")))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(&expected))
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "get org rule suite").await;

    let client = setup_octocrab(&mock_server.uri());
    let res = client
        .orgs(ORG)
        .rulesets()
        .rule_suites()
        .get(99u64)
        .await
        .unwrap();

    assert_eq!(res.id, RuleSuiteId(99));
    assert_eq!(res.rule_evaluations.len(), 1);
}
