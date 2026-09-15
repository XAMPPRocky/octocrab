use octocrab::{models::orgs::secrets::Visibility, Octocrab};
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_org_variables_crud() {
    let mock_server = MockServer::start().await;
    let list_json = json!({
        "total_count": 1,
        "variables": [
            {
                "name": "MY_VAR",
                "value": "my_val",
                "created_at": "2023-01-01T00:00:00Z",
                "updated_at": "2023-01-01T00:00:00Z",
                "visibility": "all"
            }
        ]
    });
    let single_json = json!({
        "name": "MY_VAR",
        "value": "my_val",
        "created_at": "2023-01-01T00:00:00Z",
        "updated_at": "2023-01-01T00:00:00Z",
        "visibility": "all"
    });

    Mock::given(method("GET"))
        .and(path("/orgs/my-org/actions/variables"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/orgs/my-org/actions/variables/MY_VAR"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&single_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/orgs/my-org/actions/variables"))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;

    Mock::given(method("PATCH"))
        .and(path("/orgs/my-org/actions/variables/MY_VAR"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/orgs/my-org/actions/variables/MY_VAR"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs("my-org");
    let org_vars = org.variables();

    let list = org_vars.list().await.unwrap();
    assert_eq!(list.total_count, 1);
    assert_eq!(list.variables[0].name, "MY_VAR");

    let var = org_vars.get("MY_VAR").await.unwrap();
    assert_eq!(var.name, "MY_VAR");

    assert!(org_vars
        .create("MY_VAR", "my_val", Visibility::All, None)
        .await
        .is_ok());

    assert!(org_vars
        .update("MY_VAR", None::<&str>, Some("new_val"), None, None)
        .await
        .is_ok());

    assert!(org_vars.delete("MY_VAR").await.is_ok());
}

#[tokio::test]
async fn test_org_variable_selected_repositories() {
    let mock_server = MockServer::start().await;
    let repos_json = json!({
        "total_count": 0,
        "repositories": []
    });

    Mock::given(method("GET"))
        .and(path("/orgs/my-org/actions/variables/MY_VAR/repositories"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&repos_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/orgs/my-org/actions/variables/MY_VAR/repositories"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path(
            "/orgs/my-org/actions/variables/MY_VAR/repositories/99",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path(
            "/orgs/my-org/actions/variables/MY_VAR/repositories/99",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs("my-org");
    let vars = org.variables();

    let list = vars.list_selected_repositories("MY_VAR").await.unwrap();
    assert_eq!(list.total_count, 0);

    assert!(vars
        .set_selected_repositories("MY_VAR", &[99u64.into()])
        .await
        .is_ok());
    assert!(vars
        .add_selected_repository("MY_VAR", 99u64.into())
        .await
        .is_ok());
    assert!(vars
        .remove_selected_repository("MY_VAR", 99u64.into())
        .await
        .is_ok());
}

#[tokio::test]
async fn test_org_secret_selected_repositories() {
    let mock_server = MockServer::start().await;
    let repos_json = json!({
        "total_count": 0,
        "repositories": []
    });

    Mock::given(method("GET"))
        .and(path("/orgs/my-org/actions/secrets/MY_SECRET/repositories"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&repos_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/orgs/my-org/actions/secrets/MY_SECRET/repositories"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path(
            "/orgs/my-org/actions/secrets/MY_SECRET/repositories/88",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path(
            "/orgs/my-org/actions/secrets/MY_SECRET/repositories/88",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs("my-org");
    let secrets = org.secrets();

    let list = secrets
        .list_selected_repositories("MY_SECRET")
        .await
        .unwrap();
    assert_eq!(list.total_count, 0);

    assert!(secrets
        .set_selected_repositories("MY_SECRET", &[88u64.into()])
        .await
        .is_ok());
    assert!(secrets
        .add_selected_repository("MY_SECRET", 88u64.into())
        .await
        .is_ok());
    assert!(secrets
        .remove_selected_repository("MY_SECRET", 88u64.into())
        .await
        .is_ok());
}

#[tokio::test]
async fn test_repo_org_variables_and_secrets() {
    let mock_server = MockServer::start().await;
    let vars_json = json!({
        "total_count": 1,
        "variables": [
            {
                "name": "ORG_VAR",
                "value": "val",
                "created_at": "2023-01-01T00:00:00Z",
                "updated_at": "2023-01-01T00:00:00Z"
            }
        ]
    });
    let secrets_json = json!({
        "total_count": 1,
        "secrets": [
            {
                "name": "ORG_SECRET",
                "created_at": "2023-01-01T00:00:00Z",
                "updated_at": "2023-01-01T00:00:00Z"
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/actions/organization-variables"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&vars_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/repos/owner/repo/actions/organization-secrets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&secrets_json))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let vars = client
        .repos("owner", "repo")
        .variables()
        .list_org_variables()
        .await
        .unwrap();
    assert_eq!(vars.total_count, 1);
    assert_eq!(vars.variables[0].name, "ORG_VAR");

    let secrets = client
        .repos("owner", "repo")
        .secrets()
        .list_org_secrets()
        .await
        .unwrap();
    assert_eq!(secrets.total_count, 1);
    assert_eq!(secrets.secrets[0].name, "ORG_SECRET");
}
