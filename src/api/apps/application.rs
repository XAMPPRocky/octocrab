use crate::models::apps::{AppAuthorization, CreateScopedAccessToken, TokenBody};
use crate::{Octocrab, Result};

/// A client for managing OAuth authorizations for a specific application.
///
/// Created with [`AppsRequestHandler::application`][super::AppsRequestHandler::application] or
/// [`Octocrab::applications`].
pub struct ApplicationHandler<'octo> {
    crab: &'octo Octocrab,
    client_id: String,
}

impl<'octo> ApplicationHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, client_id: String) -> Self {
        Self { crab, client_id }
    }

    /// Deletes an OAuth application authorization (grant) for a user.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/oauth-applications?apiVersion=2022-11-28#delete-an-app-authorization)
    pub async fn delete_grant(&self, access_token: impl Into<String>) -> Result<()> {
        let route = format!("/applications/{}/grant", self.client_id);
        let body = TokenBody {
            access_token: access_token.into(),
        };
        let resp = self.crab._delete(route, Some(&body)).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Checks the validity of an OAuth application token.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/oauth-applications?apiVersion=2022-11-28#check-a-token)
    pub async fn check_token(&self, access_token: impl Into<String>) -> Result<AppAuthorization> {
        let route = format!("/applications/{}/token", self.client_id);
        let body = TokenBody {
            access_token: access_token.into(),
        };
        self.crab.post(route, Some(&body)).await
    }

    /// Resets an OAuth application token.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/oauth-applications?apiVersion=2022-11-28#reset-a-token)
    pub async fn reset_token(&self, access_token: impl Into<String>) -> Result<AppAuthorization> {
        let route = format!("/applications/{}/token", self.client_id);
        let body = TokenBody {
            access_token: access_token.into(),
        };
        self.crab.patch(route, Some(&body)).await
    }

    /// Deletes an OAuth application token.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/oauth-applications?apiVersion=2022-11-28#delete-an-app-token)
    pub async fn delete_token(&self, access_token: impl Into<String>) -> Result<()> {
        let route = format!("/applications/{}/token", self.client_id);
        let body = TokenBody {
            access_token: access_token.into(),
        };
        let resp = self.crab._delete(route, Some(&body)).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Creates a scoped access token for an OAuth application.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/apps/apps?apiVersion=2022-11-28#create-a-scoped-access-token)
    pub async fn scoped_token(&self, body: &CreateScopedAccessToken) -> Result<AppAuthorization> {
        let route = format!("/applications/{}/token/scoped", self.client_id);
        self.crab.post(route, Some(body)).await
    }
}
