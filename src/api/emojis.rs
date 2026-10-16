//! GitHub Emojis API.

use crate::{Octocrab, Result};
use std::collections::BTreeMap;

/// Handler for GitHub's Emojis API.
///
/// Created with [`Octocrab::emojis`].
pub struct EmojisHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> EmojisHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Lists all the emojis available to use on GitHub.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/emojis/emojis?apiVersion=2022-11-28#get-emojis)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let emojis = octocrab.emojis().list().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<BTreeMap<String, String>> {
        self.crab.get("/emojis", None::<&()>).await
    }
}
