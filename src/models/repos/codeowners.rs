use serde::{Deserialize, Serialize};

/// Errors detected in a repository's CODEOWNERS file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodeownersErrors {
    #[serde(default)]
    pub errors: Vec<CodeownersError>,
}

impl CodeownersErrors {
    pub fn new(errors: Vec<CodeownersError>) -> Self {
        Self { errors }
    }
}

/// A single syntax or validation error in a CODEOWNERS file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodeownersError {
    pub line: u32,
    pub column: u32,
    pub kind: String,
    #[serde(default)]
    pub suggestion: Option<String>,
    pub message: String,
    pub path: String,
}

impl CodeownersError {
    pub fn new(
        line: u32,
        column: u32,
        kind: impl Into<String>,
        suggestion: Option<String>,
        message: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Self {
            line,
            column,
            kind: kind.into(),
            suggestion,
            message: message.into(),
            path: path.into(),
        }
    }
}
