//! Using GitHub's search.

use crate::{models, Octocrab};

/// Handler for the search API.
///
/// Created with [`Octocrab::search`].
pub struct SearchHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> SearchHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Searches for all the repositories matching the search query.
    /// ```no_run
    ///# async fn run() -> octocrab::Result<()> {
    /// let page = octocrab::instance()
    ///     .search()
    ///     .repositories("tetris language:rust")
    ///     .sort("stars")
    ///     .order("desc")
    ///     .send()
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    pub fn repositories<'query>(
        self,
        query: &'query (impl AsRef<str> + ?Sized),
    ) -> QueryHandler<'octo, 'query, models::Repository> {
        QueryHandler::new(self.crab, "repositories", query.as_ref())
    }

    /// Searches for all the commits matching the search query.
    /// ```no_run
    ///# async fn run() -> octocrab::Result<()> {
    /// let page = octocrab::instance()
    ///     .search()
    ///     .commits("hello world repo:XAMPPRocky/octocrab")
    ///     .sort("author-date")
    ///     .order("desc")
    ///     .send()
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    pub fn commits<'query>(
        self,
        query: &'query (impl AsRef<str> + ?Sized),
    ) -> QueryHandler<'octo, 'query, models::search::CommitSearchResultItem> {
        QueryHandler::new(self.crab, "commits", query.as_ref())
    }

    /// Searches for all users matching the search query.
    /// ```no_run
    ///# async fn run() -> octocrab::Result<()> {
    /// let page = octocrab::instance()
    ///     .search()
    ///     .users("bors type:user")
    ///     .sort("followers")
    ///     .order("desc")
    ///     .send()
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    pub fn users<'query>(
        self,
        query: &'query (impl AsRef<str> + ?Sized),
    ) -> QueryHandler<'octo, 'query, models::Author> {
        QueryHandler::new(self.crab, "users", query.as_ref())
    }

    /// Searches for all the issues matching the search query.
    /// ```no_run
    ///# async fn run() -> octocrab::Result<()> {
    /// let page = octocrab::instance()
    ///     .search()
    ///     .issues_and_pull_requests("GitHub Octocrab in:readme user:ferris")
    ///     .sort("comments")
    ///     .order("asc")
    ///     .send()
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    pub fn issues_and_pull_requests<'query>(
        self,
        query: &'query (impl AsRef<str> + ?Sized),
    ) -> QueryHandler<'octo, 'query, models::issues::Issue> {
        QueryHandler::new(self.crab, "issues", query.as_ref())
    }

    /// Searches for all code matching the search query.
    /// ```no_run
    ///# async fn run() -> octocrab::Result<()> {
    /// let page = octocrab::instance()
    ///     .search()
    ///     .code("println! language:rust repo:rust-lang/rust")
    ///     .sort("indexed")
    ///     .order("asc")
    ///     .send()
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    pub fn code<'query>(
        self,
        query: &'query (impl AsRef<str> + ?Sized),
    ) -> QueryHandler<'octo, 'query, models::Code> {
        QueryHandler::new(self.crab, "code", query.as_ref())
    }

    /// Searches for all the issues matching the search query.
    ///
    /// Alias for [`SearchHandler::issues_and_pull_requests`].
    ///
    /// See <https://docs.github.com/en/rest/search/search#search-issues-and-pull-requests>
    ///
    /// ```no_run
    ///# async fn run() -> octocrab::Result<()> {
    /// let page = octocrab::instance()
    ///     .search()
    ///     .issues("GitHub Octocrab in:readme user:ferris")
    ///     .sort("comments")
    ///     .order("asc")
    ///     .send()
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    pub fn issues<'query>(
        self,
        query: &'query (impl AsRef<str> + ?Sized),
    ) -> QueryHandler<'octo, 'query, models::issues::Issue> {
        self.issues_and_pull_requests(query)
    }

    /// Searches for all labels in a repository matching the search query.
    ///
    /// See <https://docs.github.com/en/rest/search/search#search-labels>
    ///
    /// ```no_run
    ///# async fn run() -> octocrab::Result<()> {
    /// let page = octocrab::instance()
    ///     .search()
    ///     .labels(260152030, "bug")
    ///     .sort("created")
    ///     .order("desc")
    ///     .send()
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    pub fn labels<'query>(
        self,
        repository_id: impl Into<models::RepositoryId>,
        query: &'query (impl AsRef<str> + ?Sized),
    ) -> QueryHandler<'octo, 'query, models::search::LabelSearchResultItem> {
        let mut handler = QueryHandler::new(self.crab, "labels", query.as_ref());
        handler.repository_id = Some(repository_id.into());
        handler
    }

    /// Searches for all topics matching the search query.
    ///
    /// See <https://docs.github.com/en/rest/search/search#search-topics>
    ///
    /// ```no_run
    ///# async fn run() -> octocrab::Result<()> {
    /// let page = octocrab::instance()
    ///     .search()
    ///     .topics("rust")
    ///     .per_page(10)
    ///     .send()
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    pub fn topics<'query>(
        self,
        query: &'query (impl AsRef<str> + ?Sized),
    ) -> QueryHandler<'octo, 'query, models::search::TopicSearchResultItem> {
        QueryHandler::new(self.crab, "topics", query.as_ref())
    }
}

/// A handler for handling search queries to GitHub.
#[derive(Clone, Debug, serde::Serialize)]
pub struct QueryHandler<'octo, 'query, T> {
    #[serde(skip)]
    return_type: std::marker::PhantomData<T>,
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip)]
    route: &'static str,
    #[serde(rename = "q")]
    query: &'query str,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_id: Option<models::RepositoryId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<String>,
}

impl<'octo, 'query, T> QueryHandler<'octo, 'query, T> {
    pub(crate) fn new(crab: &'octo Octocrab, route: &'static str, query: &'query str) -> Self {
        Self {
            crab,
            order: None,
            page: None,
            per_page: None,
            query,
            repository_id: None,
            return_type: std::marker::PhantomData,
            route,
            sort: None,
        }
    }

    /// Sets the `repository_id` parameter for the query (required for searching labels).
    pub fn repository_id(mut self, repository_id: impl Into<models::RepositoryId>) -> Self {
        self.repository_id = Some(repository_id.into());
        self
    }

    /// Sets the `sort` parameter for the query. The exact parameters for this
    /// method will vary based on what is being searched.
    pub fn sort<S: Into<String>>(mut self, sort: impl Into<Option<S>>) -> Self {
        self.sort = sort.into().map(S::into);
        self
    }

    /// Sets the `order` parameter for the query. The exact parameters for this
    /// method will vary based on what is being searched.
    pub fn order<S: Into<String>>(mut self, order: impl Into<Option<S>>) -> Self {
        self.order = order.into().map(S::into);
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
}

impl<T: serde::de::DeserializeOwned> QueryHandler<'_, '_, T> {
    /// Send the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<T>> {
        self.crab
            .get(&format!("/search/{}", self.route), Some(&self))
            .await
    }
}
