//! The GitHub Billing API.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/billing?apiVersion=2022-11-28)

use crate::models::billing::{
    ActionsBillingUsage, BillingAiCreditUsageReport, BillingPremiumRequestUsageReport,
    BillingUsageReport, BillingUsageSummaryReport, Budget, CombinedBillingUsage, CreateBudget,
    CreateBudgetResponse, DeleteBudgetResponse, GetAllBudgets, PackagesBillingUsage, UpdateBudget,
    UpdateBudgetResponse,
};
use crate::{Octocrab, Result};

/// The target namespace/owner for billing operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BillingOwner {
    Org(String),
    User(String),
}

impl BillingOwner {
    /// Prefix for Actions, Packages, and Shared Storage endpoints:
    /// `/orgs/{org}/settings/billing` or `/users/{user}/settings/billing`
    pub(crate) fn legacy_prefix(&self) -> String {
        match self {
            Self::Org(org) => format!("/orgs/{org}/settings/billing"),
            Self::User(user) => format!("/users/{user}/settings/billing"),
        }
    }

    /// Prefix for Usage, Summaries, AI Credit, Premium Request, and Budgets:
    /// `/organizations/{org}/settings/billing` or `/users/{user}/settings/billing`
    pub(crate) fn usage_prefix(&self) -> String {
        match self {
            Self::Org(org) => format!("/organizations/{org}/settings/billing"),
            Self::User(user) => format!("/users/{user}/settings/billing"),
        }
    }
}

/// Entry-point handler for GitHub's Billing API.
///
/// Created with [`Octocrab::billing`].
pub struct BillingHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> BillingHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Scope billing operations to an organization.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let actions_billing = octocrab.billing().org("owner").actions().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn org(&self, org: impl Into<String>) -> ScopedBillingHandler<'octo> {
        ScopedBillingHandler::new(self.crab, BillingOwner::Org(org.into()))
    }

    /// Scope billing operations to a user.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let actions_billing = octocrab.billing().user("user").actions().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn user(&self, user: impl Into<String>) -> ScopedBillingHandler<'octo> {
        ScopedBillingHandler::new(self.crab, BillingOwner::User(user.into()))
    }
}

/// Handler for billing operations scoped to an organization or user.
///
/// Created via [`BillingHandler::org`], [`BillingHandler::user`],
/// [`OrgHandler::billing`][crate::api::orgs::OrgHandler::billing], or
/// [`UserHandler::billing`][crate::api::users::UserHandler::billing].
pub struct ScopedBillingHandler<'octo> {
    crab: &'octo Octocrab,
    owner: BillingOwner,
}

impl<'octo> ScopedBillingHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: BillingOwner) -> Self {
        Self { crab, owner }
    }

    /// Scope billing operations to another organization.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let billing = octocrab.billing().org("owner").org("another-org");
    /// # Ok(())
    /// # }
    /// ```
    pub fn org(&self, org: impl Into<String>) -> Self {
        Self::new(self.crab, BillingOwner::Org(org.into()))
    }

    /// Scope billing operations to another user.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let billing = octocrab.billing().user("user").user("another-user");
    /// # Ok(())
    /// # }
    /// ```
    pub fn user(&self, user: impl Into<String>) -> Self {
        Self::new(self.crab, BillingOwner::User(user.into()))
    }

    /// Get GitHub Actions billing usage for the organization or user.
    ///
    /// See:
    /// - [Get GitHub Actions billing for an organization](https://docs.github.com/en/rest/billing?apiVersion=2022-11-28#get-github-actions-billing-for-an-organization)
    /// - [Get GitHub Actions billing for a user](https://docs.github.com/en/rest/billing?apiVersion=2022-11-28#get-github-actions-billing-for-a-user)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let usage = octocrab.billing().org("owner").actions().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn actions(&self) -> Result<ActionsBillingUsage> {
        let route = format!("{}/actions", self.owner.legacy_prefix());
        self.crab.get(route, None::<&()>).await
    }

    /// Get GitHub Packages billing usage for the organization or user.
    ///
    /// See:
    /// - [Get GitHub Packages billing for an organization](https://docs.github.com/en/rest/billing?apiVersion=2022-11-28#get-github-packages-billing-for-an-organization)
    /// - [Get GitHub Packages billing for a user](https://docs.github.com/en/rest/billing?apiVersion=2022-11-28#get-github-packages-billing-for-a-user)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let usage = octocrab.billing().org("owner").packages().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn packages(&self) -> Result<PackagesBillingUsage> {
        let route = format!("{}/packages", self.owner.legacy_prefix());
        self.crab.get(route, None::<&()>).await
    }

    /// Get shared storage billing usage for the organization or user.
    ///
    /// See:
    /// - [Get shared storage billing for an organization](https://docs.github.com/en/rest/billing?apiVersion=2022-11-28#get-shared-storage-billing-for-an-organization)
    /// - [Get shared storage billing for a user](https://docs.github.com/en/rest/billing?apiVersion=2022-11-28#get-shared-storage-billing-for-a-user)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let usage = octocrab.billing().org("owner").shared_storage().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn shared_storage(&self) -> Result<CombinedBillingUsage> {
        let route = format!("{}/shared-storage", self.owner.legacy_prefix());
        self.crab.get(route, None::<&()>).await
    }

    /// Builder for getting billing usage report for the organization or user.
    ///
    /// See:
    /// - [Get billing usage report for an organization](https://docs.github.com/en/rest/billing/usage?apiVersion=2022-11-28#get-billing-usage-report-for-an-organization)
    /// - [Get billing usage report for a user](https://docs.github.com/en/rest/billing/usage?apiVersion=2022-11-28#get-billing-usage-report-for-a-user)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let usage = octocrab.billing().org("owner").usage(2023).month(5).send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn usage(&self, year: u32) -> GetBillingUsageBuilder<'octo, '_> {
        GetBillingUsageBuilder::new(self, year)
    }

    /// Builder for getting billing usage summary for the organization or user.
    ///
    /// See:
    /// - [Get billing usage summary for an organization](https://docs.github.com/en/rest/billing/usage?apiVersion=2022-11-28#get-billing-usage-summary-for-an-organization)
    /// - [Get billing usage summary for a user](https://docs.github.com/en/rest/billing/usage?apiVersion=2022-11-28#get-billing-usage-summary-for-a-user)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let summary = octocrab.billing().org("owner").usage_summary(2023).month(5).send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn usage_summary(&self, year: u32) -> GetBillingUsageSummaryBuilder<'octo, '_> {
        GetBillingUsageSummaryBuilder::new(self, year)
    }

    /// Builder for getting billing AI credit usage report for the organization or user.
    ///
    /// See:
    /// - [Get billing AI credit usage report for an organization](https://docs.github.com/en/rest/billing/usage?apiVersion=2022-11-28#get-billing-ai-credit-usage-report-for-an-organization)
    /// - [Get billing AI credit usage report for a user](https://docs.github.com/en/rest/billing/usage?apiVersion=2022-11-28#get-billing-ai-credit-usage-report-for-a-user)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let ai_credit = octocrab.billing().org("owner").ai_credit_usage(2023).month(5).send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn ai_credit_usage(&self, year: u32) -> GetBillingAiCreditUsageBuilder<'octo, '_> {
        GetBillingAiCreditUsageBuilder::new(self, year)
    }

    /// Builder for getting billing premium request usage report for the organization or user.
    ///
    /// See:
    /// - [Get billing premium request usage report for an organization](https://docs.github.com/en/rest/billing/usage?apiVersion=2022-11-28#get-billing-premium-request-usage-report-for-an-organization)
    /// - [Get billing premium request usage report for a user](https://docs.github.com/en/rest/billing/usage?apiVersion=2022-11-28#get-billing-premium-request-usage-report-for-a-user)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let report = octocrab.billing().org("owner").premium_request_usage(2023).month(5).send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn premium_request_usage(
        &self,
        year: u32,
    ) -> GetBillingPremiumRequestUsageBuilder<'octo, '_> {
        GetBillingPremiumRequestUsageBuilder::new(self, year)
    }

    /// Access organization budgets management API.
    ///
    /// Note: Budgets are an organization-level feature.
    ///
    /// See: [Budgets](https://docs.github.com/en/rest/billing/budgets?apiVersion=2022-11-28)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let budgets = octocrab.billing().org("owner").budgets().list().send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn budgets(&self) -> OrgBudgetsHandler<'octo, '_> {
        OrgBudgetsHandler::new(self)
    }
}

/// Builder for getting billing usage reports.
#[derive(serde::Serialize)]
pub struct GetBillingUsageBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ScopedBillingHandler<'octo>,
    year: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<u8>,
}

impl<'octo, 'r> GetBillingUsageBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r ScopedBillingHandler<'octo>, year: u32) -> Self {
        Self {
            handler,
            year,
            month: None,
            day: None,
        }
    }

    /// Set the month for the report (1-12).
    pub fn month(mut self, month: impl Into<u8>) -> Self {
        self.month = Some(month.into());
        self
    }

    /// Set the day of the month for the report (1-31).
    pub fn day(mut self, day: impl Into<u8>) -> Self {
        self.day = Some(day.into());
        self
    }

    /// Send the request to retrieve the billing usage report.
    pub async fn send(self) -> Result<BillingUsageReport> {
        let route = format!("{}/usage", self.handler.owner.usage_prefix());
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for getting billing usage summary reports.
#[derive(serde::Serialize)]
pub struct GetBillingUsageSummaryBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ScopedBillingHandler<'octo>,
    year: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sku: Option<String>,
}

impl<'octo, 'r> GetBillingUsageSummaryBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r ScopedBillingHandler<'octo>, year: u32) -> Self {
        Self {
            handler,
            year,
            month: None,
            day: None,
            repository: None,
            product: None,
            sku: None,
        }
    }

    /// Set the month for the summary (1-12).
    pub fn month(mut self, month: impl Into<u8>) -> Self {
        self.month = Some(month.into());
        self
    }

    /// Set the day of the month for the summary (1-31).
    pub fn day(mut self, day: impl Into<u8>) -> Self {
        self.day = Some(day.into());
        self
    }

    /// Filter usage summary by repository name.
    pub fn repository(mut self, repository: impl Into<String>) -> Self {
        self.repository = Some(repository.into());
        self
    }

    /// Filter usage summary by product name.
    pub fn product(mut self, product: impl Into<String>) -> Self {
        self.product = Some(product.into());
        self
    }

    /// Filter usage summary by SKU name.
    pub fn sku(mut self, sku: impl Into<String>) -> Self {
        self.sku = Some(sku.into());
        self
    }

    /// Send the request to retrieve the billing usage summary.
    pub async fn send(self) -> Result<BillingUsageSummaryReport> {
        let route = format!("{}/usage/summary", self.handler.owner.usage_prefix());
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for getting billing AI credit usage reports.
#[derive(serde::Serialize)]
pub struct GetBillingAiCreditUsageBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ScopedBillingHandler<'octo>,
    year: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<String>,
}

impl<'octo, 'r> GetBillingAiCreditUsageBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r ScopedBillingHandler<'octo>, year: u32) -> Self {
        Self {
            handler,
            year,
            month: None,
            day: None,
            user: None,
            model: None,
            product: None,
        }
    }

    /// Set the month for the report (1-12).
    pub fn month(mut self, month: impl Into<u8>) -> Self {
        self.month = Some(month.into());
        self
    }

    /// Set the day of the month for the report (1-31).
    pub fn day(mut self, day: impl Into<u8>) -> Self {
        self.day = Some(day.into());
        self
    }

    /// Filter by user login.
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    /// Filter by model name.
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Filter by product name.
    pub fn product(mut self, product: impl Into<String>) -> Self {
        self.product = Some(product.into());
        self
    }

    /// Send the request to retrieve the billing AI credit usage report.
    pub async fn send(self) -> Result<BillingAiCreditUsageReport> {
        let route = format!("{}/ai_credit/usage", self.handler.owner.usage_prefix());
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for getting billing premium request usage reports.
#[derive(serde::Serialize)]
pub struct GetBillingPremiumRequestUsageBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ScopedBillingHandler<'octo>,
    year: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<String>,
}

impl<'octo, 'r> GetBillingPremiumRequestUsageBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r ScopedBillingHandler<'octo>, year: u32) -> Self {
        Self {
            handler,
            year,
            month: None,
            day: None,
            user: None,
            model: None,
            product: None,
        }
    }

    /// Set the month for the report (1-12).
    pub fn month(mut self, month: impl Into<u8>) -> Self {
        self.month = Some(month.into());
        self
    }

    /// Set the day of the month for the report (1-31).
    pub fn day(mut self, day: impl Into<u8>) -> Self {
        self.day = Some(day.into());
        self
    }

    /// Filter by user login.
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    /// Filter by model name.
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Filter by product name.
    pub fn product(mut self, product: impl Into<String>) -> Self {
        self.product = Some(product.into());
        self
    }

    /// Send the request to retrieve the billing premium request usage report.
    pub async fn send(self) -> Result<BillingPremiumRequestUsageReport> {
        let route = format!(
            "{}/premium_request/usage",
            self.handler.owner.usage_prefix()
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Handler for organization budget operations.
pub struct OrgBudgetsHandler<'octo, 'r> {
    handler: &'r ScopedBillingHandler<'octo>,
}

impl<'octo, 'r> OrgBudgetsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r ScopedBillingHandler<'octo>) -> Self {
        Self { handler }
    }

    /// List all budgets for an organization.
    ///
    /// See: [Get all budgets for an organization](https://docs.github.com/en/rest/billing/budgets?apiVersion=2022-11-28#get-all-budgets-for-an-organization)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let budgets = octocrab.billing().org("owner").budgets().list().send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListBudgetsBuilder<'octo, 'r, '_> {
        ListBudgetsBuilder::new(self)
    }

    /// Create a budget for an organization.
    ///
    /// See: [Create a budget for an organization](https://docs.github.com/en/rest/billing/budgets?apiVersion=2022-11-28#create-a-budget-for-an-organization)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::billing::CreateBudget;
    ///
    /// # let body: CreateBudget = todo!();
    /// let response = octocrab.billing().org("owner").budgets().create(&body).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, body: &CreateBudget) -> Result<CreateBudgetResponse> {
        let route = format!("{}/budgets", self.handler.owner.usage_prefix());
        self.handler.crab.post(route, Some(body)).await
    }

    /// Get a budget by ID for an organization.
    ///
    /// See: [Get a budget by ID for an organization](https://docs.github.com/en/rest/billing/budgets?apiVersion=2022-11-28#get-a-budget-by-id-for-an-organization)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let budget = octocrab.billing().org("owner").budgets().get("budget-id").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, budget_id: impl AsRef<str>) -> Result<Budget> {
        let route = format!(
            "{}/budgets/{}",
            self.handler.owner.usage_prefix(),
            budget_id.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Update a budget for an organization.
    ///
    /// See: [Update a budget for an organization](https://docs.github.com/en/rest/billing/budgets?apiVersion=2022-11-28#update-a-budget-for-an-organization)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::billing::UpdateBudget;
    ///
    /// # let body: UpdateBudget = todo!();
    /// let response = octocrab.billing().org("owner").budgets().update("budget-id", &body).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(
        &self,
        budget_id: impl AsRef<str>,
        body: &UpdateBudget,
    ) -> Result<UpdateBudgetResponse> {
        let route = format!(
            "{}/budgets/{}",
            self.handler.owner.usage_prefix(),
            budget_id.as_ref()
        );
        self.handler.crab.patch(route, Some(body)).await
    }

    /// Delete a budget for an organization.
    ///
    /// See: [Delete a budget for an organization](https://docs.github.com/en/rest/billing/budgets?apiVersion=2022-11-28#delete-a-budget-for-an-organization)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let response = octocrab.billing().org("owner").budgets().delete("budget-id").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, budget_id: impl AsRef<str>) -> Result<DeleteBudgetResponse> {
        let route = format!(
            "{}/budgets/{}",
            self.handler.owner.usage_prefix(),
            budget_id.as_ref()
        );
        self.handler.crab.delete(route, None::<&()>).await
    }
}

/// Builder for listing budgets for an organization.
#[derive(serde::Serialize)]
pub struct ListBudgetsBuilder<'octo, 'r, 'b> {
    #[serde(skip)]
    budgets_handler: &'b OrgBudgetsHandler<'octo, 'r>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

impl<'octo, 'r, 'b> ListBudgetsBuilder<'octo, 'r, 'b> {
    pub(crate) fn new(budgets_handler: &'b OrgBudgetsHandler<'octo, 'r>) -> Self {
        Self {
            budgets_handler,
            page: None,
            per_page: None,
            scope: None,
            user: None,
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

    /// Filter budgets by scope.
    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }

    /// Filter budgets by user.
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    /// Send the request to list all budgets.
    pub async fn send(self) -> Result<GetAllBudgets> {
        let route = format!(
            "{}/budgets",
            self.budgets_handler.handler.owner.usage_prefix()
        );
        self.budgets_handler
            .handler
            .crab
            .get(route, Some(&self))
            .await
    }
}
