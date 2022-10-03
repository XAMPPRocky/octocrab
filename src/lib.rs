//! # Octocrab: A modern, extensible GitHub API client.
//! Octocrab is an third party GitHub API client, allowing you to easily build
//! your own GitHub integrations or bots. `octocrab` comes with two primary
//! set of APIs for communicating with GitHub, a high level strongly typed
//! semantic API, and a lower level HTTP API for extending behaviour.
//!
//! ## Semantic API
//! The semantic API provides strong typing around GitHub's API, as well as a
//! set of [`models`] that maps to GitHub's types. Currently the following
//! modules are available.
//!
//! - [`activity`] GitHub Activity
//! - [`actions`] GitHub Actions
//! - [`apps`] GitHub Apps
//! - [`current`] Information about the current user.
//! - [`gitignore`] Gitignore templates
//! - [`Octocrab::graphql`] GraphQL.
//! - [`issues`] Issues and related items, e.g. comments, labels, etc.
//! - [`licenses`] License Metadata.
//! - [`markdown`] Rendering Markdown with GitHub
//! - [`orgs`] GitHub Organisations
//! - [`pulls`] Pull Requests
//! - [`repos`] Repositories
//! - [`repos::forks`] Repositories
//! - [`repos::releases`] Repositories
//! - [`search`] Using GitHub's search.
//! - [`teams`] Teams
//!
//! #### Getting a Pull Request
//! ```no_run
//! # async fn run() -> octocrab::Result<()> {
//! // Get pull request #404 from `octocrab/repo`.
//! let pr = octocrab::instance().pulls("octocrab", "repo").get(404).await?;
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
//! let mut page = octocrab.issues("octocrab", "repo")
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
//! let results = octocrab.all_pages::<models::issues::Issue>(page).await?;
//!
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
//!     .get("user", None::<&()>)
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
//!     ._get(octocrab.absolute_url("organizations")?, None::<&()>)
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
//!   async fn list_every_organisation(&self) -> Result<Page<models::orgs::Organization>>;
//! }
//!
//! #[async_trait::async_trait]
//! impl OrganisationExt for Octocrab {
//!   async fn list_every_organisation(&self) -> Result<Page<models::orgs::Organization>> {
//!     self.get("organizations", None::<&()>).await
//!   }
//! }
//! ```
//!
//! You can also easily access new properties that aren't available in the
//! current models using `serde`.
//!
//! ## Static API
//! `octocrab` also provides a statically reference count version of its API,
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
#![cfg_attr(test, recursion_limit = "512")]

const MAX_RETRIES: u32 = 3;

mod api;
mod error;
mod from_response;
mod page;

pub mod auth;
pub mod etag;
pub mod models;
pub mod params;

use std::fmt;
use std::sync::{Arc, RwLock};

use once_cell::sync::Lazy;
use reqwest::{header::HeaderName, StatusCode, Url};
use secrecy::{ExposeSecret, SecretString};
use serde::Serialize;
use snafu::*;

use auth::{AppAuth, Auth};
use models::{AppId, InstallationId, InstallationToken};

pub use self::{
    api::{
        actions, activity, apps, current, events, gists, gitignore, issues, licenses, markdown,
        orgs, pulls, repos, search, teams, workflows, ratelimit,
    },
    error::{Error, GitHubError},
    from_response::FromResponse,
    page::Page,
};

/// A convenience type with a default error type of [`Error`].
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
                .context(error::HttpSnafu)?,
            backtrace: Backtrace::generate(),
        })
    }
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

/// Returns a new instance of [`Octocrab`]. If it hasn't been previously
/// initialised it returns a default instance with no authentication set.
/// ```
/// let octocrab = octocrab::instance();
/// ```
pub fn instance() -> Arc<Octocrab> {
    STATIC_INSTANCE.load().clone()
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
#[derive(Default)]
pub struct OctocrabBuilder {
    auth: Auth,
    previews: Vec<&'static str>,
    extra_headers: Vec<(HeaderName, String)>,
    base_url: Option<Url>,
}

impl OctocrabBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable a GitHub preview.
    pub fn add_preview(mut self, preview: &'static str) -> Self {
        self.previews.push(preview);
        self
    }

    /// Add an additional header to include with every request.
    pub fn add_header(mut self, key: HeaderName, value: String) -> Self {
        self.extra_headers.push((key, value));
        self
    }

    /// Add a personal token to use for authentication.
    pub fn personal_token(mut self, token: String) -> Self {
        self.auth = Auth::PersonalToken(SecretString::new(token));
        self
    }

    /// Authenticate as a Github App.
    /// `key`: RSA private key in DER or PEM formats.
    pub fn app(mut self, app_id: AppId, key: jsonwebtoken::EncodingKey) -> Self {
        self.auth = Auth::App(AppAuth { app_id, key });
        self
    }

    /// Authenticate as a Basic Auth
    /// username and password
    pub fn basic_auth(mut self, username: String, password: String) -> Self {
        self.auth = Auth::Basic{ username, password };
        self
    }

    /// Set the base url for `Octocrab`.
    pub fn base_url(mut self, base_url: impl reqwest::IntoUrl) -> Result<Self> {
        self.base_url = Some(base_url.into_url().context(crate::error::HttpSnafu)?);
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

        let auth_state = match self.auth {
            Auth::None => AuthState::None,
            Auth::Basic{ username, password } => AuthState::BasicAuth { username, password },
            Auth::PersonalToken(token) => {
                hmap.append(
                    reqwest::header::AUTHORIZATION,
                    (String::from("Bearer ") + token.expose_secret())
                        .parse()
                        .unwrap(),
                );
                AuthState::None
            }
            Auth::App(app_auth) => AuthState::App(app_auth),
            Auth::OAuth(device) => {
                hmap.append(
                    reqwest::header::AUTHORIZATION,
                    (device.token_type + " " + &device.access_token.expose_secret())
                        .parse()
                        .unwrap(),
                );
                AuthState::None
            }
        };

        for (key, value) in self.extra_headers.into_iter() {
            hmap.append(key, value.parse().unwrap());
        }

        let client = reqwest::Client::builder()
            .user_agent("octocrab")
            .default_headers(hmap)
            .build()
            .context(crate::error::HttpSnafu)?;

        Ok(Octocrab {
            client,
            base_url: self
                .base_url
                .unwrap_or_else(|| Url::parse(GITHUB_BASE_URL).unwrap()),
            auth_state,
        })
    }
}

/// A cached API access token (which may be None)
struct CachedToken(RwLock<Option<SecretString>>);

impl CachedToken {
    fn clear(&self) {
        *self.0.write().unwrap() = None;
    }
    fn get(&self) -> Option<SecretString> {
        self.0.read().unwrap().clone()
    }
    fn set(&self, value: String) {
        *self.0.write().unwrap() = Some(SecretString::new(value));
    }
}

impl fmt::Debug for CachedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.read().unwrap().fmt(f)
    }
}

impl fmt::Display for CachedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let option = self.0.read().unwrap();
        option
            .as_ref()
            .map(|s| s.expose_secret().fmt(f))
            .unwrap_or_else(|| write!(f, "<none>"))
    }
}

impl Clone for CachedToken {
    fn clone(&self) -> CachedToken {
        CachedToken(RwLock::new(self.0.read().unwrap().clone()))
    }
}

impl Default for CachedToken {
    fn default() -> CachedToken {
        CachedToken(RwLock::new(None))
    }
}

/// State used for authenticate to Github
#[derive(Debug, Clone)]
enum AuthState {
    /// No state, although Auth::PersonalToken may have caused
    /// an Authorization HTTP header to be set to provide authentication.
    None,
    /// Basic Auth HTTP. (username:password)
    BasicAuth {
        /// The username
        username: String,
        /// The password
        password: String,
    },
    /// Github App authentication with the given app data
    App(AppAuth),
    /// Authentication via a Github App repo-specific installation
    Installation {
        /// The app authentication data (app ID and private key)
        app: AppAuth,
        /// The installation ID
        installation: InstallationId,
        /// The cached access token, if any
        token: CachedToken,
    },
}

/// The GitHub API client.
#[derive(Debug, Clone)]
pub struct Octocrab {
    client: reqwest::Client,
    pub base_url: Url,
    auth_state: AuthState,
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
            auth_state: AuthState::None,
        }
    }
}

/// # Constructors
impl Octocrab {
    /// Returns a new `OctocrabBuilder`.
    pub fn builder() -> OctocrabBuilder {
        OctocrabBuilder::default()
    }

    /// Returns a new `Octocrab` based on the current builder but
    /// authorizing via a specific installation ID.
    /// Typically you will first construct an `Octocrab` using
    /// `OctocrabBuilder::app` to authenticate as your Github App,
    /// then obtain an installation ID, and then pass that here to
    /// obtain a new `Octocrab` with which you can make API calls
    /// with the permissions of that installation.
    pub fn installation(&self, id: InstallationId) -> Octocrab {
        let app_auth = if let AuthState::App(ref app_auth) = self.auth_state {
            app_auth.clone()
        } else {
            panic!("Github App authorization is required to target an installation");
        };
        Octocrab {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
            auth_state: AuthState::Installation {
                app: app_auth,
                installation: id,
                token: CachedToken::default(),
            },
        }
    }

    /// Similar to `installation`, but also eagerly caches the installation
    /// token and returns the token. The returned token can be used to make
    /// https git requests to e.g. clone repositories that the installation
    /// has access to.
    ///
    /// See also https://docs.github.com/en/developers/apps/building-github-apps/authenticating-with-github-apps#http-based-git-access-by-an-installation
    pub async fn installation_and_token(
        &self,
        id: InstallationId,
    ) -> Result<(Octocrab, SecretString)> {
        let crab = self.installation(id);
        let token = crab.request_installation_auth_token().await?;
        Ok((crab, token))
    }
}

/// # GitHub API Methods
impl Octocrab {
    /// Creates a new [`actions::ActionsHandler`] for accessing information from
    /// GitHub Actions.
    pub fn actions(&self) -> actions::ActionsHandler {
        actions::ActionsHandler::new(self)
    }

    /// Creates a [`current::CurrentAuthHandler`] that allows you to access
    /// information about the current authenticated user.
    pub fn current(&self) -> current::CurrentAuthHandler {
        current::CurrentAuthHandler::new(self)
    }

    /// Creates a [`activity::ActivityHandler`] for the current authenticated user.
    pub fn activity(&self) -> activity::ActivityHandler {
        activity::ActivityHandler::new(self)
    }

    /// Creates a new [`apps::AppsRequestHandler`] for the currently authenticated app.
    pub fn apps(&self) -> apps::AppsRequestHandler {
        apps::AppsRequestHandler::new(self)
    }

    /// Creates a [`gitignore::GitignoreHandler`] for accessing information
    /// about `gitignore`.
    pub fn gitignore(&self) -> gitignore::GitignoreHandler {
        gitignore::GitignoreHandler::new(self)
    }

    /// Creates a [`issues::IssueHandler`] for the repo specified at `owner/repo`,
    /// that allows you to access GitHub's issues API.
    pub fn issues(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> issues::IssueHandler {
        issues::IssueHandler::new(self, owner.into(), repo.into())
    }

    /// Creates a [`licenses::LicenseHandler`].
    pub fn licenses(&self) -> licenses::LicenseHandler {
        licenses::LicenseHandler::new(self)
    }

    /// Creates a [`markdown::MarkdownHandler`].
    pub fn markdown(&self) -> markdown::MarkdownHandler {
        markdown::MarkdownHandler::new(self)
    }

    /// Creates an [`orgs::OrgHandler`] for the specified organization,
    /// that allows you to access GitHub's organization API.
    pub fn orgs(&self, owner: impl Into<String>) -> orgs::OrgHandler {
        orgs::OrgHandler::new(self, owner.into())
    }

    /// Creates a [`pulls::PullRequestHandler`] for the repo specified at
    /// `owner/repo`, that allows you to access GitHub's pull request API.
    pub fn pulls(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> pulls::PullRequestHandler {
        pulls::PullRequestHandler::new(self, owner.into(), repo.into())
    }

    /// Creates a [`repos::RepoHandler`] for the repo specified at `owner/repo`,
    /// that allows you to access GitHub's repository API.
    pub fn repos(&self, owner: impl Into<String>, repo: impl Into<String>) -> repos::RepoHandler {
        repos::RepoHandler::new(self, owner.into(), repo.into())
    }

    /// Creates a [`search::SearchHandler`] that allows you to construct general queries
    /// to GitHub's API.
    pub fn search(&self) -> search::SearchHandler {
        search::SearchHandler::new(self)
    }

    /// Creates a [`teams::TeamHandler`] for the specified organization that allows
    /// you to access GitHub's teams API.
    pub fn teams(&self, owner: impl Into<String>) -> teams::TeamHandler {
        teams::TeamHandler::new(self, owner.into())
    }

    /// Creates a [`workflows::WorkflowsHandler`] for the specified repository that allows
    /// you to access GitHub's workflows API.
    pub fn workflows(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> workflows::WorkflowsHandler {
        workflows::WorkflowsHandler::new(self, owner.into(), repo.into())
    }

    /// Creates an [`events::EventsBuilder`] that allows you to access
    /// GitHub's events API.
    pub fn events(&self) -> events::EventsBuilder {
        events::EventsBuilder::new(self)
    }

    /// Creates a [`gists::GistsHandler`] that allows you to access
    /// GitHub's Gists API.
    pub fn gists(&self) -> gists::GistsHandler {
        gists::GistsHandler::new(self)
    }

    /// Creates a [`ratelimit::RateLimitHandler`] that returns the API rate limit.
    pub fn ratelimit(&self) -> ratelimit::RateLimitHandler {
        ratelimit::RateLimitHandler::new(self)
    }
}

/// # GraphQL API.
impl Octocrab {
    /// Sends a graphql query to GitHub, and deserialises the response
    /// from JSON.
    /// ```no_run
    ///# async fn run() -> octocrab::Result<()> {
    /// let response: serde_json::Value = octocrab::instance()
    ///     .graphql("query { viewer { login }}")
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    pub async fn graphql<R: crate::FromResponse>(
        &self,
        body: &(impl serde::Serialize + ?Sized),
    ) -> crate::Result<R> {
        self.post(
            "graphql",
            Some(&serde_json::json!({
                "query": body,
            })),
        )
        .await
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
        R::from_response(crate::map_github_error(response).await?).await
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
        R::from_response(crate::map_github_error(response).await?).await
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
        R::from_response(crate::map_github_error(response).await?).await
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
    pub async fn put<R, A, B>(&self, route: A, body: Option<&B>) -> Result<R>
    where
        A: AsRef<str>,
        B: Serialize + ?Sized,
        R: FromResponse,
    {
        let response = self._put(self.absolute_url(route)?, body).await?;
        R::from_response(crate::map_github_error(response).await?).await
    }

    /// Send a `PATCH` request with no additional post-processing.
    pub async fn _put<B: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        body: Option<&B>,
    ) -> Result<reqwest::Response> {
        let mut request = self.client.put(url);

        if let Some(body) = body {
            request = request.json(body);
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
        R::from_response(crate::map_github_error(response).await?).await
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

    /// Construct a `reqwest::RequestBuilder` with the given http method. This can be executed
    /// with [execute](struct.Octocrab.html#method.execute).
    ///
    /// ```no_run
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// let octocrab = octocrab::instance();
    /// let url = format!("{}/events", octocrab.base_url);
    /// let builder = octocrab::instance().request_builder(&url, reqwest::Method::GET)
    ///     .header("if-none-match", "\"73ca617c70cd2bd9b6f009dab5e2d49d\"");
    /// let response = octocrab.execute(builder).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn request_builder(
        &self,
        url: impl reqwest::IntoUrl,
        method: reqwest::Method,
    ) -> reqwest::RequestBuilder {
        self.client.request(method, url)
    }

    /// Requests a fresh installation auth token and caches it. Returns the token.
    async fn request_installation_auth_token(&self) -> Result<SecretString> {
        let (app, installation, token) = if let AuthState::Installation {
            ref app,
            installation,
            ref token,
        } = self.auth_state
        {
            (app, installation, token)
        } else {
            panic!("Installation not configured");
        };
        let mut retries = 0;
        loop {
            let result = self
                .client
                .post(
                    self.absolute_url(format!("app/installations/{}/access_tokens", installation))?,
                )
                .bearer_auth(app.generate_bearer_token()?)
                .send()
                .await;
            let status = match &result {
                Ok(v) => Some(v.status()),
                Err(e) => e.status(),
            };
            if let Some(StatusCode::UNAUTHORIZED) = status {
                if retries < MAX_RETRIES {
                    retries += 1;
                    continue;
                }
            }
            let response = result.context(error::HttpSnafu)?;
            let token_object =
                InstallationToken::from_response(crate::map_github_error(response).await?).await?;
            token.set(token_object.token.clone());
            return Ok(SecretString::new(token_object.token));
        }
    }

    /// Execute the given `request` using octocrab's Client.
    pub async fn execute(&self, mut request: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        let mut retries = 0;
        loop {
            // Saved request that we can retry later if necessary
            let mut retry_request = None;
            match self.auth_state {
                AuthState::None => (),
                AuthState::App(ref app) => {
                    retry_request = Some(request.try_clone().unwrap());
                    request = request.bearer_auth(app.generate_bearer_token()?);
                }
                AuthState::BasicAuth { ref username, ref password } => {
                    retry_request = Some(request.try_clone().unwrap());
                    request = request.basic_auth(username, Some(password));
                }
                AuthState::Installation { ref token, .. } => {
                    retry_request = Some(request.try_clone().unwrap());
                    let token = if let Some(token) = token.get() {
                        token
                    } else {
                        self.request_installation_auth_token().await?
                    };
                    request = request.bearer_auth(token.expose_secret());
                }
            };

            let result = request.send().await;
            let status = match &result {
                Ok(v) => Some(v.status()),
                Err(e) => e.status(),
            };
            if let Some(StatusCode::UNAUTHORIZED) = status {
                if let AuthState::Installation { ref token, .. } = self.auth_state {
                    token.clear();
                }
                if let Some(retry) = retry_request {
                    if retries < MAX_RETRIES {
                        retries += 1;
                        request = retry;
                        continue;
                    }
                }
            }
            return result.context(error::HttpSnafu);
        }
    }
}

/// # Utility Methods
impl Octocrab {
    /// Returns an absolute url version of `url` using the `base_url` (default:
    /// `https://api.github.com`)
    pub fn absolute_url(&self, url: impl AsRef<str>) -> Result<Url> {
        self.base_url
            .join(url.as_ref())
            .context(crate::error::UrlSnafu)
    }

    /// A convenience method to get a page of results (if present).
    pub async fn get_page<R: serde::de::DeserializeOwned>(
        &self,
        url: &Option<Url>,
    ) -> crate::Result<Option<Page<R>>> {
        match url {
            Some(url) => self.get(url, None::<&()>).await.map(Some),
            None => Ok(None),
        }
    }

    /// A convenience method to get all the results starting at a given
    /// page.
    pub async fn all_pages<R: serde::de::DeserializeOwned>(
        &self,
        mut page: Page<R>,
    ) -> crate::Result<Vec<R>> {
        let mut ret = page.take_items();
        while let Some(mut next_page) = self.get_page(&page.next).await? {
            ret.append(&mut next_page.take_items());
            page = next_page;
        }
        Ok(ret)
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

    #[test]
    fn absolute_url_for_subdir() {
        assert_eq!(
            crate::OctocrabBuilder::new()
                .base_url("https://git.example.com/api/v3/")
                .unwrap()
                .build()
                .unwrap()
                .absolute_url("/my/api")
                .unwrap()
                .as_str(),
            String::from("https://git.example.com/my/api")
        );
    }

    #[test]
    fn relative_url() {
        assert_eq!(
            crate::instance().absolute_url("my/api").unwrap().as_str(),
            String::from(crate::GITHUB_BASE_URL) + "/my/api"
        );
    }

    #[test]
    fn relative_url_for_subdir() {
        assert_eq!(
            crate::OctocrabBuilder::new()
                .base_url("https://git.example.com/api/v3/")
                .unwrap()
                .build()
                .unwrap()
                .absolute_url("my/api")
                .unwrap()
                .as_str(),
            String::from("https://git.example.com/api/v3/my/api")
        );
    }

    #[tokio::test]
    async fn extra_headers() {
        use reqwest::header::HeaderName;
        use wiremock::{matchers, Mock, MockServer, ResponseTemplate};
        let response = ResponseTemplate::new(304).append_header("etag", "\"abcd\"");
        let mock_server = MockServer::start().await;
        Mock::given(matchers::method("GET"))
            .and(matchers::path_regex(".*"))
            .and(matchers::header("x-test1", "hello"))
            .and(matchers::header("x-test2", "goodbye"))
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;
        crate::OctocrabBuilder::new()
            .base_url(mock_server.uri())
            .unwrap()
            .add_header(HeaderName::from_static("x-test1"), "hello".to_string())
            .add_header(HeaderName::from_static("x-test2"), "goodbye".to_string())
            .build()
            .unwrap()
            .repos("XAMPPRocky", "octocrab")
            .events()
            .send()
            .await
            .unwrap();
    }
}
