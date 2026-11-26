use crate::client::GithubClient;
use crate::schema::FieldSchema;
use std::collections::HashMap;

/// Context passed down from parent entities to child entities
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ParentContext {
    pub values: HashMap<String, String>,
}

impl ParentContext {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn with(mut self, key: &str, value: impl Into<String>) -> Self {
        self.values.insert(key.to_string(), value.into());
        self
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }

    pub fn set(&mut self, key: &str, value: impl Into<String>) {
        self.values.insert(key.to_string(), value.into());
    }

    pub fn merge(&mut self, other: &ParentContext) {
        for (k, v) in &other.values {
            self.values.insert(k.clone(), v.clone());
        }
    }
}

pub fn get_str_param<'a>(
    params: &'a serde_json::Map<String, serde_json::Value>,
    context: &'a ParentContext,
    key: &str,
) -> Option<&'a str> {
    params
        .get(key)
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .or_else(|| context.get(key))
}

pub fn get_i64_param(
    params: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Option<i64> {
    params.get(key).and_then(|v| v.as_i64())
}

#[async_trait::async_trait(?Send)]
pub trait EntityDefinition {
    /// Unique identifier for this entity type (e.g. "org", "repo", "repo_issues")
    fn id(&self) -> &'static str;

    /// Human-friendly display label (e.g. "Repository", "Issues")
    fn display_name(&self) -> &'static str;

    /// Summary description
    fn description(&self) -> &'static str;

    /// Indicates whether this entity can be placed at the root of a query
    fn is_root(&self) -> bool;

    /// List of parameter fields required or accepted by this entity query
    fn schema(&self) -> Vec<FieldSchema>;

    /// Entities allowed to be queried as children of this entity
    fn allowed_children(&self) -> Vec<&'static str>;

    /// Target field name on the parent object where this entity's data should be grouped
    /// (e.g. "repositories", "pull_requests", "issues", "comments")
    fn target_field(&self) -> &'static str {
        self.id()
    }

    /// Indicates whether this entity returns a collection/list of items
    fn is_collection(&self) -> bool {
        false
    }

    /// Primary execution function
    async fn execute(
        &self,
        client: &GithubClient,
        params: &serde_json::Map<String, serde_json::Value>,
        context: &ParentContext,
    ) -> Result<serde_json::Value, String>;

    /// Context variables to pass down to child entities from this entity's response
    fn extract_context(
        &self,
        _params: &serde_json::Map<String, serde_json::Value>,
        _result: &serde_json::Value,
    ) -> ParentContext {
        ParentContext::new()
    }

    /// Context variables extracted from an individual item in a collection
    fn extract_item_context(
        &self,
        _params: &serde_json::Map<String, serde_json::Value>,
        _context: &ParentContext,
        _item: &serde_json::Value,
    ) -> ParentContext {
        ParentContext::new()
    }
}
