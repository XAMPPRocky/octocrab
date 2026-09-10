use super::RepoHandler;
use crate::models::repos::{AutomatedSecurityFixes, PrivateVulnerabilityReporting};
use http::StatusCode;

/// Handler for GitHub's repository vulnerability alerts (Dependabot alerts) API.
///
/// Created with [`RepoHandler::vulnerability_alerts`].
pub struct RepoVulnerabilityAlertsHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoVulnerabilityAlertsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Check if vulnerability alerts are enabled for a repository.
    ///
    /// Returns `Ok(true)` if enabled (HTTP 204 No Content), `Ok(false)` if disabled (HTTP 404 Not Found).
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#check-if-vulnerability-alerts-are-enabled-for-a-repository)
    pub async fn check(&self) -> crate::Result<bool> {
        let route = format!("/{}/vulnerability-alerts", self.handler.repo);
        let response = self.handler.crab._get(route).await?;
        match response.status() {
            StatusCode::NO_CONTENT => Ok(true),
            StatusCode::NOT_FOUND => Ok(false),
            _ => Err(crate::map_github_error(response).await.unwrap_err()),
        }
    }

    /// Enable vulnerability alerts for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#enable-vulnerability-alerts)
    pub async fn enable(&self) -> crate::Result<()> {
        let route = format!("/{}/vulnerability-alerts", self.handler.repo);
        let response = self.handler.crab._put(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Disable vulnerability alerts for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#disable-vulnerability-alerts)
    pub async fn disable(&self) -> crate::Result<()> {
        let route = format!("/{}/vulnerability-alerts", self.handler.repo);
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}

/// Handler for GitHub's repository automated security fixes (Dependabot security updates) API.
///
/// Created with [`RepoHandler::automated_security_fixes`].
pub struct RepoAutomatedSecurityFixesHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoAutomatedSecurityFixesHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Check if automated security fixes are enabled for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#check-if-automated-security-fixes-are-enabled-for-a-repository)
    pub async fn get(&self) -> crate::Result<AutomatedSecurityFixes> {
        let route = format!("/{}/automated-security-fixes", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Enable automated security fixes for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#enable-automated-security-fixes)
    pub async fn enable(&self) -> crate::Result<()> {
        let route = format!("/{}/automated-security-fixes", self.handler.repo);
        let response = self.handler.crab._put(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Disable automated security fixes for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#disable-automated-security-fixes)
    pub async fn disable(&self) -> crate::Result<()> {
        let route = format!("/{}/automated-security-fixes", self.handler.repo);
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}

/// Handler for GitHub's repository private vulnerability reporting API.
///
/// Created with [`RepoHandler::private_vulnerability_reporting`].
pub struct RepoPrivateVulnerabilityReportingHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoPrivateVulnerabilityReportingHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Check if private vulnerability reporting is enabled for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#check-if-private-vulnerability-reporting-is-enabled-for-a-repository)
    pub async fn get(&self) -> crate::Result<PrivateVulnerabilityReporting> {
        let route = format!("/{}/private-vulnerability-reporting", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Helper to check if private vulnerability reporting is enabled.
    pub async fn is_enabled(&self) -> crate::Result<bool> {
        self.get().await.map(|r| r.enabled)
    }

    /// Enable private vulnerability reporting for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#enable-private-vulnerability-reporting)
    pub async fn enable(&self) -> crate::Result<()> {
        let route = format!("/{}/private-vulnerability-reporting", self.handler.repo);
        let response = self.handler.crab._put(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Disable private vulnerability reporting for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#disable-private-vulnerability-reporting)
    pub async fn disable(&self) -> crate::Result<()> {
        let route = format!("/{}/private-vulnerability-reporting", self.handler.repo);
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}
