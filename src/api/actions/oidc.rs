use crate::{actions::ActionsHandler, models::actions::OidcCustomSub, Result};

impl<'octo> ActionsHandler<'octo> {
    /// Gets the customization template for an OpenID Connect (OIDC) subject claim for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/oidc?apiVersion=2022-11-28#get-the-customization-template-for-an-oidc-subject-claim-for-an-organization)
    pub async fn get_org_oidc_custom_sub(&self, org: impl AsRef<str>) -> Result<OidcCustomSub> {
        let route = format!(
            "/orgs/{org}/actions/oidc/customization/sub",
            org = org.as_ref(),
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Sets the customization template for an OpenID Connect (OIDC) subject claim for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/oidc?apiVersion=2022-11-28#set-the-customization-template-for-an-oidc-subject-claim-for-an-organization)
    pub async fn set_org_oidc_custom_sub(
        &self,
        org: impl AsRef<str>,
        oidc_custom_sub: &OidcCustomSub,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{org}/actions/oidc/customization/sub",
            org = org.as_ref(),
        );
        crate::map_github_error(self.crab._put(route, Some(oidc_custom_sub)).await?)
            .await
            .map(drop)
    }

    /// Gets the customization template for an OpenID Connect (OIDC) subject claim for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/oidc?apiVersion=2022-11-28#get-the-customization-template-for-an-oidc-subject-claim-for-a-repository)
    pub async fn get_repo_oidc_custom_sub(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> Result<OidcCustomSub> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/oidc/customization/sub",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Sets the customization template for an OpenID Connect (OIDC) subject claim for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/oidc?apiVersion=2022-11-28#set-the-customization-template-for-an-oidc-subject-claim-for-a-repository)
    pub async fn set_repo_oidc_custom_sub(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        oidc_custom_sub: &OidcCustomSub,
    ) -> Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/oidc/customization/sub",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
        );
        crate::map_github_error(self.crab._put(route, Some(oidc_custom_sub)).await?)
            .await
            .map(drop)
    }
}
