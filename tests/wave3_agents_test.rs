use octocrab::Octocrab;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_agent_automations_and_tasks() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 1. List user automations
    Mock::given(method("GET"))
        .and(path("/agents/automations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": "auto_1",
                "name": "Auto Issue Triage",
                "enabled": true
            }
        ])))
        .mount(&mock_server)
        .await;

    let user_autos = octocrab.agents().automations().list().await.unwrap();
    assert_eq!(user_autos.len(), 1);
    assert_eq!(user_autos[0].name, "Auto Issue Triage");

    // 2. Available tools
    Mock::given(method("GET"))
        .and(path("/agents/automations/available-tools"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": "tool_cat_1",
                "title": "Code Analysis Tools"
            }
        ])))
        .mount(&mock_server)
        .await;

    let tools = octocrab
        .agents()
        .automations()
        .get_available_tools()
        .await
        .unwrap();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].id, "tool_cat_1");

    // 3. Available triggers
    Mock::given(method("GET"))
        .and(path("/agents/automations/available-triggers"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": "trigger_1",
                "title": "On Issue Open"
            }
        ])))
        .mount(&mock_server)
        .await;

    let triggers = octocrab
        .agents()
        .automations()
        .get_available_triggers()
        .await
        .unwrap();
    assert_eq!(triggers.len(), 1);
    assert_eq!(triggers[0].id, "trigger_1");

    // 4. Get automation by ID
    Mock::given(method("GET"))
        .and(path("/agents/automations/auto_1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "auto_1",
            "name": "Auto Issue Triage",
            "enabled": true
        })))
        .mount(&mock_server)
        .await;

    let auto = octocrab.agents().automations().get("auto_1").await.unwrap();
    assert_eq!(auto.name, "Auto Issue Triage");

    // 5. List tasks for automation
    Mock::given(method("GET"))
        .and(path("/agents/automations/auto_1/tasks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "tasks": [
                {
                    "id": "task_1",
                    "status": "completed"
                }
            ],
            "total_count": 1
        })))
        .mount(&mock_server)
        .await;

    let tasks = octocrab
        .agents()
        .automations()
        .list_tasks("auto_1")
        .await
        .unwrap();
    assert_eq!(tasks.tasks.len(), 1);

    // 6. Global tasks list & get
    Mock::given(method("GET"))
        .and(path("/agents/tasks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "tasks": [
                {
                    "id": "task_1",
                    "status": "completed"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let gtasks = octocrab.agents().tasks().list().await.unwrap();
    assert_eq!(gtasks.tasks.len(), 1);

    Mock::given(method("GET"))
        .and(path("/agents/tasks/task_1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "task_1",
            "status": "completed"
        })))
        .mount(&mock_server)
        .await;

    let gtask = octocrab.agents().tasks().get("task_1").await.unwrap();
    assert_eq!(gtask.status.as_deref(), Some("completed"));

    // 7. Repo automations: list, create, get, update, delete
    Mock::given(method("GET"))
        .and(path("/agents/repos/octocat/hello-world/automations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "automations": [
                {
                    "id": "auto_repo_1",
                    "name": "Repo Triage"
                }
            ],
            "total_count": 1
        })))
        .mount(&mock_server)
        .await;

    let repo_autos = octocrab
        .repos("octocat", "hello-world")
        .agents()
        .automations()
        .list()
        .await
        .unwrap();
    assert_eq!(repo_autos.automations.len(), 1);

    Mock::given(method("POST"))
        .and(path("/agents/repos/octocat/hello-world/automations"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": "auto_repo_2",
            "name": "New Auto"
        })))
        .mount(&mock_server)
        .await;

    let created_auto = octocrab
        .repos("octocat", "hello-world")
        .agents()
        .automations()
        .create(&json!({ "name": "New Auto" }))
        .await
        .unwrap();
    assert_eq!(created_auto.name, "New Auto");

    Mock::given(method("GET"))
        .and(path(
            "/agents/repos/octocat/hello-world/automations/auto_repo_2",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "auto_repo_2",
            "name": "New Auto"
        })))
        .mount(&mock_server)
        .await;

    let fetched_auto = octocrab
        .repos("octocat", "hello-world")
        .agents()
        .automations()
        .get("auto_repo_2")
        .await
        .unwrap();
    assert_eq!(fetched_auto.name, "New Auto");

    Mock::given(method("PATCH"))
        .and(path(
            "/agents/repos/octocat/hello-world/automations/auto_repo_2",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "auto_repo_2",
            "name": "Updated Auto"
        })))
        .mount(&mock_server)
        .await;

    let updated_auto = octocrab
        .repos("octocat", "hello-world")
        .agents()
        .automations()
        .update("auto_repo_2", &json!({ "name": "Updated Auto" }))
        .await
        .unwrap();
    assert_eq!(updated_auto.name, "Updated Auto");

    Mock::given(method("DELETE"))
        .and(path(
            "/agents/repos/octocat/hello-world/automations/auto_repo_2",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .repos("octocat", "hello-world")
        .agents()
        .automations()
        .delete("auto_repo_2")
        .await
        .unwrap();

    // 8. Repo tasks: list, start, get
    Mock::given(method("GET"))
        .and(path("/agents/repos/octocat/hello-world/tasks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "tasks": [
                {
                    "id": "rtask_1",
                    "status": "pending"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let rtasks = octocrab
        .repos("octocat", "hello-world")
        .agents()
        .tasks()
        .list()
        .await
        .unwrap();
    assert_eq!(rtasks.tasks.len(), 1);

    Mock::given(method("POST"))
        .and(path("/agents/repos/octocat/hello-world/tasks"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": "rtask_2",
            "status": "started"
        })))
        .mount(&mock_server)
        .await;

    let started_task = octocrab
        .repos("octocat", "hello-world")
        .agents()
        .tasks()
        .start(&json!({ "task": "run" }))
        .await
        .unwrap();
    assert_eq!(started_task.status.as_deref(), Some("started"));

    Mock::given(method("GET"))
        .and(path("/agents/repos/octocat/hello-world/tasks/rtask_2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "rtask_2",
            "status": "started"
        })))
        .mount(&mock_server)
        .await;

    let got_rtask = octocrab
        .repos("octocat", "hello-world")
        .agents()
        .tasks()
        .get("rtask_2")
        .await
        .unwrap();
    assert_eq!(got_rtask.status.as_deref(), Some("started"));
}

#[tokio::test]
async fn test_agent_secrets_and_variables() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 1. Repo secrets & org secrets
    Mock::given(method("GET"))
        .and(path(
            "/repos/octocat/hello-world/agents/organization-secrets",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "secrets": [
                {
                    "name": "ORG_SECRET_1",
                    "created_at": "2026-01-01T00:00:00Z",
                    "updated_at": "2026-01-01T00:00:00Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let org_secrets = octocrab
        .repos("octocat", "hello-world")
        .agents()
        .secrets()
        .list_org_secrets()
        .await
        .unwrap();
    assert_eq!(org_secrets.total_count, 1);
    assert_eq!(org_secrets.secrets[0].name, "ORG_SECRET_1");

    Mock::given(method("GET"))
        .and(path("/repos/octocat/hello-world/agents/secrets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "secrets": [
                {
                    "name": "REPO_SECRET_1",
                    "created_at": "2026-01-01T00:00:00Z",
                    "updated_at": "2026-01-01T00:00:00Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let rsecrets = octocrab
        .repos("octocat", "hello-world")
        .agents()
        .secrets()
        .list()
        .await
        .unwrap();
    assert_eq!(rsecrets.total_count, 1);

    Mock::given(method("GET"))
        .and(path("/repos/octocat/hello-world/agents/secrets/public-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "key_id": "key_123",
            "key": "base64key"
        })))
        .mount(&mock_server)
        .await;

    let pkey = octocrab
        .repos("octocat", "hello-world")
        .agents()
        .secrets()
        .get_public_key()
        .await
        .unwrap();
    assert_eq!(pkey.key_id, "key_123");

    Mock::given(method("PUT"))
        .and(path(
            "/repos/octocat/hello-world/agents/secrets/REPO_SECRET_1",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .repos("octocat", "hello-world")
        .agents()
        .secrets()
        .create_or_update("REPO_SECRET_1", &json!({ "encrypted_value": "xyz" }))
        .await
        .unwrap();

    Mock::given(method("DELETE"))
        .and(path(
            "/repos/octocat/hello-world/agents/secrets/REPO_SECRET_1",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .repos("octocat", "hello-world")
        .agents()
        .secrets()
        .delete("REPO_SECRET_1")
        .await
        .unwrap();

    // 2. Org variables
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/agents/variables"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 1,
            "variables": [
                {
                    "name": "MY_VAR",
                    "value": "123",
                    "created_at": "2026-01-01T00:00:00Z",
                    "updated_at": "2026-01-01T00:00:00Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let org_vars = octocrab
        .orgs("test-org")
        .agents()
        .variables()
        .list()
        .await
        .unwrap();
    assert_eq!(org_vars.total_count, 1);
    assert_eq!(org_vars.variables[0].value, "123");

    Mock::given(method("POST"))
        .and(path("/orgs/test-org/agents/variables"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "name": "NEW_VAR",
            "value": "456",
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-01T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let created_var = octocrab
        .orgs("test-org")
        .agents()
        .variables()
        .create(&json!({ "name": "NEW_VAR", "value": "456" }))
        .await
        .unwrap();
    assert_eq!(created_var.name, "NEW_VAR");

    Mock::given(method("PUT"))
        .and(path(
            "/orgs/test-org/agents/variables/NEW_VAR/repositories/999",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .orgs("test-org")
        .agents()
        .variables()
        .add_repository("NEW_VAR", 999)
        .await
        .unwrap();

    Mock::given(method("DELETE"))
        .and(path(
            "/orgs/test-org/agents/variables/NEW_VAR/repositories/999",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .orgs("test-org")
        .agents()
        .variables()
        .remove_repository("NEW_VAR", 999)
        .await
        .unwrap();
}
