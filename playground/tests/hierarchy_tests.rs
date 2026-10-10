#[cfg(test)]
mod tests {
    use octocrab_playground::engine::codegen::generate_rust_code;
    use octocrab_playground::engine::merger::build_node_envelope;
    use octocrab_playground::engine::tree::QueryNode;
    use octocrab_playground::registry::{
        ParentContext, all_root_entities, allowed_child_entities, get_entity,
    };
    use octocrab_playground::state::AppState;
    use octocrab_playground::ui::highlight::{TokenType, highlight_json, highlight_rust};
    use serde_json::json;

    #[test]
    fn test_entity_registry_roots() {
        let roots = all_root_entities();
        assert!(!roots.is_empty());
        let ids: Vec<_> = roots.iter().map(|e| e.id()).collect();
        assert!(ids.contains(&"org"));
        assert!(ids.contains(&"repo"));
        assert!(ids.contains(&"user"));
        assert!(ids.contains(&"issue"));
        assert!(ids.contains(&"pull"));
        assert!(ids.contains(&"search_repos"));
        assert!(ids.contains(&"search_issues"));
        assert!(ids.contains(&"gist"));
        assert!(ids.contains(&"current_user"));
    }

    #[test]
    fn test_entity_target_fields() {
        assert_eq!(
            get_entity("org_repos").unwrap().target_field(),
            "repositories"
        );
        assert_eq!(get_entity("org_members").unwrap().target_field(), "members");
        assert_eq!(get_entity("org_teams").unwrap().target_field(), "teams");
        assert_eq!(get_entity("repo_issues").unwrap().target_field(), "issues");
        assert_eq!(
            get_entity("repo_pulls").unwrap().target_field(),
            "pull_requests"
        );
        assert_eq!(
            get_entity("repo_commits").unwrap().target_field(),
            "commits"
        );
        assert_eq!(
            get_entity("repo_releases").unwrap().target_field(),
            "releases"
        );
        assert_eq!(
            get_entity("repo_branches").unwrap().target_field(),
            "branches"
        );
        assert_eq!(
            get_entity("repo_workflows").unwrap().target_field(),
            "workflows"
        );
        assert_eq!(
            get_entity("issue_comments").unwrap().target_field(),
            "comments"
        );
        assert_eq!(
            get_entity("pull_commits").unwrap().target_field(),
            "commits"
        );
        assert_eq!(get_entity("pull_files").unwrap().target_field(), "files");
        assert_eq!(
            get_entity("pull_comments").unwrap().target_field(),
            "comments"
        );
        assert_eq!(
            get_entity("user_repos").unwrap().target_field(),
            "repositories"
        );
        assert_eq!(
            get_entity("user_followers").unwrap().target_field(),
            "followers"
        );
        assert_eq!(get_entity("user_gists").unwrap().target_field(), "gists");
        assert_eq!(get_entity("repo_tags").unwrap().target_field(), "tags");
        assert_eq!(
            get_entity("repo_contributors").unwrap().target_field(),
            "contributors"
        );
        assert_eq!(
            get_entity("repo_languages").unwrap().target_field(),
            "languages"
        );
        assert_eq!(get_entity("repo_labels").unwrap().target_field(), "labels");
        assert_eq!(
            get_entity("repo_milestones").unwrap().target_field(),
            "milestones"
        );
        assert_eq!(
            get_entity("gist_comments").unwrap().target_field(),
            "comments"
        );
    }

    #[test]
    fn test_allowed_children_filtering() {
        let repo_children = allowed_child_entities("repo");
        let child_ids: Vec<_> = repo_children.iter().map(|c| c.id()).collect();
        assert!(child_ids.contains(&"repo_issues"));
        assert!(child_ids.contains(&"repo_pulls"));
        assert!(child_ids.contains(&"repo_commits"));
        assert!(child_ids.contains(&"repo_releases"));
        assert!(!child_ids.contains(&"org_members"));

        let org_children = allowed_child_entities("org");
        let org_child_ids: Vec<_> = org_children.iter().map(|c| c.id()).collect();
        assert!(org_child_ids.contains(&"org_repos"));
        assert!(org_child_ids.contains(&"org_members"));
        assert!(org_child_ids.contains(&"org_teams"));

        // Multi-level hierarchy: org_repos allows repository child entities
        let org_repos_children = allowed_child_entities("org_repos");
        let org_repos_child_ids: Vec<_> = org_repos_children.iter().map(|c| c.id()).collect();
        assert!(org_repos_child_ids.contains(&"repo_issues"));
        assert!(org_repos_child_ids.contains(&"repo_pulls"));
        assert!(org_repos_child_ids.contains(&"repo_commits"));
        assert!(org_repos_child_ids.contains(&"repo_releases"));
        assert!(org_repos_child_ids.contains(&"repo_branches"));
        assert!(org_repos_child_ids.contains(&"repo_workflows"));

        // User repos also allows repository child entities
        let user_repos_children = allowed_child_entities("user_repos");
        let user_repos_child_ids: Vec<_> = user_repos_children.iter().map(|c| c.id()).collect();
        assert!(user_repos_child_ids.contains(&"repo_issues"));
        assert!(user_repos_child_ids.contains(&"repo_pulls"));

        // Issues allow comments
        let issues_children = allowed_child_entities("repo_issues");
        let issues_child_ids: Vec<_> = issues_children.iter().map(|c| c.id()).collect();
        assert!(issues_child_ids.contains(&"issue_comments"));

        // Pull requests allow comments, files, commits
        let pulls_children = allowed_child_entities("repo_pulls");
        let pulls_child_ids: Vec<_> = pulls_children.iter().map(|c| c.id()).collect();
        assert!(pulls_child_ids.contains(&"pull_commits"));
        assert!(pulls_child_ids.contains(&"pull_files"));
        assert!(pulls_child_ids.contains(&"pull_comments"));
    }

    #[test]
    fn test_extract_item_context_org_repos() {
        let org_repos = get_entity("org_repos").unwrap();
        let params = serde_json::Map::new();
        let context = ParentContext::new().with("org", "rust-lang");

        let repo_item = json!({
            "name": "cargo",
            "owner": {
                "login": "rust-lang"
            }
        });

        let item_ctx = org_repos.extract_item_context(&params, &context, &repo_item);
        assert_eq!(item_ctx.get("owner"), Some("rust-lang"));
        assert_eq!(item_ctx.get("repo"), Some("cargo"));
    }

    #[test]
    fn test_query_node_tree_operations() {
        let mut repo = QueryNode::new("repo");
        assert_eq!(repo.entity_id, "repo");

        let child_id = repo.add_child(&repo.id.clone(), "repo_issues");
        assert!(child_id.is_some());
        assert_eq!(repo.children.len(), 1);
        assert_eq!(repo.children[0].entity_id, "repo_issues");

        // Update parameter
        let cid = child_id.unwrap();
        let updated = repo.update_param(&cid, "state", json!("closed"));
        assert!(updated);

        let child_node = repo.find_node(&cid).unwrap();
        assert_eq!(child_node.params.get("state").unwrap(), "closed");

        // Remove child
        let removed = repo.remove_node(&cid);
        assert!(removed);
        assert_eq!(repo.children.len(), 0);
    }

    #[test]
    fn test_envelope_builder() {
        let mut children = serde_json::Map::new();
        children.insert(
            "repo_issues".to_string(),
            json!({
                "_meta": { "entity": "repo_issues", "status": "success", "duration_ms": 50 },
                "data": [{ "id": 1, "title": "Bug" }]
            }),
        );

        let envelope = build_node_envelope(
            "repo",
            "success",
            120,
            None,
            Some(json!({ "name": "octocrab", "stargazers_count": 2500 })),
            children,
        );

        assert_eq!(envelope["_meta"]["entity"], "repo");
        assert_eq!(envelope["_meta"]["status"], "success");
        assert_eq!(envelope["_meta"]["duration_ms"], 120);
        assert_eq!(envelope["data"]["name"], "octocrab");
        assert!(envelope["children"]["repo_issues"]["data"].is_array());
    }

    #[test]
    fn test_context_extraction() {
        let repo_entity = get_entity("repo").unwrap();
        let mut params = serde_json::Map::new();
        params.insert("owner".to_string(), json!("octocat"));
        params.insert("repo".to_string(), json!("Hello-World"));

        let result = json!({
            "name": "Hello-World",
            "owner": { "login": "octocat" }
        });

        let context = repo_entity.extract_context(&params, &result);
        assert_eq!(context.get("owner"), Some("octocat"));
        assert_eq!(context.get("repo"), Some("Hello-World"));
    }

    #[test]
    fn test_generate_rust_code_single_level() {
        let mut repo = QueryNode::new("repo");
        repo.params.insert("owner".to_string(), json!("tokio-rs"));
        repo.params.insert("repo".to_string(), json!("tokio"));

        let child_id = repo.add_child(&repo.id.clone(), "repo_issues").unwrap();
        repo.update_param(&child_id, "state", json!("open"));
        repo.update_param(&child_id, "per_page", json!(5));

        let code = generate_rust_code(&repo, true);
        assert!(code.contains("octocrab.repos(\"tokio-rs\", \"tokio\").get().await?"));
        assert!(code.contains("octocrab.issues(\"tokio-rs\", \"tokio\")"));
        assert!(code.contains(".state(octocrab::params::State::Open)"));
        assert!(code.contains(".per_page(5)"));
        assert!(code.contains("personal_token"));
        assert!(code.contains("merged_result = json!"));
        assert!(code.contains("merged_result[\"issues\"] = json!(repo_issues_data);"));
    }

    #[test]
    fn test_generate_rust_code_multilevel_org_repos_pulls() {
        // Organization -> Repos -> Pull Requests
        let mut org = QueryNode::new("org");
        org.params.insert("org".to_string(), json!("rust-lang"));

        let repos_id = org.add_child(&org.id.clone(), "org_repos").unwrap();
        org.update_param(&repos_id, "per_page", json!(3));

        let pulls_id = org.add_child(&repos_id, "repo_pulls").unwrap();
        org.update_param(&pulls_id, "per_page", json!(5));
        org.update_param(&pulls_id, "state", json!("open"));

        let code = generate_rust_code(&org, false);

        // Check organization query
        assert!(code.contains("octocrab.orgs(\"rust-lang\").get().await?"));

        // Check organization repos page query
        assert!(code.contains("let org_repos_page = octocrab.orgs(\"rust-lang\")"));
        assert!(code.contains(".list_repos()"));
        assert!(code.contains(".per_page(3)"));

        // Check for loop over repos
        assert!(code.contains("for repo_item in org_repos_page.items {"));
        assert!(code.contains("let repo_name = repo_item.name.clone();"));
        assert!(code.contains("let owner = repo_item.owner.as_ref().map(|o| o.login.clone()).unwrap_or_else(|| \"rust-lang\".to_string());"));

        // Check child pull requests queried per repo
        assert!(code.contains("octocrab.pulls(&owner, &repo_name)"));
        assert!(code.contains(".per_page(5)"));

        // Check per-item children attachment directly on target_field "pull_requests"
        assert!(code.contains("let mut repo_val = json!(repo_item);"));
        assert!(code.contains("repo_val[\"pull_requests\"] = json!(repo_pulls_data);"));
        assert!(code.contains("org_repos_data.push(repo_val);"));

        // Check root merged result attachment on target_field "repositories"
        assert!(code.contains("merged_result[\"repositories\"] = json!(org_repos_data);"));
    }

    #[test]
    fn test_generate_rust_code_deep_repo_with_comments_and_pr_commits() {
        let repo_deep = AppState::create_repo_deep_preset();
        let code = generate_rust_code(&repo_deep, false);

        // Check that list_issue_comments() is used with builder and send()
        assert!(code.contains(".list_issue_comments()"));
        assert!(!code.contains("list_comments_for_issue"));

        // Check that pr_commits() is used on pulls with builder and send()
        assert!(code.contains(".pr_commits(pull_number)"));
        assert!(!code.contains(".commits(pull_number)"));

        // Check target_fields grouping
        assert!(code.contains("issue_val[\"comments\"] = json!(issue_comments_data);"));
        assert!(code.contains("pull_val[\"commits\"] = json!(pull_commits_data);"));
    }

    #[test]
    fn test_deep_presets_structure() {
        let org_deep = AppState::create_org_deep_preset();
        assert_eq!(org_deep.entity_id, "org");
        assert_eq!(org_deep.children.len(), 1);
        assert_eq!(org_deep.children[0].entity_id, "org_repos");
        assert_eq!(org_deep.children[0].children.len(), 1);
        assert_eq!(org_deep.children[0].children[0].entity_id, "repo_pulls");

        let repo_deep = AppState::create_repo_deep_preset();
        assert_eq!(repo_deep.entity_id, "repo");
        assert_eq!(repo_deep.children.len(), 2);
        assert_eq!(repo_deep.children[0].entity_id, "repo_issues");
        assert_eq!(
            repo_deep.children[0].children[0].entity_id,
            "issue_comments"
        );
        assert_eq!(repo_deep.children[1].entity_id, "repo_pulls");
        assert_eq!(repo_deep.children[1].children[0].entity_id, "pull_commits");

        let user_eco = AppState::create_user_ecosystem_preset();
        assert_eq!(user_eco.entity_id, "user");
        assert_eq!(user_eco.children.len(), 2);
        assert_eq!(user_eco.children[0].entity_id, "user_repos");
        assert_eq!(user_eco.children[0].children[0].entity_id, "repo_releases");
        assert_eq!(user_eco.children[1].entity_id, "user_gists");

        let search_deep = AppState::create_search_deep_preset();
        assert_eq!(search_deep.entity_id, "search_repos");
        assert_eq!(search_deep.children.len(), 1);
        assert_eq!(search_deep.children[0].entity_id, "repo_issues");
    }

    #[test]
    fn test_syntax_highlight_rust() {
        let code = r#"
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // A comment
    let count = 10;
    let name = "octocrab";
    let data = json!({ "count": count });
    octocrab.repos("foo", "bar").get().await?;
}
"#;
        let lines = highlight_rust(code);
        let all_tokens: Vec<_> = lines.into_iter().flatten().collect();

        // Check attribute
        assert!(
            all_tokens
                .iter()
                .any(|t| t.token_type == TokenType::Attribute && t.text == "#[tokio::main]")
        );
        // Check keywords
        assert!(
            all_tokens
                .iter()
                .any(|t| t.token_type == TokenType::Keyword && t.text == "async")
        );
        assert!(
            all_tokens
                .iter()
                .any(|t| t.token_type == TokenType::Keyword && t.text == "fn")
        );
        assert!(
            all_tokens
                .iter()
                .any(|t| t.token_type == TokenType::Keyword && t.text == "let")
        );
        assert!(
            all_tokens
                .iter()
                .any(|t| t.token_type == TokenType::Keyword && t.text == "await")
        );
        // Check types
        assert!(
            all_tokens
                .iter()
                .any(|t| t.token_type == TokenType::Type && t.text == "Result")
        );
        assert!(
            all_tokens
                .iter()
                .any(|t| t.token_type == TokenType::Type && t.text == "Box")
        );
        assert!(
            all_tokens
                .iter()
                .any(|t| t.token_type == TokenType::Type && t.text == "Error")
        );
        // Check comments
        assert!(
            all_tokens
                .iter()
                .any(|t| t.token_type == TokenType::Comment && t.text.contains("A comment"))
        );
        // Check strings
        assert!(
            all_tokens
                .iter()
                .any(|t| t.token_type == TokenType::StringLiteral && t.text == "\"octocrab\"")
        );
        // Check numbers
        assert!(
            all_tokens
                .iter()
                .any(|t| t.token_type == TokenType::NumberLiteral && t.text == "10")
        );
        // Check macro
        assert!(
            all_tokens
                .iter()
                .any(|t| t.token_type == TokenType::Macro && t.text == "json!")
        );
        // Check function call
        assert!(
            all_tokens
                .iter()
                .any(|t| t.token_type == TokenType::Function && t.text == "get")
        );
    }

    #[test]
    fn test_syntax_highlight_json() {
        let json_text = r#"{
    "status": "success",
    "total": 42,
    "active": true,
    "details": null
}"#;
        let lines = highlight_json(json_text);
        let all_tokens: Vec<_> = lines.into_iter().flatten().collect();

        // Check key vs string literal
        assert!(
            all_tokens
                .iter()
                .any(|t| t.token_type == TokenType::Key && t.text == "\"status\"")
        );
        assert!(
            all_tokens
                .iter()
                .any(|t| t.token_type == TokenType::StringLiteral && t.text == "\"success\"")
        );
        // Check number
        assert!(
            all_tokens
                .iter()
                .any(|t| t.token_type == TokenType::NumberLiteral && t.text == "42")
        );
        // Check boolean and null keywords
        assert!(
            all_tokens
                .iter()
                .any(|t| t.token_type == TokenType::Keyword && t.text == "true")
        );
        assert!(
            all_tokens
                .iter()
                .any(|t| t.token_type == TokenType::Keyword && t.text == "null")
        );
    }

    #[test]
    fn test_generate_rust_code_repo_analytics_preset() {
        let repo_analytics = AppState::create_repo_analytics_preset();
        let code = generate_rust_code(&repo_analytics, false);

        assert!(code.contains(".list_tags()"));
        assert!(code.contains(".list_contributors()"));
        assert!(code.contains(".list_languages()"));
        assert!(code.contains(".list_milestones()"));
        assert!(code.contains(r#"merged_result["tags"] = json!(repo_tags_data);"#));
        assert!(code.contains(r#"merged_result["contributors"] = json!(repo_contributors_data);"#));
        assert!(code.contains(r#"merged_result["languages"] = json!(repo_languages_data);"#));
        assert!(code.contains(r#"merged_result["milestones"] = json!(repo_milestones_data);"#));
    }

    #[test]
    fn test_generate_rust_code_gist_preset() {
        let gist_preset = AppState::create_gist_preset();
        let code = generate_rust_code(&gist_preset, false);

        assert!(code.contains(r#"octocrab.gists().get("aa5a315d61ae9438b18d").await?"#));
        assert!(code.contains("octocrab.gists()"));
        assert!(code.contains(r#".comments_for("aa5a315d61ae9438b18d")"#));
        assert!(code.contains(".list_comments()"));
        assert!(code.contains(r#"merged_result["comments"] = json!(gist_comments_data);"#));
    }
}
