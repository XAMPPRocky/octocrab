//! The commit API.
mod associated_check_runs;
mod associated_pull_requests;
mod compare_commit;
mod create_comment;

pub use self::create_comment::CreateCommentBuilder;
use crate::params::repos::Reference;
use crate::{models, Octocrab, Result};
pub use associated_pull_requests::PullRequestTarget;
pub use compare_commit::CompareCommitsBuilder;

pub struct CommitHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

impl<'octo> CommitHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo }
    }

    // pub fn create(&self, title: impl Into<String>) -> create::CreateIssueBuilder<'_, '_> {
    //     create::CreateIssueBuilder::new(self, title.into())
    // }

    /// Compares two commits.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/commits/commits?apiVersion=2022-11-28#compare-two-commits)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let comparison = octocrab
    ///     .commits("owner", "repo")
    ///     .compare("base", "head")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn compare(
        &self,
        base: impl Into<String>,
        head: impl Into<String>,
    ) -> compare_commit::CompareCommitsBuilder<'_, '_> {
        compare_commit::CompareCommitsBuilder::new(self, base.into(), head.into())
    }

    /// Compares two commits with a range string such as `"base...head"`.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/commits/commits?apiVersion=2022-11-28#compare-two-commits)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let comparison = octocrab
    ///     .commits("owner", "repo")
    ///     .compare_range("base...head")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn compare_range(
        &self,
        basehead: impl Into<String>,
    ) -> compare_commit::CompareCommitsBuilder<'_, '_> {
        compare_commit::CompareCommitsBuilder::new_range(self, basehead.into())
    }

    /// List branches where the given commit is the HEAD commit.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/commits/commits?apiVersion=2022-11-28#list-branches-for-head-commit)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let branches = octocrab
    ///     .commits("owner", "repo")
    ///     .branches_where_head("6dcb09b5b57875f334f61aebed695e2e4193db5e")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn branches_where_head(
        &self,
        commit_sha: impl AsRef<str>,
    ) -> Result<Vec<models::repos::Branch>> {
        let route = format!(
            "/repos/{owner}/{repo}/commits/{sha}/branches-where-head",
            owner = self.owner,
            repo = self.repo,
            sha = commit_sha.as_ref()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Lists check runs for a commit reference.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/checks/runs?apiVersion=2022-11-28#list-check-runs-for-a-git-reference)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::params::repos::Reference;
    ///
    /// let check_runs = octocrab
    ///     .commits("owner", "repo")
    ///     .associated_check_runs(Reference::Branch("main".to_string()))
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn associated_check_runs(
        &self,
        reference: impl Into<Reference>,
    ) -> associated_check_runs::AssociatedCheckRunsBuilder<'_, '_> {
        associated_check_runs::AssociatedCheckRunsBuilder::new(self, reference)
    }

    /// Lists pull requests associated with a commit SHA or reference.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/commits/commits?apiVersion=2022-11-28#list-pull-requests-associated-with-a-commit)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::commits::PullRequestTarget;
    ///
    /// let prs = octocrab
    ///     .commits("owner", "repo")
    ///     .associated_pull_requests(PullRequestTarget::Sha("6dcb09b5b57875f334f61aebed695e2e4193db5e".to_string()))
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn associated_pull_requests(
        &self,
        target: PullRequestTarget,
    ) -> associated_pull_requests::AssociatedPullRequestsBuilder<'_, '_> {
        associated_pull_requests::AssociatedPullRequestsBuilder::new(self, target)
    }

    /// Creates a comment for a commit.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/commits/comments?apiVersion=2022-11-28#create-a-commit-comment)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let comment = octocrab
    ///     .commits("owner", "repo")
    ///     .create_comment("6dcb09b5b57875f334f61aebed695e2e4193db5e", "Great commit!")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_comment(
        &self,
        sha: impl Into<String>,
        body: impl Into<String>,
    ) -> create_comment::CreateCommentBuilder<'_, '_> {
        create_comment::CreateCommentBuilder::new(self, sha.into(), body.into())
    }

    /// Gets a commit from the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/commits/commits?apiVersion=2022-11-28#get-a-commit)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let commit = octocrab
    ///     .commits("owner", "repo")
    ///     .get("6dcb09b5b57875f334f61aebed695e2e4193db5e")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, reference: impl Into<String>) -> Result<models::repos::RepoCommit> {
        let route = format!(
            "/repos/{owner}/{repo}/commits/{reference}",
            owner = self.owner,
            repo = self.repo,
            reference = reference.into(),
        );
        self.crab.get(route, None::<&()>).await
    }
}
