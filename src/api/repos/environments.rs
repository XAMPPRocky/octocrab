use super::RepoHandler;
use crate::models::{
    repos::{
        BranchPolicyType, CustomDeploymentProtectionRule, CustomDeploymentProtectionRules,
        CustomDeploymentRuleApps, DeploymentBranchPolicies, DeploymentBranchPolicy,
        DeploymentBranchPolicySettings, Environment, EnvironmentReviewer, Environments,
    },
    AppId, BranchPolicyId, ProtectionRuleId,
};
use crate::Result;

/// A client to GitHub's repository environments API.
///
/// Created with [`RepoHandler::environments`].
pub struct RepoEnvironmentsHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoEnvironmentsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Creates a [`ListEnvironmentsBuilder`] to list environments for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/environments?apiVersion=2022-11-28#list-environments)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let environments = octocrab
    ///     .repos("owner", "repo")
    ///     .environments()
    ///     .list()
    ///     .per_page(10)
    ///     .page(1u32)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListEnvironmentsBuilder<'octo, 'r> {
        ListEnvironmentsBuilder::new(self.handler)
    }

    /// Gets an environment in a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/environments?apiVersion=2022-11-28#get-an-environment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let env = octocrab
    ///     .repos("owner", "repo")
    ///     .environments()
    ///     .get("staging")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, environment_name: impl AsRef<str>) -> Result<Environment> {
        let route = format!(
            "/{}/environments/{}",
            self.handler.repo,
            environment_name.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates or updates an environment in a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/environments?apiVersion=2022-11-28#create-or-update-an-environment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let env = octocrab
    ///     .repos("owner", "repo")
    ///     .environments()
    ///     .create_or_update("staging")
    ///     .wait_timer(30u32)
    ///     .prevent_self_review(true)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_or_update(
        &self,
        environment_name: impl Into<String>,
    ) -> CreateOrUpdateEnvironmentBuilder<'octo, 'r> {
        CreateOrUpdateEnvironmentBuilder::new(self.handler, environment_name.into())
    }

    /// Deletes an environment in a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/environments?apiVersion=2022-11-28#delete-an-environment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab
    ///     .repos("owner", "repo")
    ///     .environments()
    ///     .delete("staging")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, environment_name: impl AsRef<str>) -> Result<()> {
        let route = format!(
            "/{}/environments/{}",
            self.handler.repo,
            environment_name.as_ref()
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Access the deployment branch policies sub-API for an environment.
    pub fn branch_policies(
        &self,
        environment_name: impl Into<String>,
    ) -> RepoEnvironmentBranchPoliciesHandler<'octo, 'r> {
        RepoEnvironmentBranchPoliciesHandler::new(self.handler, environment_name.into())
    }

    /// Access the deployment protection rules sub-API for an environment.
    pub fn protection_rules(
        &self,
        environment_name: impl Into<String>,
    ) -> RepoEnvironmentProtectionRulesHandler<'octo, 'r> {
        RepoEnvironmentProtectionRulesHandler::new(self.handler, environment_name.into())
    }
}

/// A builder pattern struct for listing environments for a repository.
///
/// Created by [`RepoEnvironmentsHandler::list`].
#[derive(serde::Serialize)]
pub struct ListEnvironmentsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListEnvironmentsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100). Default: 30.
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
    pub async fn send(self) -> Result<Environments> {
        let route = format!("/{}/environments", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A builder pattern struct for creating or updating an environment.
///
/// Created by [`RepoEnvironmentsHandler::create_or_update`].
#[derive(serde::Serialize)]
pub struct CreateOrUpdateEnvironmentBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    environment_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_timer: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prevent_self_review: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reviewers: Option<Vec<EnvironmentReviewer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_branch_policy: Option<Option<DeploymentBranchPolicySettings>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_admins_bypass: Option<bool>,
}

impl<'octo, 'r> CreateOrUpdateEnvironmentBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, environment_name: String) -> Self {
        Self {
            handler,
            environment_name,
            wait_timer: None,
            prevent_self_review: None,
            reviewers: None,
            deployment_branch_policy: None,
            can_admins_bypass: None,
        }
    }

    /// The amount of time to delay a job after the job is initially triggered. The time (in minutes) must be an integer between 0 and 43,200 (30 days).
    pub fn wait_timer(mut self, wait_timer: impl Into<u32>) -> Self {
        self.wait_timer = Some(wait_timer.into());
        self
    }

    /// Whether or not a user who created the job is prevented from approving their own job.
    pub fn prevent_self_review(mut self, prevent_self_review: bool) -> Self {
        self.prevent_self_review = Some(prevent_self_review);
        self
    }

    /// The people or teams that may review jobs that reference the environment.
    pub fn reviewers(mut self, reviewers: impl Into<Vec<EnvironmentReviewer>>) -> Self {
        self.reviewers = Some(reviewers.into());
        self
    }

    /// Adds a single reviewer to the list of reviewers.
    pub fn reviewer(mut self, reviewer: EnvironmentReviewer) -> Self {
        self.reviewers.get_or_insert_with(Vec::new).push(reviewer);
        self
    }

    /// The type of deployment branch policy for this environment. To allow all branches to deploy, pass `None`.
    pub fn deployment_branch_policy(
        mut self,
        policy: Option<DeploymentBranchPolicySettings>,
    ) -> Self {
        self.deployment_branch_policy = Some(policy);
        self
    }

    /// Whether or not administrators can bypass the environment protections.
    pub fn can_admins_bypass(mut self, can_admins_bypass: bool) -> Self {
        self.can_admins_bypass = Some(can_admins_bypass);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Environment> {
        let route = format!(
            "/{}/environments/{}",
            self.handler.repo, self.environment_name
        );
        self.handler.crab.put(route, Some(&self)).await
    }
}

/// A client to GitHub's repository environment deployment branch policies API.
///
/// Created with [`RepoEnvironmentsHandler::branch_policies`].
pub struct RepoEnvironmentBranchPoliciesHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
    environment_name: String,
}

impl<'octo, 'r> RepoEnvironmentBranchPoliciesHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, environment_name: String) -> Self {
        Self {
            handler,
            environment_name,
        }
    }

    /// Creates a [`ListDeploymentBranchPoliciesBuilder`] to list deployment branch policies for an environment.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/branch-policies?apiVersion=2022-11-28#list-deployment-branch-policies)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let policies = octocrab
    ///     .repos("owner", "repo")
    ///     .environments()
    ///     .branch_policies("staging")
    ///     .list()
    ///     .per_page(10)
    ///     .page(1u32)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListDeploymentBranchPoliciesBuilder<'octo, 'r> {
        ListDeploymentBranchPoliciesBuilder::new(self.handler, self.environment_name.clone())
    }

    /// Gets a deployment branch policy by its ID.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/branch-policies?apiVersion=2022-11-28#get-a-deployment-branch-policy)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let policy = octocrab
    ///     .repos("owner", "repo")
    ///     .environments()
    ///     .branch_policies("staging")
    ///     .get(1u64)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(
        &self,
        branch_policy_id: impl Into<BranchPolicyId>,
    ) -> Result<DeploymentBranchPolicy> {
        let branch_policy_id = branch_policy_id.into();
        let route = format!(
            "/{}/environments/{}/deployment-branch-policies/{branch_policy_id}",
            self.handler.repo, self.environment_name
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates a [`CreateDeploymentBranchPolicyBuilder`] to create a deployment branch or tag policy.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/branch-policies?apiVersion=2022-11-28#create-a-deployment-branch-policy)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use octocrab::models::repos::BranchPolicyType;
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let policy = octocrab
    ///     .repos("owner", "repo")
    ///     .environments()
    ///     .branch_policies("staging")
    ///     .create("main")
    ///     .policy_type(BranchPolicyType::Branch)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(
        &self,
        name: impl Into<String>,
    ) -> CreateDeploymentBranchPolicyBuilder<'octo, 'r> {
        CreateDeploymentBranchPolicyBuilder::new(
            self.handler,
            self.environment_name.clone(),
            name.into(),
        )
    }

    /// Creates an [`UpdateDeploymentBranchPolicyBuilder`] to update a deployment branch policy.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/branch-policies?apiVersion=2022-11-28#update-a-deployment-branch-policy)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let policy = octocrab
    ///     .repos("owner", "repo")
    ///     .environments()
    ///     .branch_policies("staging")
    ///     .update(1u64, "release/*")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(
        &self,
        branch_policy_id: impl Into<BranchPolicyId>,
        name: impl Into<String>,
    ) -> UpdateDeploymentBranchPolicyBuilder<'octo, 'r> {
        UpdateDeploymentBranchPolicyBuilder::new(
            self.handler,
            self.environment_name.clone(),
            branch_policy_id.into(),
            name.into(),
        )
    }

    /// Deletes a deployment branch policy.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/branch-policies?apiVersion=2022-11-28#delete-a-deployment-branch-policy)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab
    ///     .repos("owner", "repo")
    ///     .environments()
    ///     .branch_policies("staging")
    ///     .delete(1u64)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, branch_policy_id: impl Into<BranchPolicyId>) -> Result<()> {
        let branch_policy_id = branch_policy_id.into();
        let route = format!(
            "/{}/environments/{}/deployment-branch-policies/{branch_policy_id}",
            self.handler.repo, self.environment_name
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}

/// A builder pattern struct for listing deployment branch policies for an environment.
///
/// Created by [`RepoEnvironmentBranchPoliciesHandler::list`].
#[derive(serde::Serialize)]
pub struct ListDeploymentBranchPoliciesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    environment_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListDeploymentBranchPoliciesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, environment_name: String) -> Self {
        Self {
            handler,
            environment_name,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100). Default: 30.
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
    pub async fn send(self) -> Result<DeploymentBranchPolicies> {
        let route = format!(
            "/{}/environments/{}/deployment-branch-policies",
            self.handler.repo, self.environment_name
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A builder pattern struct for creating a deployment branch policy.
///
/// Created by [`RepoEnvironmentBranchPoliciesHandler::create`].
#[derive(serde::Serialize)]
pub struct CreateDeploymentBranchPolicyBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    environment_name: String,
    name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    policy_type: Option<BranchPolicyType>,
}

impl<'octo, 'r> CreateDeploymentBranchPolicyBuilder<'octo, 'r> {
    pub(crate) fn new(
        handler: &'r RepoHandler<'octo>,
        environment_name: String,
        name: String,
    ) -> Self {
        Self {
            handler,
            environment_name,
            name,
            policy_type: None,
        }
    }

    /// The type of deployment branch policy to create (branch or tag).
    pub fn policy_type(mut self, policy_type: impl Into<BranchPolicyType>) -> Self {
        self.policy_type = Some(policy_type.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<DeploymentBranchPolicy> {
        let route = format!(
            "/{}/environments/{}/deployment-branch-policies",
            self.handler.repo, self.environment_name
        );
        self.handler.crab.post(route, Some(&self)).await
    }
}

/// A builder pattern struct for updating a deployment branch policy.
///
/// Created by [`RepoEnvironmentBranchPoliciesHandler::update`].
#[derive(serde::Serialize)]
pub struct UpdateDeploymentBranchPolicyBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    environment_name: String,
    #[serde(skip)]
    branch_policy_id: BranchPolicyId,
    name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    policy_type: Option<BranchPolicyType>,
}

impl<'octo, 'r> UpdateDeploymentBranchPolicyBuilder<'octo, 'r> {
    pub(crate) fn new(
        handler: &'r RepoHandler<'octo>,
        environment_name: String,
        branch_policy_id: BranchPolicyId,
        name: String,
    ) -> Self {
        Self {
            handler,
            environment_name,
            branch_policy_id,
            name,
            policy_type: None,
        }
    }

    /// The type of deployment branch policy to update (branch or tag).
    pub fn policy_type(mut self, policy_type: impl Into<BranchPolicyType>) -> Self {
        self.policy_type = Some(policy_type.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<DeploymentBranchPolicy> {
        let route = format!(
            "/{}/environments/{}/deployment-branch-policies/{}",
            self.handler.repo, self.environment_name, self.branch_policy_id
        );
        self.handler.crab.put(route, Some(&self)).await
    }
}

/// A client to GitHub's repository environment deployment protection rules API.
///
/// Created with [`RepoEnvironmentsHandler::protection_rules`].
pub struct RepoEnvironmentProtectionRulesHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
    environment_name: String,
}

#[derive(serde::Serialize)]
struct CreateCustomDeploymentProtectionRuleRequest {
    integration_id: u64,
}

impl<'octo, 'r> RepoEnvironmentProtectionRulesHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, environment_name: String) -> Self {
        Self {
            handler,
            environment_name,
        }
    }

    /// Gets all custom deployment protection rules for an environment.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/protection-rules?apiVersion=2022-11-28#get-all-deployment-protection-rules-for-an-environment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let rules = octocrab
    ///     .repos("owner", "repo")
    ///     .environments()
    ///     .protection_rules("staging")
    ///     .list()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<CustomDeploymentProtectionRules> {
        let route = format!(
            "/{}/environments/{}/deployment_protection_rules",
            self.handler.repo, self.environment_name
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates a custom deployment protection rule on an environment.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/protection-rules?apiVersion=2022-11-28#create-a-custom-deployment-protection-rule-on-an-environment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use octocrab::models::AppId;
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let rule = octocrab
    ///     .repos("owner", "repo")
    ///     .environments()
    ///     .protection_rules("staging")
    ///     .create(AppId(1515))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        integration_id: impl Into<AppId>,
    ) -> Result<CustomDeploymentProtectionRule> {
        let route = format!(
            "/{}/environments/{}/deployment_protection_rules",
            self.handler.repo, self.environment_name
        );
        let body = CreateCustomDeploymentProtectionRuleRequest {
            integration_id: integration_id.into().0,
        };
        self.handler.crab.post(route, Some(&body)).await
    }

    /// Creates a [`ListCustomDeploymentRuleAppsBuilder`] to list apps that may implement a custom deployment protection rule for an environment.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/protection-rules?apiVersion=2022-11-28#get-apps-that-may-implement-a-custom-deployment-protection-rule-for-an-environment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let apps = octocrab
    ///     .repos("owner", "repo")
    ///     .environments()
    ///     .protection_rules("staging")
    ///     .list_apps()
    ///     .per_page(10)
    ///     .page(1u32)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_apps(&self) -> ListCustomDeploymentRuleAppsBuilder<'octo, 'r> {
        ListCustomDeploymentRuleAppsBuilder::new(self.handler, self.environment_name.clone())
    }

    /// Gets a custom deployment protection rule for an environment.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/protection-rules?apiVersion=2022-11-28#get-a-custom-deployment-protection-rule)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let rule = octocrab
    ///     .repos("owner", "repo")
    ///     .environments()
    ///     .protection_rules("staging")
    ///     .get(3515u64)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(
        &self,
        protection_rule_id: impl Into<ProtectionRuleId>,
    ) -> Result<CustomDeploymentProtectionRule> {
        let protection_rule_id = protection_rule_id.into();
        let route = format!(
            "/{}/environments/{}/deployment_protection_rules/{protection_rule_id}",
            self.handler.repo, self.environment_name
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Disables a custom deployment protection rule for an environment.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/deployments/protection-rules?apiVersion=2022-11-28#disable-a-custom-deployment-protection-rule-on-an-environment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab
    ///     .repos("owner", "repo")
    ///     .environments()
    ///     .protection_rules("staging")
    ///     .disable(3515u64)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn disable(&self, protection_rule_id: impl Into<ProtectionRuleId>) -> Result<()> {
        let protection_rule_id = protection_rule_id.into();
        let route = format!(
            "/{}/environments/{}/deployment_protection_rules/{protection_rule_id}",
            self.handler.repo, self.environment_name
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}

/// A builder pattern struct for listing available custom deployment protection rule apps.
///
/// Created by [`RepoEnvironmentProtectionRulesHandler::list_apps`].
#[derive(serde::Serialize)]
pub struct ListCustomDeploymentRuleAppsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    environment_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListCustomDeploymentRuleAppsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, environment_name: String) -> Self {
        Self {
            handler,
            environment_name,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100). Default: 30.
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
    pub async fn send(self) -> Result<CustomDeploymentRuleApps> {
        let route = format!(
            "/{}/environments/{}/deployment_protection_rules/apps",
            self.handler.repo, self.environment_name
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}
