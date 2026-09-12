use octocrab::{models::actions::OidcCustomSub, Octocrab};
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_oidc_custom_sub() {
    let mock_server = MockServer::start().await;
    let oidc_json = json!({
        "use_default": false,
        "include_claim_keys": ["repo", "context"]
    });

    Mock::given(method("GET"))
        .and(path("/orgs/my-org/actions/oidc/customization/sub"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&oidc_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/orgs/my-org/actions/oidc/customization/sub"))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/actions/oidc/customization/sub"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&oidc_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/repos/owner/repo/actions/oidc/customization/sub"))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());

    let org_oidc = client
        .actions()
        .get_org_oidc_custom_sub("my-org")
        .await
        .unwrap();
    assert!(!org_oidc.use_default);
    assert_eq!(
        org_oidc.include_claim_keys,
        Some(vec!["repo".to_string(), "context".to_string()])
    );

    let repo_oidc = client
        .actions()
        .get_repo_oidc_custom_sub("owner", "repo")
        .await
        .unwrap();
    assert!(!repo_oidc.use_default);

    let new_oidc: OidcCustomSub = serde_json::from_value(json!({
        "use_default": true,
        "include_claim_keys": null
    }))
    .unwrap();

    assert!(client
        .actions()
        .set_org_oidc_custom_sub("my-org", &new_oidc)
        .await
        .is_ok());
    assert!(client
        .actions()
        .set_repo_oidc_custom_sub("owner", "repo", &new_oidc)
        .await
        .is_ok());
}
