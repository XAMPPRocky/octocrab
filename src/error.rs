use snafu::{Backtrace, Snafu};
use std::fmt;

#[derive(Snafu, Debug)]
#[snafu(visibility = "pub(crate)")]
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
    Other {
        source: Box<dyn std::error::Error>,
        backtrace: Backtrace,
    },
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct GitHubError {
    documentation_url: String,
    message: String,
}

impl fmt::Display for GitHubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: {}\nDocumentation URL: {}",
            self.message, self.documentation_url
        )
    }
}

impl std::error::Error for GitHubError {}
