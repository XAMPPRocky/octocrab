use super::entity::{EntityDefinition, ParentContext, get_i64_param, get_str_param};
use crate::client::GithubClient;
use crate::schema::FieldSchema;
use std::collections::HashMap;

pub struct SearchRepositoriesEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for SearchRepositoriesEntity {
    fn id(&self) -> &'static str {
        "search_repos"
    }

    fn display_name(&self) -> &'static str {
        "Search Repositories"
    }

    fn description(&self) -> &'static str {
        "Search GitHub repositories by query string (e.g. 'language:rust stars:>1000')"
    }

    fn is_root(&self) -> bool {
        true
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "q",
                "Search Query",
                "GitHub search syntax (e.g. 'octocrab in:name', 'stars:>5000')",
                true,
                Some("octocrab"),
                false,
            ),
            FieldSchema::new_enum(
                "sort",
                "Sort Field",
                "Sort results by field",
                &["stars", "forks", "help-wanted-issues", "updated"],
                Some("stars"),
            ),
            FieldSchema::new_enum(
                "order",
                "Sort Order",
                "Ascending or descending",
                &["desc", "asc"],
                Some("desc"),
            ),
            FieldSchema::new_integer(
                "per_page",
                "Per Page",
                "Results per page (max 100)",
                false,
                Some(10),
                Some(1),
                Some(100),
            ),
        ]
    }

    fn allowed_children(&self) -> Vec<&'static str> {
        vec![
            "repo_issues",
            "repo_pulls",
            "repo_commits",
            "repo_releases",
            "repo_branches",
            "repo_workflows",
            "repo_tags",
            "repo_contributors",
            "repo_languages",
            "repo_labels",
            "repo_milestones",
        ]
    }

    async fn execute(
        &self,
        client: &GithubClient,
        params: &serde_json::Map<String, serde_json::Value>,
        context: &ParentContext,
    ) -> Result<serde_json::Value, String> {
        let q = get_str_param(params, context, "q")
            .ok_or_else(|| "Missing required parameter 'q'".to_string())?;

        let mut query = HashMap::new();
        query.insert("q".to_string(), q.to_string());

        if let Some(s) = get_str_param(params, context, "sort") {
            query.insert("sort".to_string(), s.to_string());
        }
        if let Some(o) = get_str_param(params, context, "order") {
            query.insert("order".to_string(), o.to_string());
        }
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let resp = client.fetch("/search/repositories", &query).await?;
        Ok(resp.value)
    }

    fn extract_item_context(
        &self,
        _params: &serde_json::Map<String, serde_json::Value>,
        _context: &ParentContext,
        item: &serde_json::Value,
    ) -> ParentContext {
        let mut ctx = ParentContext::new();
        if let Some(owner) = item
            .get("owner")
            .and_then(|o| o.get("login"))
            .and_then(|l| l.as_str())
        {
            ctx.set("owner", owner);
        }
        if let Some(name) = item.get("name").and_then(|n| n.as_str()) {
            ctx.set("repo", name);
        }
        ctx
    }
}

pub struct SearchIssuesEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for SearchIssuesEntity {
    fn id(&self) -> &'static str {
        "search_issues"
    }

    fn display_name(&self) -> &'static str {
        "Search Issues & PRs"
    }

    fn description(&self) -> &'static str {
        "Search issues and pull requests by query string"
    }

    fn is_root(&self) -> bool {
        true
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "q",
                "Search Query",
                "Search query (e.g. 'repo:rust-lang/rust is:open label:A-diagnostics')",
                true,
                Some("is:open is:issue"),
                false,
            ),
            FieldSchema::new_enum(
                "sort",
                "Sort Field",
                "Sort results by field",
                &["comments", "reactions", "created", "updated"],
                Some("updated"),
            ),
            FieldSchema::new_enum(
                "order",
                "Sort Order",
                "Ascending or descending",
                &["desc", "asc"],
                Some("desc"),
            ),
            FieldSchema::new_integer(
                "per_page",
                "Per Page",
                "Results per page (max 100)",
                false,
                Some(10),
                Some(1),
                Some(100),
            ),
        ]
    }

    fn allowed_children(&self) -> Vec<&'static str> {
        vec!["issue_comments"]
    }

    async fn execute(
        &self,
        client: &GithubClient,
        params: &serde_json::Map<String, serde_json::Value>,
        context: &ParentContext,
    ) -> Result<serde_json::Value, String> {
        let q = get_str_param(params, context, "q")
            .ok_or_else(|| "Missing required parameter 'q'".to_string())?;

        let mut query = HashMap::new();
        query.insert("q".to_string(), q.to_string());

        if let Some(s) = get_str_param(params, context, "sort") {
            query.insert("sort".to_string(), s.to_string());
        }
        if let Some(o) = get_str_param(params, context, "order") {
            query.insert("order".to_string(), o.to_string());
        }
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let resp = client.fetch("/search/issues", &query).await?;
        Ok(resp.value)
    }

    fn extract_item_context(
        &self,
        _params: &serde_json::Map<String, serde_json::Value>,
        _context: &ParentContext,
        item: &serde_json::Value,
    ) -> ParentContext {
        let mut ctx = ParentContext::new();
        if let Some(num) = item.get("number").and_then(|n| n.as_i64()) {
            ctx.set("issue_number", num.to_string());
        }
        // repository_url format: "https://api.github.com/repos/owner/name"
        if let Some(repo_url) = item.get("repository_url").and_then(|u| u.as_str()) {
            let parts: Vec<&str> = repo_url.split('/').collect();
            if parts.len() >= 2 {
                ctx.set("owner", parts[parts.len() - 2]);
                ctx.set("repo", parts[parts.len() - 1]);
            }
        }
        ctx
    }
}
