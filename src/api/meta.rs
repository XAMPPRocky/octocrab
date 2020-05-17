use crate::{models, Octocrab, Result};

pub struct MetaHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> MetaHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Fetches your current rate limit status.
    /// ```no_run
    /// # async def run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab.meta().rate_limits().await?;
    /// # Ok(())
    /// # }
    pub async fn rate_limits(&self) -> Result<models::ResourcesRateLimits> {
        let limits: Result<models::RateLimits> = self.crab.get("/rate_limit", None::<&()>).await;
        limits.map(|l| l.resources)
    }
}
