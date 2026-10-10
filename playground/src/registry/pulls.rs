use super::entity::{EntityDefinition, ParentContext, get_i64_param, get_str_param};
use crate::client::GithubClient;
use crate::schema::FieldSchema;
use std::collections::HashMap;

pub struct PullRequestEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for PullRequestEntity {
    fn id(&self) -> &'static str {
        "pull"
    }

    fn display_name(&self) -> &'static str {
        "Pull Request Details"
    }

    fn description(&self) -> &'static str {
        "Fetch a single pull request by repository and pull number"
    }

    fn is_root(&self) -> bool {
        true
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "owner",
                "Repository Owner",
                "Repository owner (e.g. 'rust-lang')",
                true,
                Some("rust-lang"),
                true,
            ),
            FieldSchema::new_string(
                "repo",
                "Repository Name",
                "Repository name (e.g. 'rust')",
                true,
                Some("rust"),
                true,
            ),
            FieldSchema::new_integer(
                "pull_number",
                "Pull Request Number",
                "The integer number of the pull request",
                true,
                Some(1),
                Some(1),
                None,
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
        let pull_number = get_i64_param(params, "pull_number")
            .or_else(|| context.get("pull_number").and_then(|s| s.parse().ok()))
            .ok_or_else(|| "Missing required parameter 'pull_number'".to_string())?;

        let route = format!("/repos/{owner}/{repo}/pulls/{pull_number}");
        let resp = client.fetch(&route, &HashMap::new()).await?;
        Ok(resp.value)
    }

    fn extract_context(
        &self,
        params: &serde_json::Map<String, serde_json::Value>,
        result: &serde_json::Value,
    ) -> ParentContext {
        let number = result
            .get("number")
            .and_then(|v| v.as_i64())
            .map(|n| n.to_string())
            .or_else(|| {
                params
                    .get("pull_number")
                    .map(|v| v.to_string().replace('"', ""))
            })
            .unwrap_or_default();

        let owner = params
            .get("owner")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let repo = params
            .get("repo")
            .and_then(|v| v.as_str())
            .unwrap_or_default();

        ParentContext::new()
            .with("owner", owner)
            .with("repo", repo)
            .with("pull_number", number)
    }
}

pub struct PullCommitsEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for PullCommitsEntity {
    fn id(&self) -> &'static str {
        "pull_commits"
    }

    fn display_name(&self) -> &'static str {
        "PR Commits"
    }

    fn description(&self) -> &'static str {
        "List commits on a pull request"
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
                "Inherited from PR",
                true,
                Some("rust-lang"),
                true,
            ),
            FieldSchema::new_string(
                "repo",
                "Repository Name",
                "Inherited from PR",
                true,
                Some("rust"),
                true,
            ),
            FieldSchema::new_integer(
                "pull_number",
                "Pull Request Number",
                "Inherited from PR",
                true,
                Some(1),
                Some(1),
                None,
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
        let pull_number = get_i64_param(params, "pull_number")
            .or_else(|| context.get("pull_number").and_then(|s| s.parse().ok()))
            .ok_or_else(|| "Missing required parameter 'pull_number'".to_string())?;

        let mut query = HashMap::new();
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/repos/{owner}/{repo}/pulls/{pull_number}/commits");
        let resp = client.fetch(&route, &query).await?;
        Ok(resp.value)
    }
}

pub struct PullFilesEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for PullFilesEntity {
    fn id(&self) -> &'static str {
        "pull_files"
    }

    fn display_name(&self) -> &'static str {
        "PR Files (Diff)"
    }

    fn description(&self) -> &'static str {
        "List modified files and diff stats on a pull request"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn target_field(&self) -> &'static str {
        "files"
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "owner",
                "Repository Owner",
                "Inherited from PR",
                true,
                Some("rust-lang"),
                true,
            ),
            FieldSchema::new_string(
                "repo",
                "Repository Name",
                "Inherited from PR",
                true,
                Some("rust"),
                true,
            ),
            FieldSchema::new_integer(
                "pull_number",
                "Pull Request Number",
                "Inherited from PR",
                true,
                Some(1),
                Some(1),
                None,
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
        let pull_number = get_i64_param(params, "pull_number")
            .or_else(|| context.get("pull_number").and_then(|s| s.parse().ok()))
            .ok_or_else(|| "Missing required parameter 'pull_number'".to_string())?;

        let mut query = HashMap::new();
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/repos/{owner}/{repo}/pulls/{pull_number}/files");
        let resp = client.fetch(&route, &query).await?;
        Ok(resp.value)
    }
}

pub struct PullCommentsEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for PullCommentsEntity {
    fn id(&self) -> &'static str {
        "pull_comments"
    }

    fn display_name(&self) -> &'static str {
        "PR Comments"
    }

    fn description(&self) -> &'static str {
        "List review comments on a pull request"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn target_field(&self) -> &'static str {
        "comments"
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "owner",
                "Repository Owner",
                "Inherited from PR",
                true,
                Some("rust-lang"),
                true,
            ),
            FieldSchema::new_string(
                "repo",
                "Repository Name",
                "Inherited from PR",
                true,
                Some("rust"),
                true,
            ),
            FieldSchema::new_integer(
                "pull_number",
                "Pull Request Number",
                "Inherited from PR",
                true,
                Some(1),
                Some(1),
                None,
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
        let pull_number = get_i64_param(params, "pull_number")
            .or_else(|| context.get("pull_number").and_then(|s| s.parse().ok()))
            .ok_or_else(|| "Missing required parameter 'pull_number'".to_string())?;

        let mut query = HashMap::new();
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/repos/{owner}/{repo}/pulls/{pull_number}/comments");
        let resp = client.fetch(&route, &query).await?;
        Ok(resp.value)
    }
}
