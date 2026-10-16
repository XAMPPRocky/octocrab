//! GitHub Credentials API.

use crate::{Octocrab, Result};
use serde::Serialize;

/// Handler for GitHub's Credentials API.
///
/// Created with [`Octocrab::credentials`].
pub struct CredentialsHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> CredentialsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Revoke a list of credentials.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/credentials/revoke?apiVersion=2022-11-28#revoke-a-list-of-credentials)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// octocrab.credentials().revoke(["ghp_secret_token"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn revoke<I, S>(&self, credentials: I) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        #[derive(Serialize)]
        struct Body {
            credentials: Vec<String>,
        }

        let body = Body {
            credentials: credentials.into_iter().map(Into::into).collect(),
        };

        let response = self.crab._post("/credentials/revoke", Some(&body)).await?;
        crate::map_github_error(response).await.map(drop)
    }
}
