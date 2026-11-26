use crate::client::GithubClient;
use crate::engine::executor::{ExecutionOutcome, execute_tree};
use crate::engine::tree::QueryNode;
use crate::registry::{ParentContext, get_entity};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct AppState {
    pub root_node: Signal<QueryNode>,
    pub selected_node_id: Signal<String>,
    pub is_executing: Signal<bool>,
    pub outcome: Signal<Option<ExecutionOutcome>>,
    pub token: Signal<String>,
    pub active_tab: Signal<ActiveTab>,
    pub status_message: Signal<Option<String>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ActiveTab {
    Results,
    RustCode,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        let initial_root = Self::create_org_deep_preset();
        let selected_id = initial_root.id.clone();

        Self {
            root_node: Signal::new(initial_root),
            selected_node_id: Signal::new(selected_id),
            is_executing: Signal::new(false),
            outcome: Signal::new(None),
            token: Signal::new(String::new()),
            active_tab: Signal::new(ActiveTab::Results),
            status_message: Signal::new(None),
        }
    }

    pub fn create_repo_preset() -> QueryNode {
        let mut repo = QueryNode::new("repo");
        repo.params.insert(
            "owner".to_string(),
            serde_json::Value::String("XAMPPRocky".to_string()),
        );
        repo.params.insert(
            "repo".to_string(),
            serde_json::Value::String("octocrab".to_string()),
        );

        let mut issues = QueryNode::new("repo_issues");
        issues
            .params
            .insert("per_page".to_string(), serde_json::json!(5));
        repo.children.push(issues);

        let mut pulls = QueryNode::new("repo_pulls");
        pulls
            .params
            .insert("per_page".to_string(), serde_json::json!(5));
        repo.children.push(pulls);

        repo
    }

    pub fn create_org_preset() -> QueryNode {
        let mut org = QueryNode::new("org");
        org.params.insert(
            "org".to_string(),
            serde_json::Value::String("rust-lang".to_string()),
        );

        let mut repos = QueryNode::new("org_repos");
        repos
            .params
            .insert("per_page".to_string(), serde_json::json!(5));
        org.children.push(repos);

        let mut members = QueryNode::new("org_members");
        members
            .params
            .insert("per_page".to_string(), serde_json::json!(5));
        org.children.push(members);

        org
    }

    pub fn create_user_preset() -> QueryNode {
        let mut user = QueryNode::new("user");
        user.params.insert(
            "username".to_string(),
            serde_json::Value::String("torvalds".to_string()),
        );

        let mut repos = QueryNode::new("user_repos");
        repos
            .params
            .insert("per_page".to_string(), serde_json::json!(5));
        user.children.push(repos);

        let mut followers = QueryNode::new("user_followers");
        followers
            .params
            .insert("per_page".to_string(), serde_json::json!(5));
        user.children.push(followers);

        user
    }

    pub fn create_search_preset() -> QueryNode {
        let mut search = QueryNode::new("search_repos");
        search.params.insert(
            "q".to_string(),
            serde_json::Value::String("octocrab in:name".to_string()),
        );
        search
            .params
            .insert("per_page".to_string(), serde_json::json!(10));
        search
    }

    /// Multi-level preset: Org -> Repos -> Pull Requests
    pub fn create_org_deep_preset() -> QueryNode {
        let mut org = QueryNode::new("org");
        org.params.insert(
            "org".to_string(),
            serde_json::Value::String("rust-lang".to_string()),
        );

        let mut repos = QueryNode::new("org_repos");
        repos
            .params
            .insert("per_page".to_string(), serde_json::json!(3));

        let mut pulls = QueryNode::new("repo_pulls");
        pulls
            .params
            .insert("per_page".to_string(), serde_json::json!(5));
        repos.children.push(pulls);

        org.children.push(repos);
        org
    }

    /// Deep multi-level preset: Repo -> Issues -> Comments & PRs -> Commits
    pub fn create_repo_deep_preset() -> QueryNode {
        let mut repo = QueryNode::new("repo");
        repo.params.insert(
            "owner".to_string(),
            serde_json::Value::String("tokio-rs".to_string()),
        );
        repo.params.insert(
            "repo".to_string(),
            serde_json::Value::String("tokio".to_string()),
        );

        let mut issues = QueryNode::new("repo_issues");
        issues
            .params
            .insert("per_page".to_string(), serde_json::json!(3));

        let mut comments = QueryNode::new("issue_comments");
        comments
            .params
            .insert("per_page".to_string(), serde_json::json!(3));
        issues.children.push(comments);

        let mut pulls = QueryNode::new("repo_pulls");
        pulls
            .params
            .insert("per_page".to_string(), serde_json::json!(3));

        let mut commits = QueryNode::new("pull_commits");
        commits
            .params
            .insert("per_page".to_string(), serde_json::json!(3));
        pulls.children.push(commits);

        repo.children.push(issues);
        repo.children.push(pulls);
        repo
    }

    /// User Ecosystem preset: User -> Repositories -> Releases & Gists
    pub fn create_user_ecosystem_preset() -> QueryNode {
        let mut user = QueryNode::new("user");
        user.params.insert(
            "username".to_string(),
            serde_json::Value::String("dtolnay".to_string()),
        );

        let mut repos = QueryNode::new("user_repos");
        repos
            .params
            .insert("per_page".to_string(), serde_json::json!(5));

        let mut releases = QueryNode::new("repo_releases");
        releases
            .params
            .insert("per_page".to_string(), serde_json::json!(3));
        repos.children.push(releases);

        let mut gists = QueryNode::new("user_gists");
        gists
            .params
            .insert("per_page".to_string(), serde_json::json!(5));

        user.children.push(repos);
        user.children.push(gists);
        user
    }

    /// Search & Inspect preset: Search Repos -> Issues
    pub fn create_search_deep_preset() -> QueryNode {
        let mut search = QueryNode::new("search_repos");
        search.params.insert(
            "q".to_string(),
            serde_json::Value::String("octocrab".to_string()),
        );
        search
            .params
            .insert("per_page".to_string(), serde_json::json!(3));

        let mut issues = QueryNode::new("repo_issues");
        issues
            .params
            .insert("per_page".to_string(), serde_json::json!(3));
        search.children.push(issues);

        search
    }

    /// Repo Analytics preset: Repo -> Tags, Contributors, Languages, Milestones
    pub fn create_repo_analytics_preset() -> QueryNode {
        let mut repo = QueryNode::new("repo");
        repo.params.insert(
            "owner".to_string(),
            serde_json::Value::String("XAMPPRocky".to_string()),
        );
        repo.params.insert(
            "repo".to_string(),
            serde_json::Value::String("octocrab".to_string()),
        );

        let mut tags = QueryNode::new("repo_tags");
        tags.params
            .insert("per_page".to_string(), serde_json::json!(5));

        let mut contributors = QueryNode::new("repo_contributors");
        contributors
            .params
            .insert("per_page".to_string(), serde_json::json!(5));

        let languages = QueryNode::new("repo_languages");

        let mut milestones = QueryNode::new("repo_milestones");
        milestones
            .params
            .insert("per_page".to_string(), serde_json::json!(5));

        repo.children.push(tags);
        repo.children.push(contributors);
        repo.children.push(languages);
        repo.children.push(milestones);
        repo
    }

    /// Gist & Comments preset: Gist -> Comments
    pub fn create_gist_preset() -> QueryNode {
        let mut gist = QueryNode::new("gist");
        gist.params.insert(
            "gist_id".to_string(),
            serde_json::Value::String("aa5a315d61ae9438b18d".to_string()),
        );

        let mut comments = QueryNode::new("gist_comments");
        comments
            .params
            .insert("per_page".to_string(), serde_json::json!(10));

        gist.children.push(comments);
        gist
    }

    pub fn load_preset(&mut self, preset_name: &str) {
        let new_root = match preset_name {
            "org_deep" => Self::create_org_deep_preset(),
            "repo_deep" => Self::create_repo_deep_preset(),
            "user_ecosystem" => Self::create_user_ecosystem_preset(),
            "search_deep" => Self::create_search_deep_preset(),
            "repo_analytics" => Self::create_repo_analytics_preset(),
            "gist_deep" => Self::create_gist_preset(),
            "repo" => Self::create_repo_preset(),
            "org" => Self::create_org_preset(),
            "user" => Self::create_user_preset(),
            "search" => Self::create_search_preset(),
            _ => Self::create_org_deep_preset(),
        };
        let new_id = new_root.id.clone();
        self.root_node.set(new_root);
        self.selected_node_id.set(new_id);
        self.outcome.set(None);
        self.status_message
            .set(Some(format!("Loaded preset: {preset_name}")));
    }

    pub fn execute(&mut self) {
        let mut is_executing = self.is_executing;
        let mut outcome_sig = self.outcome;
        let mut status_msg = self.status_message;
        let root = self.root_node.read().clone();
        let token = self.token.read().clone();

        is_executing.set(true);
        status_msg.set(Some("Executing query tree...".to_string()));

        spawn(async move {
            let client = GithubClient::new(if token.is_empty() { None } else { Some(token) });
            let result = execute_tree(&root, &client).await;

            let msg = if result.has_errors {
                format!("Completed with errors in {} ms", result.total_duration_ms)
            } else {
                format!(
                    "Query completed successfully in {} ms",
                    result.total_duration_ms
                )
            };

            status_msg.set(Some(msg));
            outcome_sig.set(Some(result));
            is_executing.set(false);
        });
    }

    pub fn get_context_for_node(&self, target_id: &str) -> ParentContext {
        let root = self.root_node.read();
        let context = ParentContext::new();

        fn walk(
            node: &QueryNode,
            target_id: &str,
            curr_ctx: &ParentContext,
        ) -> Option<ParentContext> {
            if node.id == target_id {
                return Some(curr_ctx.clone());
            }

            let mut next_ctx = curr_ctx.clone();
            if let Some(entity) = get_entity(&node.entity_id) {
                // Approximate extraction from default/configured params
                let default_val = serde_json::Value::Object(node.params.clone());
                let extracted = entity.extract_context(&node.params, &default_val);
                next_ctx.merge(&extracted);
            }

            for child in &node.children {
                if let Some(found) = walk(child, target_id, &next_ctx) {
                    return Some(found);
                }
            }
            None
        }

        walk(&root, target_id, &context).unwrap_or_default()
    }
}
