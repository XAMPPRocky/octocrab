use super::OrgHandler;
use crate::models::{
    actions::{ActionsSelectedRepositories, OrgVariable, OrgVariables},
    orgs::secrets::Visibility,
    RepositoryId,
};
use crate::Result;
use serde::Serialize;

/// Client for GitHub's organization variables API.
pub struct OrgVariablesHandler<'octo> {
    org: &'octo OrgHandler<'octo>,
    per_page: Option<u8>,
    page: Option<u32>,
}

impl<'octo> OrgVariablesHandler<'octo> {
    pub(crate) fn new(org: &'octo OrgHandler<'octo>) -> Self {
        Self {
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

    fn owner(&self) -> &str {
        &self.org.owner
    }

    /// Lists all organization variables.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/variables?apiVersion=2022-11-28#list-organization-variables)
    pub async fn list(&self) -> Result<OrgVariables> {
        #[derive(Serialize)]
        struct Query {
            #[serde(skip_serializing_if = "Option::is_none")]
            per_page: Option<u8>,
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<u32>,
        }
        let route = format!("/orgs/{org}/actions/variables", org = self.owner());
        let query = Query {
            per_page: self.per_page,
            page: self.page,
        };
        self.org.crab.get(route, Some(&query)).await
    }

    /// Gets a specific organization variable.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/variables?apiVersion=2022-11-28#get-an-organization-variable)
    pub async fn get(&self, name: impl AsRef<str>) -> Result<OrgVariable> {
        let route = format!(
            "/orgs/{org}/actions/variables/{name}",
            org = self.owner(),
            name = name.as_ref(),
        );
        self.org.crab.get(route, None::<&()>).await
    }

    /// Creates an organization variable.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/variables?apiVersion=2022-11-28#create-an-organization-variable)
    pub async fn create(
        &self,
        name: impl AsRef<str>,
        value: impl AsRef<str>,
        visibility: Visibility,
        selected_repository_ids: Option<&[RepositoryId]>,
    ) -> Result<()> {
        #[derive(Serialize)]
        struct Body<'a> {
            name: &'a str,
            value: &'a str,
            visibility: Visibility,
            #[serde(skip_serializing_if = "Option::is_none")]
            selected_repository_ids: Option<&'a [RepositoryId]>,
        }
        let route = format!("/orgs/{org}/actions/variables", org = self.owner());
        let body = Body {
            name: name.as_ref(),
            value: value.as_ref(),
            visibility,
            selected_repository_ids,
        };
        crate::map_github_error(self.org.crab._post(route, Some(&body)).await?)
            .await
            .map(drop)
    }

    /// Updates an organization variable.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/variables?apiVersion=2022-11-28#update-an-organization-variable)
    pub async fn update(
        &self,
        name: impl AsRef<str>,
        new_name: Option<impl AsRef<str>>,
        value: Option<impl AsRef<str>>,
        visibility: Option<Visibility>,
        selected_repository_ids: Option<&[RepositoryId]>,
    ) -> Result<()> {
        #[derive(Serialize)]
        struct Body<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            visibility: Option<Visibility>,
            #[serde(skip_serializing_if = "Option::is_none")]
            selected_repository_ids: Option<&'a [RepositoryId]>,
        }
        let route = format!(
            "/orgs/{org}/actions/variables/{name}",
            org = self.owner(),
            name = name.as_ref(),
        );
        let body = Body {
            name: new_name.as_ref().map(AsRef::as_ref),
            value: value.as_ref().map(AsRef::as_ref),
            visibility,
            selected_repository_ids,
        };
        crate::map_github_error(self.org.crab._patch(route, Some(&body)).await?)
            .await
            .map(drop)
    }

    /// Deletes an organization variable.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/variables?apiVersion=2022-11-28#delete-an-organization-variable)
    pub async fn delete(&self, name: impl AsRef<str>) -> Result<()> {
        let route = format!(
            "/orgs/{org}/actions/variables/{name}",
            org = self.owner(),
            name = name.as_ref(),
        );
        let response = self.org.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Lists selected repositories for an organization variable.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/variables?apiVersion=2022-11-28#list-selected-repositories-for-an-organization-variable)
    pub async fn list_selected_repositories(
        &self,
        name: impl AsRef<str>,
    ) -> Result<ActionsSelectedRepositories> {
        let route = format!(
            "/orgs/{org}/actions/variables/{name}/repositories",
            org = self.owner(),
            name = name.as_ref(),
        );
        self.org.crab.get(route, None::<&()>).await
    }

    /// Sets selected repositories for an organization variable.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/variables?apiVersion=2022-11-28#set-selected-repositories-for-an-organization-variable)
    pub async fn set_selected_repositories(
        &self,
        name: impl AsRef<str>,
        selected_repository_ids: &[RepositoryId],
    ) -> Result<()> {
        #[derive(Serialize)]
        struct Body<'a> {
            selected_repository_ids: &'a [RepositoryId],
        }
        let route = format!(
            "/orgs/{org}/actions/variables/{name}/repositories",
            org = self.owner(),
            name = name.as_ref(),
        );
        let body = Body {
            selected_repository_ids,
        };
        crate::map_github_error(self.org.crab._put(route, Some(&body)).await?)
            .await
            .map(drop)
    }

    /// Adds a selected repository to an organization variable.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/variables?apiVersion=2022-11-28#add-selected-repository-to-an-organization-variable)
    pub async fn add_selected_repository(
        &self,
        name: impl AsRef<str>,
        repository_id: RepositoryId,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{org}/actions/variables/{name}/repositories/{repository_id}",
            org = self.owner(),
            name = name.as_ref(),
        );
        crate::map_github_error(self.org.crab._put(route, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Removes a selected repository from an organization variable.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/variables?apiVersion=2022-11-28#remove-selected-repository-from-an-organization-variable)
    pub async fn remove_selected_repository(
        &self,
        name: impl AsRef<str>,
        repository_id: RepositoryId,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{org}/actions/variables/{name}/repositories/{repository_id}",
            org = self.owner(),
            name = name.as_ref(),
        );
        crate::map_github_error(self.org.crab._delete(route, None::<&()>).await?)
            .await
            .map(drop)
    }
}
