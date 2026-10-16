//! The security campaigns API.
use crate::models::campaigns::{
    Campaign, CampaignSort, CampaignState, CreateCampaign, UpdateCampaign,
};
use crate::params::Direction;
use crate::{Octocrab, Page, Result};

/// Handler for GitHub's security campaigns API for an organization.
///
/// Created with [`Octocrab::campaigns`] or [`crate::orgs::OrgHandler::campaigns`].
pub struct CampaignsHandler<'octo> {
    crab: &'octo Octocrab,
    org: String,
}

impl<'octo> CampaignsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, org: String) -> Self {
        Self { crab, org }
    }

    /// List campaigns for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/campaigns/campaigns?apiVersion=2022-11-28#list-campaigns-for-an-organization)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let campaigns = octocrab
    ///     .orgs("owner")
    ///     .campaigns()
    ///     .list()
    ///     .per_page(10)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListCampaignsBuilder<'octo, '_> {
        ListCampaignsBuilder::new(self)
    }

    /// Create a campaign for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/campaigns/campaigns?apiVersion=2022-11-28#create-a-campaign-for-an-organization)
    pub async fn create(&self, body: &CreateCampaign) -> Result<Campaign> {
        let route = format!("/orgs/{}/campaigns", self.org);
        self.crab.post(route, Some(body)).await
    }

    /// Get a campaign for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/campaigns/campaigns?apiVersion=2022-11-28#get-a-campaign-for-an-organization)
    pub async fn get(&self, campaign_number: u64) -> Result<Campaign> {
        let route = format!("/orgs/{}/campaigns/{}", self.org, campaign_number);
        self.crab.get(route, None::<&()>).await
    }

    /// Update a campaign for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/campaigns/campaigns?apiVersion=2022-11-28#update-a-campaign)
    pub async fn update(&self, campaign_number: u64, body: &UpdateCampaign) -> Result<Campaign> {
        let route = format!("/orgs/{}/campaigns/{}", self.org, campaign_number);
        self.crab.patch(route, Some(body)).await
    }

    /// Delete a campaign for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/campaigns/campaigns?apiVersion=2022-11-28#delete-a-campaign-for-an-organization)
    pub async fn delete(&self, campaign_number: u64) -> Result<()> {
        let route = format!("/orgs/{}/campaigns/{}", self.org, campaign_number);
        let response = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }
}

/// Builder for listing campaigns for an organization.
#[derive(serde::Serialize)]
pub struct ListCampaignsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b CampaignsHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<CampaignState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<CampaignSort>,
}

impl<'octo, 'b> ListCampaignsBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b CampaignsHandler<'octo>) -> Self {
        Self {
            handler,
            page: None,
            per_page: None,
            direction: None,
            state: None,
            sort: None,
        }
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// The direction to sort the results by.
    pub fn direction(mut self, direction: impl Into<Direction>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    /// Filter by campaign state.
    pub fn state(mut self, state: impl Into<CampaignState>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// The property to sort the results by.
    pub fn sort(mut self, sort: impl Into<CampaignSort>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Sends the request to list campaigns.
    pub async fn send(self) -> Result<Page<Campaign>> {
        let route = format!("/orgs/{}/campaigns", self.handler.org);
        self.handler.crab.get(route, Some(&self)).await
    }
}
