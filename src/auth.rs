//! Authentication related types and functions.

use crate::models::AppId;
use crate::Result;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use secrecy::SecretString;
use serde::Serialize;
use std::fmt;
use std::time::SystemTime;

use snafu::*;

/// The data necessary to authenticate as a Github App
#[derive(Clone)]
pub struct AppAuth {
    /// Github's app ID
    pub app_id: AppId,
    /// The app's RSA private key
    pub key: EncodingKey,
}

impl fmt::Debug for AppAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppAuth")
            .field("app_id", &self.app_id)
            .finish_non_exhaustive()
    }
}

/// The forms of authentication we support
pub enum Auth {
    /// No authentication
    None,
    /// Authenticate using a Github personal access token
    PersonalToken(SecretString),
    /// Authenticate as a Github App
    App(AppAuth),
}

impl Default for Auth {
    fn default() -> Self {
        Self::None
    }
}

/// Create a JSON Web Token that can be used to authenticate an a GitHub application.
///
/// See: https://docs.github.com/en/developers/apps/getting-started-with-apps/setting-up-your-development-environment-to-create-a-github-app#authenticating-as-a-github-app
pub fn create_jwt(
    github_app_id: AppId,
    key: &EncodingKey,
) -> Result<String, jsonwebtoken::errors::Error> {
    #[derive(Serialize)]
    struct Claims {
        iss: AppId,
        iat: usize,
        exp: usize,
    }

    let now = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs() as usize;

    // Github only allows JWTs that expire in the next 10 minutes.
    // The token is issued 60 seconds in the past and expires in 9 minutes,
    // to allow some clock drift.
    let claims = Claims {
        iss: github_app_id,
        iat: now - 60,
        exp: now + (9 * 60),
    };

    let header = Header::new(Algorithm::RS256);

    jsonwebtoken::encode(&header, &claims, key)
}

impl AppAuth {
    /// Currently we don't cache these, but we could if we want to avoid
    /// an RSA signature operation per App-authorized API call.
    pub fn generate_bearer_token(&self) -> Result<String> {
        create_jwt(self.app_id, &self.key).context(crate::error::JWTSnafu)
    }
}
