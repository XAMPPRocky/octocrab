//! The repositories API.

mod file;

pub use file::UpdateFileBuilder;

use crate::{models, params, Octocrab, Result};

/// Handler for GitHub's repository API.
///
/// Created with [`Octocrab::repos`].
///
/// [`Octocrab::repos`]: ../struct.Octocrab.html#method.repos
pub struct RepoHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

impl<'octo> RepoHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo }
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
    pub async fn get_ref(&self, reference: &params::repos::Reference) -> Result<models::Ref> {
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
    ) -> Result<models::Ref> {
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
    /// use octocrab::models::AuthorUser;
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
    ///     .commiter(AuthorUser {
    ///         name: "Octocat".to_string(),
    ///         email: "octocat@github.com".to_string(),
    ///     })
    ///     .author(AuthorUser {
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
    /// use octocrab::models::AuthorUser;
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
    ///     .commiter(AuthorUser {
    ///         name: "Octocat".to_string(),
    ///         email: "octocat@github.com".to_string(),
    ///     })
    ///     .author(AuthorUser {
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

    /// Creates a new `PullRequestHandler` for the repository,
    /// that allows you to access GitHub's pull request API.
    pub fn pulls(&self) -> super::pulls::PullRequestHandler {
        super::pulls::PullRequestHandler::new(self.crab, self.owner.clone(), self.repo.clone())
    }
}
