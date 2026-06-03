use serde::{Deserialize, Serialize};
use url::Url;

use crate::models::{
    commits::{CommitElement, CommitParent},
    GitUser, MinimalRepository, SimpleUser,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SearchResultTextMatch {
    pub fragment: Option<String>,
    pub matches: Option<Vec<Match>>,
    pub object_type: Option<String>,
    pub object_url: Option<String>,
    pub property: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
