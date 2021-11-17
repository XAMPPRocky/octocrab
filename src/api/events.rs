//! GitHub Events
use crate::{
    etag::{EntityTag, Etagged},
    models::events,
    FromResponse, Octocrab, Page,
};

pub struct EventsBuilder<'octo> {
    crab: &'octo Octocrab,
    headers: Headers,
    params: Params,
}

struct Headers {
    etag: Option<EntityTag>,
}

#[derive(serde::Serialize)]
struct Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> EventsBuilder<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self {
            crab,
            headers: Headers { etag: None },
            params: Params {
                per_page: None,
                page: None,
            },
        }
    }

    /// Etag for this request.
    pub fn etag(mut self, etag: Option<EntityTag>) -> Self {
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
        let url = format!("{base_url}events", base_url = self.crab.base_url);

        let response = self
            .crab
            ._get_with_etag(url, None::<&()>, self.headers.etag)
            .await?;
        Etagged::<Page<events::Event>>::from_response(response).await
    }
}
