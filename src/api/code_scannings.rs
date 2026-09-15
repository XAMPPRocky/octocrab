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

    /// Gets a code scanning alert from the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#get-a-code-scanning-alert)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
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

    /// List code scannings in the repository or organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#list-code-scanning-alerts-for-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let alerts = octocrab
    ///     .code_scannings("owner", "repo")
    ///     .list()
    ///     .per_page(10)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> list::ListCodeScanningsBuilder<'_, '_> {
        list::ListCodeScanningsBuilder::new(self)
    }

    /// Update a code scanning alert in the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#update-a-code-scanning-alert)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use octocrab::params;
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let alert = octocrab
    ///     .code_scannings("owner", "repo")
    ///     .update(1234u64)
    ///     .state(params::AlertState::Dismissed)
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
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#get-a-code-scanning-analysis-for-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let analysis = octocrab
    ///     .code_scannings("owner", "repo")
    ///     .get_analysis(42)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#delete-a-code-scanning-analysis-from-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let response = octocrab
    ///     .code_scannings("owner", "repo")
    ///     .delete_analysis(42, None::<String>)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#list-codeql-databases-for-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let databases = octocrab
    ///     .code_scannings("owner", "repo")
    ///     .list_codeql_databases()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#get-a-codeql-database-for-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let db = octocrab
    ///     .code_scannings("owner", "repo")
    ///     .get_codeql_database("javascript")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#delete-a-codeql-database)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// octocrab
    ///     .code_scannings("owner", "repo")
    ///     .delete_codeql_database("javascript")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#get-a-code-scanning-default-setup-configuration)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let setup = octocrab
    ///     .code_scannings("owner", "repo")
    ///     .get_default_setup()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_default_setup(&self) -> Result<models::code_scannings::DefaultSetup> {
        let repo = self.repo.as_ref().expect("Repository must be specified");
        let route = format!("/repos/{}/{}/code-scanning/default-setup", self.owner, repo);
        self.crab.get(route, None::<&()>).await
    }

    /// Update a code scanning default setup configuration for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#update-a-code-scanning-default-setup-configuration)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::code_scannings::UpdateDefaultSetup;
    ///
    /// let body = UpdateDefaultSetup::default();
    /// let response = octocrab
    ///     .code_scannings("owner", "repo")
    ///     .update_default_setup(&body)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#upload-an-analysis-as-sarif-data)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::code_scannings::UploadSarif;
    ///
    /// # let body: UploadSarif = todo!();
    /// let receipt = octocrab
    ///     .code_scannings("owner", "repo")
    ///     .upload_sarif(&body)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#get-information-about-a-sarif-upload)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let sarif = octocrab
    ///     .code_scannings("owner", "repo")
    ///     .get_sarif("sarif-id")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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
