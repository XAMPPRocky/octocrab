use super::RepoHandler;
use crate::models::repos::CustomPropertyValue;

/// Handler for GitHub's repository custom properties API.
///
/// Created with [`RepoHandler::custom_properties`].
pub struct RepoCustomPropertiesHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> RepoCustomPropertiesHandler<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Get all custom property values for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/custom-properties?apiVersion=2022-11-28#get-all-custom-property-values-for-a-repository)
    pub async fn get_values(&self) -> crate::Result<Vec<CustomPropertyValue>> {
        let route = format!("/{}/properties/values", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Create or update custom property values for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/repos/custom-properties?apiVersion=2022-11-28#create-or-update-custom-property-values-for-a-repository)
    pub async fn create_or_update_values(
        &self,
        properties: impl IntoIterator<Item = CustomPropertyValue>,
    ) -> crate::Result<()> {
        let route = format!("/{}/properties/values", self.handler.repo);
        let body = serde_json::json!({
            "properties": properties.into_iter().collect::<Vec<_>>()
        });
        let response = self.handler.crab._patch(route, Some(&body)).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }
        Ok(())
    }
}
