use super::entity::{EntityDefinition, ParentContext, get_i64_param, get_str_param};
use crate::client::GithubClient;
use crate::schema::FieldSchema;
use std::collections::HashMap;

pub struct UserEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for UserEntity {
    fn id(&self) -> &'static str {
        "user"
    }

    fn display_name(&self) -> &'static str {
        "User"
    }

    fn description(&self) -> &'static str {
        "Fetch public user profile (name, bio, followers, repos count, etc.)"
    }

    fn is_root(&self) -> bool {
        true
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![FieldSchema::new_string(
            "username",
            "Username",
            "The GitHub username (e.g. 'torvalds', 'dtolnay')",
            true,
            Some("torvalds"),
            true,
        )]
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
        let username = get_str_param(params, context, "username")
            .ok_or_else(|| "Missing required parameter 'username'".to_string())?;

        let route = format!("/users/{username}");
        let resp = client.fetch(&route, &HashMap::new()).await?;
        Ok(resp.value)
    }

    fn extract_context(
        &self,
        params: &serde_json::Map<String, serde_json::Value>,
        result: &serde_json::Value,
    ) -> ParentContext {
        let username = result
            .get("login")
            .and_then(|v| v.as_str())
            .or_else(|| params.get("username").and_then(|v| v.as_str()))
            .unwrap_or_default();

        ParentContext::new().with("username", username)
    }
}

pub struct CurrentUserEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for CurrentUserEntity {
    fn id(&self) -> &'static str {
        "current_user"
    }

    fn display_name(&self) -> &'static str {
        "Authenticated User"
    }

    fn description(&self) -> &'static str {
        "Fetch profile of currently authenticated user via personal access token"
    }

    fn is_root(&self) -> bool {
        true
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![]
    }

    fn allowed_children(&self) -> Vec<&'static str> {
        vec!["user_repos", "user_followers", "user_gists"]
    }

    async fn execute(
        &self,
        client: &GithubClient,
        _params: &serde_json::Map<String, serde_json::Value>,
        _context: &ParentContext,
    ) -> Result<serde_json::Value, String> {
        let route = "/user".to_string();
        let resp = client.fetch(&route, &HashMap::new()).await?;
        Ok(resp.value)
    }

    fn extract_context(
        &self,
        _params: &serde_json::Map<String, serde_json::Value>,
        result: &serde_json::Value,
    ) -> ParentContext {
        let username = result
            .get("login")
            .and_then(|v| v.as_str())
            .unwrap_or_default();

        ParentContext::new()
            .with("username", username)
            .with("owner", username)
    }
}

pub struct UserReposEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for UserReposEntity {
    fn id(&self) -> &'static str {
        "user_repos"
    }

    fn display_name(&self) -> &'static str {
        "User Repositories"
    }

    fn description(&self) -> &'static str {
        "List public repositories for a specified user"
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
                "username",
                "Username",
                "Inherited from user",
                true,
                Some("torvalds"),
                true,
            ),
            FieldSchema::new_enum(
                "type",
                "Type",
                "Repository type filter",
                &["all", "owner", "member"],
                Some("owner"),
            ),
            FieldSchema::new_enum(
                "sort",
                "Sort Field",
                "Sort by property",
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
        let username = get_str_param(params, context, "username")
            .ok_or_else(|| "Missing required parameter 'username'".to_string())?;

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

        let route = format!("/users/{username}/repos");
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
            .or_else(|| get_str_param(params, context, "username"));
        if let Some(o) = owner {
            ctx.set("owner", o);
        }
        if let Some(name) = item.get("name").and_then(|n| n.as_str()) {
            ctx.set("repo", name);
        }
        ctx
    }
}

pub struct UserFollowersEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for UserFollowersEntity {
    fn id(&self) -> &'static str {
        "user_followers"
    }

    fn display_name(&self) -> &'static str {
        "Followers"
    }

    fn description(&self) -> &'static str {
        "List followers of the user"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn target_field(&self) -> &'static str {
        "followers"
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "username",
                "Username",
                "Inherited from user",
                true,
                Some("torvalds"),
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
        vec!["user_repos", "user_followers", "user_gists"]
    }

    async fn execute(
        &self,
        client: &GithubClient,
        params: &serde_json::Map<String, serde_json::Value>,
        context: &ParentContext,
    ) -> Result<serde_json::Value, String> {
        let username = get_str_param(params, context, "username")
            .ok_or_else(|| "Missing required parameter 'username'".to_string())?;

        let mut query = HashMap::new();
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/users/{username}/followers");
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

pub struct UserGistsEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for UserGistsEntity {
    fn id(&self) -> &'static str {
        "user_gists"
    }

    fn display_name(&self) -> &'static str {
        "Gists"
    }

    fn description(&self) -> &'static str {
        "List public gists created by the user"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_collection(&self) -> bool {
        true
    }

    fn target_field(&self) -> &'static str {
        "gists"
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![
            FieldSchema::new_string(
                "username",
                "Username",
                "Inherited from user",
                true,
                Some("torvalds"),
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
        vec!["gist_comments"]
    }

    async fn execute(
        &self,
        client: &GithubClient,
        params: &serde_json::Map<String, serde_json::Value>,
        context: &ParentContext,
    ) -> Result<serde_json::Value, String> {
        let username = get_str_param(params, context, "username")
            .ok_or_else(|| "Missing required parameter 'username'".to_string())?;

        let mut query = HashMap::new();
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/users/{username}/gists");
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
        if let Some(id) = item.get("id").and_then(|i| i.as_str()) {
            ctx.set("gist_id", id);
        }
        ctx
    }
}
