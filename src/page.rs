use hyperx::header::TypedHeaders;
use snafu::ResultExt;
use url::Url;

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
    pub next: Option<Url>,
    pub prev: Option<Url>,
}

impl<T> Page<T> {
    /// Returns the current set of items, replacing it with an empty Vec.
    pub fn take_items(&mut self) -> Vec<T> {
        std::mem::replace(&mut self.items, Vec::new())
    }
}

impl<T> Default for Page<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            next: None,
            prev: None,
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

#[async_trait::async_trait]
impl<T: serde::de::DeserializeOwned> crate::FromResponse for Page<T> {
    async fn from_response(response: reqwest::Response) -> crate::Result<Self> {
        let (prev, next) = get_links(&response)?;

        Ok(Self {
            items: crate::FromResponse::from_response(response).await?,
            next,
            prev,
        })
    }
}

fn get_links(response: &reqwest::Response) -> crate::Result<(Option<Url>, Option<Url>)> {
    let mut prev = None;
    let mut next = None;

    if let Ok(link_header) = response.headers().decode::<hyperx::header::Link>() {
        for value in link_header.values() {
            if let Some(relations) = value.rel() {
                if relations.contains(&hyperx::header::RelationType::Next) {
                    next = Some(Url::parse(value.link()).context(crate::error::Url)?);
                }

                if relations.contains(&hyperx::header::RelationType::Prev) {
                    prev = Some(Url::parse(value.link()).context(crate::error::Url)?);
                }
            }
        }
    }

    Ok((prev, next))
}
