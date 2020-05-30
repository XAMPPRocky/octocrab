//! Get data about the currently authenticated user.

use crate::{models, Octocrab};

/// Handler for the search API.
///
/// Created with [`Octocrab::search`].
///
/// [`Octocrab::search`]: ../struct.Octocrab.html#method.search
pub struct SearchHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> SearchHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Searchs for all the issues matching the search query.
    /// ```no_run
    ///# async fn run() -> octocrab::Result<()> {
    /// let page = octocrab::instance()
    ///     .search()
    ///     .issues_or_pulls("GitHub Octocrab in:readme user:ferris")
    ///     .sort("comments")
    ///     .order("asc")
    ///     .send()
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    pub fn issues_or_pulls<'query>(
        self,
        query: &'query (impl AsRef<str> + ?Sized),
    ) -> QueryHandler<'octo, 'query, models::Issue> {
        QueryHandler::new(self.crab, "issues", query.as_ref())
    }
}

#[derive(Clone, Debug)]
pub enum ContentType {
    TextMatch,
    Default,
}

impl Default for ContentType {
    fn default() -> Self {
        Self::Default
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
    #[serde(skip)]
    content_type: ContentType,
    #[serde(rename="q")]
    query: &'query str,
    #[serde(skip_serializing_if="Option::is_none")]
    sort: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    order: Option<String>,
}

impl<'octo, 'query, T> QueryHandler<'octo, 'query, T> {
    pub(crate) fn new(crab: &'octo Octocrab, route: &'static str, query: &'query str) -> Self {
        Self {
            return_type: std::marker::PhantomData,
            content_type: ContentType::Default,
            crab,
            order: None,
            query,
            route,
            sort: None,
        }
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
}

impl<'octo, 'query, T: serde::de::DeserializeOwned> QueryHandler<'octo, 'query, T> {
    /// Send the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<T>> {
        self.crab.get(&format!("/search/{}", self.route), Some(&self)).await
    }
}
