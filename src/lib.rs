//! # Octocrab: A modern, extensible GitHub API client.
//! Octocrab is an third party GitHub API client, allowing you to easily build
//! your own GitHub integrations or bots. `Octocrab` comes with two primary
//! set of APIs for communicating with GitHub, a high level strongly typed
//! semantic API, and a lower level HTTP API for extending behaviour.
//!
//! ## Semantic API
//! The semantic API provides strong typing around GitHub's API, as well as a
//! set of [`models`] that maps to GitHub's types. Currently the following
//! modules are available.
//!
//! - [`gitignore`] Gitignore templates
//! - [`issues`] Issues and related items, e.g. comments, labels, etc.
//! - [`markdown`] Rendering Markdown with GitHub
//! - [`orgs`] GitHub Organisations
//! - [`pulls`] Pull Requests
//!
//! [`gitignore`]: ./gitignore/struct.GitignoreHandler.html
//! [`issues`]: ./issues/struct.IssueHandler.html
//! [`markdown`]: ./markdown/struct.MarkdownHandler.html
//! [`models`]: ./models/index.html
//! [`orgs`]: ./orgs/struct.OrgHandler.html
//! [`pulls`]: ./pulls/struct.PullRequestHandler.html
//!
//! #### Getting a Pull Request
//! ```no_run
//! # async fn run() -> octocrab::Result<()> {
//! // Get pull request #404 from `octocrab/repo`.
//! let issue = octocrab::instance().pulls("octocrab", "repo").get(404).await?;
//! # Ok(())
//! # }
//! ```
//!
//! All methods with multiple optional parameters are built as `Builder`
//! structs, allowing you to easily specify parameters.
//!
//! #### Listing issues
//! ```no_run
//! # async fn run() -> octocrab::Result<()> {
//! use octocrab::{models, params};
//!
//! let octocrab = octocrab::instance();
//! // Returns the first page of all issues.
//! let page = octocrab.issues("octocrab", "repo")
//!     .list()
//!     // Optional Parameters
//!     .creator("octocrab")
//!     .state(params::State::All)
//!     .per_page(50)
//!     .send()
//!     .await?;
//!
//! // Go through every page of issues. Warning: There's no rate limiting so
//! // be careful.
//! while let Some(page) = octocrab.get_page::<models::Issue>(&page.next).await? {
//!     for issue in page {
//!         println!("{}", issue.title);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## HTTP API
//! The typed API currently doesn't cover all of GitHub's API at this time, and
//! even if it did GitHub is in active development and this library will
//! likely always be somewhat behind GitHub at some points in time. However that
//! shouldn't mean that in order to use those features that you have to now fork
//! or replace `octocrab` with your own solution.
//!
//! Instead `octocrab` exposes a suite of HTTP methods allowing you to easily
//! extend `Octocrab`'s existing behaviour. Using these HTTP methods allows you
//! to keep using the same authentication and configuration, while having
//! control over the request and response. There is a method for each HTTP
//! method `get`, `post`, `patch`, `put`, `delete`, all of which accept a
//! relative route and a optional body.
//!
//! ```no_run
//! # async fn run() -> octocrab::Result<()> {
//! let user: octocrab::models::User = octocrab::instance()
//!     .get("/user", None::<&()>)
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! Each of the HTTP methods expects a body, formats the URL with the base
//! URL, and errors if GitHub doesn't return a successful status, but this isn't
//! always desired when working with GitHub's API, sometimes you need to check
//! the response status or headers. As such there are companion methods `_get`,
//! `_post`, etc. that perform no additional pre or post-processing to
//! the request.
//!
//! ```no_run
//! # async fn run() -> octocrab::Result<()> {
//! let octocrab = octocrab::instance();
//! let response =  octocrab
//!     ._get("https://api.github.com/organizations", None::<&()>)
//!     .await?;
//!
//! // You can also use `Octocrab::absolute_url` if you want to still to go to
//! // the same base.
//! let response =  octocrab
//!     ._get(octocrab.absolute_url("/organizations")?, None::<&()>)
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! You can use the those HTTP methods to easily create your own extensions to
//! `Octocrab`'s typed API. (Requires `async_trait`).
//! ```
//! use octocrab::{Octocrab, Page, Result, models};
//!
//! #[async_trait::async_trait]
//! trait OrganisationExt {
//!   async fn list_every_organisation(&self) -> Result<Page<models::Organization>>;
//! }
//!
//! #[async_trait::async_trait]
//! impl OrganisationExt for Octocrab {
//!   async fn list_every_organisation(&self) -> Result<Page<models::Organization>> {
//!     self.get("/organizations", None::<&()>).await
//!   }
//! }
//! ```
//!
//! You can also easily access new properties that aren't available in the
//! current models using `serde`.
//!
//! ## Static API
//! `Octocrab` also provides a statically reference count version of its API,
//! allowing you to easily plug it into existing systems without worrying
//! about having to integrate and pass around the client.
//!
//! ```
//! // Initialises the static instance with your configuration and returns an
//! // instance of the client.
//! octocrab::initialise(octocrab::Octocrab::builder());
//! // Gets a instance of `Octocrab` from the static API. If you call this
//! // without first calling `octocrab::initialise` a default client will be
//! // initialised and returned instead.
//! octocrab::instance();
//! ```

mod api;
mod auth;
mod error;
mod from_response;
mod page;

pub mod models;
pub mod params;

use std::sync::Arc;

use once_cell::sync::Lazy;
use reqwest::Url;
use serde::Serialize;
use snafu::*;

use auth::Auth;

pub use self::{
    api::{issues, gitignore, markdown, orgs, pulls, current},
    error::{Error, GitHubError},
    from_response::FromResponse,
    page::Page,
};

/// A convenience type with a default error type of `Octocrab::Error`.
pub type Result<T, E = error::Error> = std::result::Result<T, E>;

const GITHUB_BASE_URL: &str = "https://api.github.com";

static STATIC_INSTANCE: Lazy<arc_swap::ArcSwap<Octocrab>> =
    Lazy::new(|| arc_swap::ArcSwap::from_pointee(Octocrab::default()));

/// Formats a GitHub preview from it's name into the full value for the
/// `Accept` header.
/// ```
/// assert_eq!(octocrab::format_preview("machine-man"), "application/vnd.github.machine-man-preview");
/// ```
pub fn format_preview(preview: impl AsRef<str>) -> String {
    format!("application/vnd.github.{}-preview", preview.as_ref())
}

/// Formats a media type from it's name into the full value for the
/// `Accept` header.
/// ```
/// assert_eq!(octocrab::format_media_type("html"), "application/vnd.github.v3.html+json");
/// assert_eq!(octocrab::format_media_type("json"), "application/vnd.github.v3.json");
/// assert_eq!(octocrab::format_media_type("patch"), "application/vnd.github.v3.patch");
/// ```
pub fn format_media_type(media_type: impl AsRef<str>) -> String {
    let media_type = media_type.as_ref();
    let json_suffix = match media_type {
        "raw" | "text" | "html" | "full" => "+json",
        _ => "",
    };

    format!("application/vnd.github.v3.{}{}", media_type, json_suffix)
}

/// Initialises the static instance using the configuration set by
/// `builder`.
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let octocrab = octocrab::initialise(octocrab::Octocrab::builder())?;
/// # Ok(())
/// # }
/// ```
pub fn initialise(builder: OctocrabBuilder) -> Result<Arc<Octocrab>> {
    Ok(STATIC_INSTANCE.swap(Arc::from(builder.build()?)))
}

/// Returns a new instance of `Octocrab`. If it hasn't been previously
/// initialised it returns a default instance with no authentication set.
/// ```
/// let octocrab = octocrab::instance();
/// ```
pub fn instance() -> Arc<Octocrab> {
    STATIC_INSTANCE.load().clone()
}

/// A Builder struct for `Octocrab`.
#[derive(Default)]
pub struct OctocrabBuilder {
    auth: Auth,
    previews: Vec<&'static str>,
    base_url: Option<Url>,
}

/// A builder struct for `Octocrab`, allowing you to configure the client, such
/// as using GitHub previews, the github instance, authentication, etc.
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let octocrab = octocrab::OctocrabBuilder::new()
///     .add_preview("machine-man")
///     .base_url("https://github.example.com")?
///     .build()?;
/// # Ok(())
/// # }
/// ```
impl OctocrabBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable a GitHub preview.
    pub fn add_preview(mut self, preview: &'static str) -> Self {
        self.previews.push(preview);
        self
    }

    /// Add a personal token to use for authentication.
    pub fn personal_token(mut self, token: String) -> Self {
        self.auth = Auth::PersonalToken(token);
        self
    }

    /// Set the base url for `Octocrab`.
    pub fn base_url(mut self, base_url: impl reqwest::IntoUrl) -> Result<Self> {
        self.base_url = Some(base_url.into_url().context(crate::error::Http)?);
        Ok(self)
    }

    /// Create the `Octocrab` client.
    pub fn build(self) -> Result<Octocrab> {
        let mut hmap = reqwest::header::HeaderMap::new();

        for preview in &self.previews {
            hmap.append(
                reqwest::header::ACCEPT,
                crate::format_preview(&preview).parse().unwrap(),
            );
        }

        if let Auth::PersonalToken(token) = self.auth {
            hmap.append(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", token).parse().unwrap(),
            );
        }

        let client = reqwest::Client::builder()
            .user_agent("octocrab")
            .default_headers(hmap)
            .build()
            .context(crate::error::Http)?;

        Ok(Octocrab {
            client,
            base_url: self
                .base_url
                .unwrap_or_else(|| Url::parse(GITHUB_BASE_URL).unwrap()),
        })
    }
}

/// The GitHub API client.
#[derive(Debug, Clone)]
pub struct Octocrab {
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
            base_url: Url::parse(GITHUB_BASE_URL).unwrap(),
            client: reqwest::ClientBuilder::new()
                .user_agent("octocrab")
                .build()
                .unwrap(),
        }
    }
}

impl Octocrab {
    /// Returns a new `OctocrabBuilder`.
    pub fn builder() -> OctocrabBuilder {
        OctocrabBuilder::default()
    }
}

/// GitHub API Methods
impl Octocrab {
    /// Creates a `IssueHandler` for the repo specified at `owner/repo`,
    /// that allows you to access GitHub's issues API.
    pub fn issues(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> api::issues::IssueHandler {
        api::issues::IssueHandler::new(self, owner.into(), repo.into())
    }

    /// Creates a `MarkdownHandler`.
    pub fn markdown(&self) -> markdown::MarkdownHandler {
        markdown::MarkdownHandler::new(self)
    }

    /// Creates a `GitIgnoreHandler`.
    pub fn gitignore(&self) -> gitignore::GitignoreHandler {
        gitignore::GitignoreHandler::new(self)
    }

    /// Creates a `IssueHandler` for the repo specified at `owner/repo`,
    /// that allows you to access GitHub's issues API.
    pub fn orgs(&self, owner: impl Into<String>) -> api::orgs::OrgHandler {
        api::orgs::OrgHandler::new(self, owner.into())
    }

    /// Creates a `PullRequestHandler` for the repo specified at `owner/repo`,
    /// that allows you to access GitHub's pull request API.
    pub fn pulls(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> api::pulls::PullRequestHandler {
        api::pulls::PullRequestHandler::new(self, owner.into(), repo.into())
    }

    /// Creates a `CurrentAuthHandler` that allows you to access
    /// information about the current authenticated user.
    pub fn current(&self) -> api::current::CurrentAuthHandler {
        api::current::CurrentAuthHandler::new(self)
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
    /// Send a `POST` request to `route` with an optional body, returning the body
    /// of the response.
    pub async fn post<P: Serialize + ?Sized, R: FromResponse>(
        &self,
        route: impl AsRef<str>,
        body: Option<&P>,
    ) -> Result<R> {
        let response = self._post(self.absolute_url(route)?, body).await?;
        R::from_response(Self::map_github_error(response).await?).await
    }

    /// Send a `POST` request with no additional pre/post-processing.
    pub async fn _post<P: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        body: Option<&P>,
    ) -> Result<reqwest::Response> {
        let mut request = self.client.post(url);

        if let Some(body) = body {
            request = request.json(body);
        }

        self.execute(request).await
    }

    /// Send a `GET` request to `route` with optional query parameters, returning
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

    /// Send a `GET` request with no additional post-processing.
    pub async fn _get<P: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        parameters: Option<&P>,
    ) -> Result<reqwest::Response> {
        let mut request = self.client.get(url);

        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }

        self.execute(request).await
    }

    /// Send a `PATCH` request to `route` with optional query parameters,
    /// returning the body of the response.
    pub async fn patch<R, A, B>(&self, route: A, body: Option<&B>) -> Result<R>
    where
        A: AsRef<str>,
        B: Serialize + ?Sized,
        R: FromResponse,
    {
        let response = self._patch(self.absolute_url(route)?, body).await?;
        R::from_response(Self::map_github_error(response).await?).await
    }

    /// Send a `PATCH` request with no additional post-processing.
    pub async fn _patch<B: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        parameters: Option<&B>,
    ) -> Result<reqwest::Response> {
        let mut request = self.client.patch(url);

        if let Some(parameters) = parameters {
            request = request.json(parameters);
        }

        self.execute(request).await
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

    /// Send a `PATCH` request with no additional post-processing.
    pub async fn _put<P: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        parameters: Option<&P>,
    ) -> Result<reqwest::Response> {
        let mut request = self.client.put(url);

        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }

        self.execute(request).await
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

        self.execute(request).await
    }

    /// Execute the given `request` octocrab's Client.
    pub async fn execute(&self, request: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        request.send().await.context(error::Http)
    }
}

/// # Utility Methods
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
    pub async fn get_page<R: serde::de::DeserializeOwned>(
        &self,
        url: &Option<Url>,
    ) -> crate::Result<Option<Page<R>>> {
        match url {
            Some(url) => self.get(url, None::<&()>).await.map(Some),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn absolute_url_escapes() {
        assert_eq!(
            crate::instance()
                .absolute_url("/help wanted")
                .unwrap()
                .as_str(),
            String::from(crate::GITHUB_BASE_URL) + "/help%20wanted"
        );
    }
}
