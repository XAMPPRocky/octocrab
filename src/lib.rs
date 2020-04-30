use reqwest::Url;
use snafu::*;

mod api;
mod error;
mod models;
mod page;

pub use crate::api::pulls;

use serde::Serialize;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

pub struct Octocrab {
    auth: Auth,
    client: reqwest::Client,
    base_url: Url,
}

/// Defaults for Octocrab:
/// - `base_url`: `https://api.github.com`
/// - `auth`: `None`
/// - `client`: reqwest client with the `octocrab` user agent.
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

/// GitHub API Methods
impl Octocrab {
    /// Creates a [`PullRequestHandler`] for the repo specified at `owner/repo`,
    /// that allows you to access GitHub's pull request API.
    ///
    /// [`PullRequestHandler`]: crate::pulls::PullRequestHandler
    pub fn pulls(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> api::pulls::PullRequestHandler {
        api::pulls::PullRequestHandler::new(self, owner.into(), repo.into())
    }

}

/// # HTTP Methods
/// A collection of different of HTTP methods to use with Octocrab's
/// configuration (Authenication, etc.). All of the HTTP methods (`get`, `post`,
/// etc.) perform post processing mapping any potential GitHub errors into
/// `Err()` variants, and deserializing the response body into `R`, where `R` is
/// any `serde::DeserializeOwned` type.  This isn't always ideal when working
/// with GitHub's API and as such there are additional methods available
/// prefixed with `_` (e.g.  `_get`, `_post`, etc.) that perform no post
/// processing and directly return the `reqwest::Response` struct.
impl Octocrab {
    /// Send a post request to `url` with an optional body, returning the body
    /// of the response.
    pub async fn post<P: Serialize + ?Sized, R: FromResponse>(
        &self,
        url: impl AsRef<str>,
        body: Option<&P>,
    ) -> Result<R> {
        R::from_response(Self::map_github_error(self._post(url, body).await?).await?).await
    }

    /// Send a post request with no additional post-processing.
    pub async fn _post<P: Serialize + ?Sized>(
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

    /// Send a get request to `url` with optional query parameters, returning
    /// the body of the response.
    pub async fn get<P: Serialize + ?Sized, R: FromResponse>(
        &self,
        url: impl AsRef<str>,
        parameters: Option<&P>,
    ) -> Result<R> {
        R::from_response(Self::map_github_error(self._get(url, parameters).await?).await?).await
    }

    /// Send a get request with no additional post-processing.
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
        request.send().await.context(error::Http)
    }
}

/// Utility Methods
impl Octocrab {
    /// Returns an absolute url version of `url` using the `base_url` (default:
    /// `https://api.github.com`)
    pub fn absolute_url(&self, url: impl AsRef<str>) -> Result<Url> {
        Ok(self.base_url.join(url.as_ref()).context(crate::error::Url)?)
    }

    /// Maps a GitHub error response into and `Err()` variant if the status is
    /// not a success.
    pub async fn map_github_error(response: reqwest::Response) -> Result<reqwest::Response> {
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

pub enum Auth {
    None,
    PersonalToken(String),
}

impl Default for Auth {
    fn default() -> Self {
        Self::None
    }
}
