use hyperx::header::TypedHeaders;
use url::Url;

/// A Page of GitHub results, with links to the next and previous page.
#[non_exhaustive]
pub struct Page<T> {
    pub items: Vec<T>,
    pub next: Option<Url>,
    pub prev: Option<Url>,
}

#[async_trait::async_trait]
impl<T: serde::de::DeserializeOwned> crate::FromResponse for Page<T> {
    async fn from_response(response: reqwest::Response) -> crate::Result<Self> {
        let (next, prev) = get_links(&response)?;

        Ok(Self { items: response.json().await?, next, prev })
    }
}

fn get_links(response: &reqwest::Response) -> crate::Result<(Option<Url>, Option<Url>)> {
    let mut prev = None;
    let mut next = None;

    if let Ok(link_header) = response.headers().decode::<hyperx::header::Link>() {
        for value in link_header.values() {
            if let Some(relations) = value.rel() {
                if relations.contains(&hyperx::header::RelationType::Next) {
                    next = Some(Url::parse(value.link())?);
                }

                if relations.contains(&hyperx::header::RelationType::Prev) {
                    prev = Some(Url::parse(value.link())?);
                }
            }
        }
    }

    Ok((prev, next))
}
