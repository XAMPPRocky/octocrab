//! GitHub Repository Events
use crate::{
    etag::{Etag, Etagged},
    models::events,
    repos::RepoHandler,
    FromResponse, Page,
};
use reqwest::{
    header::{self, HeaderMap},
    Method, StatusCode,
};
use std::convert::TryFrom;

pub struct ListRepoEventsBuilder<'octo, 'handler> {
    handler: &'handler RepoHandler<'octo>,
    headers: Headers,
    params: Params,
}

struct Headers {
    etag: Option<Etag>,
}

#[derive(serde::Serialize)]
struct Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'handler> ListRepoEventsBuilder<'octo, 'handler> {
    pub(crate) fn new(handler: &'handler RepoHandler<'octo>) -> Self {
        Self {
            handler,
            headers: Headers { etag: None },
            params: Params {
                per_page: None,
                page: None,
            },
        }
    }

    /// Etag for this request.
    pub fn etag(mut self, etag: Option<Etag>) -> Self {
        self.headers.etag = etag;
        self
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.params.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.params.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Etagged<Page<events::Event>>> {
        let url = format!(
            "{base_url}repos/{owner}/{repo}/events",
            base_url = self.handler.crab.base_url,
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        let mut headers = HeaderMap::new();
        if let Some(etag) = self.headers.etag {
            headers.append(header::IF_NONE_MATCH, etag.into());
        }
        let builder = self
            .handler
            .crab
            .client
            .request(Method::GET, &url)
            .headers(headers)
            .query(&self.params);
        let response = self.handler.crab.execute(builder).await?;
        let etag = response
            .headers()
            .get(header::ETAG)
            .and_then(|val| Etag::try_from(val).ok());
        if response.status() == StatusCode::NOT_MODIFIED {
            Ok(Etagged { etag, value: None })
        } else {
            <Page<events::Event>>::from_response(crate::map_github_error(response).await?)
                .await
                .map(|page| Etagged {
                    etag,
                    value: Some(page),
                })
        }
    }
}
