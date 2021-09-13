//! Authentication related types and functions.

use crate::models::AppId;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub enum Auth {
    None,
    PersonalToken(String),
}

impl Default for Auth {
    fn default() -> Self {
        Self::None
    }
}

/// Create a JSON Web Token that can be used to authenticate an a GitHub application.
///
/// See: https://docs.github.com/en/developers/apps/getting-started-with-apps/setting-up-your-development-environment-to-create-a-github-app#authenticating-as-a-github-app
pub fn create_jwt<A: AsRef<[u8]>>(
    github_app_id: AppId,
    private_key: A,
) -> Result<String, jsonwebtoken::errors::Error> {
    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        iss: AppId,
        iat: usize,
        exp: usize,
    }

    let key = EncodingKey::from_rsa_pem(private_key.as_ref())?;

    let now = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs() as usize;

    let claims = Claims {
        iss: github_app_id,
        iat: now,
        exp: now + (10 * 60),
    };

    let header = Header::new(Algorithm::RS256);

    jsonwebtoken::encode(&header, &claims, &key)
}
