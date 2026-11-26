use super::entity::{EntityDefinition, ParentContext, get_i64_param, get_str_param};
use crate::client::GithubClient;
use crate::schema::FieldSchema;
use std::collections::HashMap;

pub struct RepoEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for RepoEntity {
    fn id(&self) -> &'static str {
        "repo"
    }

    fn display_name(&self) -> &'static str {
        "Repository"
    }

    fn description(&self) -> &'static str {
        "GitHub Repository details (stars, forks, open issues count, etc.)"
    }

    fn is_root(&self) -> bool {
        true
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "owner",
                "Repository Owner",
                "The organization or user owner (e.g. rust-lang, tokio-rs)",
                true,
                Some("rust-lang"),
                false,
            ),
            FieldSchema::new_string(
                "repo",
                "Repository Name",
                "The name of the repository (e.g. rust, tokio, octocrab)",
                true,
                Some("rust"),
                false,
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
        let owner = get_str_param(params, context, "owner")
            .ok_or_else(|| "Missing required parameter 'owner'".to_string())?;
        let repo = get_str_param(params, context, "repo")
            .ok_or_else(|| "Missing required parameter 'repo'".to_string())?;

        let route = format!("/repos/{owner}/{repo}");
        let resp = client.fetch(&route, &HashMap::new()).await?;
        Ok(resp.value)
    }

    fn extract_context(
        &self,
        params: &serde_json::Map<String, serde_json::Value>,
        result: &serde_json::Value,
    ) -> ParentContext {
        let owner = result
            .get("owner")
            .and_then(|o| o.get("login"))
            .and_then(|v| v.as_str())
            .or_else(|| params.get("owner").and_then(|v| v.as_str()))
            .unwrap_or_default();

        let repo = result
            .get("name")
            .and_then(|v| v.as_str())
            .or_else(|| params.get("repo").and_then(|v| v.as_str()))
            .unwrap_or_default();

        ParentContext::new().with("owner", owner).with("repo", repo)
    }
}

pub struct RepoIssuesEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for RepoIssuesEntity {
    fn id(&self) -> &'static str {
        "repo_issues"
    }

    fn display_name(&self) -> &'static str {
        "Issues"
    }

    fn description(&self) -> &'static str {
        "List repository issues"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn target_field(&self) -> &'static str {
        "issues"
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "owner",
                "Repository Owner",
                "Inherited from repository",
                true,
                Some("rust-lang"),
                true,
            ),
            FieldSchema::new_string(
                "repo",
                "Repository Name",
                "Inherited from repository",
                true,
                Some("rust"),
                true,
            ),
            FieldSchema::new_enum(
                "state",
                "Issue State",
                "State filter",
                &["open", "closed", "all"],
                Some("open"),
            ),
            FieldSchema::new_enum(
                "sort",
                "Sort Field",
                "Order issues by property",
                &["created", "updated", "comments"],
                Some("created"),
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
        let owner = get_str_param(params, context, "owner")
            .ok_or_else(|| "Missing required parameter 'owner'".to_string())?;
        let repo = get_str_param(params, context, "repo")
            .ok_or_else(|| "Missing required parameter 'repo'".to_string())?;

        let mut query = HashMap::new();
        if let Some(s) = get_str_param(params, context, "state") {
            query.insert("state".to_string(), s.to_string());
        }
        if let Some(s) = get_str_param(params, context, "sort") {
            query.insert("sort".to_string(), s.to_string());
        }
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/repos/{owner}/{repo}/issues");
        let resp = client.fetch(&route, &query).await?;
        Ok(resp.value)
    }

    fn extract_item_context(
        &self,
        _params: &serde_json::Map<String, serde_json::Value>,
        context: &ParentContext,
        item: &serde_json::Value,
    ) -> ParentContext {
        let mut ctx = ParentContext::new();
        if let Some(owner) = context.get("owner") {
            ctx.set("owner", owner);
        }
        if let Some(repo) = context.get("repo") {
            ctx.set("repo", repo);
        }
        if let Some(num) = item.get("number").and_then(|n| n.as_i64()) {
            ctx.set("issue_number", num.to_string());
        }
        ctx
    }
}

pub struct RepoPullsEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for RepoPullsEntity {
    fn id(&self) -> &'static str {
        "repo_pulls"
    }

    fn display_name(&self) -> &'static str {
        "Pull Requests"
    }

    fn description(&self) -> &'static str {
        "List repository pull requests"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn target_field(&self) -> &'static str {
        "pull_requests"
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "owner",
                "Repository Owner",
                "Inherited from repository",
                true,
                Some("rust-lang"),
                true,
            ),
            FieldSchema::new_string(
                "repo",
                "Repository Name",
                "Inherited from repository",
                true,
                Some("rust"),
                true,
            ),
            FieldSchema::new_enum(
                "state",
                "Pull Request State",
                "Filter by state",
                &["open", "closed", "all"],
                Some("open"),
            ),
            FieldSchema::new_enum(
                "sort",
                "Sort Field",
                "Order pull requests by property",
                &["created", "updated", "popularity", "long-running"],
                Some("created"),
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
        vec!["pull_commits", "pull_files", "pull_comments"]
    }

    async fn execute(
        &self,
        client: &GithubClient,
        params: &serde_json::Map<String, serde_json::Value>,
        context: &ParentContext,
    ) -> Result<serde_json::Value, String> {
        let owner = get_str_param(params, context, "owner")
            .ok_or_else(|| "Missing required parameter 'owner'".to_string())?;
        let repo = get_str_param(params, context, "repo")
            .ok_or_else(|| "Missing required parameter 'repo'".to_string())?;

        let mut query = HashMap::new();
        if let Some(s) = get_str_param(params, context, "state") {
            query.insert("state".to_string(), s.to_string());
        }
        if let Some(s) = get_str_param(params, context, "sort") {
            query.insert("sort".to_string(), s.to_string());
        }
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/repos/{owner}/{repo}/pulls");
        let resp = client.fetch(&route, &query).await?;
        Ok(resp.value)
    }

    fn extract_item_context(
        &self,
        _params: &serde_json::Map<String, serde_json::Value>,
        context: &ParentContext,
        item: &serde_json::Value,
    ) -> ParentContext {
        let mut ctx = ParentContext::new();
        if let Some(owner) = context.get("owner") {
            ctx.set("owner", owner);
        }
        if let Some(repo) = context.get("repo") {
            ctx.set("repo", repo);
        }
        if let Some(num) = item.get("number").and_then(|n| n.as_i64()) {
            ctx.set("pull_number", num.to_string());
        }
        ctx
    }
}

pub struct RepoCommitsEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for RepoCommitsEntity {
    fn id(&self) -> &'static str {
        "repo_commits"
    }

    fn display_name(&self) -> &'static str {
        "Commits"
    }

    fn description(&self) -> &'static str {
        "List repository git commits"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn target_field(&self) -> &'static str {
        "commits"
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "owner",
                "Repository Owner",
                "Inherited from repository",
                true,
                Some("rust-lang"),
                true,
            ),
            FieldSchema::new_string(
                "repo",
                "Repository Name",
                "Inherited from repository",
                true,
                Some("rust"),
                true,
            ),
            FieldSchema::new_string(
                "sha",
                "SHA or Branch",
                "SHA or branch to start listing commits from",
                false,
                Some("main"),
                false,
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
        let owner = get_str_param(params, context, "owner")
            .ok_or_else(|| "Missing required parameter 'owner'".to_string())?;
        let repo = get_str_param(params, context, "repo")
            .ok_or_else(|| "Missing required parameter 'repo'".to_string())?;

        let mut query = HashMap::new();
        if let Some(s) = get_str_param(params, context, "sha") {
            query.insert("sha".to_string(), s.to_string());
        }
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/repos/{owner}/{repo}/commits");
        let resp = client.fetch(&route, &query).await?;
        Ok(resp.value)
    }
}

pub struct RepoReleasesEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for RepoReleasesEntity {
    fn id(&self) -> &'static str {
        "repo_releases"
    }

    fn display_name(&self) -> &'static str {
        "Releases"
    }

    fn description(&self) -> &'static str {
        "List repository releases"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn target_field(&self) -> &'static str {
        "releases"
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "owner",
                "Repository Owner",
                "Inherited from repository",
                true,
                Some("rust-lang"),
                true,
            ),
            FieldSchema::new_string(
                "repo",
                "Repository Name",
                "Inherited from repository",
                true,
                Some("rust"),
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
        let owner = get_str_param(params, context, "owner")
            .ok_or_else(|| "Missing required parameter 'owner'".to_string())?;
        let repo = get_str_param(params, context, "repo")
            .ok_or_else(|| "Missing required parameter 'repo'".to_string())?;

        let mut query = HashMap::new();
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/repos/{owner}/{repo}/releases");
        let resp = client.fetch(&route, &query).await?;
        Ok(resp.value)
    }
}

pub struct RepoBranchesEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for RepoBranchesEntity {
    fn id(&self) -> &'static str {
        "repo_branches"
    }

    fn display_name(&self) -> &'static str {
        "Branches"
    }

    fn description(&self) -> &'static str {
        "List repository branches"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn target_field(&self) -> &'static str {
        "branches"
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "owner",
                "Repository Owner",
                "Inherited from repository",
                true,
                Some("rust-lang"),
                true,
            ),
            FieldSchema::new_string(
                "repo",
                "Repository Name",
                "Inherited from repository",
                true,
                Some("rust"),
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
        let owner = get_str_param(params, context, "owner")
            .ok_or_else(|| "Missing required parameter 'owner'".to_string())?;
        let repo = get_str_param(params, context, "repo")
            .ok_or_else(|| "Missing required parameter 'repo'".to_string())?;

        let mut query = HashMap::new();
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/repos/{owner}/{repo}/branches");
        let resp = client.fetch(&route, &query).await?;
        Ok(resp.value)
    }
}

pub struct RepoWorkflowsEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for RepoWorkflowsEntity {
    fn id(&self) -> &'static str {
        "repo_workflows"
    }

    fn display_name(&self) -> &'static str {
        "Workflows"
    }

    fn description(&self) -> &'static str {
        "List GitHub Actions workflows"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn target_field(&self) -> &'static str {
        "workflows"
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "owner",
                "Repository Owner",
                "Inherited from repository",
                true,
                Some("rust-lang"),
                true,
            ),
            FieldSchema::new_string(
                "repo",
                "Repository Name",
                "Inherited from repository",
                true,
                Some("rust"),
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
        let owner = get_str_param(params, context, "owner")
            .ok_or_else(|| "Missing required parameter 'owner'".to_string())?;
        let repo = get_str_param(params, context, "repo")
            .ok_or_else(|| "Missing required parameter 'repo'".to_string())?;

        let mut query = HashMap::new();
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/repos/{owner}/{repo}/actions/workflows");
        let resp = client.fetch(&route, &query).await?;
        Ok(resp.value)
    }
}

pub struct RepoTagsEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for RepoTagsEntity {
    fn id(&self) -> &'static str {
        "repo_tags"
    }

    fn display_name(&self) -> &'static str {
        "Tags"
    }

    fn description(&self) -> &'static str {
        "List git tags for repository"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn target_field(&self) -> &'static str {
        "tags"
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "owner",
                "Repository Owner",
                "Inherited from repository",
                true,
                Some("rust-lang"),
                true,
            ),
            FieldSchema::new_string(
                "repo",
                "Repository Name",
                "Inherited from repository",
                true,
                Some("rust"),
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
        let owner = get_str_param(params, context, "owner")
            .ok_or_else(|| "Missing required parameter 'owner'".to_string())?;
        let repo = get_str_param(params, context, "repo")
            .ok_or_else(|| "Missing required parameter 'repo'".to_string())?;

        let mut query = HashMap::new();
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/repos/{owner}/{repo}/tags");
        let resp = client.fetch(&route, &query).await?;
        Ok(resp.value)
    }
}

pub struct RepoContributorsEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for RepoContributorsEntity {
    fn id(&self) -> &'static str {
        "repo_contributors"
    }

    fn display_name(&self) -> &'static str {
        "Contributors"
    }

    fn description(&self) -> &'static str {
        "List contributors to the repository"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn target_field(&self) -> &'static str {
        "contributors"
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "owner",
                "Repository Owner",
                "Inherited from repository",
                true,
                Some("rust-lang"),
                true,
            ),
            FieldSchema::new_string(
                "repo",
                "Repository Name",
                "Inherited from repository",
                true,
                Some("rust"),
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
        let owner = get_str_param(params, context, "owner")
            .ok_or_else(|| "Missing required parameter 'owner'".to_string())?;
        let repo = get_str_param(params, context, "repo")
            .ok_or_else(|| "Missing required parameter 'repo'".to_string())?;

        let mut query = HashMap::new();
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/repos/{owner}/{repo}/contributors");
        let resp = client.fetch(&route, &query).await?;
        Ok(resp.value)
    }
}

pub struct RepoLanguagesEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for RepoLanguagesEntity {
    fn id(&self) -> &'static str {
        "repo_languages"
    }

    fn display_name(&self) -> &'static str {
        "Languages"
    }

    fn description(&self) -> &'static str {
        "List programming languages and byte count for the repository"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_collection(&self) -> bool {
        false
    }

    fn target_field(&self) -> &'static str {
        "languages"
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "owner",
                "Repository Owner",
                "Inherited from repository",
                true,
                Some("rust-lang"),
                true,
            ),
            FieldSchema::new_string(
                "repo",
                "Repository Name",
                "Inherited from repository",
                true,
                Some("rust"),
                true,
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
        let owner = get_str_param(params, context, "owner")
            .ok_or_else(|| "Missing required parameter 'owner'".to_string())?;
        let repo = get_str_param(params, context, "repo")
            .ok_or_else(|| "Missing required parameter 'repo'".to_string())?;

        let route = format!("/repos/{owner}/{repo}/languages");
        let resp = client.fetch(&route, &HashMap::new()).await?;
        Ok(resp.value)
    }
}

pub struct RepoLabelsEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for RepoLabelsEntity {
    fn id(&self) -> &'static str {
        "repo_labels"
    }

    fn display_name(&self) -> &'static str {
        "Labels"
    }

    fn description(&self) -> &'static str {
        "List issue/pull request labels available in the repository"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn target_field(&self) -> &'static str {
        "labels"
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "owner",
                "Repository Owner",
                "Inherited from repository",
                true,
                Some("rust-lang"),
                true,
            ),
            FieldSchema::new_string(
                "repo",
                "Repository Name",
                "Inherited from repository",
                true,
                Some("rust"),
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
        let owner = get_str_param(params, context, "owner")
            .ok_or_else(|| "Missing required parameter 'owner'".to_string())?;
        let repo = get_str_param(params, context, "repo")
            .ok_or_else(|| "Missing required parameter 'repo'".to_string())?;

        let mut query = HashMap::new();
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/repos/{owner}/{repo}/labels");
        let resp = client.fetch(&route, &query).await?;
        Ok(resp.value)
    }
}

pub struct RepoMilestonesEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for RepoMilestonesEntity {
    fn id(&self) -> &'static str {
        "repo_milestones"
    }

    fn display_name(&self) -> &'static str {
        "Milestones"
    }

    fn description(&self) -> &'static str {
        "List milestones for the repository"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn target_field(&self) -> &'static str {
        "milestones"
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "owner",
                "Repository Owner",
                "Inherited from repository",
                true,
                Some("rust-lang"),
                true,
            ),
            FieldSchema::new_string(
                "repo",
                "Repository Name",
                "Inherited from repository",
                true,
                Some("rust"),
                true,
            ),
            FieldSchema::new_enum(
                "state",
                "State",
                "Milestone state",
                &["open", "closed", "all"],
                Some("open"),
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
        let owner = get_str_param(params, context, "owner")
            .ok_or_else(|| "Missing required parameter 'owner'".to_string())?;
        let repo = get_str_param(params, context, "repo")
            .ok_or_else(|| "Missing required parameter 'repo'".to_string())?;

        let mut query = HashMap::new();
        if let Some(s) = get_str_param(params, context, "state") {
            query.insert("state".to_string(), s.to_string());
        }
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/repos/{owner}/{repo}/milestones");
        let resp = client.fetch(&route, &query).await?;
        Ok(resp.value)
    }
}
