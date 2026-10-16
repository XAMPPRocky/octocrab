use crate::{actions::ActionsHandler, models::actions::OidcCustomSub, Octocrab, Result};

impl<'octo> ActionsHandler<'octo> {
    /// Gets the customization template for an OpenID Connect (OIDC) subject claim for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/oidc?apiVersion=2022-11-28#get-the-customization-template-for-an-oidc-subject-claim-for-an-organization)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let custom_sub = octocrab.actions().get_org_oidc_custom_sub("org").await?;
    /// # Ok(())
    /// # }
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let custom_sub = octocrab.actions().get_org_oidc_custom_sub("org").await?;
    /// octocrab.actions().set_org_oidc_custom_sub("org", &custom_sub).await?;
    /// # Ok(())
    /// # }
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let custom_sub = octocrab.actions().get_repo_oidc_custom_sub("owner", "repo").await?;
    /// # Ok(())
    /// # }
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let custom_sub = octocrab.actions().get_repo_oidc_custom_sub("owner", "repo").await?;
    /// octocrab.actions().set_repo_oidc_custom_sub("owner", "repo", &custom_sub).await?;
    /// # Ok(())
    /// # }
    /// ```
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

/// Handler for GitHub Actions OIDC customization for an enterprise.
pub struct EnterpriseActionsOidcHandler<'octo> {
    crab: &'octo Octocrab,
    enterprise: String,
}

impl<'octo> EnterpriseActionsOidcHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, enterprise: String) -> Self {
        Self { crab, enterprise }
    }

    /// List OIDC custom property inclusions for an enterprise.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/oidc?apiVersion=2022-11-28#list-oidc-custom-property-inclusions-for-an-enterprise)
    pub async fn list_repo_properties(&self) -> Result<serde_json::Value> {
        let route = format!(
            "/enterprises/{}/actions/oidc/customization/properties/repo",
            self.enterprise
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Create an OIDC custom property inclusion for an enterprise.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/oidc?apiVersion=2022-11-28#create-an-oidc-custom-property-inclusion-for-an-enterprise)
    pub async fn create_repo_property(
        &self,
        custom_property_name: impl Into<String>,
    ) -> Result<serde_json::Value> {
        let route = format!(
            "/enterprises/{}/actions/oidc/customization/properties/repo",
            self.enterprise
        );
        let body = serde_json::json!({ "custom_property_name": custom_property_name.into() });
        self.crab.post(route, Some(&body)).await
    }

    /// Delete an OIDC custom property inclusion for an enterprise.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/oidc?apiVersion=2022-11-28#delete-an-oidc-custom-property-inclusion-for-an-enterprise)
    pub async fn delete_repo_property(&self, custom_property_name: impl AsRef<str>) -> Result<()> {
        let route = format!(
            "/enterprises/{}/actions/oidc/customization/properties/repo/{}",
            self.enterprise,
            custom_property_name.as_ref()
        );
        let response = self.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}

/// Handler for GitHub Actions OIDC customization for an organization.
pub struct OrgActionsOidcHandler<'octo> {
    crab: &'octo Octocrab,
    org: String,
}

impl<'octo> OrgActionsOidcHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, org: String) -> Self {
        Self { crab, org }
    }

    /// List OIDC custom property inclusions for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/oidc?apiVersion=2022-11-28#list-oidc-custom-property-inclusions-for-an-organization)
    pub async fn list_repo_properties(&self) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/actions/oidc/customization/properties/repo",
            self.org
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Create an OIDC custom property inclusion for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/oidc?apiVersion=2022-11-28#create-an-oidc-custom-property-inclusion-for-an-organization)
    pub async fn create_repo_property(
        &self,
        custom_property_name: impl Into<String>,
    ) -> Result<serde_json::Value> {
        let route = format!(
            "/orgs/{}/actions/oidc/customization/properties/repo",
            self.org
        );
        let body = serde_json::json!({ "custom_property_name": custom_property_name.into() });
        self.crab.post(route, Some(&body)).await
    }

    /// Delete an OIDC custom property inclusion for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/oidc?apiVersion=2022-11-28#delete-an-oidc-custom-property-inclusion-for-an-organization)
    pub async fn delete_repo_property(&self, custom_property_name: impl AsRef<str>) -> Result<()> {
        let route = format!(
            "/orgs/{}/actions/oidc/customization/properties/repo/{}",
            self.org,
            custom_property_name.as_ref()
        );
        let response = self.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}
