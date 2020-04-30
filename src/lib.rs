use reqwest::Url;
use snafu::*;

mod api;
mod error;
mod models;
mod page;

pub mod pulls {
    pub use crate::api::pulls::PullRequestState;
}

use serde::Serialize;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

pub struct Octocrab {
    auth: Auth,
    client: reqwest::Client,
    base_url: Url,
}

impl Octocrab {
    fn absolute_url(&self, url: impl AsRef<str>) -> Result<Url> {
        Ok(self.base_url.join(url.as_ref()).context(crate::error::Url)?)
    }

    pub fn pulls(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> api::pulls::PullRequestHandler {
        api::pulls::PullRequestHandler::new(self, owner.into(), repo.into())
    }

    /// Send a post request to `url` with optional body.
    pub async fn post<P: Serialize + ?Sized, R: FromResponse>(
        &self,
        url: impl AsRef<str>,
        body: Option<&P>,
    ) -> Result<R> {
        R::from_response(self._post(url, body).await?).await
    }

    async fn _post<P: Serialize + ?Sized>(
        &self,
        url: impl AsRef<str>,
        body: Option<&P>,
    ) -> Result<reqwest::Response> {
        let mut request = self.client.post(self.absolute_url(url)?);

        if let Some(body) = body {
            request = request.json(body);
        }

        self.send_request(request).await
    }

    pub async fn get<P: Serialize + ?Sized, R: FromResponse>(
        &self,
        url: impl AsRef<str>,
        parameters: Option<&P>,
    ) -> Result<R> {
        R::from_response(self._get(url, parameters).await?).await
    }

    async fn _get<P: Serialize + ?Sized>(
        &self,
        url: impl AsRef<str>,
        parameters: Option<&P>,
    ) -> Result<reqwest::Response> {
        let mut request = self.client.get(self.absolute_url(url)?);

        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }

        self.send_request(request).await
    }

    async fn send_request(&self, request: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        let response = request.send().await.context(error::Http)?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(error::Error::GitHub {
                source: response
                    .json::<error::GitHubError>()
                    .await
                    .context(error::Http)?,
                backtrace: Backtrace::generate(),
            })
        }
    }
}

#[async_trait::async_trait]
pub trait FromResponse: Sized {
    async fn from_response(response: reqwest::Response) -> Result<Self>;
}

#[async_trait::async_trait]
impl<T: serde::de::DeserializeOwned> FromResponse for T {
    async fn from_response(response: reqwest::Response) -> Result<Self> {
        response.json().await.context(error::Http)
    }
}

impl Default for Octocrab {
    fn default() -> Self {
        Self {
            base_url: Url::parse("https://api.github.com").unwrap(),
            auth: Auth::default(),
            client: reqwest::ClientBuilder::new()
                .user_agent("octocrab")
                .build()
                .unwrap(),
        }
    }
}

pub enum Auth {
    None,
    PersonalToken(String),
}

impl Default for Auth {
    fn default() -> Self {
        Self::None
    }
}
