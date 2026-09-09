use crate::api::users::UserHandler;
use crate::models::GpgKey;
use crate::{FromResponse, Page};

#[derive(serde::Serialize)]
pub struct UserGpgKeysOpsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b UserHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> UserGpgKeysOpsBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b UserHandler<'octo>) -> Self {
        Self {
            handler,
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

    ///## List GPG keys for the authenticated user
    ///works with the following token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token must have the following permission set:
    ///* "GPG keys" user permissions (read)
    ///
    ///```no_run
    ///  use octocrab::models::GpgKey;
    /// use octocrab::{Page, Result};
    ///  async fn run() -> Result<Page<GpgKey>> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .gpg_keys()
    ///        .per_page(42).page(3u32)
    ///        .list()
    ///        .await
    ///  }
    pub async fn list(&self) -> crate::Result<Page<crate::models::GpgKey>> {
        let route = "/user/gpg_keys".to_string();
        self.handler.crab.get(route, Some(&self)).await
    }

    ///## View extended details for a single GPG key for the authenticated user
    ///
    ///OAuth app tokens and personal access tokens (classic) need the read:gpg_key scope to use this method.
    ///
    ///works with the following token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token must have the following permission set:
    ///* "GPG keys" user permissions (read)
    ///
    ///```no_run
    ///  use octocrab::models::GpgKey;
    /// use octocrab::Result;
    ///  async fn run() -> Result<GpgKey> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .gpg_keys()
    ///        .get(42)
    ///        .await
    ///  }
    pub async fn get(&self, gpg_key_id: u64) -> crate::Result<GpgKey> {
        let route = format!("/user/gpg_keys/{gpg_key_id}");
        self.handler.crab.get(route, None::<&()>).await
    }

    ///## Create a GPG key for the authenticated user
    ///works with the following fine-grained token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token must have the following permission set:
    ///* "GPG keys" user permissions (write)
    ///
    ///```no_run
    /// use octocrab::models::GpgKey;
    /// use octocrab::Result;
    ///  async fn run() -> Result<GpgKey> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .gpg_keys()
    ///        .add("descriptive name".to_string(), "<A GPG key in ASCII-armored format>".to_string())
    ///        .await
    ///  }
    pub async fn add(&self, name: String, armored_public_key: String) -> crate::Result<GpgKey> {
        let route = "/user/gpg_keys".to_string();

        let params = serde_json::json!({
            "name": name,
            "armored_public_key": armored_public_key,
        });
        let response = self.handler.crab._post(route, Some(&params)).await?;
        if response.status() != http::StatusCode::CREATED {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }

        <GpgKey>::from_response(crate::map_github_error(response).await?).await
    }

    ///## Delete a GPG key for the authenticated user
    ///works with the following fine-grained token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token must have the following permission set:
    ///* "GPG keys" user permissions (write)
    ///
    ///```no_run
    /// use octocrab::Result;
    ///  async fn run() -> Result<()> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .gpg_keys()
    ///        .delete(42)
    ///        .await
    ///  }
    pub async fn delete(&self, gpg_key_id: u64) -> crate::Result<()> {
        let route = format!("/user/gpg_keys/{gpg_key_id}");

        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if response.status() != http::StatusCode::NO_CONTENT {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }

        Ok(())
    }

    ///## List GPG keys for a user
    ///
    ///Lists the GPG keys for a user. This information is accessible by anyone.
    ///
    ///See: [GitHub API Documentation][docs] for `GET /users/{username}/gpg_keys`
    ///
    ///# Examples
    ///
    ///```no_run
    ///  use octocrab::models::GpgKey;
    ///  use octocrab::{Page, Result};
    ///  async fn run() -> Result<Page<GpgKey>> {
    ///    octocrab::instance()
    ///        .users("some_user")
    ///        .list_user_gpg_keys()
    ///        .per_page(42)
    ///        .page(3u32)
    ///        .send()
    ///        .await
    ///  }
    ///```
    ///
    ///[docs]: https://docs.github.com/en/rest/users/gpg-keys?apiVersion=2022-11-28#list-gpg-keys-for-a-user
    pub async fn list_user_gpg_keys(&self) -> crate::Result<Page<GpgKey>> {
        let route = format!("/{}/gpg_keys", self.handler.user);
        self.handler.crab.get(route, Some(&self)).await
    }

    /// Sends the request to list GPG keys for a user.
    ///
    /// See: [GitHub API Documentation][docs] for `GET /users/{username}/gpg_keys`
    ///
    /// [docs]: https://docs.github.com/en/rest/users/gpg-keys?apiVersion=2022-11-28#list-gpg-keys-for-a-user
    pub async fn send(&self) -> crate::Result<Page<GpgKey>> {
        self.list_user_gpg_keys().await
    }
}

/// A builder pattern struct for listing a user's GPG keys.
///
/// Created by [`UserHandler::list_user_gpg_keys`].
pub type ListUserGpgKeysBuilder<'octo, 'b> = UserGpgKeysOpsBuilder<'octo, 'b>;

