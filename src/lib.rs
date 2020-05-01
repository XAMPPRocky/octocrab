use reqwest::Url;
use snafu::*;

mod api;
mod auth;
mod error;
mod from_response;
mod page;

pub mod models;
pub mod params;

pub use crate::api::{issues, pulls};
pub use from_response::FromResponse;
pub use page::Page;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

use serde::Serialize;

use auth::Auth;

pub struct Octocrab {
    auth: Auth,
    client: reqwest::Client,
    pub base_url: Url,
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
    /// Creates a `PullRequestHandler` for the repo specified at `owner/repo`,
    /// that allows you to access GitHub's pull request API.
    pub fn pulls(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> api::pulls::PullRequestHandler {
        api::pulls::PullRequestHandler::new(self, owner.into(), repo.into())
    }

    /// Creates a `IssueHandler` for the repo specified at `owner/repo`,
    /// that allows you to access GitHub's issues API.
    pub fn issues(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> api::issues::IssueHandler {
        api::issues::IssueHandler::new(self, owner.into(), repo.into())
    }
}

/// # HTTP Methods
/// A collection of different of HTTP methods to use with Octocrab's
/// configuration (Authenication, etc.). All of the HTTP methods (`get`, `post`,
/// etc.) perform some amount of pre-processing such as making relative urls
/// absolute, and post processing such as mapping any potential GitHub errors
/// into `Err()` variants, and deserializing the response body.
///
/// This isn't always ideal when working with GitHub's API and as such there are
/// additional methods available prefixed with `_` (e.g.  `_get`, `_post`,
/// etc.) that perform no pre or post processing and directly return the
/// `reqwest::Response` struct.
impl Octocrab {
    /// Send a post request to `route` with an optional body, returning the body
    /// of the response.
    pub async fn post<P: Serialize + ?Sized, R: FromResponse>(
        &self,
        route: impl AsRef<str>,
        body: Option<&P>,
    ) -> Result<R> {
        let response = self._post(self.absolute_url(route)?, body).await?;
        R::from_response(Self::map_github_error(response).await?).await
    }

    /// Send a post request with no additional pre/post-processing.
    pub async fn _post<P: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        body: Option<&P>,
    ) -> Result<reqwest::Response> {
        let mut request = self.client.post(url);

        if let Some(body) = body {
            request = request.json(body);
        }

        self.send_request(request).await
    }

    /// Send a get request to `route` with optional query parameters, returning
    /// the body of the response.
    pub async fn get<R, A, P>(&self, route: A, parameters: Option<&P>) -> Result<R>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
        R: FromResponse,
    {
        let response = self._get(self.absolute_url(route)?, parameters).await?;
        R::from_response(Self::map_github_error(response).await?).await
    }

    /// Send a get request with no additional post-processing.
    pub async fn _get<P: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        parameters: Option<&P>,
    ) -> Result<reqwest::Response> {
        let mut request = self.client.get(url);

        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }

        self.send_request(request).await
    }

    /// Send a `PUT` request to `route` with optional query parameters,
    /// returning the body of the response.
    pub async fn put<R, A, P>(&self, route: A, parameters: Option<&P>) -> Result<R>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
        R: FromResponse,
    {
        let response = self._put(self.absolute_url(route)?, parameters).await?;
        R::from_response(Self::map_github_error(response).await?).await
    }

    /// Send a `DELETE` request with no additional post-processing.
    pub async fn _put<P: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        parameters: Option<&P>,
    ) -> Result<reqwest::Response> {
        let mut request = self.client.put(url);

        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }

        self.send_request(request).await
    }

    /// Send a `DELETE` request to `route` with optional query parameters,
    /// returning the body of the response.
    pub async fn delete<R, A, P>(&self, route: A, parameters: Option<&P>) -> Result<R>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
        R: FromResponse,
    {
        let response = self._delete(self.absolute_url(route)?, parameters).await?;
        R::from_response(Self::map_github_error(response).await?).await
    }

    /// Send a `DELETE` request with no additional post-processing.
    pub async fn _delete<P: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        parameters: Option<&P>,
    ) -> Result<reqwest::Response> {
        let mut request = self.client.delete(url);

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
        Ok(self
            .base_url
            .join(url.as_ref())
            .context(crate::error::Url)?)
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

    /// A convience method to get the a page of results (if present).
    pub async fn get_page<R: FromResponse>(&self, url: &Option<Url>) -> crate::Result<Option<R>> {
        match url {
            Some(url) => self.get(url, None::<&()>).await.map(Some),
            None => Ok(None),
        }
    }
}
