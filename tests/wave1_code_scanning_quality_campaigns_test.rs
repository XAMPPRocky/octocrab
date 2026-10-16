use octocrab::models::code_scannings::UpdateAiScan;
use octocrab::Octocrab;
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_code_scanning_ai_scan() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/orgs/org/code-scanning/ai-scan"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "pr_scan": "enabled" })))
        .mount(&mock_server)
        .await;

    Mock::given(method("PATCH"))
        .and(path("/repos/owner/repo/code-scanning/ai-scan"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "pr_scan": "disabled" })))
        .mount(&mock_server)
        .await;

    let octo = setup_octocrab(&mock_server.uri());
    let org_setting = octo
        .orgs("org")
        .code_scanning()
        .get_ai_scan()
        .await
        .unwrap();
    assert_eq!(org_setting.pr_scan, "enabled");

    let repo_setting = octo
        .repos("owner", "repo")
        .code_scanning()
        .update_ai_scan(&UpdateAiScan {
            pr_scan: Some("disabled".into()),
        })
        .await
        .unwrap();
    assert_eq!(repo_setting.pr_scan, "disabled");
}

#[tokio::test]
async fn test_code_quality_enablement_and_check() {
    let mock_server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/repos/owner/repo/code-quality"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/code-quality/check"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let octo = setup_octocrab(&mock_server.uri());
    let cq = octo.code_quality("owner", Some("repo".to_string()));
    assert!(cq.enable().await.is_ok());
    assert!(cq.check().await.unwrap());
}

#[tokio::test]
async fn test_campaigns_crud() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/orgs/org/campaigns/42"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let octo = setup_octocrab(&mock_server.uri());
    let res = octo.campaigns("org").delete(42).await;
    assert!(res.is_ok());
}
