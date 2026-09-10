use super::RepoHandler;
use crate::models::repos::{Clones, PathTraffic, ReferrerTraffic, Views};
use crate::params::repos::TrafficInterval;
use crate::Result;

/// A client to GitHub's repository traffic API.
///
/// Created with [`RepoHandler::traffic`].
pub struct RepoTrafficHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoTrafficHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Creates a [`ClonesBuilder`] to fetch clone counts and breakdown.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/metrics/traffic#get-repository-clones)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params::repos::TrafficInterval;
    ///
    /// let clones = octocrab.repos("owner", "repo")
    ///     .traffic()
    ///     .clones()
    ///     .per(TrafficInterval::Week)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn clones(&self) -> ClonesBuilder<'octo, 'r> {
        ClonesBuilder::new(self.handler)
    }

    /// Creates a [`ViewsBuilder`] to fetch page view counts and breakdown.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/metrics/traffic#get-page-views)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params::repos::TrafficInterval;
    ///
    /// let views = octocrab.repos("owner", "repo")
    ///     .traffic()
    ///     .views()
    ///     .per(TrafficInterval::Day)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn views(&self) -> ViewsBuilder<'octo, 'r> {
        ViewsBuilder::new(self.handler)
    }

    /// Get the top 10 popular contents/paths over the last 14 days.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/metrics/traffic#get-top-referral-paths)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let paths = octocrab.repos("owner", "repo")
    ///     .traffic()
    ///     .popular_paths()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn popular_paths(&self) -> Result<Vec<PathTraffic>> {
        let route = format!("/{}/traffic/popular/paths", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Alias for [`popular_paths`][Self::popular_paths].
    pub async fn paths(&self) -> Result<Vec<PathTraffic>> {
        self.popular_paths().await
    }

    /// Get the top 10 referrers over the last 14 days.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/metrics/traffic#get-top-referral-sources)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let referrers = octocrab.repos("owner", "repo")
    ///     .traffic()
    ///     .popular_referrers()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn popular_referrers(&self) -> Result<Vec<ReferrerTraffic>> {
        let route = format!("/{}/traffic/popular/referrers", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Alias for [`popular_referrers`][Self::popular_referrers].
    pub async fn referrers(&self) -> Result<Vec<ReferrerTraffic>> {
        self.popular_referrers().await
    }
}

/// A builder pattern struct for fetching repository clone traffic.
///
/// Created by [`RepoTrafficHandler::clones`].
#[derive(serde::Serialize)]
pub struct ClonesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per: Option<TrafficInterval>,
}

impl<'octo, 'r> ClonesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler, per: None }
    }

    /// Set the time frame (`day` or `week`).
    pub fn per(mut self, per: impl Into<TrafficInterval>) -> Self {
        self.per = Some(per.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Clones> {
        let route = format!("/{}/traffic/clones", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A builder pattern struct for fetching repository page view traffic.
///
/// Created by [`RepoTrafficHandler::views`].
#[derive(serde::Serialize)]
pub struct ViewsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per: Option<TrafficInterval>,
}

impl<'octo, 'r> ViewsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler, per: None }
    }

    /// Set the time frame (`day` or `week`).
    pub fn per(mut self, per: impl Into<TrafficInterval>) -> Self {
        self.per = Some(per.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Views> {
        let route = format!("/{}/traffic/views", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}
