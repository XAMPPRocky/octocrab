use serde::{Deserialize, Serialize};

/// Contextual hovercard information for a user.
///
/// [GitHub API Documentation](https://docs.github.com/en/rest/users/users?apiVersion=2022-11-28#get-contextual-information-for-a-user)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Hovercard {
    pub contexts: Vec<HovercardContext>,
}

/// A context item in a hovercard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HovercardContext {
    pub message: String,
    pub octicon: String,
}
