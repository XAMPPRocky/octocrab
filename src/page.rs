use std::slice::Iter;

use hyperx::header::TypedHeaders;
use snafu::ResultExt;
use url::Url;

cfg_if::cfg_if! {
    if #[cfg(feature = "stream")] {
        use futures_core::Stream;
        use futures_util::stream::try_unfold;
        use serde::de::DeserializeOwned;

        use crate::Octocrab;
    }
}

/// A Page of GitHub results, with links to the next and previous page.
/// ```no_run
///# async fn run() -> octocrab::Result<()> {
/// let octocrab = octocrab::instance();
///
/// // Print the titles of the first page of issues.
/// for issue in octocrab.issues("rust-lang", "rust").list().send().await? {
///     println!("{}", issue.title);
/// }
///
/// # Ok(())
/// # }
/// ```
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub incomplete_results: Option<bool>,
    pub total_count: Option<u64>,
    pub next: Option<Url>,
    pub prev: Option<Url>,
    pub first: Option<Url>,
    pub last: Option<Url>,
}

#[cfg(feature = "stream")]
struct PageIterator<'octo, T> {
    crab: &'octo Octocrab,
    next: Option<Url>,
    current: std::vec::IntoIter<T>,
}

impl<T> Page<T> {
    /// Returns the current set of items, replacing it with an empty Vec.
    pub fn take_items(&mut self) -> Vec<T> {
        std::mem::take(&mut self.items)
    }

    /// If `last` is present, return the number of pages for this navigation.
    pub fn number_of_pages(&self) -> Option<u32> {
        self.last.as_ref().and_then(|url| {
            url.query_pairs()
                .filter_map(|(k, v)| {
                    if k == "page" {
                        Some(v).and_then(|v| v.parse().ok())
                    } else {
                        None
                    }
                })
                .next()
        })
    }

    /// Convert Page into a stream of results
    ///
    /// This will fetch new pages using the next link with in the page so that
    /// it returns all the results that matched the original request.
    ///
    /// E.g. iterating across all of the repos in an org with too many to fit
    /// in one page of results:
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use futures_util::TryStreamExt;
    /// use tokio::pin;
    ///
    /// let crab = octocrab::instance();
    /// let mut stream = crab
    ///     .orgs("owner")
    ///     .list_repos()
    ///     .send()
    ///     .await?
    ///     .into_stream(&crab);
    /// pin!(stream);
    /// while let Some(repo) = stream.try_next().await? {
    ///     println!("{:?}", repo);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "stream")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
    pub fn into_stream(self, crab: &Octocrab) -> impl Stream<Item = crate::Result<T>> + '_
    where
        T: DeserializeOwned + 'static,
    {
        let state = PageIterator {
            crab,
            next: self.next,
            current: self.items.into_iter(),
        };
        try_unfold(state, |mut state| async move {
            if let Some(val) = state.current.next() {
                return Ok(Some((val, state)));
            }
            let page = state.crab.get_page::<T>(&state.next).await?;
            Ok(page.and_then(|page| {
                let mut current = page.items.into_iter();
                // If we get an empty page we'll return early here with out
                // checking next.
                // It doesn't really make much sense to have an empty page in
                // the middle so we assume this isn't going to happen
                let val = current.next()?;
                let state = PageIterator {
                    crab: state.crab,
                    next: page.next,
                    current,
                };
                Some((val, state))
            }))
        })
    }
}

impl<T> Default for Page<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            incomplete_results: None,
            total_count: None,
            next: None,
            prev: None,
            first: None,
            last: None,
        }
    }
}

impl<T> IntoIterator for Page<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'iter, T> IntoIterator for &'iter Page<T> {
    type Item = &'iter T;
    type IntoIter = Iter<'iter, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

#[async_trait::async_trait]
impl<T: serde::de::DeserializeOwned> crate::FromResponse for Page<T> {
    async fn from_response(response: reqwest::Response) -> crate::Result<Self> {
        let HeaderLinks {
            first,
            prev,
            next,
            last,
        } = get_links(&response)?;

        let json: serde_json::Value = response.json().await.context(crate::error::HttpSnafu)?;

        if json.is_array() {
            Ok(Self {
                items: serde_json::from_value(json).context(crate::error::SerdeSnafu)?,
                incomplete_results: None,
                total_count: None,
                next,
                prev,
                first,
                last,
            })
        } else {
            let attr = vec!["items", "workflows", "workflow_runs", "jobs", "artifacts"]
                .into_iter()
                .find(|v| json.get(v).is_some())
                .unwrap();

            Ok(Self {
                items: serde_json::from_value(json.get(attr).cloned().unwrap())
                    .context(crate::error::SerdeSnafu)?,
                incomplete_results: json
                    .get("incomplete_results")
                    .and_then(serde_json::Value::as_bool),
                total_count: json.get("total_count").and_then(serde_json::Value::as_u64),
                next,
                prev,
                first,
                last,
            })
        }
    }
}

struct HeaderLinks {
    next: Option<Url>,
    prev: Option<Url>,
    first: Option<Url>,
    last: Option<Url>,
}

fn get_links(response: &reqwest::Response) -> crate::Result<HeaderLinks> {
    let mut first = None;
    let mut prev = None;
    let mut next = None;
    let mut last = None;

    if let Ok(link_header) = response.headers().decode::<hyperx::header::Link>() {
        for value in link_header.values() {
            if let Some(relations) = value.rel() {
                if relations.contains(&hyperx::header::RelationType::Next) {
                    next = Some(Url::parse(value.link()).context(crate::error::UrlSnafu)?);
                }

                if relations.contains(&hyperx::header::RelationType::Prev) {
                    prev = Some(Url::parse(value.link()).context(crate::error::UrlSnafu)?);
                }

                if relations.contains(&hyperx::header::RelationType::First) {
                    first = Some(Url::parse(value.link()).context(crate::error::UrlSnafu)?)
                }

                if relations.contains(&hyperx::header::RelationType::Last) {
                    last = Some(Url::parse(value.link()).context(crate::error::UrlSnafu)?)
                }
            }
        }
    }

    Ok(HeaderLinks {
        first,
        prev,
        next,
        last,
    })
}
