//! Verification of GitHub Webhook payload signatures.
//!
//! GitHub signs webhook deliveries using an HMAC secret token configured in your
//! repository or GitHub App settings. When GitHub delivers a webhook, it includes
//! the signature in the `X-Hub-Signature-256` HTTP header as a hex-encoded HMAC-SHA256 digest:
//!
//! ```text
//! X-Hub-Signature-256: sha256=757107ea0eb2509fc211221cce984b8a37570b6d7586c22c30b4ff5f82e57770
//! ```
//!
//! ### Deprecation of SHA-1
//! Note that GitHub previously supported SHA-1 via the legacy `X-Hub-Signature` header.
//! SHA-1 has been deprecated by GitHub due to collision weaknesses. This module strictly
//! enforces SHA-256 (`X-Hub-Signature-256`) and will reject `sha1=` signatures with
//! [`WebhookVerificationError::DeprecatedSha1`].
//!
//! ### Webhook Ingestion & Verification Flow
//!
//! ```mermaid
//! sequenceDiagram
//!     autonumber
//!     actor GitHub as GitHub Webhook Sender
//!     participant Server as Application Server
//!     participant Verifier as octocrab (WebhookVerifier)
//!     participant Handler as Event Handler
//!
//!     GitHub->>Server: POST /webhook (Headers + Raw Body)
//!     Note over Server: Extract X-GitHub-Event, X-Hub-Signature-256, & raw body bytes
//!     Server->>Verifier: verify_signature(signature_header, secret, &raw_body)
//!     alt Invalid Signature / Header Mismatch
//!         Verifier-->>Server: Err(WebhookVerificationError)
//!         Server-->>GitHub: HTTP 401 Unauthorized / 400 Bad Request
//!     else Signature Valid
//!         Verifier-->>Server: Ok(())
//!         Server->>Verifier: WebhookEvent::try_from_header_and_body(event_type, &raw_body)
//!         Verifier-->>Server: Ok(WebhookEvent)
//!         Server->>Handler: Handle parsed webhook event
//!         Server-->>GitHub: HTTP 200 OK
//!     end
//! ```
//!
//! ### Usage Examples
//!
//! #### Using [`verify_signature`]
//! ```
//! use octocrab::models::webhook_events::verification::verify_signature;
//!
//! let secret = "my_webhook_secret";
//! let body = b"{\"action\":\"opened\"}";
//! let signature = "sha256=a8591b84025410fb80a4c1621f4bf8e9c2576e5c2c7715a04609250612010665";
//!
//! assert!(verify_signature(signature, secret, body).is_ok());
//! ```
//!
//! #### Using [`WebhookVerifier`]
//! ```
//! use octocrab::models::webhook_events::verification::WebhookVerifier;
//!
//! let verifier = WebhookVerifier::new("my_webhook_secret");
//! let body = b"{\"action\":\"opened\"}";
//! let signature = "sha256=a8591b84025410fb80a4c1621f4bf8e9c2576e5c2c7715a04609250612010665";
//!
//! assert!(verifier.verify(signature, body).is_ok());
//! ```
//!
//! #### Single-step verification & parsing with [`WebhookEvent::try_from_header_signature_and_body`]
//! ```
//! use octocrab::models::webhook_events::{WebhookEvent, WebhookEventType};
//!
//! let secret = "secret";
//! let body = r#"{"zen":"Design for failure.","hook_id":423885699,"hook":{"type":"App","id":423885699,"name":"web","active":true,"events":["issues"],"config":{"content_type":"json","insecure_ssl":"0","url":"https://smee.io/R"},"updated_at":"2023-07-13T09:30:45Z","created_at":"2023-07-13T09:30:45Z","app_id":360617,"deliveries_url":"https://api.github.com/app/hook/deliveries"}}"#;
//! let event_name = "ping";
//! let signature = "sha256=f101961424ae12c9b9d55cd4e562f4040c27ff1b9a8b7d2c4ba5cfc15e62d54f";
//!
//! let event = WebhookEvent::try_from_header_signature_and_body(event_name, signature, secret, body).unwrap();
//! assert_eq!(event.kind, WebhookEventType::Ping);
//! ```

use hmac::{Hmac, Mac};
use sha2::Sha256;
use snafu::Snafu;

use super::WebhookEvent;

const SHA256_PREFIX: &str = "sha256=";
const SHA1_PREFIX: &str = "sha1=";
const SHA256_DIGEST_LEN: usize = 32;

type HmacSha256 = Hmac<Sha256>;

/// Errors that can occur during webhook signature verification.
#[derive(Debug, Snafu, Clone, PartialEq)]
#[snafu(visibility(pub))]
pub enum WebhookVerificationError {
    /// Missing or invalid signature prefix (expected `sha256=`).
    #[snafu(display("Missing or invalid signature prefix: expected 'sha256='"))]
    MissingPrefix,

    /// Legacy SHA-1 signatures (`sha1=`) are deprecated by GitHub in favor of HMAC-SHA256 (`sha256=`).
    ///
    /// Note: GitHub previously supported SHA-1 via the `X-Hub-Signature` header, but SHA-1 has
    /// been deprecated by GitHub due to collision weaknesses. This verifier strictly enforces
    /// HMAC-SHA256 via `X-Hub-Signature-256` for security.
    #[snafu(display(
        "SHA-1 signatures ('sha1=') are deprecated by GitHub; please use 'X-Hub-Signature-256' with HMAC-SHA256"
    ))]
    DeprecatedSha1,

    /// The signature hex string could not be decoded.
    #[snafu(display("Invalid signature hex encoding: {source}"))]
    InvalidHexEncoding { source: hex::FromHexError },

    /// The decoded signature has an unexpected length (expected 32 bytes for SHA-256).
    #[snafu(display("Invalid signature length: expected {expected} bytes, got {actual}"))]
    InvalidLength { expected: usize, actual: usize },

    /// The payload signature does not match the expected HMAC.
    #[snafu(display("HMAC signature mismatch: payload does not match the provided signature"))]
    Mismatch,

    /// The provided secret key is invalid.
    #[snafu(display("Invalid secret key: {message}"))]
    InvalidKey { message: String },
}

/// Errors that can occur when verifying and deserializing a webhook event.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum WebhookEventError {
    /// Failed to verify webhook signature.
    #[snafu(display("Webhook signature verification failed: {source}"))]
    Verification { source: WebhookVerificationError },

    /// Failed to deserialize webhook event JSON payload.
    #[snafu(display("Webhook JSON deserialization failed: {source}"))]
    Json { source: serde_json::Error },
}

impl From<WebhookVerificationError> for WebhookEventError {
    fn from(err: WebhookVerificationError) -> Self {
        Self::Verification { source: err }
    }
}

impl From<serde_json::Error> for WebhookEventError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json { source: err }
    }
}

/// Verifies that a webhook payload matches the signature provided in the `X-Hub-Signature-256`
/// header using the shared secret.
///
/// This verification uses constant-time comparison to prevent timing attacks.
///
/// # Deprecation Notice for SHA-1
/// GitHub previously supported SHA-1 signatures via the `X-Hub-Signature` header. SHA-1 has
/// been deprecated by GitHub due to collision weaknesses. This function strictly enforces
/// HMAC-SHA256 via the `X-Hub-Signature-256` header and returns [`WebhookVerificationError::DeprecatedSha1`]
/// if a `sha1=` prefix is passed.
///
/// # Examples
///
/// ```
/// use octocrab::models::webhook_events::verification::verify_signature;
///
/// let secret = "my_webhook_secret";
/// let body = b"{\"action\":\"opened\"}";
///
/// // Expected HMAC-SHA256 signature for this body and secret
/// let signature = "sha256=a8591b84025410fb80a4c1621f4bf8e9c2576e5c2c7715a04609250612010665";
///
/// assert!(verify_signature(signature, secret, body).is_ok());
/// ```
pub fn verify_signature(
    signature_header: &str,
    secret: impl AsRef<[u8]>,
    body: impl AsRef<[u8]>,
) -> Result<(), WebhookVerificationError> {
    let trimmed = signature_header.trim();

    // GitHub has deprecated the legacy SHA-1 algorithm (X-Hub-Signature) in favor of HMAC-SHA256 (X-Hub-Signature-256).
    // See: https://docs.github.com/en/webhooks/using-webhooks/validating-webhook-deliveries
    if trimmed.starts_with(SHA1_PREFIX) {
        return Err(WebhookVerificationError::DeprecatedSha1);
    }

    let hex_signature = trimmed
        .strip_prefix(SHA256_PREFIX)
        .ok_or(WebhookVerificationError::MissingPrefix)?;

    let signature_bytes = hex::decode(hex_signature)
        .map_err(|source| WebhookVerificationError::InvalidHexEncoding { source })?;

    if signature_bytes.len() != SHA256_DIGEST_LEN {
        return Err(WebhookVerificationError::InvalidLength {
            expected: SHA256_DIGEST_LEN,
            actual: signature_bytes.len(),
        });
    }

    let mut mac = HmacSha256::new_from_slice(secret.as_ref()).map_err(|e| {
        WebhookVerificationError::InvalidKey {
            message: e.to_string(),
        }
    })?;

    mac.update(body.as_ref());

    mac.verify_slice(&signature_bytes)
        .map_err(|_| WebhookVerificationError::Mismatch)?;

    Ok(())
}

/// A reusable verifier for GitHub webhook payloads.
///
/// Holds a configured shared secret and can verify incoming request signatures
/// and optionally parse events.
///
/// # Example
/// ```
/// use octocrab::models::webhook_events::verification::WebhookVerifier;
///
/// let verifier = WebhookVerifier::new("my_webhook_secret");
/// let body = b"{\"action\":\"opened\"}";
/// let signature = "sha256=a8591b84025410fb80a4c1621f4bf8e9c2576e5c2c7715a04609250612010665";
///
/// assert!(verifier.verify(signature, body).is_ok());
/// ```
#[derive(Clone, Debug)]
pub struct WebhookVerifier {
    secret: Vec<u8>,
}

impl WebhookVerifier {
    /// Creates a new `WebhookVerifier` with the given shared secret.
    pub fn new(secret: impl AsRef<[u8]>) -> Self {
        Self {
            secret: secret.as_ref().to_vec(),
        }
    }

    /// Verifies that `body` matches `signature_header` using this verifier's secret.
    ///
    /// Constant-time comparison is used to mitigate timing attacks.
    pub fn verify(
        &self,
        signature_header: &str,
        body: impl AsRef<[u8]>,
    ) -> Result<(), WebhookVerificationError> {
        verify_signature(signature_header, &self.secret, body)
    }

    /// Verifies the signature from `signature_header` and deserializes the payload into a [`WebhookEvent`].
    pub fn verify_and_parse<B>(
        &self,
        event_header: &str,
        signature_header: &str,
        body: &B,
    ) -> Result<WebhookEvent, WebhookEventError>
    where
        B: AsRef<[u8]> + ?Sized,
    {
        self.verify(signature_header, body.as_ref())?;
        WebhookEvent::try_from_header_and_body(event_header, body).map_err(WebhookEventError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &[u8] = b"It's a Secret to Everybody";
    const PAYLOAD: &[u8] = b"Hello, World!";
    // Computed with HMAC-SHA256: 757107ea0eb2509fc211221cce984b8a37570b6d7586c22c46f4379c8b043e17
    const VALID_SIG: &str =
        "sha256=757107ea0eb2509fc211221cce984b8a37570b6d7586c22c46f4379c8b043e17";

    #[test]
    fn verify_valid_signature() {
        assert!(verify_signature(VALID_SIG, SECRET, PAYLOAD).is_ok());
    }

    #[test]
    fn verify_valid_signature_with_whitespace() {
        let sig = format!("  {}\n", VALID_SIG);
        assert!(verify_signature(&sig, SECRET, PAYLOAD).is_ok());
    }

    #[test]
    fn verify_valid_signature_with_verifier_struct() {
        let verifier = WebhookVerifier::new(SECRET);
        assert!(verifier.verify(VALID_SIG, PAYLOAD).is_ok());
    }

    #[test]
    fn verify_fails_on_tampered_payload() {
        let tampered = b"Hello, World?";
        assert_eq!(
            verify_signature(VALID_SIG, SECRET, tampered),
            Err(WebhookVerificationError::Mismatch)
        );
    }

    #[test]
    fn verify_fails_on_wrong_secret() {
        assert_eq!(
            verify_signature(VALID_SIG, b"wrong_secret", PAYLOAD),
            Err(WebhookVerificationError::Mismatch)
        );
    }

    #[test]
    fn verify_fails_on_missing_prefix() {
        let raw_hex = &VALID_SIG[7..];
        assert_eq!(
            verify_signature(raw_hex, SECRET, PAYLOAD),
            Err(WebhookVerificationError::MissingPrefix)
        );
    }

    #[test]
    fn verify_fails_on_deprecated_sha1() {
        let sha1_sig = "sha1=2a6f81a7cf73cfb841e25eecb23e75e927595c55";
        assert_eq!(
            verify_signature(sha1_sig, SECRET, PAYLOAD),
            Err(WebhookVerificationError::DeprecatedSha1)
        );
    }

    #[test]
    fn verify_fails_on_invalid_hex() {
        let invalid_hex =
            "sha256=zz4a4e5e228634e32654b443e32f65f13361b1834399b727e36bbd0f5add4b712";
        assert!(matches!(
            verify_signature(invalid_hex, SECRET, PAYLOAD),
            Err(WebhookVerificationError::InvalidHexEncoding { .. })
        ));
    }

    #[test]
    fn verify_fails_on_wrong_length() {
        let short_sig = "sha256=44a4e5e2";
        assert_eq!(
            verify_signature(short_sig, SECRET, PAYLOAD),
            Err(WebhookVerificationError::InvalidLength {
                expected: 32,
                actual: 4
            })
        );
    }

    #[test]
    fn verify_rfc4231_vector() {
        // RFC 4231 Test Case 2
        let key = b"Jefe";
        let data = b"what do ya want for nothing?";
        let sig = "sha256=5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843";
        assert!(verify_signature(sig, key, data).is_ok());
    }
}
