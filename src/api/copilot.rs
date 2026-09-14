//! GitHub Copilot API.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/copilot?apiVersion=2022-11-28)

use crate::Octocrab;

/// A client to GitHub's Copilot API.
///
/// Created with [`Octocrab::copilot`].
pub struct CopilotHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> CopilotHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Access Copilot APIs for a specific organization.
    ///
    /// # Examples
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let billing = octocrab.copilot().org("org").billing().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn org(
        &self,
        org: impl Into<String>,
    ) -> crate::api::orgs::copilot::CopilotHandler<'octo, '_> {
        crate::api::orgs::copilot::CopilotHandler::new_with_owner(self.crab, org.into())
    }
}
