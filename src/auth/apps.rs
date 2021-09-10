use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
struct Claims<'a> {
    iss: &'a str,
    iat: usize,
    exp: usize,
}

/// Create a JSON Web Token that can be used to authenticate an a GitHub application.
///
/// See: https://docs.github.com/en/developers/apps/getting-started-with-apps/setting-up-your-development-environment-to-create-a-github-app#authenticating-as-a-github-app
pub fn create_authenticate_as_app_jwt(
    github_app_id: &str,
    private_key: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let key = EncodingKey::from_rsa_pem(private_key.as_bytes())?;

    let now = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs() as usize;

    let claims = Claims {
        iss: github_app_id,
        iat: now,
        exp: now + (10 * 60),
    };

    let header = Header::new(Algorithm::RS256);

    encode(&header, &claims, &key)
}
