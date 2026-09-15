use super::RepoHandler;
use crate::models::code_scannings::{
    AlertInstance, Analysis, CodeScanningAlert, CodeqlDatabase, DefaultSetup,
    DeleteAnalysisResponse, SarifAnalysis, SarifReceipt, UpdateDefaultSetup,
    UpdateDefaultSetupResponse, UploadSarif,
};
use crate::params::{self, Direction};
use crate::{Page, Result};

/// A client to GitHub's repository Code Scanning API.
///
/// Created with [`RepoHandler::code_scanning`].
pub struct RepoCodeScanningHandler<'octo, 'b> {
    handler: &'b RepoHandler<'octo>,
}

impl<'octo, 'b> RepoCodeScanningHandler<'octo, 'b> {
    pub(crate) fn new(handler: &'b RepoHandler<'octo>) -> Self {
        Self { handler }
    }

    /// List code scanning alerts in the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#list-code-scanning-alerts-for-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let alerts = octocrab
    ///     .repos("owner", "repo")
    ///     .code_scanning()
    ///     .list()
    ///     .per_page(10)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListRepoCodeScanningAlertsBuilder<'octo, 'b, '_> {
        ListRepoCodeScanningAlertsBuilder::new(self)
    }

    /// Alias for [`Self::list`].
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#list-code-scanning-alerts-for-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let alerts = octocrab
    ///     .repos("owner", "repo")
    ///     .code_scanning()
    ///     .list_alerts()
    ///     .per_page(10)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_alerts(&self) -> ListRepoCodeScanningAlertsBuilder<'octo, 'b, '_> {
        self.list()
    }

    /// Get a single code scanning alert in the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#get-a-code-scanning-alert)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let alert = octocrab
    ///     .repos("owner", "repo")
    ///     .code_scanning()
    ///     .get(42)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, alert_number: u64) -> Result<CodeScanningAlert> {
        let route = format!(
            "/{}/code-scanning/alerts/{}",
            self.handler.repo, alert_number
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Update a code scanning alert in the repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#update-a-code-scanning-alert)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let alert = octocrab
    ///     .repos("owner", "repo")
    ///     .code_scanning()
    ///     .update(42)
    ///     .dismissed_reason("false positive")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(&self, alert_number: u64) -> UpdateRepoCodeScanningAlertBuilder<'octo, 'b, '_> {
        UpdateRepoCodeScanningAlertBuilder::new(self, alert_number)
    }

    /// List instances of a code scanning alert.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#list-instances-of-a-code-scanning-alert)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let instances = octocrab
    ///     .repos("owner", "repo")
    ///     .code_scanning()
    ///     .list_instances(42)
    ///     .per_page(10)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_instances(&self, alert_number: u64) -> ListAlertInstancesBuilder<'octo, 'b, '_> {
        ListAlertInstancesBuilder::new(self, alert_number)
    }

    /// List code scanning analyses for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#list-code-scanning-analyses-for-a-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let analyses = octocrab
    ///     .repos("owner", "repo")
    ///     .code_scanning()
    ///     .list_analyses()
    ///     .per_page(10)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_analyses(&self) -> ListAnalysesBuilder<'octo, 'b, '_> {
        ListAnalysesBuilder::new(self)
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
    ///     .repos("owner", "repo")
    ///     .code_scanning()
    ///     .get_analysis(42)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_analysis(&self, analysis_id: u64) -> Result<Analysis> {
        let route = format!(
            "/{}/code-scanning/analyses/{}",
            self.handler.repo, analysis_id
        );
        self.handler.crab.get(route, None::<&()>).await
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
    ///     .repos("owner", "repo")
    ///     .code_scanning()
    ///     .delete_analysis(42, None::<String>)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_analysis(
        &self,
        analysis_id: u64,
        confirm_delete: Option<impl Into<String>>,
    ) -> Result<DeleteAnalysisResponse> {
        let route = match confirm_delete {
            Some(cd) => format!(
                "/{}/code-scanning/analyses/{}?confirm_delete={}",
                self.handler.repo,
                analysis_id,
                cd.into()
            ),
            None => format!(
                "/{}/code-scanning/analyses/{}",
                self.handler.repo, analysis_id
            ),
        };
        self.handler.crab.delete(route, None::<&()>).await
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
    ///     .repos("owner", "repo")
    ///     .code_scanning()
    ///     .list_codeql_databases()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_codeql_databases(&self) -> Result<Vec<CodeqlDatabase>> {
        let route = format!("/{}/code-scanning/codeql/databases", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
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
    ///     .repos("owner", "repo")
    ///     .code_scanning()
    ///     .get_codeql_database("javascript")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_codeql_database(&self, language: impl AsRef<str>) -> Result<CodeqlDatabase> {
        let route = format!(
            "/{}/code-scanning/codeql/databases/{}",
            self.handler.repo,
            language.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
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
    ///     .repos("owner", "repo")
    ///     .code_scanning()
    ///     .delete_codeql_database("javascript")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_codeql_database(&self, language: impl AsRef<str>) -> Result<()> {
        let route = format!(
            "/{}/code-scanning/codeql/databases/{}",
            self.handler.repo,
            language.as_ref()
        );
        crate::map_github_error(self.handler.crab._delete(route, None::<&()>).await?)
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
    ///     .repos("owner", "repo")
    ///     .code_scanning()
    ///     .get_default_setup()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_default_setup(&self) -> Result<DefaultSetup> {
        let route = format!("/{}/code-scanning/default-setup", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
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
    ///     .repos("owner", "repo")
    ///     .code_scanning()
    ///     .update_default_setup(&body)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_default_setup(
        &self,
        body: &UpdateDefaultSetup,
    ) -> Result<UpdateDefaultSetupResponse> {
        let route = format!("/{}/code-scanning/default-setup", self.handler.repo);
        self.handler.crab.patch(route, Some(body)).await
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
    ///     .repos("owner", "repo")
    ///     .code_scanning()
    ///     .upload_sarif(&body)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upload_sarif(&self, body: &UploadSarif) -> Result<SarifReceipt> {
        let route = format!("/{}/code-scanning/sarifs", self.handler.repo);
        self.handler.crab.post(route, Some(body)).await
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
    ///     .repos("owner", "repo")
    ///     .code_scanning()
    ///     .get_sarif("sarif-id")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_sarif(&self, sarif_id: impl AsRef<str>) -> Result<SarifAnalysis> {
        let route = format!(
            "/{}/code-scanning/sarifs/{}",
            self.handler.repo,
            sarif_id.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }
}

/// Builder for listing repository code scanning alerts.
#[derive(serde::Serialize)]
pub struct ListRepoCodeScanningAlertsBuilder<'octo, 'b, 'c> {
    #[serde(skip)]
    handler: &'c RepoCodeScanningHandler<'octo, 'b>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_guid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    r#ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<params::code_scannings::Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<params::State>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<params::code_scannings::Severity>,
}

impl<'octo, 'b, 'c> ListRepoCodeScanningAlertsBuilder<'octo, 'b, 'c> {
    pub(crate) fn new(handler: &'c RepoCodeScanningHandler<'octo, 'b>) -> Self {
        Self {
            handler,
            tool_name: None,
            tool_guid: None,
            per_page: None,
            page: None,
            r#ref: None,
            direction: None,
            sort: None,
            state: None,
            severity: None,
        }
    }

    /// Filter by the name of the tool used to generate the code scanning analysis.
    pub fn tool_name(mut self, tool_name: impl Into<String>) -> Self {
        self.tool_name = Some(tool_name.into());
        self
    }

    /// Filter by the GUID of the tool used to generate the code scanning analysis.
    pub fn tool_guid(mut self, tool_guid: impl Into<String>) -> Self {
        self.tool_guid = Some(tool_guid.into());
        self
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

    /// Filter by Git reference (e.g. "heads/main").
    pub fn r#ref(mut self, r#ref: impl Into<String>) -> Self {
        self.r#ref = Some(r#ref.into());
        self
    }

    /// The direction to sort the results by.
    pub fn direction(mut self, direction: impl Into<Direction>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    /// The property to sort the results by.
    pub fn sort(mut self, sort: impl Into<params::code_scannings::Sort>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Filter by the state of the code scanning alert.
    pub fn state(mut self, state: impl Into<params::State>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Filter by the severity of the code scanning alert.
    pub fn severity(mut self, severity: impl Into<params::code_scannings::Severity>) -> Self {
        self.severity = Some(severity.into());
        self
    }

    /// Sends the request to list code scanning alerts.
    pub async fn send(self) -> Result<Page<CodeScanningAlert>> {
        let route = format!("/{}/code-scanning/alerts", self.handler.handler.repo);
        self.handler.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for updating a code scanning alert.
#[derive(serde::Serialize)]
pub struct UpdateRepoCodeScanningAlertBuilder<'octo, 'b, 'c> {
    #[serde(skip)]
    handler: &'c RepoCodeScanningHandler<'octo, 'b>,
    #[serde(skip)]
    number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<params::AlertState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dismissed_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dismissed_comment: Option<String>,
}

impl<'octo, 'b, 'c> UpdateRepoCodeScanningAlertBuilder<'octo, 'b, 'c> {
    pub(crate) fn new(handler: &'c RepoCodeScanningHandler<'octo, 'b>, number: u64) -> Self {
        Self {
            handler,
            number,
            state: None,
            dismissed_reason: None,
            dismissed_comment: None,
        }
    }

    /// The required state of the code scanning alert.
    pub fn state(mut self, state: impl Into<params::AlertState>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// The reason for dismissing or closing the alert.
    pub fn dismissed_reason(mut self, dismissed_reason: impl Into<String>) -> Self {
        self.dismissed_reason = Some(dismissed_reason.into());
        self
    }

    /// An optional comment when dismissing or closing the alert.
    pub fn dismissed_comment(mut self, dismissed_comment: impl Into<String>) -> Self {
        self.dismissed_comment = Some(dismissed_comment.into());
        self
    }

    /// Sends the request to update a code scanning alert.
    pub async fn send(self) -> Result<CodeScanningAlert> {
        let route = format!(
            "/{}/code-scanning/alerts/{}",
            self.handler.handler.repo, self.number
        );
        self.handler.handler.crab.patch(route, Some(&self)).await
    }
}

/// Builder for listing instances of a code scanning alert.
#[derive(serde::Serialize)]
pub struct ListAlertInstancesBuilder<'octo, 'b, 'c> {
    #[serde(skip)]
    handler: &'c RepoCodeScanningHandler<'octo, 'b>,
    #[serde(skip)]
    alert_number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    r#ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pr: Option<u64>,
}

impl<'octo, 'b, 'c> ListAlertInstancesBuilder<'octo, 'b, 'c> {
    pub(crate) fn new(handler: &'c RepoCodeScanningHandler<'octo, 'b>, alert_number: u64) -> Self {
        Self {
            handler,
            alert_number,
            page: None,
            per_page: None,
            r#ref: None,
            pr: None,
        }
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Filter by Git reference (e.g. "heads/main").
    pub fn r#ref(mut self, r#ref: impl Into<String>) -> Self {
        self.r#ref = Some(r#ref.into());
        self
    }

    /// Filter by pull request number.
    pub fn pr(mut self, pr: impl Into<u64>) -> Self {
        self.pr = Some(pr.into());
        self
    }

    /// Sends the request to list instances of a code scanning alert.
    pub async fn send(self) -> Result<Page<AlertInstance>> {
        let route = format!(
            "/{}/code-scanning/alerts/{}/instances",
            self.handler.handler.repo, self.alert_number
        );
        self.handler.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for listing repository code scanning analyses.
#[derive(serde::Serialize)]
pub struct ListAnalysesBuilder<'octo, 'b, 'c> {
    #[serde(skip)]
    handler: &'c RepoCodeScanningHandler<'octo, 'b>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_guid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    r#ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pr: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sarif_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
}

impl<'octo, 'b, 'c> ListAnalysesBuilder<'octo, 'b, 'c> {
    pub(crate) fn new(handler: &'c RepoCodeScanningHandler<'octo, 'b>) -> Self {
        Self {
            handler,
            tool_name: None,
            tool_guid: None,
            page: None,
            per_page: None,
            r#ref: None,
            pr: None,
            sarif_id: None,
            direction: None,
            sort: None,
        }
    }

    /// Filter by the name of the tool used to generate the code scanning analysis.
    pub fn tool_name(mut self, tool_name: impl Into<String>) -> Self {
        self.tool_name = Some(tool_name.into());
        self
    }

    /// Filter by the GUID of the tool used to generate the code scanning analysis.
    pub fn tool_guid(mut self, tool_guid: impl Into<String>) -> Self {
        self.tool_guid = Some(tool_guid.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Filter by Git reference (e.g. "heads/main").
    pub fn r#ref(mut self, r#ref: impl Into<String>) -> Self {
        self.r#ref = Some(r#ref.into());
        self
    }

    /// Filter by pull request number.
    pub fn pr(mut self, pr: impl Into<u64>) -> Self {
        self.pr = Some(pr.into());
        self
    }

    /// Filter by SARIF ID.
    pub fn sarif_id(mut self, sarif_id: impl Into<String>) -> Self {
        self.sarif_id = Some(sarif_id.into());
        self
    }

    /// The direction to sort the results by.
    pub fn direction(mut self, direction: impl Into<Direction>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    /// The property to sort the results by.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Sends the request to list code scanning analyses.
    pub async fn send(self) -> Result<Page<Analysis>> {
        let route = format!("/{}/code-scanning/analyses", self.handler.handler.repo);
        self.handler.handler.crab.get(route, Some(&self)).await
    }
}
