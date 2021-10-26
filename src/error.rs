use snafu::{Backtrace, Snafu};
use std::fmt;

/// An error that could have occurred while using [`crate::Octocrab`].
#[derive(Snafu, Debug)]
#[snafu(visibility = "pub")]
pub enum Error {
    GitHub {
        source: GitHubError,
        backtrace: Backtrace,
    },
    Url {
        source: url::ParseError,
        backtrace: Backtrace,
    },
    #[snafu(display("HTTP Error: {}\n\nFound at {}", source, backtrace))]
    Http {
        source: reqwest::Error,
        backtrace: Backtrace,
    },
    #[snafu(display("Serde Error: {}\nFound at {}", source, backtrace))]
    Serde {
        source: serde_json::Error,
        backtrace: Backtrace,
    },
    #[snafu(display("JSON Error in {}: {}\nFound at {}", source.path(), source.inner(), backtrace))]
    Json {
        source: serde_path_to_error::Error<serde_json::Error>,
        backtrace: Backtrace,
    },
    Other {
        source: Box<dyn std::error::Error + Send + Sync>,
        backtrace: Backtrace,
    },
}

/// An error returned from GitHub's API.
#[derive(serde::Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct GitHubError {
    pub documentation_url: Option<String>,
    pub errors: Option<Vec<serde_json::Value>>,
    pub message: String,
}

impl fmt::Display for GitHubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)?;

        if let Some(documentation_url) = &self.documentation_url {
            write!(f, "\nDocumentation URL: {}", documentation_url)?;
        }

        if let Some(errors) = &self.errors.filter(|errors| !errors.is_empty()) {
            write!(f, "\nErrors:")?;
            for error in errors {
                write!(f, "\n- {}", error)?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for GitHubError {}
