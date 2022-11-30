//! The repositories API.

use reqwest::header::ACCEPT;

mod branches;
mod commits;
pub mod events;
mod file;
pub mod forks;
mod generate;
mod pulls;
pub mod releases;
mod stargazers;
mod status;
mod tags;

use crate::{models, params, Octocrab, Result};
pub use commits::ListCommitsBuilder;
pub use file::{GetContentBuilder, UpdateFileBuilder, DeleteFileBuilder};
pub use generate::GenerateRepositoryBuilder;
pub use pulls::ListPullsBuilder;
pub use releases::ReleasesHandler;
pub use stargazers::ListStarGazersBuilder;
pub use status::{CreateStatusBuilder, ListStatusesBuilder};
pub use tags::ListTagsBuilder;
pub use branches::ListBranchesBuilder;

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
            "repos/{owner}/{repo}/license",
            owner = self.owner,
            repo = self.repo,
        );

        self.crab.get(url, None::<&()>).await
    }

    /// Get's a repository's public key.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let public_key = octocrab::instance().repos("owner", "repo").public_key().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn public_key(&self) -> Result<models::PublicKey> {
        let url = format!(
            "repos/{owner}/{repo}/actions/secrets/public-key",
            owner = self.owner,
            repo = self.repo,
        );

        self.crab.get(url, None::<&()>).await
    }

    /// Fetches a single repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let repo = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .get()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self) -> Result<models::Repository> {
        let url = format!("repos/{owner}/{repo}", owner = self.owner, repo = self.repo,);
        self.crab.get(url, None::<&()>).await
    }

    /// Fetches a repository's metrics.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let repo = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .get_community_profile_metrics()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_community_profile_metrics(&self) -> Result<models::RepositoryMetrics> {
        let url = format!(
            "repos/{owner}/{repo}/community/profile",
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
            "repos/{owner}/{repo}/git/ref/{reference}",
            owner = self.owner,
            repo = self.repo,
            reference = reference.ref_url(),
        );
        self.crab.get(url, None::<&()>).await
    }

    /// Fetches information about a git tag with the given `tag_sha`.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params::repos::Reference;
    ///
    /// let master = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .get_tag("402b2026a41b26b691c429ddb0b9c27a31b27a6b")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_tag(&self, tag_sha: impl Into<String>) -> Result<models::repos::GitTag> {
        let url = format!(
            "repos/{owner}/{repo}/git/tags/{tag_sha}",
            owner = self.owner,
            repo = self.repo,
            tag_sha = tag_sha.into(),
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
            "repos/{owner}/{repo}/git/refs",
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

    /// Get repository content.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    ///
    /// octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .get_content()
    ///     .path("path/to/file")
    ///     .r#ref("main")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_content(&self) -> GetContentBuilder<'_, '_> {
        GetContentBuilder::new(self)
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

    /// Deletes a file in the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let blob_sha = "";
    /// use octocrab::models::repos::GitUser;
    ///
    /// // Commit to delete "crabs/ferris.txt"
    /// octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .delete_file(
    ///         "crabs/ferris.txt",
    ///         "Deleted ferris.txt",
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
    pub fn delete_file(
        &self,
        path: impl Into<String>,
        message: impl Into<String>,
        sha: impl Into<String>,
    ) -> DeleteFileBuilder<'_, '_> {
        DeleteFileBuilder::new(self, path.into(), message.into(), sha.into())
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

    /// List branches from a repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let branches = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .list_branches()
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_branches(&self) -> ListBranchesBuilder<'_, '_> {
        ListBranchesBuilder::new(self)
    }

    /// List commits from a repository
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let commits = octocrab::instance().repos("owner", "repo").list_commits().send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_commits(&self) -> ListCommitsBuilder<'_, '_> {
        ListCommitsBuilder::new(self)
    }

    /// List star_gazers from a repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let stargazers = octocrab::instance().repos("owner", "repo").list_stargazers().send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_stargazers(&self) -> ListStarGazersBuilder<'_, '_> {
        ListStarGazersBuilder::new(self)
    }

    /// Creates a `ReleasesHandler` for the specified repository.
    pub fn releases(&self) -> releases::ReleasesHandler<'_, '_> {
        releases::ReleasesHandler::new(self)
    }

    /// Create a status for a specified commit in the specified repository.
    pub fn create_status(&self, sha: String, state: models::StatusState) -> CreateStatusBuilder {
        CreateStatusBuilder::new(self, sha, state)
    }

    /// List statuses for a reference.
    pub fn list_statuses(&self, sha: String) -> ListStatusesBuilder<'_, '_> {
        ListStatusesBuilder::new(self, sha)
    }

    /// List pull requests for a reference.
    pub fn list_pulls(&self, sha: String) -> ListPullsBuilder<'_, '_> {
        ListPullsBuilder::new(self, sha)
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
            "repos/{owner}/{repo}/commits/{reference}/status",
            owner = self.owner,
            repo = self.repo,
            reference = reference.ref_url(),
        );
        self.crab.get(url, None::<&()>).await
    }

    /// Creates a new repository from repository if it is a template.
    /// ```no_run
    /// # use reqwest::Response;
    ///  async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .generate("rust")
    ///     .owner("new_owner")
    ///     .description("Description")
    ///     .include_all_branches(true)
    ///     .private(true)
    ///     .send()
    ///     .await
    /// # }
    /// ```
    pub fn generate(&self, name: &str) -> GenerateRepositoryBuilder<'_, '_> {
        GenerateRepositoryBuilder::new(self, name)
    }

    /// Retrieve the contents of a file in raw format
    pub async fn raw_file(
        self,
        reference: impl Into<params::repos::Commitish>,
        path: impl AsRef<str>,
    ) -> Result<reqwest::Response> {
        let url = self.crab.absolute_url(format!(
            "repos/{owner}/{repo}/contents/{path}",
            owner = self.owner,
            repo = self.repo,
            path = path.as_ref(),
        ))?;
        let mut request = self.crab.request_builder(url, reqwest::Method::GET);
        request = request.query(&[("ref", &reference.into().0)]);
        request = request.header(ACCEPT, "application/vnd.github.v3.raw");
        self.crab.execute(request).await
    }

    /// Deletes this repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance().repos("owner", "repo").delete().await
    /// # }
    /// ```
    pub async fn delete(self) -> Result<()> {
        let url = format!("repos/{owner}/{repo}", owner = self.owner, repo = self.repo);
        crate::map_github_error(
            self.crab
                ._delete(self.crab.absolute_url(url)?, None::<&()>)
                .await?,
        )
        .await
        .map(drop)
    }

    /// Stream the repository contents as a .tar.gz
    pub async fn download_tarball(
        &self,
        reference: impl Into<params::repos::Commitish>,
    ) -> Result<reqwest::Response> {
        let url = self.crab.absolute_url(format!(
            "repos/{owner}/{repo}/tarball/{reference}",
            owner = self.owner,
            repo = self.repo,
            reference = reference.into(),
        ))?;
        self.crab._get(url, None::<&()>).await
    }
}
