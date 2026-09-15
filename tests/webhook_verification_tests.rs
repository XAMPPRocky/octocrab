use octocrab::models::webhook_events::{
    verify_signature, WebhookEvent, WebhookEventError, WebhookEventType, WebhookVerificationError,
    WebhookVerifier,
};

#[test]
fn test_rfc4231_test_case_1() {
    // RFC 4231 Test Case 1:
    // Key = 0x0b repeated 20 times
    // Data = "Hi There"
    // Digest = b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7
    let key = [0x0bu8; 20];
    let data = b"Hi There";
    let sig = "sha256=b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7";

    assert!(verify_signature(sig, key, data).is_ok());
}

#[test]
fn test_rfc4231_test_case_2() {
    // RFC 4231 Test Case 2:
    // Key = "Jefe"
    // Data = "what do ya want for nothing?"
    // Digest = 5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843
    let key = b"Jefe";
    let data = b"what do ya want for nothing?";
    let sig = "sha256=5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843";

    assert!(verify_signature(sig, key, data).is_ok());
}

#[test]
fn test_rfc4231_test_case_3() {
    // RFC 4231 Test Case 3:
    // Key = 0xaa repeated 20 times
    // Data = 0xdd repeated 50 times
    // Digest = 773ea91e36800e46854db8ebd09181a72959098b3ef8c122d9635514ced565fe
    let key = [0xaau8; 20];
    let data = [0xddu8; 50];
    let sig = "sha256=773ea91e36800e46854db8ebd09181a72959098b3ef8c122d9635514ced565fe";

    assert!(verify_signature(sig, key, data).is_ok());
}

#[test]
fn test_webhook_verifier_positive() {
    let secret = "my_super_secret_webhook_key";
    let verifier = WebhookVerifier::new(secret);
    let body = br#"{"action":"opened","issue":{"number":1}}"#;

    // HMAC-SHA256 for body and secret
    let signature = "sha256=595685dd78af239765cb7383cf96e137f33a8b92f08b290f463cd22582f65007";
    assert!(verifier.verify(signature, body).is_ok());
}

#[test]
fn test_webhook_event_try_from_header_signature_and_body() {
    let json_content = include_str!("resources/ping_webhook_event.json");
    // Normalize CRLF to LF so that the hardcoded HMAC matches on Windows where git may checkout with CRLF
    let json_string = json_content.replace("\r\n", "\n");
    let json = json_string.as_str();

    let secret = "secret";
    // Known signature for secret "secret" and ping_webhook_event.json
    let signature = "sha256=644edc258faa19127a9f816e1c61b092b59d715fbefb0c8cc26b8a2cab781de1";

    let event = WebhookEvent::try_from_header_signature_and_body("ping", signature, secret, json)
        .expect("should verify and parse ping webhook");

    assert_eq!(event.kind, WebhookEventType::Ping);
}

#[test]
fn test_deprecated_sha1_rejected() {
    let secret = "my_secret";
    let body = b"payload";
    let sha1_sig = "sha1=2a6f81a7cf73cfb841e25eecb23e75e927595c55";

    let err = verify_signature(sha1_sig, secret, body).unwrap_err();
    assert_eq!(err, WebhookVerificationError::DeprecatedSha1);

    // Verify error message mentions deprecation and X-Hub-Signature-256
    let error_string = err.to_string();
    assert!(error_string.contains("deprecated"));
    assert!(error_string.contains("X-Hub-Signature-256"));
}

#[test]
fn test_tampered_payload_rejected() {
    let secret = "my_secret";
    let body = b"original payload";
    let tampered = b"tampered payload";
    // Signature for "original payload"
    let sig = "sha256=b8fb82936b2a2022ac5dc5b0f88269f0fae0e1944a94f0ef8aabd4d7efa84adb";

    assert!(verify_signature(sig, secret, body).is_ok());
    assert_eq!(
        verify_signature(sig, secret, tampered),
        Err(WebhookVerificationError::Mismatch)
    );
}

#[test]
fn test_wrong_secret_rejected() {
    let secret = "my_secret";
    let wrong_secret = "wrong_secret";
    let body = b"payload";
    let sig = "sha256=32bb1c45ea46f3dd1f297ae646e28e70fd1772c3a4b9ff2b8668c49d77470969";

    assert!(verify_signature(sig, secret, body).is_ok());
    assert_eq!(
        verify_signature(sig, wrong_secret, body),
        Err(WebhookVerificationError::Mismatch)
    );
}

#[test]
fn test_missing_prefix_rejected() {
    let secret = "my_secret";
    let body = b"payload";
    let sig_no_prefix = "32bb1c45ea46f3dd1f297ae646e28e70fd1772c3a4b9ff2b8668c49d77470969";

    assert_eq!(
        verify_signature(sig_no_prefix, secret, body),
        Err(WebhookVerificationError::MissingPrefix)
    );
}

#[test]
fn test_invalid_hex_rejected() {
    let secret = "my_secret";
    let body = b"payload";
    let bad_hex = "sha256=zzzz6cb2ea44e45d448dc397c8d9c57d76ee1e330ca0cecb6e2d1d4d38275908";

    assert!(matches!(
        verify_signature(bad_hex, secret, body),
        Err(WebhookVerificationError::InvalidHexEncoding { .. })
    ));
}

#[test]
fn test_invalid_length_rejected() {
    let secret = "my_secret";
    let body = b"payload";
    let short_sig = "sha256=23ee6cb2ea";

    assert_eq!(
        verify_signature(short_sig, secret, body),
        Err(WebhookVerificationError::InvalidLength {
            expected: 32,
            actual: 5
        })
    );
}

#[test]
fn test_error_conversion_into_octocrab_error() {
    let verification_err = WebhookVerificationError::MissingPrefix;
    let octocrab_err: octocrab::Error = verification_err.into();

    match octocrab_err {
        octocrab::Error::WebhookVerification { source, .. } => {
            assert_eq!(source, WebhookVerificationError::MissingPrefix);
        }
        _ => panic!("Expected WebhookVerification error variant"),
    }

    let event_err = WebhookEventError::Verification {
        source: WebhookVerificationError::DeprecatedSha1,
    };
    let octocrab_err_2: octocrab::Error = event_err.into();
    match octocrab_err_2 {
        octocrab::Error::WebhookVerification { source, .. } => {
            assert_eq!(source, WebhookVerificationError::DeprecatedSha1);
        }
        _ => panic!("Expected WebhookVerification error variant"),
    }
}
