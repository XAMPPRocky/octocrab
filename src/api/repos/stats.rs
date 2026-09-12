//! GitHub Repository Statistics API.
//!
//! See: [GitHub API Documentation](https://docs.github.com/en/rest/metrics/statistics?apiVersion=2022-11-28)

use super::RepoHandler;
use crate::{
    models::repos::{
        CodeFrequency, CommitActivity, ContributorActivity, ParticipationStats, PunchCard,
    },
    FromResponse, Result,
};

/// A client to GitHub's repository statistics API.
///
/// Created with [`RepoHandler::stats`].
///
/// See also: [GitHub API Documentation](https://docs.github.com/en/rest/metrics/statistics?apiVersion=2022-11-28)
pub struct RepoStatsHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoStatsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Returns a weekly aggregate of the number of additions and deletions of code to a repository.
    ///
    /// If GitHub is in the process of generating this data, a 202 Accepted status is returned,
    /// which maps to `Ok(None)`. When the data is ready, it returns `Ok(Some(stats))`.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/metrics/statistics?apiVersion=2022-11-28#get-the-weekly-commit-activity)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// if let Some(code_freq) = octocrab
    ///     .repos("owner", "repo")
    ///     .stats()
    ///     .code_frequency()
    ///     .await?
    /// {
    ///     for stat in code_freq {
    ///         println!("Week {}: +{} -{}", stat.week(), stat.additions(), stat.deletions());
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn code_frequency(&self) -> Result<Option<Vec<CodeFrequency>>> {
        let route = format!("/{}/stats/code_frequency", self.handler.repo);
        let response = self.handler.crab._get(route).await?;
        match response.status() {
            http::StatusCode::ACCEPTED => Ok(None),
            _ => {
                let response = crate::map_github_error(response).await?;
                let stats = Vec::<CodeFrequency>::from_response(response).await?;
                Ok(Some(stats))
            }
        }
    }

    /// Returns the last year of commit activity grouped by week.
    ///
    /// If GitHub is in the process of generating this data, a 202 Accepted status is returned,
    /// which maps to `Ok(None)`. When the data is ready, it returns `Ok(Some(stats))`.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/metrics/statistics?apiVersion=2022-11-28#get-the-last-year-of-commit-activity)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// if let Some(activity) = octocrab
    ///     .repos("owner", "repo")
    ///     .stats()
    ///     .commit_activity()
    ///     .await?
    /// {
    ///     for week in activity {
    ///         println!("Week {}: {} commits", week.week, week.total);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn commit_activity(&self) -> Result<Option<Vec<CommitActivity>>> {
        let route = format!("/{}/stats/commit_activity", self.handler.repo);
        let response = self.handler.crab._get(route).await?;
        match response.status() {
            http::StatusCode::ACCEPTED => Ok(None),
            _ => {
                let response = crate::map_github_error(response).await?;
                let stats = Vec::<CommitActivity>::from_response(response).await?;
                Ok(Some(stats))
            }
        }
    }

    /// Returns the total number of commits made by each contributor, along with weekly additions, deletions, and commit counts.
    ///
    /// If GitHub is in the process of generating this data, a 202 Accepted status is returned,
    /// which maps to `Ok(None)`. When the data is ready, it returns `Ok(Some(stats))`.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/metrics/statistics?apiVersion=2022-11-28#get-all-contributor-commit-activity)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// if let Some(contributors) = octocrab
    ///     .repos("owner", "repo")
    ///     .stats()
    ///     .contributors()
    ///     .await?
    /// {
    ///     for c in contributors {
    ///         println!("Total commits: {}", c.total);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn contributors(&self) -> Result<Option<Vec<ContributorActivity>>> {
        let route = format!("/{}/stats/contributors", self.handler.repo);
        let response = self.handler.crab._get(route).await?;
        match response.status() {
            http::StatusCode::ACCEPTED => Ok(None),
            _ => {
                let response = crate::map_github_error(response).await?;
                let stats = Vec::<ContributorActivity>::from_response(response).await?;
                Ok(Some(stats))
            }
        }
    }

    /// Returns the total commit counts for the repository owner and all committers over the last 52 weeks.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/metrics/statistics?apiVersion=2022-11-28#get-the-weekly-commit-count)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let participation = octocrab
    ///     .repos("owner", "repo")
    ///     .stats()
    ///     .participation()
    ///     .await?;
    /// println!("All commits count: {}", participation.all.iter().sum::<i64>());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn participation(&self) -> Result<ParticipationStats> {
        let route = format!("/{}/stats/participation", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Returns the hourly commit count for each day.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/metrics/statistics?apiVersion=2022-11-28#get-the-hourly-commit-count-for-each-day)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let punch_card = octocrab
    ///     .repos("owner", "repo")
    ///     .stats()
    ///     .punch_card()
    ///     .await?;
    /// for card in punch_card {
    ///     println!("Day {}, Hour {}: {} commits", card.day(), card.hour(), card.commits());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn punch_card(&self) -> Result<Vec<PunchCard>> {
        let route = format!("/{}/stats/punch_card", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }
}
