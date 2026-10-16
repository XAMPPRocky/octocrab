use octocrab::Octocrab;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn test_org_projects_v2_endpoints() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 1. List org projects
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/projectsV2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": 101,
                "node_id": "PVT_kwDOA101",
                "title": "Roadmap 2026",
                "number": 1
            }
        ])))
        .mount(&mock_server)
        .await;

    let projects = octocrab
        .orgs("test-org")
        .projects_v2()
        .list()
        .send()
        .await
        .unwrap();
    assert_eq!(projects.items.len(), 1);
    assert_eq!(projects.items[0].id, 101);
    assert_eq!(projects.items[0].title, "Roadmap 2026");

    // 2. Get org project
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/projectsV2/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 101,
            "node_id": "PVT_kwDOA101",
            "title": "Roadmap 2026",
            "number": 1
        })))
        .mount(&mock_server)
        .await;

    let project = octocrab.projects_v2().org("test-org").get(1).await.unwrap();
    assert_eq!(project.id, 101);

    // 3. Create draft item
    Mock::given(method("POST"))
        .and(path("/orgs/test-org/projectsV2/1/drafts"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": 201,
            "node_id": "PVTI_201",
            "content_type": "DraftIssue"
        })))
        .mount(&mock_server)
        .await;

    let draft = octocrab
        .orgs("test-org")
        .projects_v2()
        .create_draft(1, "Draft Item Title")
        .body("Draft details")
        .send()
        .await
        .unwrap();
    assert_eq!(draft.id, 201);

    // 4. List fields
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/projectsV2/1/fields"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": 301,
                "node_id": "PVTF_301",
                "name": "Status",
                "data_type": "single_select"
            }
        ])))
        .mount(&mock_server)
        .await;

    let fields = octocrab
        .orgs("test-org")
        .projects_v2()
        .list_fields(1)
        .send()
        .await
        .unwrap();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].name, "Status");

    // 5. Add field
    Mock::given(method("POST"))
        .and(path("/orgs/test-org/projectsV2/1/fields"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": 302,
            "node_id": "PVTF_302",
            "name": "Priority",
            "data_type": "single_select"
        })))
        .mount(&mock_server)
        .await;

    let new_field = octocrab
        .orgs("test-org")
        .projects_v2()
        .add_field(1, "Priority", "single_select")
        .send()
        .await
        .unwrap();
    assert_eq!(new_field.id, 302);

    // 6. Get field
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/projectsV2/1/fields/301"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 301,
            "node_id": "PVTF_301",
            "name": "Status",
            "data_type": "single_select"
        })))
        .mount(&mock_server)
        .await;

    let field = octocrab
        .orgs("test-org")
        .projects_v2()
        .get_field(1, "301")
        .await
        .unwrap();
    assert_eq!(field.id, 301);

    // 7. List items
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/projectsV2/1/items"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": 401,
                "node_id": "PVTI_401"
            }
        ])))
        .mount(&mock_server)
        .await;

    let items = octocrab
        .orgs("test-org")
        .projects_v2()
        .list_items(1)
        .send()
        .await
        .unwrap();
    assert_eq!(items.items.len(), 1);
    assert_eq!(items.items[0].id, 401);

    // 8. Add item (wrapped value)
    Mock::given(method("POST"))
        .and(path("/orgs/test-org/projectsV2/1/items"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "value": {
                "id": 402,
                "node_id": "PVTI_402"
            }
        })))
        .mount(&mock_server)
        .await;

    let item = octocrab
        .orgs("test-org")
        .projects_v2()
        .add_item(1, &json!({ "type": "Issue", "id": 123 }))
        .await
        .unwrap();
    assert_eq!(item.id, 402);

    // 9. Get item
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/projectsV2/1/items/401"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 401,
            "node_id": "PVTI_401"
        })))
        .mount(&mock_server)
        .await;

    let item = octocrab
        .orgs("test-org")
        .projects_v2()
        .get_item(1, 401, None::<&str>)
        .await
        .unwrap();
    assert_eq!(item.id, 401);

    // 10. Update item
    Mock::given(method("PATCH"))
        .and(path("/orgs/test-org/projectsV2/1/items/401"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 401,
            "node_id": "PVTI_401"
        })))
        .mount(&mock_server)
        .await;

    let updated = octocrab
        .orgs("test-org")
        .projects_v2()
        .update_item(1, 401, &json!({ "fields": [] }))
        .await
        .unwrap();
    assert_eq!(updated.id, 401);

    // 11. Delete item
    Mock::given(method("DELETE"))
        .and(path("/orgs/test-org/projectsV2/1/items/401"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .orgs("test-org")
        .projects_v2()
        .delete_item(1, 401)
        .await
        .unwrap();

    // 12. List view items
    Mock::given(method("GET"))
        .and(path("/orgs/test-org/projectsV2/1/views/2/items"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": 403,
                "node_id": "PVTI_403"
            }
        ])))
        .mount(&mock_server)
        .await;

    let view_items = octocrab
        .orgs("test-org")
        .projects_v2()
        .list_view_items(1, 2)
        .send()
        .await
        .unwrap();
    assert_eq!(view_items.items.len(), 1);
    assert_eq!(view_items.items[0].id, 403);

    // 13. Create view
    Mock::given(method("POST"))
        .and(path("/orgs/test-org/projectsV2/1/views"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "value": {
                "id": 501,
                "number": 1,
                "name": "Sprint Board",
                "layout": "board"
            }
        })))
        .mount(&mock_server)
        .await;

    let view = octocrab
        .orgs("test-org")
        .projects_v2()
        .create_view(1, "Sprint Board", "board")
        .send()
        .await
        .unwrap();
    assert_eq!(view.id, 501);
    assert_eq!(view.name, "Sprint Board");
}

#[tokio::test]
async fn test_user_projects_v2_endpoints() {
    let mock_server = MockServer::start().await;
    let octocrab = setup_octocrab(&mock_server.uri());

    // 14. List user projects
    Mock::given(method("GET"))
        .and(path("/users/octocat/projectsV2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": 601,
                "node_id": "PVT_601",
                "title": "Personal Tasks",
                "number": 1
            }
        ])))
        .mount(&mock_server)
        .await;

    let user_projects = octocrab
        .users("octocat")
        .projects_v2()
        .list()
        .send()
        .await
        .unwrap();
    assert_eq!(user_projects.items.len(), 1);
    assert_eq!(user_projects.items[0].id, 601);

    // 15. Get user project
    Mock::given(method("GET"))
        .and(path("/users/octocat/projectsV2/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 601,
            "node_id": "PVT_601",
            "title": "Personal Tasks",
            "number": 1
        })))
        .mount(&mock_server)
        .await;

    let project = octocrab
        .projects()
        .v2()
        .user("octocat")
        .get(1)
        .await
        .unwrap();
    assert_eq!(project.id, 601);

    // 16. Create user draft (uses /user/{id}/projectsV2/...)
    Mock::given(method("POST"))
        .and(path("/user/octocat/projectsV2/1/drafts"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": 701,
            "node_id": "PVTI_701"
        })))
        .mount(&mock_server)
        .await;

    let draft = octocrab
        .users("octocat")
        .projects_v2()
        .create_draft(1, "User Draft")
        .send()
        .await
        .unwrap();
    assert_eq!(draft.id, 701);

    // 17. List user fields
    Mock::given(method("GET"))
        .and(path("/users/octocat/projectsV2/1/fields"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": 801,
                "node_id": "PVTF_801",
                "name": "Estimate",
                "data_type": "number"
            }
        ])))
        .mount(&mock_server)
        .await;

    let fields = octocrab
        .users("octocat")
        .projects_v2()
        .list_fields(1)
        .send()
        .await
        .unwrap();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].id, 801);

    // 18. Add user field
    Mock::given(method("POST"))
        .and(path("/users/octocat/projectsV2/1/fields"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": 802,
            "node_id": "PVTF_802",
            "name": "Story Points",
            "data_type": "number"
        })))
        .mount(&mock_server)
        .await;

    let field = octocrab
        .users("octocat")
        .projects_v2()
        .add_field(1, "Story Points", "number")
        .send()
        .await
        .unwrap();
    assert_eq!(field.id, 802);

    // 19. Get user field
    Mock::given(method("GET"))
        .and(path("/users/octocat/projectsV2/1/fields/801"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 801,
            "node_id": "PVTF_801",
            "name": "Estimate",
            "data_type": "number"
        })))
        .mount(&mock_server)
        .await;

    let field = octocrab
        .users("octocat")
        .projects_v2()
        .get_field(1, "801")
        .await
        .unwrap();
    assert_eq!(field.id, 801);

    // 20. List user items
    Mock::given(method("GET"))
        .and(path("/users/octocat/projectsV2/1/items"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": 901,
                "node_id": "PVTI_901"
            }
        ])))
        .mount(&mock_server)
        .await;

    let items = octocrab
        .users("octocat")
        .projects_v2()
        .list_items(1)
        .send()
        .await
        .unwrap();
    assert_eq!(items.items.len(), 1);

    // 21. Add user item
    Mock::given(method("POST"))
        .and(path("/users/octocat/projectsV2/1/items"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": 902,
            "node_id": "PVTI_902"
        })))
        .mount(&mock_server)
        .await;

    let item = octocrab
        .users("octocat")
        .projects_v2()
        .add_item(1, &json!({ "type": "Issue", "id": 456 }))
        .await
        .unwrap();
    assert_eq!(item.id, 902);

    // 22. Get user item
    Mock::given(method("GET"))
        .and(path("/users/octocat/projectsV2/1/items/901"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 901,
            "node_id": "PVTI_901"
        })))
        .mount(&mock_server)
        .await;

    let item = octocrab
        .users("octocat")
        .projects_v2()
        .get_item(1, 901, None::<&str>)
        .await
        .unwrap();
    assert_eq!(item.id, 901);

    // 23. Update user item
    Mock::given(method("PATCH"))
        .and(path("/users/octocat/projectsV2/1/items/901"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 901,
            "node_id": "PVTI_901"
        })))
        .mount(&mock_server)
        .await;

    let updated = octocrab
        .users("octocat")
        .projects_v2()
        .update_item(1, 901, &json!({ "fields": [] }))
        .await
        .unwrap();
    assert_eq!(updated.id, 901);

    // 24. Delete user item
    Mock::given(method("DELETE"))
        .and(path("/users/octocat/projectsV2/1/items/901"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    octocrab
        .users("octocat")
        .projects_v2()
        .delete_item(1, 901)
        .await
        .unwrap();

    // 25. List user view items
    Mock::given(method("GET"))
        .and(path("/users/octocat/projectsV2/1/views/3/items"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": 903,
                "node_id": "PVTI_903"
            }
        ])))
        .mount(&mock_server)
        .await;

    let view_items = octocrab
        .users("octocat")
        .projects_v2()
        .list_view_items(1, 3)
        .send()
        .await
        .unwrap();
    assert_eq!(view_items.items.len(), 1);

    // 26. Create user view (uses /users/{id}/projectsV2/{project_number}/views)
    Mock::given(method("POST"))
        .and(path("/users/octocat/projectsV2/1/views"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": 1001,
            "number": 2,
            "name": "My Table",
            "layout": "table"
        })))
        .mount(&mock_server)
        .await;

    let view = octocrab
        .users("octocat")
        .projects_v2()
        .create_view(1, "My Table", "table")
        .send()
        .await
        .unwrap();
    assert_eq!(view.id, 1001);
    assert_eq!(view.layout, "table");
}
