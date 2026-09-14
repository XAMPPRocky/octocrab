//! GitHub Marketplace API.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/apps/marketplace?apiVersion=2022-11-28)

use crate::models::marketplace::{MarketplaceAccount, MarketplacePlan, UserMarketplacePurchase};
use crate::models::{MarketplaceAccountId, PlanId};
use crate::{Octocrab, Page, Result};

/// Handler for GitHub's Marketplace API.
///
/// Created with [`Octocrab::marketplace`].
pub struct MarketplaceHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> MarketplaceHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Lists all plans for your Marketplace listing.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/marketplace?apiVersion=2022-11-28#list-plans)
    pub fn list_plans(&self) -> ListMarketplacePlansBuilder<'octo> {
        ListMarketplacePlansBuilder::new(self.crab, false)
    }

    /// Lists all plans for your Marketplace listing (stubbed for testing).
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/marketplace?apiVersion=2022-11-28#list-plans-stubbed)
    pub fn list_plans_stubbed(&self) -> ListMarketplacePlansBuilder<'octo> {
        ListMarketplacePlansBuilder::new(self.crab, true)
    }

    /// Gets a subscription plan for an account.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/marketplace?apiVersion=2022-11-28#get-a-subscription-plan-for-an-account)
    pub async fn get_plan_for_account(
        &self,
        account_id: impl Into<MarketplaceAccountId>,
    ) -> Result<MarketplaceAccount> {
        let route = format!("/marketplace_listing/accounts/{}", account_id.into());
        self.crab.get(route, None::<&()>).await
    }

    /// Gets a subscription plan for an account (stubbed for testing).
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/marketplace?apiVersion=2022-11-28#get-a-subscription-plan-for-an-account-stubbed)
    pub async fn get_plan_for_account_stubbed(
        &self,
        account_id: impl Into<MarketplaceAccountId>,
    ) -> Result<MarketplaceAccount> {
        let route = format!(
            "/marketplace_listing/stubbed/accounts/{}",
            account_id.into()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Lists accounts for a plan.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/marketplace?apiVersion=2022-11-28#list-accounts-for-a-plan)
    pub fn list_accounts_for_plan(
        &self,
        plan_id: impl Into<PlanId>,
    ) -> ListMarketplaceAccountsForPlanBuilder<'octo> {
        ListMarketplaceAccountsForPlanBuilder::new(self.crab, plan_id.into(), false)
    }

    /// Lists accounts for a plan (stubbed for testing).
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/marketplace?apiVersion=2022-11-28#list-accounts-for-a-plan-stubbed)
    pub fn list_accounts_for_plan_stubbed(
        &self,
        plan_id: impl Into<PlanId>,
    ) -> ListMarketplaceAccountsForPlanBuilder<'octo> {
        ListMarketplaceAccountsForPlanBuilder::new(self.crab, plan_id.into(), true)
    }

    /// Lists subscriptions for the authenticated user.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/marketplace?apiVersion=2022-11-28#list-subscriptions-for-the-authenticated-user)
    pub fn list_purchases_for_user(&self) -> ListUserMarketplacePurchasesBuilder<'octo> {
        ListUserMarketplacePurchasesBuilder::new(self.crab, false)
    }

    /// Lists subscriptions for the authenticated user (stubbed for testing).
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/marketplace?apiVersion=2022-11-28#list-subscriptions-for-the-authenticated-user-stubbed)
    pub fn list_purchases_for_user_stubbed(&self) -> ListUserMarketplacePurchasesBuilder<'octo> {
        ListUserMarketplacePurchasesBuilder::new(self.crab, true)
    }
}

/// A builder pattern struct for listing Marketplace plans.
///
/// Created by [`MarketplaceHandler::list_plans`] or [`MarketplaceHandler::list_plans_stubbed`].
#[derive(serde::Serialize)]
pub struct ListMarketplacePlansBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    stubbed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> ListMarketplacePlansBuilder<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, stubbed: bool) -> Self {
        Self {
            crab,
            stubbed,
            per_page: None,
            page: None,
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

    /// Sends the actual request.
    pub async fn send(self) -> Result<Page<MarketplacePlan>> {
        let route = if self.stubbed {
            "/marketplace_listing/stubbed/plans"
        } else {
            "/marketplace_listing/plans"
        };
        self.crab.get(route, Some(&self)).await
    }
}

/// A builder pattern struct for listing accounts for a Marketplace plan.
///
/// Created by [`MarketplaceHandler::list_accounts_for_plan`] or [`MarketplaceHandler::list_accounts_for_plan_stubbed`].
#[derive(serde::Serialize)]
pub struct ListMarketplaceAccountsForPlanBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    plan_id: PlanId,
    #[serde(skip)]
    stubbed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> ListMarketplaceAccountsForPlanBuilder<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, plan_id: PlanId, stubbed: bool) -> Self {
        Self {
            crab,
            plan_id,
            stubbed,
            sort: None,
            direction: None,
            per_page: None,
            page: None,
        }
    }

    /// Key for sorting the results. Can be `created` or `updated`.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Direction of the sort. Can be `asc` or `desc`.
    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.direction = Some(direction.into());
        self
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

    /// Sends the actual request.
    pub async fn send(self) -> Result<Page<MarketplaceAccount>> {
        let route = if self.stubbed {
            format!(
                "/marketplace_listing/stubbed/plans/{}/accounts",
                self.plan_id
            )
        } else {
            format!("/marketplace_listing/plans/{}/accounts", self.plan_id)
        };
        self.crab.get(route, Some(&self)).await
    }
}

/// A builder pattern struct for listing user Marketplace purchases.
///
/// Created by [`MarketplaceHandler::list_purchases_for_user`] or [`MarketplaceHandler::list_purchases_for_user_stubbed`].
#[derive(serde::Serialize)]
pub struct ListUserMarketplacePurchasesBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    stubbed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> ListUserMarketplacePurchasesBuilder<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, stubbed: bool) -> Self {
        Self {
            crab,
            stubbed,
            per_page: None,
            page: None,
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

    /// Sends the actual request.
    pub async fn send(self) -> Result<Page<UserMarketplacePurchase>> {
        let route = if self.stubbed {
            "/user/marketplace_purchases/stubbed"
        } else {
            "/user/marketplace_purchases"
        };
        self.crab.get(route, Some(&self)).await
    }
}
