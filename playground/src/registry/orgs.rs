use super::entity::{EntityDefinition, ParentContext, get_i64_param, get_str_param};
use crate::client::GithubClient;
use crate::schema::FieldSchema;
use std::collections::HashMap;

pub struct OrgEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for OrgEntity {
    fn id(&self) -> &'static str {
        "org"
    }

    fn display_name(&self) -> &'static str {
        "Organization"
    }

    fn description(&self) -> &'static str {
        "GitHub Organization details and child resources"
    }

    fn is_root(&self) -> bool {
        true
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![FieldSchema::new_string(
            "org",
            "Organization Login",
            "The GitHub organization name (e.g. rust-lang, tokio-rs)",
            true,
            Some("rust-lang"),
            false,
        )]
    }

    fn allowed_children(&self) -> Vec<&'static str> {
        vec!["org_repos", "org_members", "org_teams"]
    }

    async fn execute(
        &self,
        client: &GithubClient,
        params: &serde_json::Map<String, serde_json::Value>,
        context: &ParentContext,
    ) -> Result<serde_json::Value, String> {
        let org = get_str_param(params, context, "org")
            .ok_or_else(|| "Missing required parameter 'org'".to_string())?;

        let route = format!("/orgs/{org}");
        let resp = client.fetch(&route, &HashMap::new()).await?;
        Ok(resp.value)
    }

    fn extract_context(
        &self,
        params: &serde_json::Map<String, serde_json::Value>,
        result: &serde_json::Value,
    ) -> ParentContext {
        let login = result
            .get("login")
            .and_then(|v| v.as_str())
            .or_else(|| params.get("org").and_then(|v| v.as_str()))
            .unwrap_or_default();

        ParentContext::new().with("org", login)
    }
}

pub struct OrgReposEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for OrgReposEntity {
    fn id(&self) -> &'static str {
        "org_repos"
    }

    fn display_name(&self) -> &'static str {
        "Repositories"
    }

    fn description(&self) -> &'static str {
        "List repositories belonging to the organization"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn target_field(&self) -> &'static str {
        "repositories"
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "org",
                "Organization Login",
                "Inherited from organization",
                true,
                Some("rust-lang"),
                true,
            ),
            FieldSchema::new_enum(
                "type",
                "Repository Type",
                "Filter by repository visibility or association",
                &["all", "public", "private", "forks", "sources", "member"],
                Some("all"),
            ),
            FieldSchema::new_enum(
                "sort",
                "Sort Field",
                "Sort repositories by property",
                &["created", "updated", "pushed", "full_name"],
                Some("updated"),
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
        let org = get_str_param(params, context, "org")
            .ok_or_else(|| "Missing required parameter 'org'".to_string())?;

        let mut query = HashMap::new();
        if let Some(t) = get_str_param(params, context, "type") {
            query.insert("type".to_string(), t.to_string());
        }
        if let Some(s) = get_str_param(params, context, "sort") {
            query.insert("sort".to_string(), s.to_string());
        }
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/orgs/{org}/repos");
        let resp = client.fetch(&route, &query).await?;
        Ok(resp.value)
    }

    fn extract_item_context(
        &self,
        params: &serde_json::Map<String, serde_json::Value>,
        context: &ParentContext,
        item: &serde_json::Value,
    ) -> ParentContext {
        let mut ctx = ParentContext::new();
        let owner = item
            .get("owner")
            .and_then(|o| o.get("login"))
            .and_then(|l| l.as_str())
            .or_else(|| get_str_param(params, context, "org"));
        if let Some(o) = owner {
            ctx.set("owner", o);
        }
        if let Some(name) = item.get("name").and_then(|n| n.as_str()) {
            ctx.set("repo", name);
        }
        ctx
    }
}

pub struct OrgMembersEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for OrgMembersEntity {
    fn id(&self) -> &'static str {
        "org_members"
    }

    fn display_name(&self) -> &'static str {
        "Members"
    }

    fn description(&self) -> &'static str {
        "List public members of the organization"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn target_field(&self) -> &'static str {
        "members"
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "org",
                "Organization Login",
                "Inherited from organization",
                true,
                Some("rust-lang"),
                true,
            ),
            FieldSchema::new_enum(
                "role",
                "Member Role",
                "Filter members by role",
                &["all", "admin", "member"],
                Some("all"),
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
        vec!["user_repos", "user_followers", "user_gists"]
    }

    async fn execute(
        &self,
        client: &GithubClient,
        params: &serde_json::Map<String, serde_json::Value>,
        context: &ParentContext,
    ) -> Result<serde_json::Value, String> {
        let org = get_str_param(params, context, "org")
            .ok_or_else(|| "Missing required parameter 'org'".to_string())?;

        let mut query = HashMap::new();
        if let Some(r) = get_str_param(params, context, "role") {
            query.insert("role".to_string(), r.to_string());
        }
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/orgs/{org}/members");
        let resp = client.fetch(&route, &query).await?;
        Ok(resp.value)
    }

    fn extract_item_context(
        &self,
        _params: &serde_json::Map<String, serde_json::Value>,
        _context: &ParentContext,
        item: &serde_json::Value,
    ) -> ParentContext {
        let mut ctx = ParentContext::new();
        if let Some(login) = item.get("login").and_then(|l| l.as_str()) {
            ctx.set("username", login);
        }
        ctx
    }
}

pub struct OrgTeamsEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for OrgTeamsEntity {
    fn id(&self) -> &'static str {
        "org_teams"
    }

    fn display_name(&self) -> &'static str {
        "Teams"
    }

    fn description(&self) -> &'static str {
        "List teams within the organization"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn target_field(&self) -> &'static str {
        "teams"
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "org",
                "Organization Login",
                "Inherited from organization",
                true,
                Some("rust-lang"),
                true,
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
        vec![]
    }

    async fn execute(
        &self,
        client: &GithubClient,
        params: &serde_json::Map<String, serde_json::Value>,
        context: &ParentContext,
    ) -> Result<serde_json::Value, String> {
        let org = get_str_param(params, context, "org")
            .ok_or_else(|| "Missing required parameter 'org'".to_string())?;

        let mut query = HashMap::new();
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/orgs/{org}/teams");
        let resp = client.fetch(&route, &query).await?;
        Ok(resp.value)
    }
}
