use octocrab::models::issues::IssueFieldValueInput;
use octocrab::models::repos::secret_scanning_alert::{
    CreateSecretScanningCustomPattern, CustomPatternSetting, DeleteSecretScanningCustomPattern,
    ProviderPatternSetting, PushProtectionBypassReason, UpdateSecretScanningCustomPattern,
    UpdateSecretScanningPatternConfigurations,
};
use octocrab::models::IssueId;
use octocrab::Octocrab;
use serde_json::json;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

fn sample_issue_json() -> serde_json::Value {
    json!({
        "id": 1,
        "node_id": "MDU6SXNzdWUx",
        "url": "https://api.github.com/repos/octocat/Hello-World/issues/1347",
        "repository_url": "https://api.github.com/repos/octocat/Hello-World",
        "labels_url": "https://api.github.com/repos/octocat/Hello-World/issues/1347/labels{/name}",
        "comments_url": "https://api.github.com/repos/octocat/Hello-World/issues/1347/comments",
        "events_url": "https://api.github.com/repos/octocat/Hello-World/issues/1347/events",
        "html_url": "https://github.com/octocat/Hello-World/issues/1347",
        "number": 1347,
        "state": "open",
        "state_reason": "completed",
        "title": "Found a bug",
        "body": "I'm having a problem with this.",
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
        "labels": [],
        "assignee": null,
        "assignees": [],
        "author_association": "COLLABORATOR",
        "milestone": null,
        "locked": false,
        "active_lock_reason": null,
        "comments": 0,
        "pull_request": null,
        "closed_at": null,
        "created_at": "2011-04-22T13:33:48Z",
        "updated_at": "2011-04-22T13:33:48Z"
    })
}

#[tokio::test]
async fn test_org_secret_scanning_custom_patterns_and_pattern_configs() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 1. List organization custom patterns
    Mock::given(method("GET"))
        .and(path("/orgs/my-org/secret-scanning/custom-patterns"))
        .and(query_param("state", "published"))
        .and(query_param("push_protection", "enabled"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": 1,
                "name": "Example Custom Pattern",
                "pattern": "[a-z]+_token_[0-9]+",
                "slug": "example-custom-pattern",
                "state": "published",
                "push_protection_enabled": true,
                "start_delimiter": null,
                "end_delimiter": null,
                "must_match": null,
                "must_not_match": null,
                "created_at": "2024-01-15T10:30:00Z",
                "updated_at": "2024-01-15T10:30:00Z"
            }
        ])))
        .mount(&mock_server)
        .await;

    let page = octocrab
        .orgs("my-org")
        .secret_scanning()
        .state("published")
        .push_protection("enabled")
        .list_custom_patterns()
        .await
        .unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].id, 1);
    assert_eq!(page.items[0].slug, "example-custom-pattern");

    // 2. Bulk create organization custom patterns
    Mock::given(method("POST"))
        .and(path("/orgs/my-org/secret-scanning/custom-patterns"))
        .and(body_json(json!({
            "patterns": [
                {
                    "name": "Example Custom Pattern",
                    "pattern": "[a-z]+_token_[0-9]+"
                }
            ]
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "created_patterns": [
                {
                    "id": 1,
                    "name": "Example Custom Pattern",
                    "pattern": "[a-z]+_token_[0-9]+",
                    "slug": "example-custom-pattern",
                    "state": "unpublished",
                    "push_protection_enabled": false,
                    "created_at": "2024-01-15T10:30:00Z",
                    "updated_at": "2024-01-15T10:30:00Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let created = octocrab
        .orgs("my-org")
        .secret_scanning()
        .create_custom_patterns(&[CreateSecretScanningCustomPattern::new(
            "Example Custom Pattern",
            "[a-z]+_token_[0-9]+",
        )])
        .await
        .unwrap();
    assert_eq!(created.created_patterns.len(), 1);
    assert_eq!(created.created_patterns[0].id, 1);

    // 3. Bulk delete organization custom patterns
    Mock::given(method("DELETE"))
        .and(path("/orgs/my-org/secret-scanning/custom-patterns"))
        .and(body_json(json!({
            "patterns": [{ "id": 1 }],
            "post_delete_action": "delete_alerts"
        })))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .orgs("my-org")
        .secret_scanning()
        .delete_custom_patterns(
            &[DeleteSecretScanningCustomPattern::new(1)],
            Some("delete_alerts"),
        )
        .await
        .unwrap();

    // 4. Update an organization custom pattern
    Mock::given(method("PATCH"))
        .and(path("/orgs/my-org/secret-scanning/custom-patterns/1"))
        .and(body_json(json!({
            "custom_pattern_version": "0ujsswThIGTUYm2K8FjOOfXtY1K",
            "pattern": "updated_secret_[0-9A-Z]{16}"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 1,
            "name": "Example Custom Pattern",
            "pattern": "updated_secret_[0-9A-Z]{16}",
            "slug": "example_custom_pattern",
            "state": "published",
            "push_protection_enabled": true,
            "custom_pattern_version": "0ujsswThIGTUYm2K8FjOOfXtY1K",
            "created_at": "2024-01-15T10:30:00Z",
            "updated_at": "2024-06-01T12:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let mut update =
        UpdateSecretScanningCustomPattern::new(Some("0ujsswThIGTUYm2K8FjOOfXtY1K".to_string()));
    update.pattern = Some("updated_secret_[0-9A-Z]{16}".to_string());
    let updated_pattern = octocrab
        .orgs("my-org")
        .secret_scanning()
        .update_custom_pattern(1, &update)
        .await
        .unwrap();
    assert_eq!(updated_pattern.pattern, "updated_secret_[0-9A-Z]{16}");

    // 5. List organization pattern configurations
    Mock::given(method("GET"))
        .and(path("/orgs/my-org/secret-scanning/pattern-configurations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "pattern_config_version": "0ujsswThIGTUYm2K8FjOOfXtY1K",
            "provider_pattern_overrides": [
                {
                    "token_type": "GITHUB_PERSONAL_ACCESS_TOKEN",
                    "slug": "github_personal_access_token_legacy_v2",
                    "display_name": "GitHub Personal Access Token (Legacy v2)",
                    "alert_total": 15,
                    "alert_total_percentage": 36,
                    "false_positives": 2,
                    "false_positive_rate": 13,
                    "bypass_rate": 13,
                    "default_setting": "enabled",
                    "setting": "enabled",
                    "enterprise_setting": "enabled"
                }
            ],
            "custom_pattern_overrides": [
                {
                    "token_type": "cp_2",
                    "custom_pattern_version": "0ujsswThIGTUYm2K8FjOOfXtY1K",
                    "slug": "custom-api-key",
                    "display_name": "Custom API Key",
                    "alert_total": 15,
                    "alert_total_percentage": 36,
                    "false_positives": 3,
                    "false_positive_rate": 20,
                    "bypass_rate": 20,
                    "default_setting": "disabled",
                    "setting": "enabled"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let configs = octocrab
        .orgs("my-org")
        .secret_scanning()
        .list_pattern_configurations()
        .await
        .unwrap();
    assert_eq!(
        configs.pattern_config_version.as_deref(),
        Some("0ujsswThIGTUYm2K8FjOOfXtY1K")
    );
    assert_eq!(configs.provider_pattern_overrides.unwrap().len(), 1);
    assert_eq!(configs.custom_pattern_overrides.unwrap().len(), 1);

    // 6. Update organization pattern configurations
    Mock::given(method("PATCH"))
        .and(path("/orgs/my-org/secret-scanning/pattern-configurations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "pattern_config_version": "0ujsswThIGTUYm2K8FjOOfXtY1K"
        })))
        .mount(&mock_server)
        .await;

    let config_update = UpdateSecretScanningPatternConfigurations {
        pattern_config_version: Some("0ujsswThIGTUYm2K8FjOOfXtY1K".to_string()),
        provider_pattern_settings: Some(vec![ProviderPatternSetting {
            token_type: "GITHUB_PERSONAL_ACCESS_TOKEN".to_string(),
            push_protection_setting: "enabled".to_string(),
        }]),
        custom_pattern_settings: Some(vec![CustomPatternSetting {
            token_type: "cp_2".to_string(),
            custom_pattern_version: Some("0ujsswThIGTUYm2K8FjOOfXtY1K".to_string()),
            push_protection_setting: "enabled".to_string(),
        }]),
    };
    let updated_configs = octocrab
        .orgs("my-org")
        .secret_scanning()
        .update_pattern_configurations(&config_update)
        .await
        .unwrap();
    assert_eq!(
        updated_configs.pattern_config_version.as_deref(),
        Some("0ujsswThIGTUYm2K8FjOOfXtY1K")
    );
}

#[tokio::test]
async fn test_repo_secret_scanning_custom_patterns_bypass_and_scan_history() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 7. List repository custom patterns
    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/secret-scanning/custom-patterns"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": 2,
                "name": "Repo Custom Pattern",
                "pattern": "prefix_[a-zA-Z0-9]{32}",
                "slug": "repo-custom-pattern",
                "state": "published",
                "push_protection_enabled": false,
                "created_at": "2024-01-16T14:20:00Z",
                "updated_at": "2024-01-17T09:15:00Z"
            }
        ])))
        .mount(&mock_server)
        .await;

    let page = octocrab
        .repos("owner", "repo")
        .secret_scanning()
        .list_custom_patterns()
        .await
        .unwrap();
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].id, 2);

    // 8. Bulk create repository custom patterns
    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/secret-scanning/custom-patterns"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "created_patterns": [
                {
                    "id": 2,
                    "name": "Repo Custom Pattern",
                    "pattern": "prefix_[a-zA-Z0-9]{32}",
                    "slug": "repo-custom-pattern",
                    "state": "unpublished",
                    "push_protection_enabled": false
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let created = octocrab
        .repos("owner", "repo")
        .secret_scanning()
        .create_custom_patterns(&[CreateSecretScanningCustomPattern::new(
            "Repo Custom Pattern",
            "prefix_[a-zA-Z0-9]{32}",
        )])
        .await
        .unwrap();
    assert_eq!(created.created_patterns.len(), 1);

    // 9. Bulk delete repository custom patterns
    Mock::given(method("DELETE"))
        .and(path("/repos/owner/repo/secret-scanning/custom-patterns"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .repos("owner", "repo")
        .secret_scanning()
        .delete_custom_patterns(&[DeleteSecretScanningCustomPattern::new(2)], None)
        .await
        .unwrap();

    // 10. Update a repository custom pattern
    Mock::given(method("PATCH"))
        .and(path("/repos/owner/repo/secret-scanning/custom-patterns/2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 2,
            "name": "Repo Custom Pattern",
            "pattern": "updated_prefix_[a-zA-Z0-9]{32}",
            "slug": "repo-custom-pattern",
            "state": "published",
            "push_protection_enabled": true
        })))
        .mount(&mock_server)
        .await;

    let mut update = UpdateSecretScanningCustomPattern::new(None);
    update.pattern = Some("updated_prefix_[a-zA-Z0-9]{32}".to_string());
    let updated = octocrab
        .repos("owner", "repo")
        .secret_scanning()
        .update_custom_pattern(2, &update)
        .await
        .unwrap();
    assert!(updated.push_protection_enabled);

    // 11. Create a push protection bypass
    Mock::given(method("POST"))
        .and(path(
            "/repos/owner/repo/secret-scanning/push-protection-bypasses",
        ))
        .and(body_json(json!({
            "reason": "will_fix_later",
            "placeholder_id": "placeholder-123"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "reason": "will_fix_later",
            "expire_at": "2020-11-06T18:18:30Z",
            "token_type": "mailchimp_api_key"
        })))
        .mount(&mock_server)
        .await;

    let bypass = octocrab
        .repos("owner", "repo")
        .secret_scanning()
        .create_push_protection_bypass(PushProtectionBypassReason::WillFixLater, "placeholder-123")
        .await
        .unwrap();
    assert_eq!(bypass.reason, PushProtectionBypassReason::WillFixLater);
    assert_eq!(bypass.token_type, "mailchimp_api_key");

    // 12. Get secret scanning scan history for a repository
    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/secret-scanning/scan-history"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "incremental_scans": [
                {
                    "type": "git",
                    "status": "completed",
                    "completed_at": "2024-10-07T02:47:00Z"
                }
            ],
            "backfill_scans": [
                {
                    "type": "git",
                    "status": "completed",
                    "started_at": "2024-10-07T02:47:00Z",
                    "completed_at": "2024-10-07T02:50:00Z"
                }
            ],
            "pattern_update_scans": [],
            "custom_pattern_backfill_scans": [
                {
                    "type": "git",
                    "status": "completed",
                    "started_at": "2024-10-07T02:47:00Z",
                    "completed_at": "2024-10-07T02:55:00Z",
                    "pattern_slug": "my-custom-pattern",
                    "pattern_scope": "organization"
                }
            ],
            "generic_secrets_backfill_scans": []
        })))
        .mount(&mock_server)
        .await;

    let history = octocrab
        .repos("owner", "repo")
        .secret_scanning()
        .get_scan_history()
        .await
        .unwrap();
    assert_eq!(history.incremental_scans.unwrap().len(), 1);
    assert_eq!(
        history.custom_pattern_backfill_scans.unwrap()[0]
            .pattern_slug
            .as_deref(),
        Some("my-custom-pattern")
    );
}

#[tokio::test]
async fn test_issues_wave1_endpoints() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());
    let issue_json = sample_issue_json();

    // 1. List dependencies an issue is blocked by
    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/issues/10/dependencies/blocked_by"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([issue_json.clone()])))
        .mount(&mock_server)
        .await;

    let blocked_by = octocrab
        .issues("owner", "repo")
        .list_dependencies_blocked_by(10)
        .per_page(10)
        .send()
        .await
        .unwrap();
    assert_eq!(blocked_by.items.len(), 1);

    // 2. Add a dependency an issue is blocked by
    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/issues/10/dependencies/blocked_by"))
        .and(body_json(json!({ "issue_id": 1 })))
        .respond_with(ResponseTemplate::new(201).set_body_json(issue_json.clone()))
        .mount(&mock_server)
        .await;

    let added_dep = octocrab
        .issues("owner", "repo")
        .add_dependency_blocked_by(10, IssueId(1))
        .await
        .unwrap();
    assert_eq!(added_dep.number, 1347);

    // 3. Remove dependency an issue is blocked by
    Mock::given(method("DELETE"))
        .and(path(
            "/repos/owner/repo/issues/10/dependencies/blocked_by/1",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(issue_json.clone()))
        .mount(&mock_server)
        .await;

    let removed_dep = octocrab
        .issues("owner", "repo")
        .remove_dependency_blocked_by(10, IssueId(1))
        .await
        .unwrap();
    assert_eq!(removed_dep.number, 1347);

    // 4. List dependencies an issue is blocking
    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/issues/10/dependencies/blocking"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([issue_json.clone()])))
        .mount(&mock_server)
        .await;

    let blocking = octocrab
        .issues("owner", "repo")
        .list_dependencies_blocking(10)
        .send()
        .await
        .unwrap();
    assert_eq!(blocking.items.len(), 1);

    // 5-8. Issue field values (list, add, set, delete)
    let field_values_json = json!([
        {
            "issue_field_id": 1,
            "issue_field_name": "DRI",
            "node_id": "IFT_GDKND",
            "data_type": "text",
            "value": "DRI"
        },
        {
            "issue_field_id": 2,
            "issue_field_name": "Priority",
            "node_id": "IFSS_SADMS",
            "data_type": "single_select",
            "value": 1,
            "single_select_option": {
                "id": 1,
                "name": "High",
                "color": "red"
            }
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/issues/10/issue-field-values"))
        .respond_with(ResponseTemplate::new(200).set_body_json(field_values_json.clone()))
        .mount(&mock_server)
        .await;

    let fields = octocrab
        .issues("owner", "repo")
        .list_issue_field_values(10)
        .send()
        .await
        .unwrap();
    assert_eq!(fields.items.len(), 2);
    assert_eq!(fields.items[0].issue_field_name, "DRI");

    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/issues/10/issue-field-values"))
        .respond_with(ResponseTemplate::new(200).set_body_json(field_values_json.clone()))
        .mount(&mock_server)
        .await;

    let added_fields = octocrab
        .issues("owner", "repo")
        .add_issue_field_values(10, &[IssueFieldValueInput::new(1, "DRI")])
        .await
        .unwrap();
    assert_eq!(added_fields.len(), 2);

    Mock::given(method("PUT"))
        .and(path("/repos/owner/repo/issues/10/issue-field-values"))
        .respond_with(ResponseTemplate::new(200).set_body_json(field_values_json))
        .mount(&mock_server)
        .await;

    let set_fields = octocrab
        .issues("owner", "repo")
        .set_issue_field_values(10, &[IssueFieldValueInput::new(1, "DRI")])
        .await
        .unwrap();
    assert_eq!(set_fields.len(), 2);

    Mock::given(method("DELETE"))
        .and(path("/repos/owner/repo/issues/10/issue-field-values/1"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .issues("owner", "repo")
        .delete_issue_field_value(10, 1)
        .await
        .unwrap();

    // 9-11. Related issues (list, add, remove)
    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/issues/10/relates_to"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([issue_json.clone()])))
        .mount(&mock_server)
        .await;

    let related = octocrab
        .issues("owner", "repo")
        .list_related_issues(10)
        .send()
        .await
        .unwrap();
    assert_eq!(related.items.len(), 1);

    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/issues/10/relates_to"))
        .and(body_json(json!({ "issue_id": 1 })))
        .respond_with(ResponseTemplate::new(201).set_body_json(issue_json.clone()))
        .mount(&mock_server)
        .await;

    let added_related = octocrab
        .issues("owner", "repo")
        .add_related_issue(10, IssueId(1))
        .await
        .unwrap();
    assert_eq!(added_related.number, 1347);

    Mock::given(method("DELETE"))
        .and(path("/repos/owner/repo/issues/10/relates_to/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(issue_json.clone()))
        .mount(&mock_server)
        .await;

    let removed_related = octocrab
        .issues("owner", "repo")
        .remove_related_issue(10, IssueId(1))
        .await
        .unwrap();
    assert_eq!(removed_related.number, 1347);

    // 12-14. Issue suggestions (list, approve, dismiss)
    let suggestion_json = json!({
        "id": 12,
        "issue_id": 4567,
        "action": "add_label",
        "state": "approved",
        "target_id": 208045946,
        "target_value": null,
        "rationale": "This looks like a bug report based on the stack trace in the description.",
        "confidence": "HIGH",
        "actor_id": 41898282,
        "issue_event_id": 6430295168u64,
        "resolved_by": 583231,
        "created_at": "2026-06-01T14:20:00Z",
        "updated_at": "2026-06-01T15:05:00Z"
    });

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/issues/10/suggestions"))
        .and(query_param("state", "pending"))
        .and(query_param("action", "add_label"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([suggestion_json.clone()])))
        .mount(&mock_server)
        .await;

    let suggestions = octocrab
        .issues("owner", "repo")
        .list_suggestions(10)
        .state("pending")
        .action("add_label")
        .send()
        .await
        .unwrap();
    assert_eq!(suggestions.items.len(), 1);
    assert_eq!(suggestions.items[0].id, 12);

    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/issues/10/suggestions/12/approve"))
        .respond_with(ResponseTemplate::new(200).set_body_json(suggestion_json.clone()))
        .mount(&mock_server)
        .await;

    let approved = octocrab
        .issues("owner", "repo")
        .approve_suggestion(10, 12)
        .await
        .unwrap();
    assert_eq!(approved.state, "approved");

    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/issues/10/suggestions/12/dismiss"))
        .respond_with(ResponseTemplate::new(200).set_body_json(suggestion_json))
        .mount(&mock_server)
        .await;

    let dismissed = octocrab
        .issues("owner", "repo")
        .dismiss_suggestion(10, 12)
        .await
        .unwrap();
    assert_eq!(dismissed.id, 12);

    // 15. Remove all labels from an issue
    Mock::given(method("DELETE"))
        .and(path("/repos/owner/repo/issues/10/labels"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .issues("owner", "repo")
        .remove_all_labels(10)
        .await
        .unwrap();

    // 16. Remove sub-issue (DELETE /repos/{owner}/{repo}/issues/{issue_number}/sub_issue)
    Mock::given(method("DELETE"))
        .and(path("/repos/owner/repo/issues/10/sub_issue"))
        .and(body_json(json!({ "sub_issue_id": 101 })))
        .respond_with(ResponseTemplate::new(200).set_body_json(issue_json))
        .mount(&mock_server)
        .await;

    let unlinked = octocrab
        .issues("owner", "repo")
        .remove_sub_issue(10, IssueId(101))
        .await
        .unwrap();
    assert_eq!(unlinked.number, 1347);
}
