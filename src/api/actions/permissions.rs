use crate::{
    actions::ActionsHandler,
    models::{
        actions::{
            AccessPermissions, ActionsSelectedRepositories, AllowedActions,
            DefaultWorkflowPermissions, EnabledRepositories, OrgActionsPermissions,
            RepoActionsPermissions, SelectedActions,
        },
        RepositoryId,
    },
    Result,
};
use serde::Serialize;

/// Builder for listing selected repositories enabled for GitHub Actions in an organization.
#[derive(Serialize)]
pub struct ListOrgSelectedRepositoriesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ActionsHandler<'octo>,
    #[serde(skip)]
    org: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListOrgSelectedRepositoriesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r ActionsHandler<'octo>, org: String) -> Self {
        Self {
            handler,
            org,
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

    /// Sends the request and returns selected repositories.
    pub async fn send(self) -> Result<ActionsSelectedRepositories> {
        let route = format!(
            "/orgs/{org}/actions/permissions/repositories",
            org = self.org,
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

impl<'octo> ActionsHandler<'octo> {
    /// Gets the GitHub Actions permissions policy for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#get-github-actions-permissions-for-an-organization)
    pub async fn get_org_actions_permissions(
        &self,
        org: impl AsRef<str>,
    ) -> Result<OrgActionsPermissions> {
        let route = format!("/orgs/{org}/actions/permissions", org = org.as_ref());
        self.crab.get(route, None::<&()>).await
    }

    /// Sets the GitHub Actions permissions policy for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#set-github-actions-permissions-for-an-organization)
    pub async fn set_org_actions_permissions(
        &self,
        org: impl AsRef<str>,
        enabled_repositories: EnabledRepositories,
        allowed_actions: Option<AllowedActions>,
        sha_pinning_required: Option<bool>,
    ) -> Result<()> {
        #[derive(Serialize)]
        struct Body {
            enabled_repositories: EnabledRepositories,
            #[serde(skip_serializing_if = "Option::is_none")]
            allowed_actions: Option<AllowedActions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sha_pinning_required: Option<bool>,
        }
        let route = format!("/orgs/{org}/actions/permissions", org = org.as_ref());
        let body = Body {
            enabled_repositories,
            allowed_actions,
            sha_pinning_required,
        };
        crate::map_github_error(self.crab._put(route, Some(&body)).await?)
            .await
            .map(drop)
    }

    /// Lists selected repositories enabled for GitHub Actions in an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#list-selected-repositories-enabled-for-github-actions-in-an-organization)
    pub fn list_org_selected_repositories(
        &self,
        org: impl Into<String>,
    ) -> ListOrgSelectedRepositoriesBuilder<'octo, '_> {
        ListOrgSelectedRepositoriesBuilder::new(self, org.into())
    }

    /// Sets selected repositories enabled for GitHub Actions in an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#set-selected-repositories-enabled-for-github-actions-in-an-organization)
    pub async fn set_org_selected_repositories(
        &self,
        org: impl AsRef<str>,
        selected_repository_ids: &[RepositoryId],
    ) -> Result<()> {
        #[derive(Serialize)]
        struct Body<'a> {
            selected_repository_ids: &'a [RepositoryId],
        }
        let route = format!(
            "/orgs/{org}/actions/permissions/repositories",
            org = org.as_ref()
        );
        let body = Body {
            selected_repository_ids,
        };
        crate::map_github_error(self.crab._put(route, Some(&body)).await?)
            .await
            .map(drop)
    }

    /// Enables a selected repository for GitHub Actions in an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#enable-a-selected-repository-for-github-actions-in-an-organization)
    pub async fn enable_org_selected_repository(
        &self,
        org: impl AsRef<str>,
        repository_id: RepositoryId,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{org}/actions/permissions/repositories/{repository_id}",
            org = org.as_ref()
        );
        crate::map_github_error(self.crab._put(route, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Disables a selected repository for GitHub Actions in an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#disable-a-selected-repository-for-github-actions-in-an-organization)
    pub async fn disable_org_selected_repository(
        &self,
        org: impl AsRef<str>,
        repository_id: RepositoryId,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{org}/actions/permissions/repositories/{repository_id}",
            org = org.as_ref()
        );
        crate::map_github_error(self.crab._delete(route, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Gets allowed actions and reusable workflows for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#get-allowed-actions-and-reusable-workflows-for-an-organization)
    pub async fn get_org_selected_actions(&self, org: impl AsRef<str>) -> Result<SelectedActions> {
        let route = format!(
            "/orgs/{org}/actions/permissions/selected-actions",
            org = org.as_ref()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Sets allowed actions and reusable workflows for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#set-allowed-actions-and-reusable-workflows-for-an-organization)
    pub async fn set_org_selected_actions(
        &self,
        org: impl AsRef<str>,
        selected_actions: &SelectedActions,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{org}/actions/permissions/selected-actions",
            org = org.as_ref()
        );
        crate::map_github_error(self.crab._put(route, Some(selected_actions)).await?)
            .await
            .map(drop)
    }

    /// Gets default workflow permissions for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#get-default-workflow-permissions-for-an-organization)
    pub async fn get_org_default_workflow_permissions(
        &self,
        org: impl AsRef<str>,
    ) -> Result<DefaultWorkflowPermissions> {
        let route = format!(
            "/orgs/{org}/actions/permissions/workflow",
            org = org.as_ref()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Sets default workflow permissions for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#set-default-workflow-permissions-for-an-organization)
    pub async fn set_org_default_workflow_permissions(
        &self,
        org: impl AsRef<str>,
        default_workflow_permissions: &DefaultWorkflowPermissions,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{org}/actions/permissions/workflow",
            org = org.as_ref()
        );
        crate::map_github_error(
            self.crab
                ._put(route, Some(default_workflow_permissions))
                .await?,
        )
        .await
        .map(drop)
    }

    /// Gets GitHub Actions permissions for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#get-github-actions-permissions-for-a-repository)
    pub async fn get_repo_actions_permissions(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> Result<RepoActionsPermissions> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/permissions",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Sets GitHub Actions permissions for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#set-github-actions-permissions-for-a-repository)
    pub async fn set_repo_actions_permissions(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        enabled: bool,
        allowed_actions: Option<AllowedActions>,
        sha_pinning_required: Option<bool>,
    ) -> Result<()> {
        #[derive(Serialize)]
        struct Body {
            enabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            allowed_actions: Option<AllowedActions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sha_pinning_required: Option<bool>,
        }
        let route = format!(
            "/repos/{owner}/{repo}/actions/permissions",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
        );
        let body = Body {
            enabled,
            allowed_actions,
            sha_pinning_required,
        };
        crate::map_github_error(self.crab._put(route, Some(&body)).await?)
            .await
            .map(drop)
    }

    /// Gets GitHub Actions permissions for repository access.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#get-github-actions-permissions-for-repository-access)
    pub async fn get_repo_actions_access_permissions(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> Result<AccessPermissions> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/permissions/access",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Sets GitHub Actions permissions for repository access.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#set-github-actions-permissions-for-repository-access)
    pub async fn set_repo_actions_access_permissions(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        access_level: impl Into<String>,
    ) -> Result<()> {
        #[derive(Serialize)]
        struct Body {
            access_level: String,
        }
        let route = format!(
            "/repos/{owner}/{repo}/actions/permissions/access",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
        );
        let body = Body {
            access_level: access_level.into(),
        };
        crate::map_github_error(self.crab._put(route, Some(&body)).await?)
            .await
            .map(drop)
    }

    /// Gets allowed actions and reusable workflows for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#get-allowed-actions-and-reusable-workflows-for-a-repository)
    pub async fn get_repo_selected_actions(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> Result<SelectedActions> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/permissions/selected-actions",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Sets allowed actions and reusable workflows for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#set-allowed-actions-and-reusable-workflows-for-a-repository)
    pub async fn set_repo_selected_actions(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        selected_actions: &SelectedActions,
    ) -> Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/permissions/selected-actions",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
        );
        crate::map_github_error(self.crab._put(route, Some(selected_actions)).await?)
            .await
            .map(drop)
    }

    /// Gets default workflow permissions for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#get-default-workflow-permissions-for-a-repository)
    pub async fn get_repo_default_workflow_permissions(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> Result<DefaultWorkflowPermissions> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/permissions/workflow",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Sets default workflow permissions for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#set-default-workflow-permissions-for-a-repository)
    pub async fn set_repo_default_workflow_permissions(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        default_workflow_permissions: &DefaultWorkflowPermissions,
    ) -> Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/permissions/workflow",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
        );
        crate::map_github_error(
            self.crab
                ._put(route, Some(default_workflow_permissions))
                .await?,
        )
        .await
        .map(drop)
    }
}
