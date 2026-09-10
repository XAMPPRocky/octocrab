use super::*;
use crate::models::rulesets::{
    RepoRule, RuleSuite, RuleSuiteId, RuleSuiteSummary, Ruleset, RulesetId,
};

/// A client to GitHub's repository rulesets API.
///
/// Created with [`RepoHandler::rulesets`].
pub struct RepoRulesetsHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoRulesetsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// List all rulesets for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/rules?apiVersion=2022-11-28#get-all-repository-rulesets)
    pub fn list(&self) -> ListRepoRulesetsBuilder<'octo, 'r> {
        ListRepoRulesetsBuilder::new(self.handler)
    }

    /// Get a repository ruleset.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/rules?apiVersion=2022-11-28#get-a-repository-ruleset)
    pub fn get(&self, ruleset_id: impl Into<RulesetId>) -> GetRepoRulesetBuilder<'octo, 'r> {
        GetRepoRulesetBuilder::new(self.handler, ruleset_id.into())
    }

    /// Create a repository ruleset.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/rules?apiVersion=2022-11-28#create-a-repository-ruleset)
    pub async fn create(&self, ruleset: &impl serde::Serialize) -> crate::Result<Ruleset> {
        let route = format!("/{}/rulesets", self.handler.repo);
        self.handler.crab.post(route, Some(ruleset)).await
    }

    /// Update a repository ruleset.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/rules?apiVersion=2022-11-28#update-a-repository-ruleset)
    pub async fn update(
        &self,
        ruleset_id: impl Into<RulesetId>,
        ruleset: &impl serde::Serialize,
    ) -> crate::Result<Ruleset> {
        let route = format!(
            "/{repo}/rulesets/{id}",
            repo = self.handler.repo,
            id = ruleset_id.into(),
        );
        self.handler.crab.put(route, Some(ruleset)).await
    }

    /// Delete a repository ruleset.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/rules?apiVersion=2022-11-28#delete-a-repository-ruleset)
    pub async fn delete(&self, ruleset_id: impl Into<RulesetId>) -> crate::Result<()> {
        let route = format!(
            "/{repo}/rulesets/{id}",
            repo = self.handler.repo,
            id = ruleset_id.into(),
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Access rule suites for this repository.
    pub fn rule_suites(&self) -> RepoRuleSuitesHandler<'octo, 'r> {
        RepoRuleSuitesHandler::new(self.handler)
    }

    /// Get all active rules that apply to the specified branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/rules?apiVersion=2022-11-28#get-rules-for-a-branch)
    pub fn rules_for_branch(
        &self,
        branch: impl Into<String>,
    ) -> ListRulesForBranchBuilder<'octo, 'r> {
        ListRulesForBranchBuilder::new(self.handler, branch.into())
    }
}

/// Builder for listing repository rulesets.
#[derive(serde::Serialize)]
pub struct ListRepoRulesetsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    includes_parents: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    targets: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListRepoRulesetsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            includes_parents: None,
            targets: None,
            per_page: None,
            page: None,
        }
    }

    /// Include rulesets configured at higher levels that apply to this repository.
    pub fn includes_parents(mut self, includes_parents: bool) -> Self {
        self.includes_parents = Some(includes_parents);
        self
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
        let route = format!("/{}/rulesets", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for getting a repository ruleset.
#[derive(serde::Serialize)]
pub struct GetRepoRulesetBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    ruleset_id: RulesetId,
    #[serde(skip_serializing_if = "Option::is_none")]
    includes_parents: Option<bool>,
}

impl<'octo, 'r> GetRepoRulesetBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, ruleset_id: RulesetId) -> Self {
        Self {
            handler,
            ruleset_id,
            includes_parents: None,
        }
    }

    /// Include rulesets configured at higher levels that apply to this repository.
    pub fn includes_parents(mut self, includes_parents: bool) -> Self {
        self.includes_parents = Some(includes_parents);
        self
    }

    /// Sends the request and returns the ruleset.
    pub async fn send(self) -> crate::Result<Ruleset> {
        let route = format!(
            "/{repo}/rulesets/{id}",
            repo = self.handler.repo,
            id = self.ruleset_id,
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A client to GitHub's repository rule suites API.
pub struct RepoRuleSuitesHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoRuleSuitesHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Lists suites of rule evaluations at the repository level.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/rule-suites?apiVersion=2022-11-28#list-repository-rule-suites)
    pub fn list(&self) -> ListRepoRuleSuitesBuilder<'octo, 'r> {
        ListRepoRuleSuitesBuilder::new(self.handler)
    }

    /// Gets information about a suite of rule evaluations from within a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/rule-suites?apiVersion=2022-11-28#get-a-repository-rule-suite)
    pub async fn get(&self, rule_suite_id: impl Into<RuleSuiteId>) -> crate::Result<RuleSuite> {
        let route = format!(
            "/{repo}/rulesets/rule-suites/{id}",
            repo = self.handler.repo,
            id = rule_suite_id.into(),
        );
        self.handler.crab.get(route, None::<&()>).await
    }
}

/// Builder for listing repository rule suites.
#[derive(serde::Serialize)]
pub struct ListRepoRuleSuitesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    ref_name: Option<String>,
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

impl<'octo, 'r> ListRepoRuleSuitesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            ref_name: None,
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
        let route = format!("/{}/rulesets/rule-suites", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for listing active rules that apply to a branch.
#[derive(serde::Serialize)]
pub struct ListRulesForBranchBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    branch: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListRulesForBranchBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, branch: String) -> Self {
        Self {
            handler,
            branch,
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

    /// Sends the request and returns the rules for the branch.
    pub async fn send(self) -> crate::Result<Vec<RepoRule>> {
        let route = format!(
            "/{repo}/rules/branches/{branch}",
            repo = self.handler.repo,
            branch = self.branch,
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}
