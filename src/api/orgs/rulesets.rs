use super::OrgHandler;
use crate::models::rulesets::{RuleSuite, RuleSuiteId, RuleSuiteSummary, Ruleset, RulesetId};

/// A client to GitHub's organization rulesets API.
///
/// Created with [`OrgHandler::rulesets`].
pub struct OrgRulesetsHandler<'octo, 'r> {
    handler: &'r OrgHandler<'octo>,
}

impl<'octo, 'r> OrgRulesetsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// List all repository rulesets for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/rules?apiVersion=2022-11-28#get-all-organization-repository-rulesets)
    pub fn list(&self) -> ListOrgRulesetsBuilder<'octo, 'r> {
        ListOrgRulesetsBuilder::new(self.handler)
    }

    /// Get an organization repository ruleset.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/rules?apiVersion=2022-11-28#get-an-organization-repository-ruleset)
    pub async fn get(&self, ruleset_id: impl Into<RulesetId>) -> crate::Result<Ruleset> {
        let route = format!(
            "/orgs/{org}/rulesets/{id}",
            org = self.handler.owner,
            id = ruleset_id.into(),
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Create a repository ruleset for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/rules?apiVersion=2022-11-28#create-an-organization-repository-ruleset)
    pub async fn create(&self, ruleset: &impl serde::Serialize) -> crate::Result<Ruleset> {
        let route = format!("/orgs/{org}/rulesets", org = self.handler.owner);
        self.handler.crab.post(route, Some(ruleset)).await
    }

    /// Update an organization repository ruleset.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/rules?apiVersion=2022-11-28#update-an-organization-repository-ruleset)
    pub async fn update(
        &self,
        ruleset_id: impl Into<RulesetId>,
        ruleset: &impl serde::Serialize,
    ) -> crate::Result<Ruleset> {
        let route = format!(
            "/orgs/{org}/rulesets/{id}",
            org = self.handler.owner,
            id = ruleset_id.into(),
        );
        self.handler.crab.put(route, Some(ruleset)).await
    }

    /// Delete an organization repository ruleset.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/rules?apiVersion=2022-11-28#delete-an-organization-repository-ruleset)
    pub async fn delete(&self, ruleset_id: impl Into<RulesetId>) -> crate::Result<()> {
        let route = format!(
            "/orgs/{org}/rulesets/{id}",
            org = self.handler.owner,
            id = ruleset_id.into(),
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Access rule suites for this organization.
    pub fn rule_suites(&self) -> OrgRuleSuitesHandler<'octo, 'r> {
        OrgRuleSuitesHandler::new(self.handler)
    }
}

/// Builder for listing organization repository rulesets.
#[derive(serde::Serialize)]
pub struct ListOrgRulesetsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    targets: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListOrgRulesetsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self {
            handler,
            targets: None,
            per_page: None,
            page: None,
        }
    }

    /// A comma-separated list of rule targets to filter by (e.g. `branch,tag,push`).
    pub fn targets(mut self, targets: impl Into<String>) -> Self {
        self.targets = Some(targets.into());
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

    /// Sends the request and returns a vector of rulesets.
    pub async fn send(self) -> crate::Result<Vec<Ruleset>> {
        let route = format!("/orgs/{org}/rulesets", org = self.handler.owner);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A client to GitHub's organization rule suites API.
pub struct OrgRuleSuitesHandler<'octo, 'r> {
    handler: &'r OrgHandler<'octo>,
}

impl<'octo, 'r> OrgRuleSuitesHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Lists suites of rule evaluations at the organization level.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/rule-suites?apiVersion=2022-11-28#list-organization-rule-suites)
    pub fn list(&self) -> ListOrgRuleSuitesBuilder<'octo, 'r> {
        ListOrgRuleSuitesBuilder::new(self.handler)
    }

    /// Gets information about a suite of rule evaluations from within an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/rule-suites?apiVersion=2022-11-28#get-an-organization-rule-suite)
    pub async fn get(&self, rule_suite_id: impl Into<RuleSuiteId>) -> crate::Result<RuleSuite> {
        let route = format!(
            "/orgs/{org}/rulesets/rule-suites/{id}",
            org = self.handler.owner,
            id = rule_suite_id.into(),
        );
        self.handler.crab.get(route, None::<&()>).await
    }
}

/// Builder for listing organization rule suites.
#[derive(serde::Serialize)]
pub struct ListOrgRuleSuitesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgHandler<'octo>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    ref_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actor_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_suite_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluate_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListOrgRuleSuitesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self {
            handler,
            ref_name: None,
            repository_name: None,
            time_period: None,
            actor_name: None,
            rule_suite_result: None,
            evaluate_status: None,
            per_page: None,
            page: None,
        }
    }

    /// The name of the ref (branch or tag).
    pub fn ref_name(mut self, ref_name: impl Into<String>) -> Self {
        self.ref_name = Some(ref_name.into());
        self
    }

    /// The name of the repository to filter on.
    pub fn repository_name(mut self, repository_name: impl Into<String>) -> Self {
        self.repository_name = Some(repository_name.into());
        self
    }

    /// The time period to filter by (`hour`, `day`, `week`, `month`).
    pub fn time_period(mut self, time_period: impl Into<String>) -> Self {
        self.time_period = Some(time_period.into());
        self
    }

    /// The handle for the GitHub user account to filter on.
    pub fn actor_name(mut self, actor_name: impl Into<String>) -> Self {
        self.actor_name = Some(actor_name.into());
        self
    }

    /// The rule suite results to filter on (`pass`, `fail`, `bypass`, `all`).
    pub fn rule_suite_result(mut self, rule_suite_result: impl Into<String>) -> Self {
        self.rule_suite_result = Some(rule_suite_result.into());
        self
    }

    /// The evaluate status to filter on (`all`, `active`, `evaluate`).
    pub fn evaluate_status(mut self, evaluate_status: impl Into<String>) -> Self {
        self.evaluate_status = Some(evaluate_status.into());
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

    /// Sends the request and returns a vector of rule suite summaries.
    pub async fn send(self) -> crate::Result<Vec<RuleSuiteSummary>> {
        let route = format!("/orgs/{org}/rulesets/rule-suites", org = self.handler.owner);
        self.handler.crab.get(route, Some(&self)).await
    }
}
