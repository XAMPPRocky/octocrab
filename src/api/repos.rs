//! The repositories API.

use bytes::Bytes;
use http::header::ACCEPT;
use http::request::Builder;
use http::Uri;
use http_body_util::combinators::BoxBody;
use snafu::ResultExt;

mod branches;
mod collaborators;
mod commits;
mod contributors;
pub mod events;
mod file;
pub mod forks;
mod generate;
mod merges;
mod pulls;
pub mod releases;
mod secrets;
mod stargazers;
mod status;
mod tags;
mod teams;

use crate::error::HttpSnafu;
use crate::models::commits::GitCommitObject;
use crate::models::repos;
use crate::repos::file::GetReadmeBuilder;
use crate::{models, params, Octocrab, Result};
pub use branches::ListBranchesBuilder;
pub use collaborators::ListCollaboratorsBuilder;
pub use commits::ListCommitsBuilder;
pub use contributors::ListContributorsBuilder;
pub use file::{DeleteFileBuilder, GetContentBuilder, UpdateFileBuilder};
pub use generate::GenerateRepositoryBuilder;
pub use merges::MergeBranchBuilder;
pub use pulls::ListPullsBuilder;
pub use releases::ReleasesHandler;
pub use secrets::RepoSecretsHandler;
pub use stargazers::ListStarGazersBuilder;
pub use status::{CreateStatusBuilder, ListStatusesBuilder};
pub use tags::ListTagsBuilder;
pub use teams::ListTeamsBuilder;

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
        let route = format!(
            "/repos/{owner}/{repo}/license",
            owner = self.owner,
            repo = self.repo,
        );

        self.crab.get(route, None::<&()>).await
    }

    /// Get's a repository's public key.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let public_key = octocrab::instance().repos("owner", "repo").public_key().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn public_key(&self) -> Result<models::PublicKey> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/secrets/public-key",
            owner = self.owner,
            repo = self.repo,
        );

        self.crab.get(route, None::<&()>).await
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
        let route = format!(
            "/repos/{owner}/{repo}",
            owner = self.owner,
            repo = self.repo,
        );
        self.crab.get(route, None::<&()>).await
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
        let route = format!(
            "/repos/{owner}/{repo}/community/profile",
            owner = self.owner,
            repo = self.repo,
        );
        self.crab.get(route, None::<&()>).await
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
        let route = format!(
            "/repos/{owner}/{repo}/git/ref/{reference}",
            owner = self.owner,
            repo = self.repo,
            reference = reference.ref_url(),
        );
        self.crab.get(route, None::<&()>).await
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
        let route = format!(
            "/repos/{owner}/{repo}/git/tags/{tag_sha}",
            owner = self.owner,
            repo = self.repo,
            tag_sha = tag_sha.into(),
        );
        self.crab.get(route, None::<&()>).await
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
        let route = format!(
            "/repos/{owner}/{repo}/git/refs",
            owner = self.owner,
            repo = self.repo,
        );
        self.crab
            .post(
                route,
                Some(&serde_json::json!({
                    "ref": reference.full_ref_url(),
                    "sha": sha.into(),
                })),
            )
            .await
    }

    /// Deletes an existing reference from the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let master_sha = "";
    /// use octocrab::params::repos::Reference;
    ///
    /// // Deletes the "heads/temporary-branch" reference.
    /// octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .delete_ref(&Reference::Branch("temporary-branch".to_string()))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_ref(&self, reference: &params::repos::Reference) -> Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/git/refs/{ref}",
            owner = self.owner,
            repo = self.repo,
            ref = reference.ref_url()
        );
        crate::map_github_error(self.crab._delete(route, None::<&()>).await?)
            .await
            .map(drop)
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

    /// Get repository readme.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    ///
    /// octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .get_readme()
    ///     .path("path/to/file")
    ///     .r#ref("main")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_readme(&self) -> GetReadmeBuilder<'_, '_> {
        GetReadmeBuilder::new(self)
    }

    /// Creates a new file in the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::models::repos::CommitAuthor;
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
    ///     .commiter(CommitAuthor {
    ///         name: "Octocat".to_string(),
    ///         email: "octocat@github.com".to_string(),
    ///         date: None,
    ///     })
    ///     .author(CommitAuthor {
    ///         name: "Ferris".to_string(),
    ///         email: "ferris@rust-lang.org".to_string(),
    ///         date: None,
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
        use base64::Engine;
        UpdateFileBuilder::new(
            self,
            path.into(),
            message.into(),
            base64::prelude::BASE64_STANDARD.encode(content),
            None,
        )
    }

    /// Update an existing file.
    ///
    /// - `path`: the path of the updated file.
    /// - `message`: the message of the commit used to update the file
    /// - `content`: the updated contents of the file (base64 encoding is done
    ///   automatically).
    /// - `sha`: the blob SHA of the file being updated. This can be obtained
    ///   using the [RepoHandler::get_content] function.
    ///
    /// [GitHub API documentation](https://docs.github.com/en/rest/repos/contents?apiVersion=2022-11-28#create-or-update-file-contents)
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let blob_sha = "";
    /// use octocrab::models::repos::CommitAuthor;
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
    ///     .commiter(CommitAuthor {
    ///         name: "Octocat".to_string(),
    ///         email: "octocat@github.com".to_string(),
    ///         date: None,
    ///     })
    ///     .author(CommitAuthor {
    ///         name: "Ferris".to_string(),
    ///         email: "ferris@rust-lang.org".to_string(),
    ///         date: None,
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
        use base64::Engine;
        UpdateFileBuilder::new(
            self,
            path.into(),
            message.into(),
            base64::prelude::BASE64_STANDARD.encode(content),
            Some(sha.into()),
        )
    }

    /// Deletes a file in the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let blob_sha = "";
    /// use octocrab::models::repos::CommitAuthor;
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
    ///     .commiter(CommitAuthor {
    ///         name: "Octocat".to_string(),
    ///         email: "octocat@github.com".to_string(),
    ///         date: None,
    ///     })
    ///     .author(CommitAuthor {
    ///         name: "Ferris".to_string(),
    ///         email: "ferris@rust-lang.org".to_string(),
    ///         date: None,
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

    /// List teams from a repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let teams = octocrab::instance().repos("owner", "repo").list_teams().send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_teams(&self) -> ListTeamsBuilder<'_, '_> {
        ListTeamsBuilder::new(self)
    }

    /// List collaborators from a repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let collaborators = octocrab::instance().repos("owner", "repo").list_collaborators().send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_collaborators(&self) -> ListCollaboratorsBuilder<'_, '_> {
        ListCollaboratorsBuilder::new(self)
    }

    /// List contributors from a repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let contributors = octocrab::instance().repos("owner", "repo").list_contributors().send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_contributors(&self) -> ListContributorsBuilder<'_, '_> {
        ListContributorsBuilder::new(self)
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

    /// Lists languages for the specified repository.
    /// The value shown for each language is the number of bytes of code written in that language.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    ///
    /// // Get the languages used in the repository
    /// let languages = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .list_languages()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_languages(&self) -> Result<models::repos::Languages> {
        let route = format!(
            "/repos/{owner}/{repo}/languages",
            owner = self.owner,
            repo = self.repo,
        );
        self.crab.get(route, None::<&()>).await
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
        let route = format!(
            "/repos/{owner}/{repo}/commits/{reference}/status",
            owner = self.owner,
            repo = self.repo,
            reference = reference.ref_url(),
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Creates a new repository from repository if it is a template.
    /// ```no_run
    /// # use http::Response;
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
    ) -> Result<http::Response<BoxBody<Bytes, crate::Error>>> {
        let route = format!(
            "/repos/{owner}/{repo}/contents/{path}",
            owner = self.owner,
            repo = self.repo,
            path = path.as_ref(),
        );

        let uri = self
            .crab
            .parameterized_uri(route, Some(&[("ref", &reference.into().0)]))?;
        let request = Builder::new()
            .uri(uri)
            .method(http::Method::GET)
            .header(ACCEPT, "application/vnd.github.v3.raw");
        self.crab
            .execute(self.crab.build_request(request, None::<&()>)?)
            .await
    }

    /// Deletes this repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance().repos("owner", "repo").delete().await
    /// # }
    /// ```
    pub async fn delete(self) -> Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}",
            owner = self.owner,
            repo = self.repo
        );
        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        crate::map_github_error(self.crab._delete(uri, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Stream the repository contents as a .tar.gz
    pub async fn download_tarball(
        &self,
        reference: impl Into<params::repos::Commitish>,
    ) -> Result<http::Response<BoxBody<Bytes, crate::Error>>> {
        let route = format!(
            "/repos/{owner}/{repo}/tarball/{reference}",
            owner = self.owner,
            repo = self.repo,
            reference = reference.into(),
        );
        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        self.crab
            .follow_location_to_data(self.crab._get(uri).await?)
            .await
    }

    /// Check if a user is a repository collaborator
    pub async fn is_collaborator(&self, username: impl AsRef<str>) -> Result<bool> {
        let route = format!(
            "/repos/{owner}/{repo}/collaborators/{username}",
            owner = self.owner,
            repo = self.repo,
            username = username.as_ref(),
        );
        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;

        let response = self.crab._get(uri).await?;
        Ok(response.status().is_success())
    }

    /// Merges `head` into the `base` branch.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    ///
    /// // Merges a feature branch into the master branch.
    /// octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .merge("feature", "master")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn merge(
        &self,
        head: impl Into<String>,
        base: impl Into<String>,
    ) -> MergeBranchBuilder<'octo, '_> {
        MergeBranchBuilder::new(self, head, base)
    }

    /// Handle secrets on the repository
    pub fn secrets(&self) -> RepoSecretsHandler<'_> {
        RepoSecretsHandler::new(self)
    }

    /// Creates a new Git commit object.
    /// See https://docs.github.com/en/rest/git/commits?apiVersion=2022-11-28#create-a-commit
    /// ```no_run
    /// # use octocrab::models::commits::GitCommitObject;
    /// use octocrab::models::repos::CommitAuthor;
    ///  async fn run() -> octocrab::Result<(GitCommitObject)> {
    ///
    /// let git_commit_object = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .create_git_commit_object("message", "tree")
    ///     .signature("signature")
    ///     .author(CommitAuthor{
    ///             name: "name".to_owned(),
    ///             email: "email".to_owned(),
    ///             date: None
    ///         })
    ///     .send()
    ///     .await;
    /// #   git_commit_object
    /// # }
    /// ```
    pub fn create_git_commit_object(
        &self,
        message: impl Into<String>,
        tree: impl Into<String>,
    ) -> CreateGitCommitObjectBuilder<'_, '_> {
        CreateGitCommitObjectBuilder::new(
            self,
            self.owner.clone(),
            self.repo.clone(),
            message.into().to_owned(),
            tree.into().to_owned(),
        )
    }
}

#[derive(serde::Serialize)]
pub struct CreateGitCommitObjectBuilder<'octo, 'req> {
    #[serde(skip)]
    handler: &'octo RepoHandler<'req>,
    owner: String,
    repo: String,
    message: String,
    tree: String,
    parents: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<repos::CommitAuthor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    committer: Option<repos::CommitAuthor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<String>,
}

impl<'octo, 'req> CreateGitCommitObjectBuilder<'octo, 'req> {
    pub(crate) fn new(
        handler: &'octo RepoHandler<'req>,
        owner: String,
        repo: String,
        message: String,
        tree: String,
    ) -> Self {
        Self {
            handler,
            owner,
            repo,
            message,
            tree,
            parents: Vec::new(),
            author: None,
            committer: None,
            signature: None,
        }
    }

    /// The author of the commit.
    pub fn author(mut self, author: impl Into<repos::CommitAuthor>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// The committer of the commit.
    pub fn committer(mut self, committer: impl Into<repos::CommitAuthor>) -> Self {
        self.committer = Some(committer.into());
        self
    }

    /// The signature of the commit.
    pub fn signature(mut self, signature: impl Into<String>) -> Self {
        self.signature = Some(signature.into());
        self
    }

    /// The parents of the commit.
    pub fn parents(mut self, parents: Vec<String>) -> Self {
        self.parents = parents;
        self
    }

    /// Sends the request
    pub async fn send(&self) -> Result<GitCommitObject> {
        let route = format!(
            "/repos/{owner}/{repo}/git/commits",
            owner = self.owner,
            repo = self.repo,
        );
        self.handler.crab.post(route, Some(&self)).await
    }
}
