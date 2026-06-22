# Fix for Issue #224: Octocrab doesn't compile with Yew (for me)

mod api;
mod auth;
mod error;
mod from_response;
mod page;

pub mod etag;
pub mod models;
pub mod params;

use std::sync::Arc;

use arc_swap::ArcSwap;
use auth::{AppAuth, Auth};
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use reqwest::Url;
use secrecy::{ExposeSecret, SecretString};
use serde::Serialize;

pub use self::{
    api::{
        actions, activity, apps, checks, commits, current, events, gists, gitignore, issues,
        licenses, markdown, orgs, pulls, ratelimit, repos, search, teams, users,
    },
    auth::AuthState,
    error::{Error, GitHubError},
    from_response::FromResponse,
    page::Page,
};

/// A convenience type with a default error type of [`Error`].
pub type Result<T, E = error::Error> = std::result::Result<T, E>;

const GITHUB_BASE_URL: &str = "https://api.github.com";

static STATIC_INSTANCE: Lazy<ArcSwap<Octocrab>> =
    Lazy::new(|| ArcSwap::from_pointee(Octocrab::default()));

/// Returns a new `Octocrab` based on the current builder.
///
/// # Example
///