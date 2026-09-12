use crate::{
    actions::ActionsHandler,
    models::actions::{
        ActionsCacheList, OrgActionsCacheUsage, OrgActionsCacheUsageByRepository,
        RepoActionsCacheUsage,
    },
    FromResponse, Result,
};
use serde::Serialize;

/// Builder for listing GitHub Actions caches for a repository.
#[derive(Serialize)]
pub struct ListActionsCachesBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ActionsHandler<'octo>,
    #[serde(skip)]
    owner: String,
    #[serde(skip)]
    repo: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ref")]
    r#ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListActionsCachesBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r ActionsHandler<'octo>, owner: String, repo: String) -> Self {
        Self {
            handler,
            owner,
            repo,
            key: None,
            r#ref: None,
            sort: None,
            direction: None,
            per_page: None,
            page: None,
        }
    }

    /// An explicit key or prefix for identifying the cache.
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// The full Git reference for narrowing down the cache (e.g. `refs/heads/main`).
    pub fn r#ref(mut self, r#ref: impl Into<String>) -> Self {
        self.r#ref = Some(r#ref.into());
        self
    }

    /// Sort results by `created_at`, `last_accessed_at`, or `size_in_bytes`.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Sort direction (`asc` or `desc`).
    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.direction = Some(direction.into());
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

    /// Sends the request and returns the list of caches.
    pub async fn send(self) -> Result<ActionsCacheList> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/caches",
            owner = self.owner,
            repo = self.repo,
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Builder for listing repositories with cache usage for an organization.
#[derive(Serialize)]
pub struct ListOrgCacheUsageByRepositoryBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ActionsHandler<'octo>,
    #[serde(skip)]
    org: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListOrgCacheUsageByRepositoryBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r ActionsHandler<'octo>, org: String) -> Self {
        Self {
            handler,
            org,
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

    /// Sends the request and returns cache usage by repository.
    pub async fn send(self) -> Result<OrgActionsCacheUsageByRepository> {
        let route = format!(
            "/orgs/{org}/actions/cache/usage-by-repository",
            org = self.org,
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

impl<'octo> ActionsHandler<'octo> {
    /// Gets GitHub Actions cache usage for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/cache?apiVersion=2022-11-28#get-github-actions-cache-usage-for-a-repository)
    pub async fn get_repo_cache_usage(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> Result<RepoActionsCacheUsage> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/cache/usage",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Lists GitHub Actions caches for a repository.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/cache?apiVersion=2022-11-28#list-github-actions-caches-for-a-repository)
    pub fn list_repo_caches(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> ListActionsCachesBuilder<'octo, '_> {
        ListActionsCachesBuilder::new(self, owner.into(), repo.into())
    }

    /// Deletes GitHub Actions caches for a repository using a cache key.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/cache?apiVersion=2022-11-28#delete-github-actions-caches-for-a-repository-using-a-cache-key)
    pub async fn delete_repo_caches_by_key(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        key: impl AsRef<str>,
        r#ref: Option<impl AsRef<str>>,
    ) -> Result<ActionsCacheList> {
        #[derive(Serialize)]
        struct Query<'a> {
            key: &'a str,
            #[serde(skip_serializing_if = "Option::is_none", rename = "ref")]
            r#ref: Option<&'a str>,
        }
        let query = Query {
            key: key.as_ref(),
            r#ref: r#ref.as_ref().map(AsRef::as_ref),
        };
        let route = format!(
            "/repos/{owner}/{repo}/actions/caches",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
        );
        let uri = self.crab.parameterized_uri(route, Some(&query))?;
        let response = self.crab._delete(uri, None::<&()>).await?;
        <ActionsCacheList>::from_response(crate::map_github_error(response).await?).await
    }

    /// Deletes a GitHub Actions cache for a repository using a cache ID.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/cache?apiVersion=2022-11-28#delete-a-github-actions-cache-for-a-repository-using-a-cache-id)
    pub async fn delete_repo_cache(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        cache_id: i64,
    ) -> Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/caches/{cache_id}",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
        );
        let response = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Gets total GitHub Actions cache usage for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/cache?apiVersion=2022-11-28#get-github-actions-cache-usage-for-an-organization)
    pub async fn get_org_cache_usage(&self, org: impl AsRef<str>) -> Result<OrgActionsCacheUsage> {
        let route = format!("/orgs/{org}/actions/cache/usage", org = org.as_ref());
        self.crab.get(route, None::<&()>).await
    }

    /// Lists repositories with GitHub Actions cache usage for an organization.
    ///
    /// See: [GitHub API Documentation](https://docs.github.com/en/rest/actions/cache?apiVersion=2022-11-28#list-repositories-with-github-actions-cache-usage-for-an-organization)
    pub fn list_org_cache_usage_by_repo(
        &self,
        org: impl Into<String>,
    ) -> ListOrgCacheUsageByRepositoryBuilder<'octo, '_> {
        ListOrgCacheUsageByRepositoryBuilder::new(self, org.into())
    }
}
