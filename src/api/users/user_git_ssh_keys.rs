use crate::api::users::UserHandler;
use crate::models::GitSshKey;
use crate::{FromResponse, Page};

#[derive(serde::Serialize)]
pub struct UserGitSshKeysOpsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b UserHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> UserGitSshKeysOpsBuilder<'octo, 'b> {
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

    ///## List public SSH keys for the authenticated user
    ///OAuth app tokens and personal access tokens (classic) need the read:public_key scope
    ///
    ///works with the following fine-grained token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token must have the following permission set:
    ///* "Git SSH keys" user permissions (read)
    ///
    ///```no_run
    ///  use octocrab::models::GitSshKey;
    /// use octocrab::{Page, Result};
    ///  async fn run() -> Result<Page<GitSshKey>> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .git_ssh_keys()
    ///        .per_page(42).page(3u32)
    ///        .list()
    ///        .await
    ///  }
    pub async fn list(&self) -> crate::Result<Page<crate::models::GitSshKey>> {
        let route = "/user/keys".to_string();
        self.handler.crab.get(route, Some(&self)).await
    }

    ///## Create a public SSH key for the authenticated user
    /// OAuth app tokens and personal access tokens (classic) need the `write:gpg_key` scope
    ///
    ///works with the following fine-grained token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token must have the following permission set:
    ///* "Git SSH keys" user permissions (write)
    ///
    ///```no_run
    /// use octocrab::models::GitSshKey;
    /// use octocrab::Result;
    ///  async fn run() -> Result<GitSshKey> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .git_ssh_keys()
    ///        .add("ssh-rsa AAAAB3NzaC1yc2EAAA".to_string(), "2Sg8iYjAxxmI2LvUXpJjkYrMxURPc8r+dB7TJyvv1234".to_string())
    ///        .await
    ///  }
    pub async fn add(&self, title: String, key: String) -> crate::Result<GitSshKey> {
        let route = "/user/keys".to_string();

        let params = serde_json::json!({
            "title": title,
            "key": key,
        });
        let response = self.handler.crab._post(route, Some(&params)).await?;
        if response.status() != http::StatusCode::CREATED {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }

        <GitSshKey>::from_response(crate::map_github_error(response).await?).await
    }

    ///## Delete a public SSH key for the authenticated user
    /// OAuth app tokens and personal access tokens (classic) need the `admin:public_key` scope
    ///
    ///works with the following fine-grained token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token must have the following permission set:
    ///* "Git SSH keys" user permissions (write)
    ///
    ///```no_run
    /// use octocrab::Result;
    ///  async fn run() -> Result<()> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .git_ssh_keys()
    ///        .delete(42)
    ///        .await
    ///  }
    pub async fn delete(&self, git_ssh_key_id: u64) -> crate::Result<()> {
        let route = format!("/user/keys/{git_ssh_key_id}");

        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if response.status() != http::StatusCode::NO_CONTENT {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }

        Ok(())
    }

    ///## Get a public SSH key for the authenticated user
    ///
    ///OAuth app tokens and personal access tokens (classic) need the `read:public_key` scope to use this method.
    ///
    ///works with the following token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token must have the following permission set:
    ///* "Git SSH keys" user permissions (read)
    ///
    ///```no_run
    ///  use octocrab::models::GitSshKey;
    /// use octocrab::Result;
    ///  async fn run() -> Result<GitSshKey> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .git_ssh_keys()
    ///        .get(42)
    ///        .await
    ///  }
    pub async fn get(&self, git_ssh_key_id: u64) -> crate::Result<GitSshKey> {
        let route = format!("/user/keys/{git_ssh_key_id}");
        self.handler.crab.get(route, None::<&()>).await
    }

    ///## List public keys for a user
    ///
    ///Lists the verified public SSH keys for a user. This is accessible by anyone.
    ///
    ///See: [GitHub API Documentation][docs] for `GET /users/{username}/keys`
    ///
    ///[docs]: https://docs.github.com/en/rest/users/keys?apiVersion=2022-11-28#list-public-keys-for-a-user
    pub async fn list_user_git_ssh_keys(&self) -> crate::Result<Page<GitSshKey>> {
        let route = format!("/{}/keys", self.handler.user);
        self.handler.crab.get(route, Some(&self)).await
    }

    /// Sends the request to list public SSH keys for a user.
    pub async fn send(&self) -> crate::Result<Page<GitSshKey>> {
        self.list_user_git_ssh_keys().await
    }
}

/// A builder pattern struct for listing a user's public SSH keys.
///
/// Created by [`UserHandler::list_user_git_ssh_keys`].
pub type ListUserGitSshKeysBuilder<'octo, 'b> = UserGitSshKeysOpsBuilder<'octo, 'b>;
