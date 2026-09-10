//! The code scanning API.
use crate::{models, params, Octocrab, Result};

mod list;
mod update;

/// Handler for GitHub's code scanning API.
///
/// Created with [`Octocrab::code_scannings`].
pub struct CodeScanningHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: Option<String>,
}

impl<'octo> CodeScanningHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: Option<String>) -> Self {
        Self { crab, owner, repo }
    }

    /// Gets an code scanning from the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let code_scanning = octocrab.code_scannings("owner", "repo").get(3).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&mut self, number: u64) -> Result<models::code_scannings::CodeScanningAlert> {
        let route = format!(
            "/repos/{owner}/{repo}/code-scanning/alerts/{number}",
            owner = self.owner,
            repo = self.repo.as_mut().expect("Repository must be specified"),
            number = number,
        );

        self.crab.get(route, None::<&()>).await
    }

    /// List code scannings in the repository.
    pub fn list(&self) -> list::ListCodeScanningsBuilder<'_, '_> {
        list::ListCodeScanningsBuilder::new(self)
    }

    /// Update a code scanning alert
    /// ```no_run
    /// # use octocrab::params;
    ///
    ///  async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::models;
    ///
    /// let issue = octocrab.code_scannings("owner", "repo")
    ///     .update(1234u64)
    ///     .state(params::AlertState::Dismissed)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(&self, number: u64) -> update::UpdateCodeScanningBuilder<'_, '_> {
        update::UpdateCodeScanningBuilder::new(self, number)
    }

    /// Get a code scanning analysis for a repository.
    ///
    /// See: https://docs.github.com/en/rest/code-scanning/code-scanning#get-a-code-scanning-analysis-for-a-repository
    pub async fn get_analysis(&self, analysis_id: u64) -> Result<models::code_scannings::Analysis> {
        let repo = self.repo.as_ref().expect("Repository must be specified");
        let route = format!(
            "/repos/{}/{}/code-scanning/analyses/{}",
            self.owner, repo, analysis_id
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Delete a code scanning analysis from a repository.
    ///
    /// See: https://docs.github.com/en/rest/code-scanning/code-scanning#delete-a-code-scanning-analysis-from-a-repository
    pub async fn delete_analysis(
        &self,
        analysis_id: u64,
        confirm_delete: Option<impl Into<String>>,
    ) -> Result<models::code_scannings::DeleteAnalysisResponse> {
        let repo = self.repo.as_ref().expect("Repository must be specified");
        let route = match confirm_delete {
            Some(cd) => format!(
                "/repos/{}/{}/code-scanning/analyses/{}?confirm_delete={}",
                self.owner,
                repo,
                analysis_id,
                cd.into()
            ),
            None => format!(
                "/repos/{}/{}/code-scanning/analyses/{}",
                self.owner, repo, analysis_id
            ),
        };
        self.crab.delete(route, None::<&()>).await
    }

    /// List CodeQL databases for a repository.
    ///
    /// See: https://docs.github.com/en/rest/code-scanning/code-scanning#list-codeql-databases-for-a-repository
    pub async fn list_codeql_databases(
        &self,
    ) -> Result<Vec<models::code_scannings::CodeqlDatabase>> {
        let repo = self.repo.as_ref().expect("Repository must be specified");
        let route = format!(
            "/repos/{}/{}/code-scanning/codeql/databases",
            self.owner, repo
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Get a CodeQL database for a language in a repository.
    ///
    /// See: https://docs.github.com/en/rest/code-scanning/code-scanning#get-a-codeql-database-for-a-repository
    pub async fn get_codeql_database(
        &self,
        language: impl AsRef<str>,
    ) -> Result<models::code_scannings::CodeqlDatabase> {
        let repo = self.repo.as_ref().expect("Repository must be specified");
        let route = format!(
            "/repos/{}/{}/code-scanning/codeql/databases/{}",
            self.owner,
            repo,
            language.as_ref()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Delete a CodeQL database for a language in a repository.
    ///
    /// See: https://docs.github.com/en/rest/code-scanning/code-scanning#delete-a-codeql-database
    pub async fn delete_codeql_database(&self, language: impl AsRef<str>) -> Result<()> {
        let repo = self.repo.as_ref().expect("Repository must be specified");
        let route = format!(
            "/repos/{}/{}/code-scanning/codeql/databases/{}",
            self.owner,
            repo,
            language.as_ref()
        );
        crate::map_github_error(self.crab._delete(route, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Get a code scanning default setup configuration for a repository.
    ///
    /// See: https://docs.github.com/en/rest/code-scanning/code-scanning#get-a-code-scanning-default-setup-configuration
    pub async fn get_default_setup(&self) -> Result<models::code_scannings::DefaultSetup> {
        let repo = self.repo.as_ref().expect("Repository must be specified");
        let route = format!("/repos/{}/{}/code-scanning/default-setup", self.owner, repo);
        self.crab.get(route, None::<&()>).await
    }

    /// Update a code scanning default setup configuration for a repository.
    ///
    /// See: https://docs.github.com/en/rest/code-scanning/code-scanning#update-a-code-scanning-default-setup-configuration
    pub async fn update_default_setup(
        &self,
        body: &models::code_scannings::UpdateDefaultSetup,
    ) -> Result<models::code_scannings::UpdateDefaultSetupResponse> {
        let repo = self.repo.as_ref().expect("Repository must be specified");
        let route = format!("/repos/{}/{}/code-scanning/default-setup", self.owner, repo);
        self.crab.patch(route, Some(body)).await
    }

    /// Upload an analysis as SARIF data.
    ///
    /// See: https://docs.github.com/en/rest/code-scanning/code-scanning#upload-an-analysis-as-sarif-data
    pub async fn upload_sarif(
        &self,
        body: &models::code_scannings::UploadSarif,
    ) -> Result<models::code_scannings::SarifReceipt> {
        let repo = self.repo.as_ref().expect("Repository must be specified");
        let route = format!("/repos/{}/{}/code-scanning/sarifs", self.owner, repo);
        self.crab.post(route, Some(body)).await
    }

    /// Get information about a SARIF upload.
    ///
    /// See: https://docs.github.com/en/rest/code-scanning/code-scanning#get-information-about-a-sarif-upload
    pub async fn get_sarif(
        &self,
        sarif_id: impl AsRef<str>,
    ) -> Result<models::code_scannings::SarifAnalysis> {
        let repo = self.repo.as_ref().expect("Repository must be specified");
        let route = format!(
            "/repos/{}/{}/code-scanning/sarifs/{}",
            self.owner,
            repo,
            sarif_id.as_ref()
        );
        self.crab.get(route, None::<&()>).await
    }
}
