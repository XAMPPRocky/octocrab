//! Github RateLimit API

use crate::{models, Octocrab, Result};

/// Handler for GitHub's rate_limit API.
///
/// Created with [`Octocrab::ratelimit`].
pub struct RateLimitHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> RateLimitHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Get the rate limit.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/rest/rate-limit/rate-limit#get-rate-limit-status-for-the-authenticated-user)
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let ratelimit = octocrab::instance()
    ///     .ratelimit()
    ///     .get()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self) -> Result<models::RateLimit> {
        self.crab.get("/rate_limit", None::<&()>).await
    }
}
