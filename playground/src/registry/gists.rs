use super::entity::{EntityDefinition, ParentContext, get_i64_param, get_str_param};
use crate::client::GithubClient;
use crate::schema::FieldSchema;
use std::collections::HashMap;

pub struct GistEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for GistEntity {
    fn id(&self) -> &'static str {
        "gist"
    }

    fn display_name(&self) -> &'static str {
        "Gist"
    }

    fn description(&self) -> &'static str {
        "Fetch GitHub Gist details by ID"
    }

    fn is_root(&self) -> bool {
        true
    }

    fn schema(&self) -> Vec<FieldSchema> {
        vec![FieldSchema::new_string(
            "gist_id",
            "Gist ID",
            "The unique ID or hash of the Gist",
            true,
            Some("aa5a315d61ae9438b18d"),
            false,
        )]
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
        let gist_id = get_str_param(params, context, "gist_id")
            .ok_or_else(|| "Missing required parameter 'gist_id'".to_string())?;

        let route = format!("/gists/{gist_id}");
        let resp = client.fetch(&route, &HashMap::new()).await?;
        Ok(resp.value)
    }

    fn extract_context(
        &self,
        params: &serde_json::Map<String, serde_json::Value>,
        result: &serde_json::Value,
    ) -> ParentContext {
        let gist_id = result
            .get("id")
            .and_then(|v| v.as_str())
            .or_else(|| params.get("gist_id").and_then(|v| v.as_str()))
            .unwrap_or_default();

        ParentContext::new().with("gist_id", gist_id)
    }
}

pub struct GistCommentsEntity;

#[async_trait::async_trait(?Send)]
impl EntityDefinition for GistCommentsEntity {
    fn id(&self) -> &'static str {
        "gist_comments"
    }

    fn display_name(&self) -> &'static str {
        "Gist Comments"
    }

    fn description(&self) -> &'static str {
        "List comments for the specified Gist"
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
                "gist_id",
                "Gist ID",
                "Inherited from gist",
                true,
                Some("aa5a315d61ae9438b18d"),
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
        let gist_id = get_str_param(params, context, "gist_id")
            .ok_or_else(|| "Missing required parameter 'gist_id'".to_string())?;

        let mut query = HashMap::new();
        if let Some(pp) = get_i64_param(params, "per_page") {
            query.insert("per_page".to_string(), pp.to_string());
        }

        let route = format!("/gists/{gist_id}/comments");
        let resp = client.fetch(&route, &query).await?;
        Ok(resp.value)
    }
}
