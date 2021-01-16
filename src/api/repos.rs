//! The repositories API.

pub mod events;
mod file;
pub mod forks;
pub mod releases;
mod status;
mod tags;

use crate::{models, params, Octocrab, Result};
pub use file::UpdateFileBuilder;
pub use releases::ReleasesHandler;
pub use status::CreateStatusBuilder;
pub use tags::ListTagsBuilder;

/// Handler for GitHub's repository API.
///
/// Created with [`Octocrab::repos`].
pub struct RepoHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

impl<'octo> RepoHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo }
    }

    /// Get's a repository's license.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let license = octocrab::instance().repos("owner", "repo").license().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn license(&self) -> Result<models::repos::Content> {
        let url = format!(
            "/repos/{owner}/{repo}/license",
            owner = self.owner,
            repo = self.repo,
        );

        self.crab.get(url, None::<&()>).await
    }

    /// Fetches a single reference in the Git database.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params::repos::Reference;
    ///
    /// let master = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .get_ref(&Reference::Branch("master".to_string()))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_ref(
        &self,
        reference: &params::repos::Reference,
    ) -> Result<models::repos::Ref> {
        let url = format!(
            "/repos/{owner}/{repo}/git/ref/{reference}",
            owner = self.owner,
            repo = self.repo,
            reference = reference.ref_url(),
        );
        self.crab.get(url, None::<&()>).await
    }

    /// Creates a new reference for the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let master_sha = "";
    /// use octocrab::params::repos::Reference;
    ///
    /// // Given the SHA of the master branch, creates a 1.0 tag (🎉)
    /// octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .create_ref(&Reference::Tag("1.0".to_string()), master_sha)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_ref(
        &self,
        reference: &params::repos::Reference,
        sha: impl Into<String>,
    ) -> Result<models::repos::Ref> {
        let url = format!(
            "/repos/{owner}/{repo}/git/refs",
            owner = self.owner,
            repo = self.repo,
        );
        self.crab
            .post(
                url,
                Some(&serde_json::json!({
                    "ref": reference.full_ref_url(),
                    "sha": sha.into(),
                })),
            )
            .await
    }

    /// Creates a new file in the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::models::repos::GitUser;
    ///
    /// // Commit to add "crabs/ferris.txt"
    /// octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .create_file(
    ///         "crabs/ferris.txt",
    ///         "Created ferris.txt",
    ///         "Thought there’d never be a Rust Rap?\n"
    ///     )
    ///     .branch("master")
    ///     .commiter(GitUser {
    ///         name: "Octocat".to_string(),
    ///         email: "octocat@github.com".to_string(),
    ///     })
    ///     .author(GitUser {
    ///         name: "Ferris".to_string(),
    ///         email: "ferris@rust-lang.org".to_string(),
    ///     })
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_file(
        &self,
        path: impl Into<String>,
        message: impl Into<String>,
        content: impl AsRef<[u8]>,
    ) -> UpdateFileBuilder<'_, '_> {
        UpdateFileBuilder::new(
            self,
            path.into(),
            message.into(),
            base64::encode(content),
            None,
        )
    }

    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let blob_sha = "";
    /// use octocrab::models::repos::GitUser;
    ///
    /// // Given the file blob for "crabs/ferris.txt", commit to update the file.
    /// octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .update_file(
    ///         "crabs/ferris.txt",
    ///         "Updated ferris.txt",
    ///         "But me and Ferris Crab: best friends to the end.\n",
    ///         blob_sha
    ///     )
    ///     .branch("master")
    ///     .commiter(GitUser {
    ///         name: "Octocat".to_string(),
    ///         email: "octocat@github.com".to_string(),
    ///     })
    ///     .author(GitUser {
    ///         name: "Ferris".to_string(),
    ///         email: "ferris@rust-lang.org".to_string(),
    ///     })
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update_file(
        &self,
        path: impl Into<String>,
        message: impl Into<String>,
        content: impl AsRef<[u8]>,
        sha: impl Into<String>,
    ) -> UpdateFileBuilder<'_, '_> {
        UpdateFileBuilder::new(
            self,
            path.into(),
            message.into(),
            base64::encode(content),
            Some(sha.into()),
        )
    }

    /// List tags from a repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let tags = octocrab::instance().repos("owner", "repo").list_tags().send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_tags(&self) -> ListTagsBuilder<'_, '_> {
        ListTagsBuilder::new(self)
    }

    /// Creates a `ReleasesHandler` for the specified repository.
    pub fn releases(&self) -> releases::ReleasesHandler<'_, '_> {
        releases::ReleasesHandler::new(self)
    }

    /// Create a status for a specified commit in the specified repository.
    pub fn create_status(&self, sha: String, state: models::StatusState) -> CreateStatusBuilder {
        CreateStatusBuilder::new(self, sha, state)
    }

    /// List events on this repository.
    ///
    /// Takes an optional etag which allows for efficient polling. Here is a quick example to poll a
    /// repositories events.
    /// ```no_run
    /// # use std::convert::TryFrom;
    /// # use octocrab::{models::events::Event, etag::{Etagged,EntityTag}, Page};
    /// # async fn run() -> octocrab::Result<()> {
    /// let mut etag = None;
    /// loop {
    ///     let response: Etagged<Page<Event>> = octocrab::instance()
    ///         .repos("owner", "repo")
    ///         .events()
    ///         .etag(etag)
    ///         .send()
    ///         .await?;
    ///     if let Some(page) = response.value {
    ///         // do something with the page ...
    ///     } else {
    ///         println!("No new data received, trying again soon");
    ///     }
    ///     etag = response.etag;
    ///     // add a delay before the next iteration
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn events(&self) -> events::ListRepoEventsBuilder<'_, '_> {
        events::ListRepoEventsBuilder::new(self)
    }

    /// Gets the combined status for the specified reference.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params::repos::Reference;
    ///
    /// let master = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .combined_status_for_ref(&Reference::Branch("main".to_string()))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn combined_status_for_ref(
        &self,
        reference: &params::repos::Reference,
    ) -> Result<models::CombinedStatus> {
        let url = format!(
            "/repos/{owner}/{repo}/commits/{reference}/status",
            owner = self.owner,
            repo = self.repo,
            reference = reference.ref_url(),
        );
        self.crab.get(url, None::<&()>).await
    }

    /// Deletes this repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance().repos("owner", "repo").delete().await
    /// # }
    /// ```
    pub async fn delete(self) -> Result<()> {
        let url = format!("/repos/{owner}/{repo}", owner = self.owner, repo = self.repo);
        crate::map_github_error(self.crab._delete(self.crab.absolute_url(url)?, None::<&()>).await?)
            .await
            .map(drop)
    }
}
