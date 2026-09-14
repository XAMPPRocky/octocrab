//! GitHub Git Database API.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/git?apiVersion=2022-11-28)

use crate::api::repos::RepoRef;
use crate::models::commits::GitCommitObject;
use crate::models::git::{CreateTreeEntry, CreatedBlob, GitBlob, GitTree};
use crate::models::repos::{CommitAuthor, GitTag, Ref};
use crate::params::repos::Reference;
use crate::{Octocrab, Result};

/// Handler for GitHub's Git database API.
///
/// Created with [`Octocrab::git`].
pub struct GitHandler<'octo> {
    crab: &'octo Octocrab,
    repo: RepoRef,
}

impl<'octo> GitHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, repo: RepoRef) -> Self {
        Self { crab, repo }
    }

    // -----------------------------------------------------------------------
    // Blobs
    // -----------------------------------------------------------------------

    /// Gets a Git blob from the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/git/blobs?apiVersion=2022-11-28#get-a-blob)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let blob = octocrab
    ///     .git("owner", "repo")
    ///     .get_blob("3a0f86fb8db8eea7ccbb9a95f325ddbedfb25e15")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_blob(&self, file_sha: impl Into<String>) -> Result<GitBlob> {
        let route = format!("/{}/git/blobs/{}", self.repo, file_sha.into());
        self.crab.get(route, None::<&()>).await
    }

    /// Creates a new Git blob in the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/git/blobs?apiVersion=2022-11-28#create-a-blob)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let blob = octocrab
    ///     .git("owner", "repo")
    ///     .create_blob("Hello World")
    ///     .encoding("utf-8")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_blob(&self, content: impl Into<String>) -> CreateBlobBuilder<'octo> {
        CreateBlobBuilder::new(self.crab, self.repo.clone(), content.into())
    }

    // -----------------------------------------------------------------------
    // Commits
    // -----------------------------------------------------------------------

    /// Gets a Git commit object from the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/git/commits?apiVersion=2022-11-28#get-a-commit-object)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let commit = octocrab
    ///     .git("owner", "repo")
    ///     .get_commit("7638417db6d59f3c431d3e1f261cc637155684cd")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_commit(&self, commit_sha: impl Into<String>) -> Result<GitCommitObject> {
        let route = format!("/{}/git/commits/{}", self.repo, commit_sha.into());
        self.crab.get(route, None::<&()>).await
    }

    /// Creates a new Git commit object in the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/git/commits?apiVersion=2022-11-28#create-a-commit)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let commit = octocrab
    ///     .git("owner", "repo")
    ///     .create_commit("commit message", "691272480426f78a0138979dd3ce63b77f706feb")
    ///     .parents(vec!["1acc419d4d6a9ce985db7be48c6349a0475975b5".to_string()])
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_commit(
        &self,
        message: impl Into<String>,
        tree: impl Into<String>,
    ) -> CreateGitCommitObjectBuilder<'octo> {
        CreateGitCommitObjectBuilder::new(self.crab, self.repo.clone(), message.into(), tree.into())
    }

    // -----------------------------------------------------------------------
    // References
    // -----------------------------------------------------------------------

    /// Fetches information about a Git reference.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/git/refs?apiVersion=2022-11-28#get-a-reference)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::params::repos::Reference;
    ///
    /// let master = octocrab
    ///     .git("owner", "repo")
    ///     .get_ref(&Reference::Branch("master".to_string()))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_ref(&self, reference: &Reference) -> Result<Ref> {
        let route = format!(
            "/{repo}/git/ref/{reference}",
            repo = self.repo,
            reference = reference.ref_url(),
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Creates a new Git reference in the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/git/refs?apiVersion=2022-11-28#create-a-reference)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::params::repos::Reference;
    ///
    /// let master = octocrab
    ///     .git("owner", "repo")
    ///     .create_ref(
    ///         &Reference::Tag("1.0".to_string()),
    ///         "c5b97d5ae6c19d5c5df71a34c7fbeeda2479ccbc",
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_ref(&self, reference: &Reference, sha: impl Into<String>) -> Result<Ref> {
        let route = format!("/{}/git/refs", self.repo);
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

    /// Updates a Git reference in the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/git/refs?apiVersion=2022-11-28#update-a-reference)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::params::repos::Reference;
    ///
    /// let updated = octocrab
    ///     .git("owner", "repo")
    ///     .update_ref(
    ///         &Reference::Branch("feature-a".to_string()),
    ///         "aa218f56b14c9653891f9e74264a383fa43fefbd",
    ///     )
    ///     .force(true)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update_ref(
        &self,
        reference: &Reference,
        sha: impl Into<String>,
    ) -> UpdateRefBuilder<'octo> {
        UpdateRefBuilder::new(self.crab, self.repo.clone(), reference.clone(), sha.into())
    }

    /// Deletes an existing Git reference from the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/git/refs?apiVersion=2022-11-28#delete-a-reference)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::params::repos::Reference;
    ///
    /// octocrab
    ///     .git("owner", "repo")
    ///     .delete_ref(&Reference::Branch("temporary-branch".to_string()))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_ref(&self, reference: &Reference) -> Result<()> {
        let route = format!(
            "/{repo}/git/refs/{ref}",
            repo = self.repo,
            ref = reference.ref_url()
        );
        crate::map_github_error(self.crab._delete(route, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Lists Git references that match the supplied sub-string.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/git/refs?apiVersion=2022-11-28#list-matching-references)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let refs = octocrab
    ///     .git("owner", "repo")
    ///     .list_matching_refs("heads")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_matching_refs(&self, reference: impl AsRef<str>) -> Result<Vec<Ref>> {
        let ref_path = reference
            .as_ref()
            .strip_prefix("refs/")
            .unwrap_or(reference.as_ref());
        let route = format!("/{}/git/matching-refs/{}", self.repo, ref_path);
        self.crab.get(route, None::<&()>).await
    }

    // -----------------------------------------------------------------------
    // Tags
    // -----------------------------------------------------------------------

    /// Fetches information about a Git tag with the given `tag_sha`.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/git/tags?apiVersion=2022-11-28#get-a-tag)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let tag = octocrab
    ///     .git("owner", "repo")
    ///     .get_tag("940bd336248efae0f9ee5bc7b2d5c985887b16ac")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_tag(&self, tag_sha: impl Into<String>) -> Result<GitTag> {
        let route = format!(
            "/{repo}/git/tags/{tag_sha}",
            repo = self.repo,
            tag_sha = tag_sha.into(),
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Creates a new Git tag object in the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/git/tags?apiVersion=2022-11-28#create-a-tag-object)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let tag = octocrab
    ///     .git("owner", "repo")
    ///     .create_tag(
    ///         "v0.0.1",
    ///         "initial version",
    ///         "c3d0be41ecbe669545ee3e94d31ed9a4bc91ee3c",
    ///         "commit",
    ///     )
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_tag(
        &self,
        tag: impl Into<String>,
        message: impl Into<String>,
        object: impl Into<String>,
        object_type: impl Into<String>,
    ) -> CreateTagBuilder<'octo> {
        CreateTagBuilder::new(
            self.crab,
            self.repo.clone(),
            tag.into(),
            message.into(),
            object.into(),
            object_type.into(),
        )
    }

    // -----------------------------------------------------------------------
    // Trees
    // -----------------------------------------------------------------------

    /// Gets a Git tree object from the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/git/trees?apiVersion=2022-11-28#get-a-tree)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let tree = octocrab
    ///     .git("owner", "repo")
    ///     .get_tree("9fb037999f264ba9a7fc6274d15fa3ae2ab98312")
    ///     .recursive(true)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_tree(&self, tree_sha: impl Into<String>) -> GetTreeBuilder<'octo> {
        GetTreeBuilder::new(self.crab, self.repo.clone(), tree_sha.into())
    }

    /// Creates a new Git tree object in the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/git/trees?apiVersion=2022-11-28#create-a-tree)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::git::CreateTreeEntry;
    ///
    /// let entry = CreateTreeEntry::new("file.rb", "100644", "blob")
    ///     .with_sha("44b4fc6d56897b048c772eb4087f854f46256132");
    ///
    /// let tree = octocrab
    ///     .git("owner", "repo")
    ///     .create_tree(vec![entry])
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_tree(&self, tree: Vec<CreateTreeEntry>) -> CreateTreeBuilder<'octo> {
        CreateTreeBuilder::new(self.crab, self.repo.clone(), tree)
    }
}

// ===========================================================================
// Builders
// ===========================================================================

#[derive(serde::Serialize)]
pub struct CreateBlobBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    repo: RepoRef,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<String>,
}

impl<'octo> CreateBlobBuilder<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, repo: RepoRef, content: String) -> Self {
        Self {
            crab,
            repo,
            content,
            encoding: None,
        }
    }

    /// The encoding used for content. Currently, "utf-8" and "base64" are supported.
    pub fn encoding(mut self, encoding: impl Into<String>) -> Self {
        self.encoding = Some(encoding.into());
        self
    }

    /// Sends the request.
    pub async fn send(self) -> Result<CreatedBlob> {
        let route = format!("/{}/git/blobs", self.repo);
        self.crab.post(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct CreateGitCommitObjectBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    repo: RepoRef,
    message: String,
    tree: String,
    parents: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<CommitAuthor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    committer: Option<CommitAuthor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<String>,
}

impl<'octo> CreateGitCommitObjectBuilder<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, repo: RepoRef, message: String, tree: String) -> Self {
        Self {
            crab,
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
    pub fn author(mut self, author: impl Into<CommitAuthor>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// The committer of the commit.
    pub fn committer(mut self, committer: impl Into<CommitAuthor>) -> Self {
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

    /// Sends the request.
    pub async fn send(&self) -> Result<GitCommitObject> {
        let route = format!("/{}/git/commits", self.repo);
        self.crab.post(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct UpdateRefBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    repo: RepoRef,
    #[serde(skip)]
    reference: Reference,
    sha: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
}

impl<'octo> UpdateRefBuilder<'octo> {
    pub(crate) fn new(
        crab: &'octo Octocrab,
        repo: RepoRef,
        reference: Reference,
        sha: String,
    ) -> Self {
        Self {
            crab,
            repo,
            reference,
            sha,
            force: None,
        }
    }

    /// Indicates whether to force the update or to make sure the update is a fast-forward update.
    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }

    /// Sends the request.
    pub async fn send(self) -> Result<Ref> {
        let route = format!(
            "/{repo}/git/refs/{ref}",
            repo = self.repo,
            ref = self.reference.ref_url()
        );
        self.crab.patch(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct CreateTagBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    repo: RepoRef,
    tag: String,
    message: String,
    object: String,
    r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tagger: Option<CommitAuthor>,
}

impl<'octo> CreateTagBuilder<'octo> {
    pub(crate) fn new(
        crab: &'octo Octocrab,
        repo: RepoRef,
        tag: String,
        message: String,
        object: String,
        object_type: String,
    ) -> Self {
        Self {
            crab,
            repo,
            tag,
            message,
            object,
            r#type: object_type,
            tagger: None,
        }
    }

    /// Information about the individual creating the tag.
    pub fn tagger(mut self, tagger: impl Into<CommitAuthor>) -> Self {
        self.tagger = Some(tagger.into());
        self
    }

    /// Sends the request.
    pub async fn send(self) -> Result<GitTag> {
        let route = format!("/{}/git/tags", self.repo);
        self.crab.post(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
struct GetTreeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    recursive: Option<u8>,
}

pub struct GetTreeBuilder<'octo> {
    crab: &'octo Octocrab,
    repo: RepoRef,
    tree_sha: String,
    recursive: Option<u8>,
}

impl<'octo> GetTreeBuilder<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, repo: RepoRef, tree_sha: String) -> Self {
        Self {
            crab,
            repo,
            tree_sha,
            recursive: None,
        }
    }

    /// Setting this parameter to true returns the objects or subtrees referenced by the tree specified in tree_sha.
    pub fn recursive(mut self, recursive: bool) -> Self {
        self.recursive = if recursive { Some(1) } else { None };
        self
    }

    /// Sends the request.
    pub async fn send(self) -> Result<GitTree> {
        let route = format!("/{}/git/trees/{}", self.repo, self.tree_sha);
        let params = GetTreeParams {
            recursive: self.recursive,
        };
        self.crab.get(route, Some(&params)).await
    }
}

#[derive(serde::Serialize)]
pub struct CreateTreeBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    repo: RepoRef,
    tree: Vec<CreateTreeEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_tree: Option<String>,
}

impl<'octo> CreateTreeBuilder<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, repo: RepoRef, tree: Vec<CreateTreeEntry>) -> Self {
        Self {
            crab,
            repo,
            tree,
            base_tree: None,
        }
    }

    /// The SHA1 of an existing Git tree object which will be used as the base for the new tree.
    pub fn base_tree(mut self, base_tree: impl Into<String>) -> Self {
        self.base_tree = Some(base_tree.into());
        self
    }

    /// Sends the request.
    pub async fn send(self) -> Result<GitTree> {
        let route = format!("/{}/git/trees", self.repo);
        self.crab.post(route, Some(&self)).await
    }
}
