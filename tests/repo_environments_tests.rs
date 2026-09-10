use http::StatusCode;
use wiremock::{
    matchers::{body_json, method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::repos::{
    BranchPolicyType, CustomDeploymentProtectionRule, CustomDeploymentProtectionRules,
    CustomDeploymentRuleApps, DeploymentBranchPolicies, DeploymentBranchPolicy,
    DeploymentBranchPolicySettings, Environment, EnvironmentReviewer, Environments,
};
use octocrab::models::{AppId, BranchPolicyId, EnvironmentId, ProtectionRuleId};
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "owner";
const REPO: &str = "repo";
const ENV_NAME: &str = "staging";
const BRANCH_POLICY_ID: u64 = 1;
const PROTECTION_RULE_ID: u64 = 3515;

async fn setup_environments_mock(
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
        &format!("http method {http_method} on {mocked_path} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_list_environments() {
    let mocked_response: Environments =
        serde_json::from_str(include_str!("resources/repo_environments.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = setup_environments_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/environments").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .environments()
        .list()
        .per_page(10)
        .page(1u32)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result.err()
    );

    let environments = result.unwrap();
    assert_eq!(environments.total_count, 1);
    assert_eq!(environments.environments.len(), 1);
    let env = &environments.environments[0];
    assert_eq!(env.id, EnvironmentId(1610879487));
    assert_eq!(env.name, "staging");
    assert_eq!(env.can_admins_bypass, Some(true));
    assert_eq!(env.protection_rules.len(), 3);
    assert_eq!(env.protection_rules[0].rule_type, "wait_timer");
    assert_eq!(env.protection_rules[0].wait_timer, Some(30));
    assert_eq!(
        env.deployment_branch_policy,
        Some(DeploymentBranchPolicySettings::new(false, true))
    );
}

#[tokio::test]
async fn should_get_environment() {
    let mocked_response: Environment =
        serde_json::from_str(include_str!("resources/repo_environment.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = setup_environments_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/environments/{ENV_NAME}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.repos(OWNER, REPO).environments().get(ENV_NAME).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result.err()
    );

    let env = result.unwrap();
    assert_eq!(env.id, EnvironmentId(1610879487));
    assert_eq!(env.name, "staging");
    assert_eq!(env.can_admins_bypass, Some(true));
}

#[tokio::test]
async fn should_create_or_update_environment() {
    let mocked_response: Environment =
        serde_json::from_str(include_str!("resources/repo_environment.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = MockServer::start().await;

    let expected_body = serde_json::json!({
        "wait_timer": 30,
        "prevent_self_review": true,
        "reviewers": [
            {
                "type": "User",
                "id": 1
            }
        ],
        "deployment_branch_policy": {
            "protected_branches": false,
            "custom_branch_policies": true
        },
        "can_admins_bypass": true
    });

    Mock::given(method("PUT"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/environments/{ENV_NAME}"
        )))
        .and(body_json(&expected_body))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "PUT on /environments/{ENV_NAME} failed").await;

    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .environments()
        .create_or_update(ENV_NAME)
        .wait_timer(30u32)
        .prevent_self_review(true)
        .reviewer(EnvironmentReviewer::user(1u64))
        .deployment_branch_policy(Some(DeploymentBranchPolicySettings::new(false, true)))
        .can_admins_bypass(true)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result.err()
    );

    let env = result.unwrap();
    assert_eq!(env.name, "staging");
    assert_eq!(env.id, EnvironmentId(1610879487));
}

#[tokio::test]
async fn should_delete_environment() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_environments_mock(
        "DELETE",
        format!("/repos/{OWNER}/{REPO}/environments/{ENV_NAME}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .environments()
        .delete(ENV_NAME)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result.err()
    );
}

#[tokio::test]
async fn should_list_deployment_branch_policies() {
    let mocked_response: DeploymentBranchPolicies = serde_json::from_str(include_str!(
        "resources/repo_environment_branch_policies.json"
    ))
    .unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = setup_environments_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/environments/{ENV_NAME}/deployment-branch-policies")
            .as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .environments()
        .branch_policies(ENV_NAME)
        .list()
        .per_page(10)
        .page(1u32)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result.err()
    );

    let policies = result.unwrap();
    assert_eq!(policies.total_count, 1);
    assert_eq!(policies.branch_policies.len(), 1);
    assert_eq!(policies.branch_policies[0].name, "main");
    assert_eq!(
        policies.branch_policies[0].policy_type,
        Some(BranchPolicyType::Branch)
    );
}

#[tokio::test]
async fn should_get_deployment_branch_policy() {
    let mocked_response: DeploymentBranchPolicy = serde_json::from_str(include_str!(
        "resources/repo_environment_branch_policy.json"
    ))
    .unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = setup_environments_mock(
        "GET",
        format!(
            "/repos/{OWNER}/{REPO}/environments/{ENV_NAME}/deployment-branch-policies/{BRANCH_POLICY_ID}"
        )
        .as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .environments()
        .branch_policies(ENV_NAME)
        .get(BRANCH_POLICY_ID)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result.err()
    );

    let policy = result.unwrap();
    assert_eq!(policy.id, Some(BranchPolicyId(BRANCH_POLICY_ID)));
    assert_eq!(policy.name, "main");
    assert_eq!(policy.policy_type, Some(BranchPolicyType::Branch));
}

#[tokio::test]
async fn should_create_deployment_branch_policy() {
    let mocked_response: DeploymentBranchPolicy = serde_json::from_str(include_str!(
        "resources/repo_environment_branch_policy.json"
    ))
    .unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = MockServer::start().await;

    let expected_body = serde_json::json!({
        "name": "main",
        "type": "branch"
    });

    Mock::given(method("POST"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/environments/{ENV_NAME}/deployment-branch-policies"
        )))
        .and(body_json(&expected_body))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "POST on /deployment-branch-policies failed").await;

    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .environments()
        .branch_policies(ENV_NAME)
        .create("main")
        .policy_type(BranchPolicyType::Branch)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result.err()
    );

    let policy = result.unwrap();
    assert_eq!(policy.name, "main");
}

#[tokio::test]
async fn should_update_deployment_branch_policy() {
    let mocked_response: DeploymentBranchPolicy = serde_json::from_str(include_str!(
        "resources/repo_environment_branch_policy.json"
    ))
    .unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = MockServer::start().await;

    let expected_body = serde_json::json!({
        "name": "main",
        "type": "branch"
    });

    Mock::given(method("PUT"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/environments/{ENV_NAME}/deployment-branch-policies/{BRANCH_POLICY_ID}"
        )))
        .and(body_json(&expected_body))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "PUT on /deployment-branch-policies/{id} failed",
    )
    .await;

    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .environments()
        .branch_policies(ENV_NAME)
        .update(BRANCH_POLICY_ID, "main")
        .policy_type(BranchPolicyType::Branch)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result.err()
    );

    let policy = result.unwrap();
    assert_eq!(policy.name, "main");
}

#[tokio::test]
async fn should_delete_deployment_branch_policy() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_environments_mock(
        "DELETE",
        format!(
            "/repos/{OWNER}/{REPO}/environments/{ENV_NAME}/deployment-branch-policies/{BRANCH_POLICY_ID}"
        )
        .as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .environments()
        .branch_policies(ENV_NAME)
        .delete(BRANCH_POLICY_ID)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result.err()
    );
}

#[tokio::test]
async fn should_list_deployment_protection_rules() {
    let mocked_response: CustomDeploymentProtectionRules = serde_json::from_str(include_str!(
        "resources/repo_environment_protection_rules.json"
    ))
    .unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = setup_environments_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/environments/{ENV_NAME}/deployment_protection_rules")
            .as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .environments()
        .protection_rules(ENV_NAME)
        .list()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result.err()
    );

    let rules = result.unwrap();
    assert_eq!(rules.total_count, 1);
    assert_eq!(rules.custom_deployment_protection_rules.len(), 1);
    assert_eq!(
        rules.custom_deployment_protection_rules[0].id,
        ProtectionRuleId(PROTECTION_RULE_ID)
    );
    assert!(rules.custom_deployment_protection_rules[0].enabled);
    assert_eq!(
        rules.custom_deployment_protection_rules[0].app.id,
        AppId(1515)
    );
}

#[tokio::test]
async fn should_get_deployment_protection_rule() {
    let mocked_response: CustomDeploymentProtectionRule = serde_json::from_str(include_str!(
        "resources/repo_environment_protection_rule.json"
    ))
    .unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = setup_environments_mock(
        "GET",
        format!(
            "/repos/{OWNER}/{REPO}/environments/{ENV_NAME}/deployment_protection_rules/{PROTECTION_RULE_ID}"
        )
        .as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .environments()
        .protection_rules(ENV_NAME)
        .get(PROTECTION_RULE_ID)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result.err()
    );

    let rule = result.unwrap();
    assert_eq!(rule.id, ProtectionRuleId(PROTECTION_RULE_ID));
    assert!(rule.enabled);
    assert_eq!(rule.app.slug, "my-custom-app");
}

#[tokio::test]
async fn should_create_deployment_protection_rule() {
    let mocked_response: CustomDeploymentProtectionRule = serde_json::from_str(include_str!(
        "resources/repo_environment_protection_rule.json"
    ))
    .unwrap();
    let template = ResponseTemplate::new(StatusCode::CREATED).set_body_json(&mocked_response);
    let mock_server = MockServer::start().await;

    let expected_body = serde_json::json!({
        "integration_id": 1515
    });

    Mock::given(method("POST"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/environments/{ENV_NAME}/deployment_protection_rules"
        )))
        .and(body_json(&expected_body))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, "POST on /deployment_protection_rules failed").await;

    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .environments()
        .protection_rules(ENV_NAME)
        .create(AppId(1515))
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result.err()
    );

    let rule = result.unwrap();
    assert_eq!(rule.id, ProtectionRuleId(PROTECTION_RULE_ID));
    assert_eq!(rule.app.slug, "my-custom-app");
}

#[tokio::test]
async fn should_disable_deployment_protection_rule() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_environments_mock(
        "DELETE",
        format!(
            "/repos/{OWNER}/{REPO}/environments/{ENV_NAME}/deployment_protection_rules/{PROTECTION_RULE_ID}"
        )
        .as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .environments()
        .protection_rules(ENV_NAME)
        .disable(PROTECTION_RULE_ID)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result.err()
    );
}

#[tokio::test]
async fn should_list_custom_deployment_rule_apps() {
    let mocked_response: CustomDeploymentRuleApps = serde_json::from_str(include_str!(
        "resources/repo_environment_protection_rule_apps.json"
    ))
    .unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = setup_environments_mock(
        "GET",
        format!("/repos/{OWNER}/{REPO}/environments/{ENV_NAME}/deployment_protection_rules/apps")
            .as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .environments()
        .protection_rules(ENV_NAME)
        .list_apps()
        .per_page(10)
        .page(1u32)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result.err()
    );

    let apps = result.unwrap();
    assert_eq!(apps.total_count, 1);
    assert_eq!(
        apps.available_custom_deployment_protection_rule_integrations
            .len(),
        1
    );
    assert_eq!(
        apps.available_custom_deployment_protection_rule_integrations[0].id,
        AppId(1515)
    );
    assert_eq!(
        apps.available_custom_deployment_protection_rule_integrations[0].slug,
        "my-custom-app"
    );
}
