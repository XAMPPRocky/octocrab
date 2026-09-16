//! Organization Migrations API.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/orgs?apiVersion=2022-11-28)

use http::Uri;
use http_body_util::{BodyExt, Collected};
use snafu::ResultExt;

use crate::error::HttpSnafu;
use crate::models::migrations::{Migration, StartMigration};
use crate::models::{MigrationId, Repository};
use crate::{Octocrab, Page, Result};

/// Handler for GitHub's Organization Migrations API.
///
/// Created with [`crate::api::orgs::OrgHandler::migrations`] or [`crate::api::migrations::MigrationsHandler::org`].
pub struct OrgMigrationsHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
}

impl<'octo> OrgMigrationsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: impl Into<String>) -> Self {
        Self {
            crab,
            owner: owner.into(),
        }
    }

    /// Lists organization migrations.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/orgs?apiVersion=2022-11-28#list-organization-migrations)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let migrations = octocrab.orgs("org").migrations().list().per_page(50).send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListOrgMigrationsBuilder<'octo, '_> {
        ListOrgMigrationsBuilder::new(self)
    }

    /// Starts an organization migration.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/orgs?apiVersion=2022-11-28#start-an-organization-migration)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::migrations::StartMigration;
    ///
    /// let migration = octocrab
    ///     .orgs("org")
    ///     .migrations()
    ///     .start(&StartMigration::new(["org/repo"]))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start(&self, body: &StartMigration) -> Result<Migration> {
        let route = format!("/orgs/{}/migrations", self.owner);
        self.crab.post(route, Some(body)).await
    }

    /// Gets status for an organization migration.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/orgs?apiVersion=2022-11-28#get-an-organization-migration-status)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::MigrationId;
    ///
    /// let migration = octocrab.orgs("org").migrations().get(MigrationId(1)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, migration_id: impl Into<MigrationId>) -> Result<Migration> {
        self.get_status(migration_id).send().await
    }

    /// Returns a builder to get an organization migration status with optional parameters.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/orgs?apiVersion=2022-11-28#get-an-organization-migration-status)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::MigrationId;
    ///
    /// let migration = octocrab
    ///     .orgs("org")
    ///     .migrations()
    ///     .get_status(MigrationId(1))
    ///     .exclude(["repositories"])
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_status(
        &self,
        migration_id: impl Into<MigrationId>,
    ) -> GetOrgMigrationStatusBuilder<'octo, '_> {
        GetOrgMigrationStatusBuilder::new(self, migration_id.into())
    }

    /// Downloads an organization migration archive.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/orgs?apiVersion=2022-11-28#download-an-organization-migration-archive)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::MigrationId;
    ///
    /// let bytes = octocrab.orgs("org").migrations().download_archive(MigrationId(1)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_archive(
        &self,
        migration_id: impl Into<MigrationId>,
    ) -> Result<bytes::Bytes> {
        let route = format!(
            "/orgs/{}/migrations/{}/archive",
            self.owner,
            migration_id.into()
        );
        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        let response = self.crab._get(uri).await?;
        let data_response = self.crab.follow_location_to_data(response).await?;
        data_response
            .into_body()
            .collect()
            .await
            .map(Collected::to_bytes)
    }

    /// Deletes an organization migration archive.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/orgs?apiVersion=2022-11-28#delete-an-organization-migration-archive)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::MigrationId;
    ///
    /// octocrab.orgs("org").migrations().delete_archive(MigrationId(1)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_archive(&self, migration_id: impl Into<MigrationId>) -> Result<()> {
        let route = format!(
            "/orgs/{}/migrations/{}/archive",
            self.owner,
            migration_id.into()
        );
        let resp = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Unlocks a repository that was locked for organization migration.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/orgs?apiVersion=2022-11-28#unlock-an-organization-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::MigrationId;
    ///
    /// octocrab.orgs("org").migrations().unlock_repo(MigrationId(1), "repo").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn unlock_repo(
        &self,
        migration_id: impl Into<MigrationId>,
        repo_name: impl AsRef<str>,
    ) -> Result<()> {
        let route = format!(
            "/orgs/{}/migrations/{}/repos/{}/lock",
            self.owner,
            migration_id.into(),
            repo_name.as_ref()
        );
        let resp = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Lists repositories in an organization migration.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/orgs?apiVersion=2022-11-28#list-repositories-in-an-organization-migration)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::MigrationId;
    ///
    /// let repos = octocrab
    ///     .orgs("org")
    ///     .migrations()
    ///     .list_repos(MigrationId(1))
    ///     .per_page(50)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_repos(
        &self,
        migration_id: impl Into<MigrationId>,
    ) -> ListOrgMigrationReposBuilder<'octo, '_> {
        ListOrgMigrationReposBuilder::new(self, migration_id.into())
    }

    /// Alias for [`list_repos`][OrgMigrationsHandler::list_repos].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::MigrationId;
    ///
    /// let repos = octocrab
    ///     .orgs("org")
    ///     .migrations()
    ///     .list_repositories(MigrationId(1))
    ///     .per_page(50)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_repositories(
        &self,
        migration_id: impl Into<MigrationId>,
    ) -> ListOrgMigrationReposBuilder<'octo, '_> {
        self.list_repos(migration_id)
    }
}

/// A builder to list organization migrations.
#[derive(serde::Serialize)]
pub struct ListOrgMigrationsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgMigrationsHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::api::migrations::serialize_comma_separated"
    )]
    exclude: Option<Vec<String>>,
}

impl<'octo, 'r> ListOrgMigrationsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgMigrationsHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
            exclude: None,
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

    /// Exclude attributes from the API response to improve performance (e.g. "repositories").
    pub fn exclude(mut self, exclude: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.exclude = Some(exclude.into_iter().map(Into::into).collect());
        self
    }

    /// Sends the request and returns a page of migrations.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let migrations = octocrab.orgs("org").migrations().list().per_page(50).send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<Page<Migration>> {
        let route = format!("/orgs/{}/migrations", self.handler.owner);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A builder to get an organization migration status with optional parameters.
#[derive(serde::Serialize)]
pub struct GetOrgMigrationStatusBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgMigrationsHandler<'octo>,
    #[serde(skip)]
    migration_id: MigrationId,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::api::migrations::serialize_comma_separated"
    )]
    exclude: Option<Vec<String>>,
}

impl<'octo, 'r> GetOrgMigrationStatusBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgMigrationsHandler<'octo>, migration_id: MigrationId) -> Self {
        Self {
            handler,
            migration_id,
            exclude: None,
        }
    }

    /// Exclude attributes from the API response to improve performance (e.g. "repositories").
    pub fn exclude(mut self, exclude: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.exclude = Some(exclude.into_iter().map(Into::into).collect());
        self
    }

    /// Sends the request and returns the migration status.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::MigrationId;
    ///
    /// let migration = octocrab
    ///     .orgs("org")
    ///     .migrations()
    ///     .get_status(MigrationId(1))
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<Migration> {
        let route = format!(
            "/orgs/{}/migrations/{}",
            self.handler.owner, self.migration_id
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A builder to list repositories for an organization migration.
#[derive(serde::Serialize)]
pub struct ListOrgMigrationReposBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgMigrationsHandler<'octo>,
    #[serde(skip)]
    migration_id: MigrationId,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListOrgMigrationReposBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r OrgMigrationsHandler<'octo>, migration_id: MigrationId) -> Self {
        Self {
            handler,
            migration_id,
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

    /// Sends the request and returns a page of repositories.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::MigrationId;
    ///
    /// let repos = octocrab
    ///     .orgs("org")
    ///     .migrations()
    ///     .list_repos(MigrationId(1))
    ///     .per_page(50)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<Page<Repository>> {
        let route = format!(
            "/orgs/{}/migrations/{}/repositories",
            self.handler.owner, self.migration_id
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}
