//! The GitHub Packages API.
//!
//! See: [GitHub REST API Documentation](https://docs.github.com/en/rest/packages/packages?apiVersion=2022-11-28)

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

use crate::models::packages::{
    Package, PackageType, PackageVersion, PackageVersionId, PackageVersionState, PackageVisibility,
};
use crate::{Octocrab, Page, Result};

const PACKAGE_NAME_ENCODE_SET: &AsciiSet = &CONTROLS.add(b'/').add(b' ').add(b'?').add(b'#');

pub(crate) fn encode_package_name(name: &str) -> percent_encoding::PercentEncode<'_> {
    utf8_percent_encode(name, PACKAGE_NAME_ENCODE_SET)
}

/// The target namespace/owner for package operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackagesOwner {
    Org(String),
    User(String),
    AuthenticatedUser,
}

impl PackagesOwner {
    pub(crate) fn prefix(&self) -> String {
        match self {
            Self::Org(org) => format!("/orgs/{org}"),
            Self::User(user) => format!("/users/{user}"),
            Self::AuthenticatedUser => "/user".to_string(),
        }
    }
}

/// A client to GitHub's Packages API.
///
/// Created with [`Octocrab::packages`], [`OrgHandler::packages`][crate::api::orgs::OrgHandler::packages],
/// [`UserHandler::packages`][crate::api::users::UserHandler::packages], or
/// [`CurrentAuthHandler::packages`][crate::api::current::CurrentAuthHandler::packages].
pub struct PackagesHandler<'octo> {
    crab: &'octo Octocrab,
    owner: PackagesOwner,
}

impl<'octo> PackagesHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: PackagesOwner) -> Self {
        Self { crab, owner }
    }

    /// Scope package operations to a specific organization.
    pub fn org(&self, org: impl Into<String>) -> Self {
        Self::new(self.crab, PackagesOwner::Org(org.into()))
    }

    /// Scope package operations to a specific user.
    pub fn user(&self, user: impl Into<String>) -> Self {
        Self::new(self.crab, PackagesOwner::User(user.into()))
    }

    /// Scope package operations to the authenticated user.
    pub fn current(&self) -> Self {
        Self::new(self.crab, PackagesOwner::AuthenticatedUser)
    }

    /// Lists all packages that encountered a conflict during a Docker migration.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/packages/packages?apiVersion=2022-11-28#get-list-of-conflicting-packages-during-docker-migration-for-organization)
    pub async fn docker_conflicts(&self) -> Result<Vec<Package>> {
        let route = format!("{}/docker/conflicts", self.owner.prefix());
        self.crab.get(route, None::<&()>).await
    }

    /// Creates a builder to list packages readable by the user.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/packages/packages?apiVersion=2022-11-28#list-packages-for-an-organization)
    pub fn list(&self, package_type: PackageType) -> ListPackagesBuilder<'octo, '_> {
        ListPackagesBuilder::new(self, package_type)
    }

    /// Gets a specific package.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/packages/packages?apiVersion=2022-11-28#get-a-package-for-an-organization)
    pub async fn get(
        &self,
        package_type: PackageType,
        package_name: impl AsRef<str>,
    ) -> Result<Package> {
        let route = format!(
            "{}/packages/{package_type}/{}",
            self.owner.prefix(),
            encode_package_name(package_name.as_ref())
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Deletes a specific package.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/packages/packages?apiVersion=2022-11-28#delete-a-package-for-an-organization)
    pub async fn delete(
        &self,
        package_type: PackageType,
        package_name: impl AsRef<str>,
    ) -> Result<()> {
        let route = format!(
            "{}/packages/{package_type}/{}",
            self.owner.prefix(),
            encode_package_name(package_name.as_ref())
        );
        crate::map_github_error(self.crab._delete(route, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Creates a builder to restore an entire package.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/packages/packages?apiVersion=2022-11-28#restore-a-package-for-an-organization)
    pub fn restore(
        &self,
        package_type: PackageType,
        package_name: impl AsRef<str>,
    ) -> RestorePackageBuilder<'octo, '_> {
        RestorePackageBuilder::new(self, package_type, package_name.as_ref())
    }

    /// Creates a builder to list package versions for a package.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/packages/packages?apiVersion=2022-11-28#list-package-versions-for-a-package-owned-by-an-organization)
    pub fn list_versions(
        &self,
        package_type: PackageType,
        package_name: impl AsRef<str>,
    ) -> ListPackageVersionsBuilder<'octo, '_> {
        ListPackageVersionsBuilder::new(self, package_type, package_name.as_ref())
    }

    /// Gets a specific package version.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/packages/packages?apiVersion=2022-11-28#get-a-package-version-for-an-organization)
    pub async fn get_version(
        &self,
        package_type: PackageType,
        package_name: impl AsRef<str>,
        package_version_id: impl Into<PackageVersionId>,
    ) -> Result<PackageVersion> {
        let route = format!(
            "{}/packages/{package_type}/{}/versions/{}",
            self.owner.prefix(),
            encode_package_name(package_name.as_ref()),
            package_version_id.into()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Deletes a specific package version.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/packages/packages?apiVersion=2022-11-28#delete-package-version-for-an-organization)
    pub async fn delete_version(
        &self,
        package_type: PackageType,
        package_name: impl AsRef<str>,
        package_version_id: impl Into<PackageVersionId>,
    ) -> Result<()> {
        let route = format!(
            "{}/packages/{package_type}/{}/versions/{}",
            self.owner.prefix(),
            encode_package_name(package_name.as_ref()),
            package_version_id.into()
        );
        crate::map_github_error(self.crab._delete(route, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Restores a specific package version.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/packages/packages?apiVersion=2022-11-28#restore-package-version-for-an-organization)
    pub async fn restore_version(
        &self,
        package_type: PackageType,
        package_name: impl AsRef<str>,
        package_version_id: impl Into<PackageVersionId>,
    ) -> Result<()> {
        let route = format!(
            "{}/packages/{package_type}/{}/versions/{}/restore",
            self.owner.prefix(),
            encode_package_name(package_name.as_ref()),
            package_version_id.into()
        );
        crate::map_github_error(self.crab._post(route, None::<&()>).await?)
            .await
            .map(drop)
    }
}

/// A builder to list packages for an organization, user, or authenticated user.
#[derive(serde::Serialize)]
pub struct ListPackagesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r PackagesHandler<'octo>,
    package_type: PackageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<PackageVisibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListPackagesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r PackagesHandler<'octo>, package_type: PackageType) -> Self {
        Self {
            handler,
            package_type,
            visibility: None,
            per_page: None,
            page: None,
        }
    }

    /// Filter by package type.
    pub fn package_type(mut self, package_type: PackageType) -> Self {
        self.package_type = package_type;
        self
    }

    /// Filter packages by visibility.
    pub fn visibility(mut self, visibility: PackageVisibility) -> Self {
        self.visibility = Some(visibility);
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

    /// Sends the request and returns a page of packages.
    pub async fn send(self) -> Result<Page<Package>> {
        let route = format!("{}/packages", self.handler.owner.prefix());
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A builder to list package versions.
#[derive(serde::Serialize)]
pub struct ListPackageVersionsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r PackagesHandler<'octo>,
    #[serde(skip)]
    package_type: PackageType,
    #[serde(skip)]
    package_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PackageVersionState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListPackageVersionsBuilder<'octo, 'r> {
    pub(crate) fn new(
        handler: &'r PackagesHandler<'octo>,
        package_type: PackageType,
        package_name: impl Into<String>,
    ) -> Self {
        Self {
            handler,
            package_type,
            package_name: package_name.into(),
            state: None,
            per_page: None,
            page: None,
        }
    }

    /// Filter package versions by state (`active` or `deleted`). Default is `active`.
    pub fn state(mut self, state: PackageVersionState) -> Self {
        self.state = Some(state);
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

    /// Sends the request and returns a page of package versions.
    pub async fn send(self) -> Result<Page<PackageVersion>> {
        let route = format!(
            "{}/packages/{package_type}/{}/versions",
            self.handler.owner.prefix(),
            encode_package_name(&self.package_name),
            package_type = self.package_type,
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A builder to restore a package.
pub struct RestorePackageBuilder<'octo, 'r> {
    handler: &'r PackagesHandler<'octo>,
    package_type: PackageType,
    package_name: String,
    token: Option<String>,
}

impl<'octo, 'r> RestorePackageBuilder<'octo, 'r> {
    pub(crate) fn new(
        handler: &'r PackagesHandler<'octo>,
        package_type: PackageType,
        package_name: impl Into<String>,
    ) -> Self {
        Self {
            handler,
            package_type,
            package_name: package_name.into(),
            token: None,
        }
    }

    /// Optional package token for restoring a package.
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Sends the restore request.
    pub async fn send(self) -> Result<()> {
        let mut route = format!(
            "{}/packages/{package_type}/{}/restore",
            self.handler.owner.prefix(),
            encode_package_name(&self.package_name),
            package_type = self.package_type,
        );
        if let Some(token) = &self.token {
            let encoded_token: String =
                url::form_urlencoded::byte_serialize(token.as_bytes()).collect();
            route = format!("{route}?token={encoded_token}");
        }
        crate::map_github_error(self.handler.crab._post(route, None::<&()>).await?)
            .await
            .map(drop)
    }
}
