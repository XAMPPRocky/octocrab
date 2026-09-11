use super::OrgHandler;
use crate::models::orgs::custom_properties::{
    CustomPropertyValue, OrgCustomPropertyDefinition, OrgRepoCustomPropertyValues,
};
use crate::{Page, Result};

/// A client to GitHub's organization custom properties API.
///
/// Created with [`OrgHandler::custom_properties`].
pub struct OrgCustomPropertiesHandler<'octo, 'r> {
    handler: &'r OrgHandler<'octo>,
}

impl<'octo, 'r> OrgCustomPropertiesHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Gets all custom properties for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/custom-properties?apiVersion=2022-11-28#get-all-custom-properties-for-an-organization)
    pub async fn get_schema(&self) -> Result<Vec<OrgCustomPropertyDefinition>> {
        let route = format!("/orgs/{org}/properties/schema", org = self.handler.owner);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates or updates custom properties for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/custom-properties?apiVersion=2022-11-28#create-or-update-custom-properties-for-an-organization)
    pub async fn create_or_update_schema(
        &self,
        properties: impl IntoIterator<Item = OrgCustomPropertyDefinition>,
    ) -> Result<Vec<OrgCustomPropertyDefinition>> {
        let route = format!("/orgs/{org}/properties/schema", org = self.handler.owner);
        let body = serde_json::json!({
            "properties": properties.into_iter().collect::<Vec<_>>()
        });
        self.handler.crab.patch(route, Some(&body)).await
    }

    /// Gets a custom property for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/custom-properties?apiVersion=2022-11-28#get-a-custom-property-for-an-organization)
    pub async fn get_property_schema(
        &self,
        custom_property_name: impl AsRef<str>,
    ) -> Result<OrgCustomPropertyDefinition> {
        let route = format!(
            "/orgs/{org}/properties/schema/{name}",
            org = self.handler.owner,
            name = custom_property_name.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates or updates a custom property for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/custom-properties?apiVersion=2022-11-28#create-or-update-a-custom-property-for-an-organization)
    pub async fn create_or_update_property(
        &self,
        custom_property_name: impl AsRef<str>,
        property: &OrgCustomPropertyDefinition,
    ) -> Result<OrgCustomPropertyDefinition> {
        let route = format!(
            "/orgs/{org}/properties/schema/{name}",
            org = self.handler.owner,
            name = custom_property_name.as_ref()
        );
        self.handler.crab.put(route, Some(property)).await
    }

    /// Removes a custom property for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/custom-properties?apiVersion=2022-11-28#remove-a-custom-property-for-an-organization)
    pub async fn remove_property(&self, custom_property_name: impl AsRef<str>) -> Result<()> {
        let route = format!(
            "/orgs/{org}/properties/schema/{name}",
            org = self.handler.owner,
            name = custom_property_name.as_ref()
        );
        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }

    /// Lists custom property values for organization repositories.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/custom-properties?apiVersion=2022-11-28#list-custom-property-values-for-organization-repositories)
    pub fn get_values(&self) -> ListOrgCustomPropertyValuesBuilder<'octo, 'r> {
        ListOrgCustomPropertyValuesBuilder::new(self.handler)
    }

    /// Creates or updates custom property values for organization repositories.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/orgs/custom-properties?apiVersion=2022-11-28#create-or-update-custom-property-values-for-organization-repositories)
    pub async fn create_or_update_values(
        &self,
        repository_names: impl IntoIterator<Item = impl Into<String>>,
        properties: impl IntoIterator<Item = CustomPropertyValue>,
    ) -> Result<()> {
        let route = format!("/orgs/{org}/properties/values", org = self.handler.owner);
        let body = serde_json::json!({
            "repository_names": repository_names.into_iter().map(Into::into).collect::<Vec<_>>(),
            "properties": properties.into_iter().collect::<Vec<_>>()
        });
        let response = self.handler.crab._patch(route, Some(&body)).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}

/// Builder for listing custom property values for organization repositories.
#[derive(serde::Serialize)]
pub struct ListOrgCustomPropertyValuesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_query: Option<String>,
}

impl<'octo, 'r> ListOrgCustomPropertyValuesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
            repository_query: None,
        }
    }

    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    pub fn repository_query(mut self, query: impl Into<String>) -> Self {
        self.repository_query = Some(query.into());
        self
    }

    pub async fn send(self) -> Result<Page<OrgRepoCustomPropertyValues>> {
        let route = format!("/orgs/{org}/properties/values", org = self.handler.owner);
        self.handler.crab.get(route, Some(&self)).await
    }
}
