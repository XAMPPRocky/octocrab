use octocrab::{
    models::actions::{CreateEnvironmentSecret, CreateEnvironmentSecretResponse},
    Octocrab,
};
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_environment_secrets() {
    let mock_server = MockServer::start().await;
    let pubkey_json = json!({
        "key_id": "key123",
        "key": "base64-pub-key"
    });
    let list_json = json!({
        "total_count": 1,
        "secrets": [
            {
                "name": "ENV_SECRET",
                "created_at": "2023-01-01T00:00:00Z",
                "updated_at": "2023-01-01T00:00:00Z"
            }
        ]
    });
    let secret_json = json!({
        "name": "ENV_SECRET",
        "created_at": "2023-01-01T00:00:00Z",
        "updated_at": "2023-01-01T00:00:00Z"
    });

    Mock::given(method("GET"))
        .and(path(
            "/repositories/555/environments/production/secrets/public-key",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(&pubkey_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/repositories/555/environments/production/secrets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path(
            "/repositories/555/environments/production/secrets/ENV_SECRET",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(&secret_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path(
            "/repositories/555/environments/production/secrets/ENV_SECRET",
        ))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path(
            "/repositories/555/environments/production/secrets/ENV_SECRET",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let env_secrets = client
        .actions()
        .environment_secrets(555u64.into(), "production");

    let key = env_secrets.get_public_key().await.unwrap();
    assert_eq!(key.key_id, "key123");

    let list = env_secrets.list().await.unwrap();
    assert_eq!(list.total_count, 1);
    assert_eq!(list.secrets[0].name, "ENV_SECRET");

    let secret = env_secrets.get("ENV_SECRET").await.unwrap();
    assert_eq!(secret.name, "ENV_SECRET");

    let res = env_secrets
        .create_or_update(
            "ENV_SECRET",
            &CreateEnvironmentSecret {
                encrypted_value: "enc123",
                key_id: "key123",
            },
        )
        .await
        .unwrap();
    assert_eq!(res, CreateEnvironmentSecretResponse::Created);

    assert!(env_secrets.delete("ENV_SECRET").await.is_ok());
}

#[tokio::test]
async fn test_environment_variables() {
    let mock_server = MockServer::start().await;
    let list_json = json!({
        "total_count": 1,
        "variables": [
            {
                "name": "ENV_VAR",
                "value": "var_val",
                "created_at": "2023-01-01T00:00:00Z",
                "updated_at": "2023-01-01T00:00:00Z"
            }
        ]
    });
    let var_json = json!({
        "name": "ENV_VAR",
        "value": "var_val",
        "created_at": "2023-01-01T00:00:00Z",
        "updated_at": "2023-01-01T00:00:00Z"
    });

    Mock::given(method("GET"))
        .and(path("/repositories/555/environments/production/variables"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path(
            "/repositories/555/environments/production/variables/ENV_VAR",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(&var_json))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/repositories/555/environments/production/variables"))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;

    Mock::given(method("PATCH"))
        .and(path(
            "/repositories/555/environments/production/variables/ENV_VAR",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path(
            "/repositories/555/environments/production/variables/ENV_VAR",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = setup_octocrab(&mock_server.uri());
    let env_vars = client
        .actions()
        .environment_variables(555u64.into(), "production");

    let list = env_vars.list().await.unwrap();
    assert_eq!(list.total_count, 1);
    assert_eq!(list.variables[0].name, "ENV_VAR");

    let var = env_vars.get("ENV_VAR").await.unwrap();
    assert_eq!(var.name, "ENV_VAR");

    assert!(env_vars.create("ENV_VAR", "var_val").await.is_ok());
    assert!(env_vars
        .update("ENV_VAR", None::<&str>, Some("new_val"))
        .await
        .is_ok());
    assert!(env_vars.delete("ENV_VAR").await.is_ok());
}
