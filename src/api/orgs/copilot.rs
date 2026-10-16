//! Copilot organization API.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/copilot?apiVersion=2022-11-28)

use std::marker::PhantomData;

use super::*;

#[derive(serde::Serialize)]
pub struct CopilotHandler<'octo, 'r> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    owner: String,
    #[serde(skip)]
    _phantom: PhantomData<&'r ()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    until: Option<chrono::DateTime<chrono::Utc>>,
}

impl<'octo, 'r> CopilotHandler<'octo, 'r> {
    pub fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self {
            crab: handler.crab,
            owner: handler.owner.clone(),
            _phantom: PhantomData,
            per_page: None,
            page: None,
            since: None,
            until: None,
        }
    }

    pub(crate) fn new_with_owner(crab: &'octo Octocrab, owner: String) -> Self {
        Self {
            crab,
            owner,
            _phantom: PhantomData,
            per_page: None,
            page: None,
            since: None,
            until: None,
        }
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Show usage metrics since this date.
    /// This is a timestamp in ISO 8601 format (YYYY-MM-DDTHH:MM:SSZ).
    /// Maximum value is 28 days ago.
    pub fn since(mut self, since: chrono::DateTime<chrono::Utc>) -> Self {
        self.since = Some(since);
        self
    }

    /// Show usage metrics until this date.
    /// This is a timestamp in ISO 8601 format (YYYY-MM-DDTHH:MM:SSZ) and should not precede the since date if it is passed.
    pub fn until(mut self, until: chrono::DateTime<chrono::Utc>) -> Self {
        self.until = Some(until);
        self
    }

    /// Retrieve copilot metrics for the entire organization
    pub async fn metrics(
        self,
    ) -> crate::Result<Vec<crate::models::orgs_copilot::metrics::CopilotMetrics>> {
        let route = format!("/orgs/{org}/copilot/metrics", org = self.owner);

        self.crab.get(route, Some(&self)).await
    }

    /// Retrieve copilot metrics for a specific team within the organization
    pub async fn metrics_team<T: ToString>(
        self,
        team: T,
    ) -> crate::Result<Vec<crate::models::orgs_copilot::metrics::CopilotMetrics>> {
        let route = format!(
            "/orgs/{org}/team/{team}/copilot/metrics",
            org = self.owner,
            team = team.to_string()
        );

        self.crab.get(route, Some(&self)).await
    }

    /// Get Copilot seat information and settings for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/copilot/copilot-user-management?apiVersion=2022-11-28#get-copilot-seat-information-and-settings-for-an-organization)
    pub async fn billing(
        self,
    ) -> crate::Result<crate::models::orgs_copilot::billing::CopilotBilling> {
        let route = format!("/orgs/{org}/copilot/billing", org = self.owner);

        self.crab.get(route, Some(&self)).await
    }

    /// List all Copilot seat assignments for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/copilot/copilot-user-management?apiVersion=2022-11-28#list-all-copilot-seat-assignments-for-an-organization)
    pub async fn billing_seats(
        self,
    ) -> crate::Result<crate::models::orgs_copilot::billing::CopilotBillingSeats> {
        let route = format!("/orgs/{org}/copilot/billing/seats", org = self.owner);

        self.crab.get(route, Some(&self)).await
    }

    /// Get Copilot seat assignment details for a user.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/copilot/copilot-user-management?apiVersion=2022-11-28#get-copilot-seat-assignment-details-for-a-user)
    pub async fn seat_assignment(
        &self,
        username: impl AsRef<str>,
    ) -> crate::Result<crate::models::orgs_copilot::billing::CopilotSeat> {
        let route = format!(
            "/orgs/{org}/members/{username}/copilot",
            org = self.owner,
            username = username.as_ref(),
        );

        self.crab.get(route, None::<&()>).await
    }

    /// Shortcut for [`Self::seat_assignment`].
    ///
    /// Note: This is an alias for [`seat_assignment`][Self::seat_assignment].
    pub async fn get_user_seat(
        &self,
        username: impl AsRef<str>,
    ) -> crate::Result<crate::models::orgs_copilot::billing::CopilotSeat> {
        self.seat_assignment(username).await
    }

    /// Perform seat management operations, such as adding or removing seats.
    /// These will typically affect your billing and require admin/manage billing permissions.
    pub fn manage_seats(&self) -> copilot_seat_manager::CopilotSeatHandler<'octo, 'r> {
        copilot_seat_manager::CopilotSeatHandler::new_with_owner(self.crab, self.owner.clone())
    }

    /// Get Copilot cloud agent permissions for an organization.
    pub async fn get_coding_agent_permissions(
        &self,
    ) -> crate::Result<crate::models::CopilotCodingAgentPermissions> {
        let route = format!("/orgs/{}/copilot/coding-agent/permissions", self.owner);
        self.crab.get(route, None::<&()>).await
    }

    /// Set Copilot cloud agent permissions for an organization.
    pub async fn set_coding_agent_permissions(
        &self,
        body: &impl serde::Serialize,
    ) -> crate::Result<()> {
        let route = format!("/orgs/{}/copilot/coding-agent/permissions", self.owner);
        let res = self.crab._put(route, Some(body)).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// List repositories enabled for Copilot cloud agent in an organization.
    pub async fn list_coding_agent_repositories(&self) -> crate::Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/copilot/coding-agent/permissions/repositories",
            self.owner
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Set selected repositories for Copilot cloud agent in an organization.
    pub async fn set_coding_agent_repositories(
        &self,
        body: &impl serde::Serialize,
    ) -> crate::Result<()> {
        let route = format!(
            "/orgs/{}/copilot/coding-agent/permissions/repositories",
            self.owner
        );
        let res = self.crab._put(route, Some(body)).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Enable a repository for Copilot cloud agent in an organization.
    pub async fn enable_coding_agent_repository(&self, repo_id: u64) -> crate::Result<()> {
        let route = format!(
            "/orgs/{}/copilot/coding-agent/permissions/repositories/{}",
            self.owner, repo_id
        );
        let res = self.crab._put(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Disable a repository for Copilot cloud agent in an organization.
    pub async fn disable_coding_agent_repository(&self, repo_id: u64) -> crate::Result<()> {
        let route = format!(
            "/orgs/{}/copilot/coding-agent/permissions/repositories/{}",
            self.owner, repo_id
        );
        let res = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Get Copilot content exclusion rules for an organization.
    pub async fn get_content_exclusion(&self) -> crate::Result<serde_json::Value> {
        let route = format!("/orgs/{}/copilot/content_exclusion", self.owner);
        self.crab.get(route, None::<&()>).await
    }

    /// Set Copilot content exclusion rules for an organization.
    pub async fn set_content_exclusion(&self, body: &impl serde::Serialize) -> crate::Result<()> {
        let route = format!("/orgs/{}/copilot/content_exclusion", self.owner);
        let res = self.crab._put(route, Some(body)).await?;
        crate::map_github_error(res).await.map(drop)
    }

    /// Get Copilot organization usage metrics for a specific day.
    pub async fn metrics_reports_organization_1_day(
        &self,
        day: Option<impl AsRef<str>>,
    ) -> crate::Result<crate::models::CopilotMetricsReport> {
        let mut route = format!(
            "/orgs/{}/copilot/metrics/reports/organization-1-day",
            self.owner
        );
        if let Some(d) = day {
            route.push_str("?day=");
            route.push_str(d.as_ref());
        }
        self.crab.get(route, None::<&()>).await
    }

    /// Get Copilot organization usage metrics (28-day latest).
    pub async fn metrics_reports_organization_28_day_latest(
        &self,
    ) -> crate::Result<crate::models::CopilotMetricsReport> {
        let route = format!(
            "/orgs/{}/copilot/metrics/reports/organization-28-day/latest",
            self.owner
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Get Copilot organization repository report for a specific day.
    pub async fn metrics_reports_repos_1_day(
        &self,
        day: Option<impl AsRef<str>>,
    ) -> crate::Result<crate::models::CopilotMetricsReport> {
        let mut route = format!("/orgs/{}/copilot/metrics/reports/repos-1-day", self.owner);
        if let Some(d) = day {
            route.push_str("?day=");
            route.push_str(d.as_ref());
        }
        self.crab.get(route, None::<&()>).await
    }

    /// Get Copilot organization user-teams report for a specific day.
    pub async fn metrics_reports_user_teams_1_day(
        &self,
        day: Option<impl AsRef<str>>,
    ) -> crate::Result<crate::models::CopilotMetricsReport> {
        let mut route = format!(
            "/orgs/{}/copilot/metrics/reports/user-teams-1-day",
            self.owner
        );
        if let Some(d) = day {
            route.push_str("?day=");
            route.push_str(d.as_ref());
        }
        self.crab.get(route, None::<&()>).await
    }

    /// Get Copilot organization users usage metrics for a specific day.
    pub async fn metrics_reports_users_1_day(
        &self,
        day: Option<impl AsRef<str>>,
    ) -> crate::Result<crate::models::CopilotMetricsReport> {
        let mut route = format!("/orgs/{}/copilot/metrics/reports/users-1-day", self.owner);
        if let Some(d) = day {
            route.push_str("?day=");
            route.push_str(d.as_ref());
        }
        self.crab.get(route, None::<&()>).await
    }

    /// Get Copilot organization users usage metrics (28-day latest).
    pub async fn metrics_reports_users_28_day_latest(
        &self,
    ) -> crate::Result<crate::models::CopilotMetricsReport> {
        let route = format!(
            "/orgs/{}/copilot/metrics/reports/users-28-day/latest",
            self.owner
        );
        self.crab.get(route, None::<&()>).await
    }
}
