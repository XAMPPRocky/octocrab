use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::models::{
    commits::{CommitElement, CommitParent},
    GitUser, LabelId, MinimalRepository, SimpleUser,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SearchResultTextMatch {
    pub fragment: Option<String>,
    pub matches: Option<Vec<Match>>,
    pub object_type: Option<String>,
    pub object_url: Option<String>,
    pub property: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Match {
    pub indices: Option<Vec<i64>>,
    pub text: Option<String>,
}

/// Commit Search Result Item
///  
/// OpenAPI reference: #/components/schemas/commit-search-result-item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommitSearchResultItem {
    pub url: Url,
    pub sha: String,
    pub comments_url: Url,
    pub commit: CommitElement,
    pub author: Option<SimpleUser>,
    pub committer: Option<GitUser>,
    pub parents: Vec<CommitParent>,
    pub repository: MinimalRepository,
    pub score: f64,
    pub node_id: String,
    pub text_matches: Option<Vec<SearchResultTextMatch>>,
}

/// Label Search Result Item
///
/// OpenAPI reference: #/components/schemas/label-search-result-item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LabelSearchResultItem {
    pub id: LabelId,
    pub node_id: String,
    pub url: Url,
    pub name: String,
    pub color: String,
    pub default: bool,
    pub description: Option<String>,
    pub score: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_matches: Option<Vec<SearchResultTextMatch>>,
}

/// Topic Search Result Item
///
/// OpenAPI reference: #/components/schemas/topic-search-result-item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TopicSearchResultItem {
    pub name: String,
    pub display_name: Option<String>,
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub released: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub featured: bool,
    pub curated: bool,
    pub score: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_matches: Option<Vec<SearchResultTextMatch>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related: Option<Vec<TopicRelationItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<TopicRelationItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TopicRelationItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_relation: Option<TopicRelation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TopicRelation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_type: Option<String>,
}
