use super::*;
use crate::models::apps::App;
use crate::models::repos::branches::{
    AdminEnforcement, AppsRestrictionsRequest, BranchProtection, BranchProtectionRestrictions,
    ContextsRequest, DetailedBranch, DismissalRestrictionsRequest, ProtectionFlag,
    RequiredPullRequestReviews, RequiredStatusChecks, StatusCheck, TeamsRestrictionsRequest,
    UpdatePullRequestReviews, UpdateRestrictions, UpdateStatusChecks, UsersRestrictionsRequest,
};
use crate::models::teams::Team;
use crate::models::Author;

/// A client to GitHub's repository branches API.
///
/// Created with [`RepoHandler::branches`].
pub struct RepoBranchesHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

#[derive(serde::Serialize)]
struct RenameBranchRequest<'a> {
    new_name: &'a str,
}

impl<'octo, 'r> RepoBranchesHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Creates a [`ListBranchesBuilder`] to list branches from a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branches?apiVersion=2022-11-28#list-branches)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let branches = octocrab
    ///     .repos("owner", "repo")
    ///     .branches()
    ///     .list()
    ///     .protected(true)
    ///     .per_page(50)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListBranchesBuilder<'octo, 'r> {
        ListBranchesBuilder::new(self.handler)
    }

    /// Gets detailed information about a single branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branches?apiVersion=2022-11-28#get-a-branch)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let branch = octocrab
    ///     .repos("owner", "repo")
    ///     .branches()
    ///     .get("main")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, branch: impl AsRef<str>) -> Result<DetailedBranch> {
        let route = format!("/{}/branches/{}", self.handler.repo, branch.as_ref());
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Renames a branch in a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branches?apiVersion=2022-11-28#rename-a-branch)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let branch = octocrab
    ///     .repos("owner", "repo")
    ///     .branches()
    ///     .rename("master", "main")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn rename(
        &self,
        branch: impl AsRef<str>,
        new_name: impl AsRef<str>,
    ) -> Result<DetailedBranch> {
        let route = format!("/{}/branches/{}/rename", self.handler.repo, branch.as_ref());
        let body = RenameBranchRequest {
            new_name: new_name.as_ref(),
        };
        self.handler.crab.post(route, Some(&body)).await
    }

    /// Access the branch protection sub-API for a specific branch.
    pub fn protection(&self, branch: impl Into<String>) -> RepoBranchProtectionHandler<'octo, 'r> {
        RepoBranchProtectionHandler::new(self.handler, branch.into())
    }

    /// Sync a fork branch with the upstream repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branches?apiVersion=2022-11-28#sync-a-fork-branch-with-the-upstream-repository)
    pub async fn merge_upstream(
        &self,
        branch: impl Into<String>,
    ) -> Result<models::repos::MergedUpstream> {
        self.handler.merge_upstream(branch).await
    }

    /// List branches where the given commit is the HEAD commit.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/commits/commits?apiVersion=2022-11-28#list-branches-for-head-commit)
    pub async fn where_head(
        &self,
        commit_sha: impl AsRef<str>,
    ) -> Result<Vec<models::repos::Branch>> {
        self.handler.branches_where_head(commit_sha).await
    }
}

/// A builder pattern struct for listing branches in a repository.
///
/// Created by [`RepoBranchesHandler::list`] or [`RepoHandler::list_branches`].
#[derive(serde::Serialize)]
pub struct ListBranchesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListBranchesBuilder<'octo, 'r> {
    pub fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            protected: None,
            per_page: None,
            page: None,
        }
    }

    /// Setting to true returns only protected branches. When set to false, only
    /// unprotected branches are returned. Omitting this parameter returns all
    /// branches.
    pub fn protected(mut self, protected: impl Into<bool>) -> Self {
        self.protected = Some(protected.into());
        self
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
    pub async fn send(self) -> Result<crate::Page<models::repos::Branch>> {
        let route = format!("/{}/branches", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A client to GitHub's branch protection API for a specific branch.
///
/// Created with [`RepoBranchesHandler::protection`].
pub struct RepoBranchProtectionHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
    branch: String,
}

impl<'octo, 'r> RepoBranchProtectionHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, branch: String) -> Self {
        Self { handler, branch }
    }

    /// Gets protection configuration for the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#get-branch-protection)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let protection = octocrab
    ///     .repos("owner", "repo")
    ///     .branches()
    ///     .protection("main")
    ///     .get()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self) -> Result<BranchProtection> {
        let route = format!("/{}/branches/{}/protection", self.handler.repo, self.branch);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates an [`UpdateBranchProtectionBuilder`] to update protection settings for the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#update-branch-protection)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let protection = octocrab
    ///     .repos("owner", "repo")
    ///     .branches()
    ///     .protection("main")
    ///     .update()
    ///     .enforce_admins(Some(true))
    ///     .required_linear_history(true)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(&self) -> UpdateBranchProtectionBuilder<'octo, 'r> {
        UpdateBranchProtectionBuilder::new(self.handler, self.branch.clone())
    }

    /// Deletes branch protection for the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#delete-branch-protection)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// octocrab
    ///     .repos("owner", "repo")
    ///     .branches()
    ///     .protection("main")
    ///     .delete()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self) -> Result<()> {
        let route = format!("/{}/branches/{}/protection", self.handler.repo, self.branch);
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Access the admin enforcement sub-API for the branch.
    pub fn enforce_admins(&self) -> RepoBranchAdminEnforcementHandler<'octo, 'r> {
        RepoBranchAdminEnforcementHandler::new(self.handler, self.branch.clone())
    }

    /// Access the pull request reviews protection sub-API for the branch.
    pub fn pull_request_reviews(&self) -> RepoBranchPullRequestReviewsHandler<'octo, 'r> {
        RepoBranchPullRequestReviewsHandler::new(self.handler, self.branch.clone())
    }

    /// Access the commit signatures protection sub-API for the branch.
    pub fn signatures(&self) -> RepoBranchSignaturesHandler<'octo, 'r> {
        RepoBranchSignaturesHandler::new(self.handler, self.branch.clone())
    }

    /// Access the required status checks sub-API for the branch.
    pub fn status_checks(&self) -> RepoBranchStatusChecksHandler<'octo, 'r> {
        RepoBranchStatusChecksHandler::new(self.handler, self.branch.clone())
    }

    /// Access the access restrictions sub-API for the branch.
    pub fn restrictions(&self) -> RepoBranchRestrictionsHandler<'octo, 'r> {
        RepoBranchRestrictionsHandler::new(self.handler, self.branch.clone())
    }
}

/// A builder pattern struct for updating branch protection settings.
///
/// Created by [`RepoBranchProtectionHandler::update`].
#[derive(serde::Serialize)]
pub struct UpdateBranchProtectionBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    branch: String,
    pub required_status_checks: Option<UpdateStatusChecks>,
    pub enforce_admins: Option<bool>,
    pub required_pull_request_reviews: Option<UpdatePullRequestReviews>,
    pub restrictions: Option<UpdateRestrictions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_linear_history: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_force_pushes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_deletions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_creations: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_conversation_resolution: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_branch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_fork_syncing: Option<bool>,
}

impl<'octo, 'r> UpdateBranchProtectionBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, branch: String) -> Self {
        Self {
            handler,
            branch,
            required_status_checks: None,
            enforce_admins: None,
            required_pull_request_reviews: None,
            restrictions: None,
            required_linear_history: None,
            allow_force_pushes: None,
            allow_deletions: None,
            block_creations: None,
            required_conversation_resolution: None,
            lock_branch: None,
            allow_fork_syncing: None,
        }
    }

    /// Require status checks to pass before merging. Set to `None` to disable.
    pub fn required_status_checks(mut self, checks: Option<UpdateStatusChecks>) -> Self {
        self.required_status_checks = checks;
        self
    }

    /// Enforce all configured restrictions for administrators. Set to `None` or `Some(false)` to disable.
    pub fn enforce_admins(mut self, enforce_admins: Option<bool>) -> Self {
        self.enforce_admins = enforce_admins;
        self
    }

    /// Require at least one approving review on a pull request, before merging. Set to `None` to disable.
    pub fn required_pull_request_reviews(
        mut self,
        reviews: Option<UpdatePullRequestReviews>,
    ) -> Self {
        self.required_pull_request_reviews = reviews;
        self
    }

    /// Restrict who can push to the protected branch. Set to `None` to disable.
    pub fn restrictions(mut self, restrictions: Option<UpdateRestrictions>) -> Self {
        self.restrictions = restrictions;
        self
    }

    /// Enforces a linear commit Git history, which prevents anyone from pushing merge commits to the branch.
    pub fn required_linear_history(mut self, enabled: bool) -> Self {
        self.required_linear_history = Some(enabled);
        self
    }

    /// Permits force pushes to the protected branch by anyone with write access to the repository.
    pub fn allow_force_pushes(mut self, enabled: bool) -> Self {
        self.allow_force_pushes = Some(enabled);
        self
    }

    /// Allows deletion of the protected branch by anyone with write access to the repository.
    pub fn allow_deletions(mut self, enabled: bool) -> Self {
        self.allow_deletions = Some(enabled);
        self
    }

    /// If set to true, the `restrictions` branch protection rule also applies to creating branches that match the rule.
    pub fn block_creations(mut self, enabled: bool) -> Self {
        self.block_creations = Some(enabled);
        self
    }

    /// Requires all conversations on code to be resolved before a pull request can be merged into a branch that matches this rule.
    pub fn required_conversation_resolution(mut self, enabled: bool) -> Self {
        self.required_conversation_resolution = Some(enabled);
        self
    }

    /// Whether to set the branch to read-only. If this is true, users will not be able to push to the branch.
    pub fn lock_branch(mut self, enabled: bool) -> Self {
        self.lock_branch = Some(enabled);
        self
    }

    /// Whether users can pull changes from upstream when the branch is locked.
    pub fn allow_fork_syncing(mut self, enabled: bool) -> Self {
        self.allow_fork_syncing = Some(enabled);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<BranchProtection> {
        let route = format!("/{}/branches/{}/protection", self.handler.repo, self.branch);
        self.handler.crab.put(route, Some(&self)).await
    }
}

/// A client to GitHub's branch admin enforcement API.
///
/// Created with [`RepoBranchProtectionHandler::enforce_admins`].
pub struct RepoBranchAdminEnforcementHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
    branch: String,
}

impl<'octo, 'r> RepoBranchAdminEnforcementHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, branch: String) -> Self {
        Self { handler, branch }
    }

    /// Gets whether admin enforcement is enabled for the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#get-admin-branch-protection)
    pub async fn get(&self) -> Result<AdminEnforcement> {
        let route = format!(
            "/{}/branches/{}/protection/enforce_admins",
            self.handler.repo, self.branch
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Enables admin enforcement for the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#set-admin-branch-protection)
    pub async fn set(&self) -> Result<AdminEnforcement> {
        let route = format!(
            "/{}/branches/{}/protection/enforce_admins",
            self.handler.repo, self.branch
        );
        self.handler.crab.post(route, None::<&()>).await
    }

    /// Disables admin enforcement for the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#delete-admin-branch-protection)
    pub async fn delete(&self) -> Result<()> {
        let route = format!(
            "/{}/branches/{}/protection/enforce_admins",
            self.handler.repo, self.branch
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}

/// A client to GitHub's branch pull request reviews protection API.
///
/// Created with [`RepoBranchProtectionHandler::pull_request_reviews`].
pub struct RepoBranchPullRequestReviewsHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
    branch: String,
}

impl<'octo, 'r> RepoBranchPullRequestReviewsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, branch: String) -> Self {
        Self { handler, branch }
    }

    /// Gets pull request review protection settings for the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#get-pull-request-review-protection)
    pub async fn get(&self) -> Result<RequiredPullRequestReviews> {
        let route = format!(
            "/{}/branches/{}/protection/required_pull_request_reviews",
            self.handler.repo, self.branch
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates an [`UpdatePullRequestReviewsBuilder`] to update PR review protection settings.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#update-pull-request-review-protection)
    pub fn update(&self) -> UpdatePullRequestReviewsBuilder<'octo, 'r> {
        UpdatePullRequestReviewsBuilder::new(self.handler, self.branch.clone())
    }

    /// Deletes pull request review protection for the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#delete-pull-request-review-protection)
    pub async fn delete(&self) -> Result<()> {
        let route = format!(
            "/{}/branches/{}/protection/required_pull_request_reviews",
            self.handler.repo, self.branch
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}

/// A builder pattern struct for updating pull request review protection settings.
///
/// Created by [`RepoBranchPullRequestReviewsHandler::update`].
#[derive(serde::Serialize)]
pub struct UpdatePullRequestReviewsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    branch: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    dismiss_stale_reviews: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_code_owner_reviews: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_approving_review_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_last_push_approval: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dismissal_restrictions: Option<DismissalRestrictionsRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_pull_request_allowances:
        Option<crate::models::repos::branches::BypassPullRequestAllowancesRequest>,
}

impl<'octo, 'r> UpdatePullRequestReviewsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, branch: String) -> Self {
        Self {
            handler,
            branch,
            dismiss_stale_reviews: None,
            require_code_owner_reviews: None,
            required_approving_review_count: None,
            require_last_push_approval: None,
            dismissal_restrictions: None,
            bypass_pull_request_allowances: None,
        }
    }

    /// Set to true if you want to automatically dismiss approving reviews when someone pushes a new commit.
    pub fn dismiss_stale_reviews(mut self, dismiss: bool) -> Self {
        self.dismiss_stale_reviews = Some(dismiss);
        self
    }

    /// Blocks merging pull requests until code owners review them.
    pub fn require_code_owner_reviews(mut self, require: bool) -> Self {
        self.require_code_owner_reviews = Some(require);
        self
    }

    /// Specifies the number of reviewers required to approve pull requests.
    pub fn required_approving_review_count(mut self, count: u32) -> Self {
        self.required_approving_review_count = Some(count);
        self
    }

    /// Whether the most recent push must be approved by someone other than the person who pushed it.
    pub fn require_last_push_approval(mut self, require: bool) -> Self {
        self.require_last_push_approval = Some(require);
        self
    }

    /// Specify which users and teams can dismiss pull request reviews.
    pub fn dismissal_restrictions(mut self, restrictions: DismissalRestrictionsRequest) -> Self {
        self.dismissal_restrictions = Some(restrictions);
        self
    }

    /// Allow specific users, teams, or apps to bypass pull request requirements.
    pub fn bypass_pull_request_allowances(
        mut self,
        allowances: crate::models::repos::branches::BypassPullRequestAllowancesRequest,
    ) -> Self {
        self.bypass_pull_request_allowances = Some(allowances);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<RequiredPullRequestReviews> {
        let route = format!(
            "/{}/branches/{}/protection/required_pull_request_reviews",
            self.handler.repo, self.branch
        );
        self.handler.crab.patch(route, Some(&self)).await
    }
}

/// A client to GitHub's branch commit signatures protection API.
///
/// Created with [`RepoBranchProtectionHandler::signatures`].
pub struct RepoBranchSignaturesHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
    branch: String,
}

impl<'octo, 'r> RepoBranchSignaturesHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, branch: String) -> Self {
        Self { handler, branch }
    }

    /// Gets commit signature protection status for the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#get-commit-signature-protection)
    pub async fn get(&self) -> Result<ProtectionFlag> {
        let route = format!(
            "/{}/branches/{}/protection/required_signatures",
            self.handler.repo, self.branch
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates commit signature protection for the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#create-commit-signature-protection)
    pub async fn create(&self) -> Result<ProtectionFlag> {
        let route = format!(
            "/{}/branches/{}/protection/required_signatures",
            self.handler.repo, self.branch
        );
        self.handler.crab.post(route, None::<&()>).await
    }

    /// Deletes commit signature protection for the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#delete-commit-signature-protection)
    pub async fn delete(&self) -> Result<()> {
        let route = format!(
            "/{}/branches/{}/protection/required_signatures",
            self.handler.repo, self.branch
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}

/// A client to GitHub's branch required status checks API.
///
/// Created with [`RepoBranchProtectionHandler::status_checks`].
pub struct RepoBranchStatusChecksHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
    branch: String,
}

impl<'octo, 'r> RepoBranchStatusChecksHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, branch: String) -> Self {
        Self { handler, branch }
    }

    /// Gets status check protection configuration for the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#get-status-checks-protection)
    pub async fn get(&self) -> Result<RequiredStatusChecks> {
        let route = format!(
            "/{}/branches/{}/protection/required_status_checks",
            self.handler.repo, self.branch
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates an [`UpdateStatusChecksBuilder`] to update status check protection settings.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#update-status-check-protection)
    pub fn update(&self) -> UpdateStatusChecksBuilder<'octo, 'r> {
        UpdateStatusChecksBuilder::new(self.handler, self.branch.clone())
    }

    /// Removes status check protection from the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#remove-status-check-protection)
    pub async fn remove(&self) -> Result<()> {
        let route = format!(
            "/{}/branches/{}/protection/required_status_checks",
            self.handler.repo, self.branch
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Access the status check contexts sub-API for the branch.
    pub fn contexts(&self) -> RepoBranchStatusCheckContextsHandler<'octo, 'r> {
        RepoBranchStatusCheckContextsHandler::new(self.handler, self.branch.clone())
    }
}

/// A builder pattern struct for updating required status check settings.
///
/// Created by [`RepoBranchStatusChecksHandler::update`].
#[derive(serde::Serialize)]
pub struct UpdateStatusChecksBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    branch: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contexts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    checks: Option<Vec<StatusCheck>>,
}

impl<'octo, 'r> UpdateStatusChecksBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, branch: String) -> Self {
        Self {
            handler,
            branch,
            strict: None,
            contexts: None,
            checks: None,
        }
    }

    /// Require branches to be up to date before merging.
    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = Some(strict);
        self
    }

    /// The list of status checks to require in order to merge into this branch.
    pub fn contexts(mut self, contexts: impl Into<Vec<String>>) -> Self {
        self.contexts = Some(contexts.into());
        self
    }

    /// The list of status checks to require in order to merge into this branch.
    pub fn checks(mut self, checks: impl Into<Vec<StatusCheck>>) -> Self {
        self.checks = Some(checks.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<RequiredStatusChecks> {
        let route = format!(
            "/{}/branches/{}/protection/required_status_checks",
            self.handler.repo, self.branch
        );
        self.handler.crab.patch(route, Some(&self)).await
    }
}

/// A client to GitHub's branch status check contexts API.
///
/// Created with [`RepoBranchStatusChecksHandler::contexts`].
pub struct RepoBranchStatusCheckContextsHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
    branch: String,
}

impl<'octo, 'r> RepoBranchStatusCheckContextsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, branch: String) -> Self {
        Self { handler, branch }
    }

    /// Gets all status check contexts for the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#get-all-status-check-contexts)
    pub async fn get(&self) -> Result<Vec<String>> {
        let route = format!(
            "/{}/branches/{}/protection/required_status_checks/contexts",
            self.handler.repo, self.branch
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Adds status check contexts to the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#add-status-check-contexts)
    pub async fn add(&self, contexts: impl Into<Vec<String>>) -> Result<Vec<String>> {
        let route = format!(
            "/{}/branches/{}/protection/required_status_checks/contexts",
            self.handler.repo, self.branch
        );
        let body = ContextsRequest {
            contexts: contexts.into(),
        };
        self.handler.crab.post(route, Some(&body)).await
    }

    /// Sets status check contexts for the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#set-status-check-contexts)
    pub async fn set(&self, contexts: impl Into<Vec<String>>) -> Result<Vec<String>> {
        let route = format!(
            "/{}/branches/{}/protection/required_status_checks/contexts",
            self.handler.repo, self.branch
        );
        let body = ContextsRequest {
            contexts: contexts.into(),
        };
        self.handler.crab.put(route, Some(&body)).await
    }

    /// Removes status check contexts from the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#remove-status-check-contexts)
    pub async fn remove(&self, contexts: impl Into<Vec<String>>) -> Result<Vec<String>> {
        let route = format!(
            "/{}/branches/{}/protection/required_status_checks/contexts",
            self.handler.repo, self.branch
        );
        let body = ContextsRequest {
            contexts: contexts.into(),
        };
        self.handler.crab.delete(route, Some(&body)).await
    }
}

/// A client to GitHub's branch access restrictions API.
///
/// Created with [`RepoBranchProtectionHandler::restrictions`].
pub struct RepoBranchRestrictionsHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
    branch: String,
}

impl<'octo, 'r> RepoBranchRestrictionsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, branch: String) -> Self {
        Self { handler, branch }
    }

    /// Gets access restrictions for the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#get-access-restrictions)
    pub async fn get(&self) -> Result<BranchProtectionRestrictions> {
        let route = format!(
            "/{}/branches/{}/protection/restrictions",
            self.handler.repo, self.branch
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Deletes access restrictions for the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#delete-access-restrictions)
    pub async fn delete(&self) -> Result<()> {
        let route = format!(
            "/{}/branches/{}/protection/restrictions",
            self.handler.repo, self.branch
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Access the user restrictions sub-API.
    pub fn users(&self) -> RepoBranchRestrictionUsersHandler<'octo, 'r> {
        RepoBranchRestrictionUsersHandler::new(self.handler, self.branch.clone())
    }

    /// Access the team restrictions sub-API.
    pub fn teams(&self) -> RepoBranchRestrictionTeamsHandler<'octo, 'r> {
        RepoBranchRestrictionTeamsHandler::new(self.handler, self.branch.clone())
    }

    /// Access the app restrictions sub-API.
    pub fn apps(&self) -> RepoBranchRestrictionAppsHandler<'octo, 'r> {
        RepoBranchRestrictionAppsHandler::new(self.handler, self.branch.clone())
    }
}

/// A client to GitHub's branch user restrictions API.
///
/// Created with [`RepoBranchRestrictionsHandler::users`].
pub struct RepoBranchRestrictionUsersHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
    branch: String,
}

impl<'octo, 'r> RepoBranchRestrictionUsersHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, branch: String) -> Self {
        Self { handler, branch }
    }

    /// Lists users with push access to the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#list-users-with-access-to-the-protected-branch)
    pub async fn list(&self) -> Result<Vec<Author>> {
        let route = format!(
            "/{}/branches/{}/protection/restrictions/users",
            self.handler.repo, self.branch
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Adds users to push access restrictions.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#add-users-to-the-protected-branch-push-access-restriction)
    pub async fn add(&self, users: impl Into<Vec<String>>) -> Result<Vec<Author>> {
        let route = format!(
            "/{}/branches/{}/protection/restrictions/users",
            self.handler.repo, self.branch
        );
        let body = UsersRestrictionsRequest {
            users: users.into(),
        };
        self.handler.crab.post(route, Some(&body)).await
    }

    /// Sets users with push access to the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#set-users-with-access-to-the-protected-branch)
    pub async fn set(&self, users: impl Into<Vec<String>>) -> Result<Vec<Author>> {
        let route = format!(
            "/{}/branches/{}/protection/restrictions/users",
            self.handler.repo, self.branch
        );
        let body = UsersRestrictionsRequest {
            users: users.into(),
        };
        self.handler.crab.put(route, Some(&body)).await
    }

    /// Removes users from push access restrictions.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#remove-users-from-the-protected-branch-push-access-restriction)
    pub async fn remove(&self, users: impl Into<Vec<String>>) -> Result<Vec<Author>> {
        let route = format!(
            "/{}/branches/{}/protection/restrictions/users",
            self.handler.repo, self.branch
        );
        let body = UsersRestrictionsRequest {
            users: users.into(),
        };
        self.handler.crab.delete(route, Some(&body)).await
    }
}

/// A client to GitHub's branch team restrictions API.
///
/// Created with [`RepoBranchRestrictionsHandler::teams`].
pub struct RepoBranchRestrictionTeamsHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
    branch: String,
}

impl<'octo, 'r> RepoBranchRestrictionTeamsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, branch: String) -> Self {
        Self { handler, branch }
    }

    /// Lists teams with push access to the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#list-teams-with-access-to-the-protected-branch)
    pub async fn list(&self) -> Result<Vec<Team>> {
        let route = format!(
            "/{}/branches/{}/protection/restrictions/teams",
            self.handler.repo, self.branch
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Adds teams to push access restrictions.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#add-teams-to-the-protected-branch-push-access-restriction)
    pub async fn add(&self, teams: impl Into<Vec<String>>) -> Result<Vec<Team>> {
        let route = format!(
            "/{}/branches/{}/protection/restrictions/teams",
            self.handler.repo, self.branch
        );
        let body = TeamsRestrictionsRequest {
            teams: teams.into(),
        };
        self.handler.crab.post(route, Some(&body)).await
    }

    /// Sets teams with push access to the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#set-teams-with-access-to-the-protected-branch)
    pub async fn set(&self, teams: impl Into<Vec<String>>) -> Result<Vec<Team>> {
        let route = format!(
            "/{}/branches/{}/protection/restrictions/teams",
            self.handler.repo, self.branch
        );
        let body = TeamsRestrictionsRequest {
            teams: teams.into(),
        };
        self.handler.crab.put(route, Some(&body)).await
    }

    /// Removes teams from push access restrictions.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#remove-teams-from-the-protected-branch-push-access-restriction)
    pub async fn remove(&self, teams: impl Into<Vec<String>>) -> Result<Vec<Team>> {
        let route = format!(
            "/{}/branches/{}/protection/restrictions/teams",
            self.handler.repo, self.branch
        );
        let body = TeamsRestrictionsRequest {
            teams: teams.into(),
        };
        self.handler.crab.delete(route, Some(&body)).await
    }
}

/// A client to GitHub's branch app restrictions API.
///
/// Created with [`RepoBranchRestrictionsHandler::apps`].
pub struct RepoBranchRestrictionAppsHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
    branch: String,
}

impl<'octo, 'r> RepoBranchRestrictionAppsHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, branch: String) -> Self {
        Self { handler, branch }
    }

    /// Lists apps with push access to the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#list-apps-with-access-to-the-protected-branch)
    pub async fn list(&self) -> Result<Vec<App>> {
        let route = format!(
            "/{}/branches/{}/protection/restrictions/apps",
            self.handler.repo, self.branch
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Adds apps to push access restrictions.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#add-apps-to-the-protected-branch-push-access-restriction)
    pub async fn add(&self, apps: impl Into<Vec<String>>) -> Result<Vec<App>> {
        let route = format!(
            "/{}/branches/{}/protection/restrictions/apps",
            self.handler.repo, self.branch
        );
        let body = AppsRestrictionsRequest { apps: apps.into() };
        self.handler.crab.post(route, Some(&body)).await
    }

    /// Sets apps with push access to the branch.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#set-apps-with-access-to-the-protected-branch)
    pub async fn set(&self, apps: impl Into<Vec<String>>) -> Result<Vec<App>> {
        let route = format!(
            "/{}/branches/{}/protection/restrictions/apps",
            self.handler.repo, self.branch
        );
        let body = AppsRestrictionsRequest { apps: apps.into() };
        self.handler.crab.put(route, Some(&body)).await
    }

    /// Removes apps from push access restrictions.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#remove-apps-from-the-protected-branch-push-access-restriction)
    pub async fn remove(&self, apps: impl Into<Vec<String>>) -> Result<Vec<App>> {
        let route = format!(
            "/{}/branches/{}/protection/restrictions/apps",
            self.handler.repo, self.branch
        );
        let body = AppsRestrictionsRequest { apps: apps.into() };
        self.handler.crab.delete(route, Some(&body)).await
    }
}
