//! User Migrations API.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/users?apiVersion=2022-11-28)

use http::Uri;
use http_body_util::{BodyExt, Collected};
use snafu::ResultExt;

use crate::error::HttpSnafu;
use crate::models::migrations::{Migration, StartMigration};
use crate::models::{MigrationId, Repository};
use crate::{Octocrab, Page, Result};

/// Handler for GitHub's User Migrations API.
///
/// Created with [`Octocrab::migrations`].
pub struct MigrationsHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> MigrationsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Access organization migrations API for a specific organization.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/orgs?apiVersion=2022-11-28)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let org_migrations = octocrab.migrations().org("org");
    /// # Ok(())
    /// # }
    /// ```
    pub fn org(
        &self,
        org: impl Into<String>,
    ) -> crate::api::orgs::migrations::OrgMigrationsHandler<'octo> {
        crate::api::orgs::migrations::OrgMigrationsHandler::new(self.crab, org)
    }

    /// Lists user migrations.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/users?apiVersion=2022-11-28#list-user-migrations)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let migrations = octocrab.migrations().list().per_page(50).send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListUserMigrationsBuilder<'octo, '_> {
        ListUserMigrationsBuilder::new(self)
    }

    /// Starts a user migration.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/users?apiVersion=2022-11-28#start-a-user-migration)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::migrations::StartMigration;
    ///
    /// let migration = octocrab
    ///     .migrations()
    ///     .start(&StartMigration::new(["owner/repo"]))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start(&self, body: &StartMigration) -> Result<Migration> {
        self.crab.post("/user/migrations", Some(body)).await
    }

    /// Gets status for a user migration.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/users?apiVersion=2022-11-28#get-a-user-migration-status)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::MigrationId;
    ///
    /// let migration = octocrab.migrations().get(MigrationId(1)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, migration_id: impl Into<MigrationId>) -> Result<Migration> {
        self.get_status(migration_id).send().await
    }

    /// Returns a builder to get a user migration status with optional parameters.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/users?apiVersion=2022-11-28#get-a-user-migration-status)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::MigrationId;
    ///
    /// let migration = octocrab
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
    ) -> GetUserMigrationStatusBuilder<'octo, '_> {
        GetUserMigrationStatusBuilder::new(self, migration_id.into())
    }

    /// Downloads a user migration archive.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/users?apiVersion=2022-11-28#download-a-user-migration-archive)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::MigrationId;
    ///
    /// let bytes = octocrab.migrations().download_archive(MigrationId(1)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_archive(
        &self,
        migration_id: impl Into<MigrationId>,
    ) -> Result<bytes::Bytes> {
        let route = format!("/user/migrations/{}/archive", migration_id.into());
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

    /// Deletes a user migration archive.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/users?apiVersion=2022-11-28#delete-a-user-migration-archive)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::MigrationId;
    ///
    /// octocrab.migrations().delete_archive(MigrationId(1)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_archive(&self, migration_id: impl Into<MigrationId>) -> Result<()> {
        let route = format!("/user/migrations/{}/archive", migration_id.into());
        let resp = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Unlocks a repository that was locked for user migration.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/users?apiVersion=2022-11-28#unlock-a-user-repository)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::MigrationId;
    ///
    /// octocrab.migrations().unlock_repo(MigrationId(1), "repo").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn unlock_repo(
        &self,
        migration_id: impl Into<MigrationId>,
        repo_name: impl AsRef<str>,
    ) -> Result<()> {
        let route = format!(
            "/user/migrations/{}/repos/{}/lock",
            migration_id.into(),
            repo_name.as_ref()
        );
        let resp = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }

    /// Lists repositories in a user migration.
    ///
    /// See: [GitHub REST API Documentation](https://docs.github.com/en/rest/migrations/users?apiVersion=2022-11-28#list-repositories-for-a-user-migration)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::MigrationId;
    ///
    /// let repos = octocrab
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
    ) -> ListUserMigrationReposBuilder<'octo, '_> {
        ListUserMigrationReposBuilder::new(self, migration_id.into())
    }

    /// Alias for [`list_repos`][MigrationsHandler::list_repos].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// use octocrab::models::MigrationId;
    ///
    /// let repos = octocrab
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
    ) -> ListUserMigrationReposBuilder<'octo, '_> {
        self.list_repos(migration_id)
    }
}

/// A builder to list user migrations.
#[derive(serde::Serialize)]
pub struct ListUserMigrationsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r MigrationsHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListUserMigrationsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r MigrationsHandler<'octo>) -> Self {
        Self {
            handler,
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

    /// Sends the request and returns a page of migrations.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run(octocrab: &octocrab::Octocrab) -> octocrab::Result<()> {
    /// let migrations = octocrab.migrations().list().per_page(50).send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<Page<Migration>> {
        self.handler.crab.get("/user/migrations", Some(&self)).await
    }
}

pub(crate) fn serialize_comma_separated<S>(
    val: &Option<Vec<String>>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match val {
        Some(v) => serializer.serialize_str(&v.join(",")),
        None => serializer.serialize_none(),
    }
}

/// A builder to get a user migration status with optional parameters.
#[derive(serde::Serialize)]
pub struct GetUserMigrationStatusBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r MigrationsHandler<'octo>,
    #[serde(skip)]
    migration_id: MigrationId,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_comma_separated"
    )]
    exclude: Option<Vec<String>>,
}

impl<'octo, 'r> GetUserMigrationStatusBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r MigrationsHandler<'octo>, migration_id: MigrationId) -> Self {
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
    ///     .migrations()
    ///     .get_status(MigrationId(1))
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<Migration> {
        let route = format!("/user/migrations/{}", self.migration_id);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A builder to list repositories for a user migration.
#[derive(serde::Serialize)]
pub struct ListUserMigrationReposBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r MigrationsHandler<'octo>,
    #[serde(skip)]
    migration_id: MigrationId,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListUserMigrationReposBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r MigrationsHandler<'octo>, migration_id: MigrationId) -> Self {
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
    ///     .migrations()
    ///     .list_repos(MigrationId(1))
    ///     .per_page(50)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<Page<Repository>> {
        let route = format!("/user/migrations/{}/repositories", self.migration_id);
        self.handler.crab.get(route, Some(&self)).await
    }
}
