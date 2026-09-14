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
}
